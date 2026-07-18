//! AC-DL-1 §4 — tool/MCP egress blocked at the proxy boundary, verified with
//! a planted proxy-honoring test server standing in for an excluded
//! endpoint — plus the loopback leg of the §5 positive control. Hermetic:
//! every socket in this file is 127.0.0.1.
//! This file is an AC-DL-2 scan exemption (names excluded hosts as fixtures).

use holmes_guard::proxy::{Decision, EgressProxy, ProxyConfig};
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Planted proxy-honoring test server: counts every accepted connection and
/// answers with an identifiable payload. If the guard ever lets a denied
/// request through, the counter catches it.
fn planted_server(listener: TcpListener) -> (SocketAddr, Arc<AtomicUsize>) {
    let addr = listener.local_addr().expect("planted server addr");
    let hits = Arc::new(AtomicUsize::new(0));
    let thread_hits = Arc::clone(&hits);
    thread::spawn(move || {
        for conn in listener.incoming() {
            let Ok(mut conn) = conn else { break };
            thread_hits.fetch_add(1, Ordering::SeqCst);
            let _ = conn.set_read_timeout(Some(Duration::from_secs(5)));
            let mut buf = [0u8; 1024];
            let _ = conn.read(&mut buf);
            let _ = conn.write_all(
                b"HTTP/1.1 200 OK\r\nContent-Length: 7\r\nConnection: close\r\n\r\nplanted",
            );
            let _ = conn.shutdown(Shutdown::Both);
        }
    });
    (addr, hits)
}

fn ephemeral_planted_server() -> (SocketAddr, Arc<AtomicUsize>) {
    let listener = TcpListener::bind(("127.0.0.1", 0)).expect("bind planted server");
    planted_server(listener)
}

fn send_and_read(proxy: SocketAddr, request: &[u8]) -> String {
    let mut client = TcpStream::connect(proxy).expect("connect proxy");
    client
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap();
    client.write_all(request).expect("write request");
    let mut response = Vec::new();
    let mut tmp = [0u8; 4096];
    while let Ok(n) = client.read(&mut tmp) {
        if n == 0 {
            break;
        }
        response.extend_from_slice(&tmp[..n]);
        if response.len() > 64 * 1024 {
            break;
        }
    }
    String::from_utf8_lossy(&response).into_owned()
}

#[test]
fn s4_connect_to_planted_excluded_port_is_blocked_at_boundary() {
    let proxy = EgressProxy::spawn(ProxyConfig::default()).expect("spawn proxy");
    let (planted, hits) = ephemeral_planted_server();
    // Loopback on any port other than the permitted Ollama port stands in
    // for an excluded endpoint the tool could actually reach if unguarded.
    let req = format!(
        "CONNECT 127.0.0.1:{} HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
        planted.port(),
        planted.port()
    );
    let response = send_and_read(proxy.addr(), req.as_bytes());
    assert!(response.starts_with("HTTP/1.1 403"), "got: {response}");
    assert!(response.contains("egress denied"), "got: {response}");
    assert_eq!(hits.load(Ordering::SeqCst), 0, "planted server was reached");
    assert!(proxy
        .events()
        .iter()
        .any(|e| e.decision == Decision::Denied && e.port == planted.port()));
    proxy.shutdown();
}

#[test]
fn s4_absolute_form_http_to_planted_excluded_port_is_blocked() {
    let proxy = EgressProxy::spawn(ProxyConfig::default()).expect("spawn proxy");
    let (planted, hits) = ephemeral_planted_server();
    let req = format!(
        "GET http://127.0.0.1:{}/exfil HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
        planted.port(),
        planted.port()
    );
    let response = send_and_read(proxy.addr(), req.as_bytes());
    assert!(response.starts_with("HTTP/1.1 403"), "got: {response}");
    assert_eq!(hits.load(Ordering::SeqCst), 0, "planted server was reached");
    proxy.shutdown();
}

