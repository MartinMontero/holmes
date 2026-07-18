//! L1a — deny-by-default local egress proxy (AC-DL-1 §4, as amended by F4).
//!
//! Policy is compiled ([`crate::policy`]); this module is transport only.
//! Supports HTTP/1.1 CONNECT (TLS tunneling) and absolute-form plain HTTP.
//! Any request it cannot positively parse and permit is denied — fail closed.
//!
//! Event records are born-redacted: host names, ports, decisions — never
//! request or response content.
//!
//! Honest residual: a hostile binary that ignores proxy environment variables
//! escapes this library-level boundary; artifact/OS-level enforcement is an
//! Alfred obligation (cross-repo obligations ledger).

use crate::policy;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const MAX_HEAD_BYTES: usize = 16 * 1024;
const IO_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Allowed,
    Denied,
}

/// Born-redacted egress record: names and counts only, never content.
#[derive(Debug, Clone)]
pub struct EgressEvent {
    pub host: String,
    pub port: u16,
    pub decision: Decision,
}

#[derive(Debug, Clone, Default)]
pub struct ProxyConfig {
    /// Optional upstream forward proxy ("host:port") used as *transport* for
    /// non-loopback targets in containerized environments. Never consulted
    /// for policy: the allowlist decision is made on the client's requested
    /// target before any upstream byte is sent.
    pub upstream: Option<String>,
}

pub struct EgressProxy {
    addr: SocketAddr,
    stop: Arc<AtomicBool>,
    events: Arc<Mutex<Vec<EgressEvent>>>,
    accept_thread: Option<thread::JoinHandle<()>>,
}

impl EgressProxy {
    /// Bind 127.0.0.1 on an ephemeral port and start accepting.
    pub fn spawn(cfg: ProxyConfig) -> std::io::Result<Self> {
        let listener = TcpListener::bind(("127.0.0.1", 0))?;
        listener.set_nonblocking(true)?;
        let addr = listener.local_addr()?;
        let stop = Arc::new(AtomicBool::new(false));
        let events = Arc::new(Mutex::new(Vec::new()));
        let thread_stop = Arc::clone(&stop);
        let thread_events = Arc::clone(&events);
        let accept_thread = thread::spawn(move || {
            while !thread_stop.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((conn, _)) => {
                        let conn_events = Arc::clone(&thread_events);
                        let conn_cfg = cfg.clone();
                        thread::spawn(move || handle_conn(conn, conn_cfg, conn_events));
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(20));
                    }
                    Err(_) => break,
                }
            }
        });
        Ok(EgressProxy {
            addr,
            stop,
            events,
            accept_thread: Some(accept_thread),
        })
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Value for HTTP(S)_PROXY in sanitized child environments.
    pub fn proxy_url(&self) -> String {
        format!("http://{}", self.addr)
    }

    pub fn events(&self) -> Vec<EgressEvent> {
        self.events.lock().expect("event lock poisoned").clone()
    }

    pub fn shutdown(mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(h) = self.accept_thread.take() {
            let _ = h.join();
        }
    }
}

impl Drop for EgressProxy {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

fn record(events: &Mutex<Vec<EgressEvent>>, host: &str, port: u16, decision: Decision) {
    events
        .lock()
        .expect("event lock poisoned")
        .push(EgressEvent {
            host: host.to_owned(),
            port,
            decision,
        });
}

fn deny(client: &mut TcpStream, events: &Mutex<Vec<EgressEvent>>, host: &str, port: u16) {
    record(events, host, port, Decision::Denied);
    let body = format!("holmes-guard L1a: egress denied for {host}:{port}\n");
    let _ = client.write_all(
        format!(
            "HTTP/1.1 403 Forbidden\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{body}",
            body.len()
        )
        .as_bytes(),
    );
    let _ = client.shutdown(Shutdown::Both);
}

fn find_head_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n").map(|p| p + 4)
}

/// Split "host:port" / "[v6]:port" / bare host into (host, port).
fn split_host_port(authority: &str, default_port: u16) -> Option<(String, u16)> {
    if authority.is_empty() || authority.contains('@') {
        return None;
    }
    if let Some(rest) = authority.strip_prefix('[') {
        // Bracketed IPv6: [addr] or [addr]:port
        let end = rest.find(']')?;
        let host = &rest[..end];
        let after = &rest[end + 1..];
        let port = match after.strip_prefix(':') {
            Some(p) => p.parse().ok()?,
            None if after.is_empty() => default_port,
            None => return None,
        };
        return Some((host.to_owned(), port));
    }
    match authority.rsplit_once(':') {
        Some((host, port)) if !host.contains(':') => Some((host.to_owned(), port.parse().ok()?)),
        Some(_) => None, // unbracketed IPv6 or multiple colons: refuse to guess
        None => Some((authority.to_owned(), default_port)),
    }
}

fn is_loopback_host(host: &str) -> bool {
    matches!(host, "127.0.0.1" | "localhost" | "::1")
}

fn connect_direct(host: &str, port: u16) -> std::io::Result<TcpStream> {
    let addrs: Vec<SocketAddr> = (host, port).to_socket_addrs()?.collect();
    let mut last_err = None;
    for addr in addrs {
        match TcpStream::connect_timeout(&addr, IO_TIMEOUT) {
            Ok(s) => return Ok(s),
            Err(e) => last_err = Some(e),
        }
    }
    Err(last_err.unwrap_or_else(|| std::io::Error::other("no addresses resolved")))
}

/// Open transport for an *already-permitted* CONNECT target.
fn open_tunnel(cfg: &ProxyConfig, host: &str, port: u16) -> std::io::Result<TcpStream> {
    match (&cfg.upstream, is_loopback_host(host)) {
        (Some(up), false) => {
            let (up_host, up_port) =
                split_host_port(up, 3128).ok_or_else(|| std::io::Error::other("bad upstream"))?;
            let mut s = connect_direct(&up_host, up_port)?;
            s.set_read_timeout(Some(IO_TIMEOUT))?;
            s.write_all(
                format!("CONNECT {host}:{port} HTTP/1.1\r\nHost: {host}:{port}\r\n\r\n").as_bytes(),
            )?;
            let mut buf = Vec::new();
            let mut tmp = [0u8; 1024];
            loop {
                let n = s.read(&mut tmp)?;
                if n == 0 {
                    return Err(std::io::Error::other("upstream closed during CONNECT"));
                }
                buf.extend_from_slice(&tmp[..n]);
                if find_head_end(&buf).is_some() {
                    break;
                }
                if buf.len() > MAX_HEAD_BYTES {
                    return Err(std::io::Error::other("oversized upstream CONNECT reply"));
                }
            }
            let head = String::from_utf8_lossy(&buf);
            let status_ok = head
                .split("\r\n")
                .next()
                .map(|l| l.contains(" 200"))
                .unwrap_or(false);
            if !status_ok {
                return Err(std::io::Error::other("upstream refused CONNECT"));
            }
            Ok(s)
        }
        _ => connect_direct(host, port),
    }
}

fn splice(client: TcpStream, upstream: TcpStream) {
    let mut c_read = match client.try_clone() {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut u_write = match upstream.try_clone() {
        Ok(s) => s,
        Err(_) => return,
    };
    let mut u_read = upstream;
    let mut c_write = client;
    let _ = c_read.set_read_timeout(Some(Duration::from_secs(60)));
    let _ = u_read.set_read_timeout(Some(Duration::from_secs(60)));
    let up = thread::spawn(move || {
        let _ = std::io::copy(&mut c_read, &mut u_write);
        let _ = u_write.shutdown(Shutdown::Write);
    });
    let _ = std::io::copy(&mut u_read, &mut c_write);
    let _ = c_write.shutdown(Shutdown::Write);
    let _ = up.join();
}

fn handle_conn(mut client: TcpStream, cfg: ProxyConfig, events: Arc<Mutex<Vec<EgressEvent>>>) {
    let _ = client.set_read_timeout(Some(IO_TIMEOUT));
    let mut buf: Vec<u8> = Vec::with_capacity(2048);
    let mut tmp = [0u8; 4096];
    let head_end = loop {
        match client.read(&mut tmp) {
            Ok(0) => return,
            Ok(n) => {
                buf.extend_from_slice(&tmp[..n]);
                if let Some(pos) = find_head_end(&buf) {
                    break pos;
                }
                if buf.len() > MAX_HEAD_BYTES {
                    return deny(&mut client, &events, "<oversized-head>", 0);
                }
            }
            Err(_) => return deny(&mut client, &events, "<read-error>", 0),
        }
    };

    let Ok(head) = std::str::from_utf8(&buf[..head_end]) else {
        return deny(&mut client, &events, "<non-utf8-head>", 0);
    };
    let request_line = head.split("\r\n").next().unwrap_or("");
    let mut parts = request_line.split_whitespace();
    let (Some(method), Some(target)) = (parts.next(), parts.next()) else {
        return deny(&mut client, &events, "<malformed-request>", 0);
    };
    // Bytes already read past the head (e.g. an eager TLS ClientHello) must
    // be forwarded before splicing.
    let remainder = buf[head_end..].to_vec();

    if method.eq_ignore_ascii_case("CONNECT") {
        let Some((raw_host, port)) = split_host_port(target, 443) else {
            return deny(&mut client, &events, "<malformed-authority>", 0);
        };
        let host = policy::normalize_host(&raw_host);
        if !policy::host_permitted(&host, port) {
            return deny(&mut client, &events, &host, port);
        }
        record(&events, &host, port, Decision::Allowed);
        match open_tunnel(&cfg, &host, port) {
            Ok(mut upstream) => {
                if client
                    .write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n")
                    .is_err()
                {
                    return;
                }
                if !remainder.is_empty() && upstream.write_all(&remainder).is_err() {
                    return;
                }
                splice(client, upstream);
            }
            Err(_) => {
                let _ = client.write_all(
                    b"HTTP/1.1 502 Bad Gateway\r\nConnection: close\r\nContent-Length: 0\r\n\r\n",
                );
            }
        }
    } else if let Some(rest) = target.strip_prefix("http://") {
        // Absolute-form plain HTTP, the only non-CONNECT form a proxy client
        // legitimately sends.
        let authority = rest.split('/').next().unwrap_or("");
        let Some((raw_host, port)) = split_host_port(authority, 80) else {
            return deny(&mut client, &events, "<malformed-authority>", 0);
        };
        let host = policy::normalize_host(&raw_host);
        if !policy::host_permitted(&host, port) {
            return deny(&mut client, &events, &host, port);
        }
        record(&events, &host, port, Decision::Allowed);
        let transport = match (&cfg.upstream, is_loopback_host(&host)) {
            (Some(up), false) => split_host_port(up, 3128)
                .ok_or_else(|| std::io::Error::other("bad upstream"))
                .and_then(|(h, p)| connect_direct(&h, p)),
            _ => connect_direct(&host, port),
        };
        match transport {
            Ok(mut upstream) => {
                if upstream.write_all(&buf).is_err() {
                    return;
                }
                splice(client, upstream);
            }
            Err(_) => {
                let _ = client.write_all(
                    b"HTTP/1.1 502 Bad Gateway\r\nConnection: close\r\nContent-Length: 0\r\n\r\n",
                );
            }
        }
    } else {
        // Origin-form, https:// absolute-form, or anything else: fail closed.
        deny(&mut client, &events, "<unsupported-form>", 0);
    }
}