#[test]
fn s4_excluded_vendor_hostname_denied_without_any_network_activity() {
    let proxy = EgressProxy::spawn(ProxyConfig::default()).expect("spawn proxy");
    for target in [
        "api.openai.com:443",
        "api.x.ai:443",
        "graph.facebook.com:443",
    ] {
        let req = format!("CONNECT {target} HTTP/1.1\r\nHost: {target}\r\n\r\n");
        let response = send_and_read(proxy.addr(), req.as_bytes());
        assert!(
            response.starts_with("HTTP/1.1 403"),
            "{target} must deny at the boundary, got: {response}"
        );
    }
    // Denials precede DNS/connect: every recorded event is a denial.
    assert!(proxy
        .events()
        .iter()
        .all(|e| e.decision == Decision::Denied));
    proxy.shutdown();
}

#[test]
fn s4_fail_closed_on_malformed_and_unsupported_forms() {
    let proxy = EgressProxy::spawn(ProxyConfig::default()).expect("spawn proxy");
    for req in [
        "FLORP\r\n\r\n".to_string(),
        "GET / HTTP/1.1\r\nHost: 127.0.0.1:11434\r\n\r\n".to_string(), // origin-form
        "GET https://api.anthropic.com/ HTTP/1.1\r\n\r\n".to_string(), // https absolute-form
        "CONNECT not:a:valid:authority HTTP/1.1\r\n\r\n".to_string(),
        "CONNECT user@127.0.0.1:11434 HTTP/1.1\r\n\r\n".to_string(),
    ] {
        let response = send_and_read(proxy.addr(), req.as_bytes());
        assert!(
            response.starts_with("HTTP/1.1 403"),
            "must fail closed for {req:?}, got: {response}"
        );
    }
    proxy.shutdown();
}

#[test]
fn s5_positive_control_permitted_loopback_tunnel_round_trips() {
    // The one host:port the compiled allowlist permits on loopback is the
    // Ollama default. A planted server there must be reachable through the
    // proxy — proving the denylist has not become a block-everything wall.
    let listener = match TcpListener::bind(("127.0.0.1", 11434)) {
        Ok(l) => l,
        Err(e) => panic!(
            "cannot bind 127.0.0.1:11434 for the positive control (is something else on the Ollama port?): {e}"
        ),
    };
    let (addr, hits) = planted_server(listener);
    assert_eq!(addr.port(), 11434);

    let proxy = EgressProxy::spawn(ProxyConfig::default()).expect("spawn proxy");

    // CONNECT tunnel.
    let mut client = TcpStream::connect(proxy.addr()).expect("connect proxy");
    client
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap();
    client
        .write_all(b"CONNECT 127.0.0.1:11434 HTTP/1.1\r\nHost: 127.0.0.1:11434\r\n\r\n")
        .unwrap();
    let mut head = [0u8; 256];
    let n = client.read(&mut head).expect("read CONNECT reply");
    let reply = String::from_utf8_lossy(&head[..n]).into_owned();
    assert!(reply.contains(" 200 "), "tunnel refused: {reply}");
    client.write_all(b"ping-through-tunnel").unwrap();
    let mut body = Vec::new();
    let mut tmp = [0u8; 1024];
    while let Ok(n) = client.read(&mut tmp) {
        if n == 0 {
            break;
        }
        body.extend_from_slice(&tmp[..n]);
    }
    let body = String::from_utf8_lossy(&body).into_owned();
    assert!(body.contains("planted"), "no round-trip payload: {body}");

    // Absolute-form HTTP to the same permitted pair.
    let response = send_and_read(
        proxy.addr(),
        b"GET http://127.0.0.1:11434/api/tags HTTP/1.1\r\nHost: 127.0.0.1:11434\r\nConnection: close\r\n\r\n",
    );
    assert!(response.contains("planted"), "got: {response}");

    assert!(
        hits.load(Ordering::SeqCst) >= 2,
        "planted server not reached"
    );
    assert!(proxy
        .events()
        .iter()
        .any(|e| e.decision == Decision::Allowed && e.port == 11434));
    proxy.shutdown();
}
