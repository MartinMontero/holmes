//! Shared guarded ACP session plumbing for the harness binaries
//! (`holmes-smoke`, Phase 0c; `holmes-case`, Phase 1b).
//!
//! One `AcpSession` = one `goose acp` child spawned through the full
//! guard path: L1b resolution pre-handshake, L1a egress proxy, L2
//! sanitized spawn with the BYOK credential seam. Frames and egress
//! events are recorded born-redacted (names, counts, structure — never
//! credential values).

use holmes_guard::proxy::{Decision, EgressProxy, ProxyConfig};
use holmes_guard::resolution::{self, ResolvedModel};
use holmes_guard::spawn::{sanitized_spawn, CredentialVar, SpawnSpec, PROVIDER_CREDENTIAL_KEYS};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::time::Instant;

/// Typed session failure; the string carries the precise denial/report.
#[derive(Debug)]
pub enum SessionError {
    GuardDenial(String),
    CredentialDenial(String),
    Spawn(String),
    Protocol(String),
    Timeout(String),
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::GuardDenial(s)
            | SessionError::CredentialDenial(s)
            | SessionError::Spawn(s)
            | SessionError::Protocol(s)
            | SessionError::Timeout(s) => f.write_str(s),
        }
    }
}

impl std::error::Error for SessionError {}

/// Map an operator-named credential variable onto the resolved provider's
/// accepted key (platform env stores often reserve the vendor's own key
/// name). The operator name must equal the accepted key or end with
/// `_<ACCEPTED_KEY>`; anything else refuses — a cross-provider credential
/// never reaches the seam's own re-check.
pub fn map_operator_credential(
    resolved_provider: &str,
    operator_key: &str,
    value: String,
) -> Result<CredentialVar, SessionError> {
    let accepted = PROVIDER_CREDENTIAL_KEYS
        .iter()
        .find(|(p, _)| *p == resolved_provider)
        .and_then(|(_, keys)| {
            keys.iter().find(|accepted| {
                operator_key == **accepted || operator_key.ends_with(&format!("_{accepted}"))
            })
        });
    match accepted {
        Some(key) => Ok(CredentialVar {
            key: (*key).to_owned(),
            value,
        }),
        None => Err(SessionError::CredentialDenial(format!(
            "credential denial: operator variable '{operator_key}' does not name a credential \
             for provider '{resolved_provider}'"
        ))),
    }
}

pub struct AcpSession {
    child: Child,
    stdin: ChildStdin,
    rx: Receiver<Value>,
    proxy: EgressProxy,
    frames: Vec<Value>,
    next_id: u64,
    session_id: Option<String>,
    pub resolved: ResolvedModel,
    pub home: PathBuf,
    pub stderr_path: PathBuf,
}

impl AcpSession {
    /// Resolve, guard, spawn, and complete `initialize` + `session/new`.
    #[allow(clippy::too_many_arguments)]
    pub fn start(
        goose: &Path,
        provider: &str,
        model: &str,
        credential_env: Option<&str>,
        upstream: Option<String>,
        deadline: Instant,
    ) -> Result<Self, SessionError> {
        let resolved = resolution::resolve(provider, model)
            .map_err(|d| SessionError::GuardDenial(format!("pre-handshake L1b denial: {d}")))?;

        let proxy = EgressProxy::spawn(ProxyConfig { upstream })
            .map_err(|e| SessionError::Spawn(format!("spawn L1a proxy: {e}")))?;

        let home = std::env::temp_dir().join(format!("holmes-acp-{}", std::process::id()));
        std::fs::create_dir_all(&home)
            .map_err(|e| SessionError::Spawn(format!("create isolated home: {e}")))?;

        let credential = match credential_env {
            None => None,
            Some(operator_key) => {
                let value = std::env::var(operator_key).map_err(|_| {
                    SessionError::CredentialDenial(format!(
                        "credential variable {operator_key} is not set in the environment"
                    ))
                })?;
                Some(map_operator_credential(
                    &resolved.provider,
                    operator_key,
                    value,
                )?)
            }
        };

        let sanitized = sanitized_spawn(&SpawnSpec {
            goose_binary: goose,
            provider,
            model,
            proxy_addr: proxy.addr(),
            isolated_home: &home,
            credential,
        })
        .map_err(|d| SessionError::GuardDenial(format!("L2 spawn denial: {d}")))?;

        let mut command = sanitized.command;
        let stderr_path = home.join("goose-stderr.log");
        let stderr_file = std::fs::File::create(&stderr_path)
            .map_err(|e| SessionError::Spawn(format!("stderr log: {e}")))?;
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(stderr_file);
        let mut child = command.spawn().map_err(|e| {
            SessionError::Spawn(format!("failed to spawn {}: {e}", goose.display()))
        })?;
        let stdin = child.stdin.take().expect("child stdin");
        let stdout = child.stdout.take().expect("child stdout");

        let (tx, rx) = mpsc::channel::<Value>();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                let Ok(line) = line else { break };
                if line.trim().is_empty() {
                    continue;
                }
                if let Ok(v) = serde_json::from_str::<Value>(&line) {
                    if tx.send(v).is_err() {
                        break;
                    }
                }
            }
        });

        let mut session = Self {
            child,
            stdin,
            rx,
            proxy,
            frames: Vec::new(),
            next_id: 0,
            session_id: None,
            resolved,
            home,
            stderr_path,
        };

        session.request(
            "initialize",
            json!({
                "protocolVersion": 1,
                "clientCapabilities": {},
                "clientInfo": {"name": "holmes-harness", "version": "0.2.0"}
            }),
            deadline,
        )?;
        let new_session = session.request(
            "session/new",
            json!({
                "cwd": session.home.display().to_string(),
                "mcpServers": []
            }),
            deadline,
        )?;
        let sid = new_session["result"]["sessionId"]
            .as_str()
            .ok_or_else(|| SessionError::Protocol("session/new returned no sessionId".into()))?
            .to_owned();
        session.session_id = Some(sid);
        Ok(session)
    }

    fn send(&mut self, msg: Value) -> Result<(), SessionError> {
        self.frames.push(msg.clone());
        self.stdin
            .write_all(msg.to_string().as_bytes())
            .and_then(|_| self.stdin.write_all(b"\n"))
            .and_then(|_| self.stdin.flush())
            .map_err(|e| SessionError::Protocol(format!("write to agent: {e}")))
    }

    /// Send one request and pump frames until its response; agent-side
    /// message chunks seen along the way are appended to `chunks_out`
    /// when supplied.
    fn request_collecting(
        &mut self,
        method: &str,
        params: Value,
        deadline: Instant,
        mut chunks_out: Option<&mut String>,
    ) -> Result<Value, SessionError> {
        self.next_id += 1;
        let id = self.next_id;
        self.send(json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}))?;
        loop {
            let now = Instant::now();
            if now >= deadline {
                return Err(SessionError::Timeout(format!(
                    "timed out waiting for response id {id}"
                )));
            }
            match self.rx.recv_timeout(deadline - now) {
                Ok(v) => {
                    self.frames.push(v.clone());
                    if v.get("method").is_some() {
                        if let Some(req_id) = v.get("id") {
                            let req_id = req_id.clone();
                            let reply = json!({"jsonrpc": "2.0", "id": req_id,
                                "error": {"code": -32601,
                                          "message": "holmes harness supports no client methods"}});
                            let _ = self.send(reply);
                        } else if v["method"] == "session/update" {
                            if let Some(out) = chunks_out.as_deref_mut() {
                                collect_text_chunks(&v["params"]["update"], out);
                            }
                        }
                        continue;
                    }
                    if v.get("id").and_then(Value::as_u64) == Some(id) {
                        if let Some(err) = v.get("error") {
                            return Err(SessionError::Protocol(format!(
                                "agent returned error for id {id}: {err}"
                            )));
                        }
                        return Ok(v);
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    return Err(SessionError::Timeout(format!(
                        "timed out waiting for response id {id}"
                    )));
                }
                Err(RecvTimeoutError::Disconnected) => {
                    return Err(SessionError::Protocol(
                        "agent closed stdout before responding".into(),
                    ));
                }
            }
        }
    }

    pub fn request(
        &mut self,
        method: &str,
        params: Value,
        deadline: Instant,
    ) -> Result<Value, SessionError> {
        self.request_collecting(method, params, deadline, None)
    }

    /// One prompt over the established session; returns the streamed
    /// agent text for *this* prompt.
    pub fn prompt(&mut self, text: &str, deadline: Instant) -> Result<String, SessionError> {
        let sid = self
            .session_id
            .clone()
            .ok_or_else(|| SessionError::Protocol("no established session".into()))?;
        let mut chunks = String::new();
        self.request_collecting(
            "session/prompt",
            json!({
                "sessionId": sid,
                "prompt": [{"type": "text", "text": text}]
            }),
            deadline,
            Some(&mut chunks),
        )?;
        Ok(chunks)
    }

    pub fn frames(&self) -> &[Value] {
        &self.frames
    }

    /// Born-redacted egress events from the L1a proxy.
    pub fn egress_events(&self) -> Vec<Value> {
        self.proxy
            .events()
            .iter()
            .map(|e| {
                json!({"host": e.host, "port": e.port,
                       "decision": match e.decision {
                           Decision::Allowed => "allowed",
                           Decision::Denied => "denied" }})
            })
            .collect()
    }

    /// Harvest the agent's own post-handshake (provider, model) pair and
    /// re-run L1b over it plus every reported model id.
    pub fn post_handshake_l1b(&self) -> (Vec<Value>, bool) {
        let (mut provider, mut model) = (None, None);
        let mut reported = Vec::new();
        for frame in &self.frames {
            harvest_config_pair(frame, &mut provider, &mut model);
            harvest_model_ids(frame, &mut reported);
        }
        reported.sort();
        reported.dedup();
        let mut out = Vec::new();
        if provider.is_some() || model.is_some() {
            let p = provider.as_deref().unwrap_or(&self.resolved.provider);
            let m = model.as_deref().unwrap_or(&self.resolved.model);
            out.push(json!({
                "source": "configOptions", "provider": p, "model": m,
                "l1b": match resolution::resolve(p, m) {
                    Ok(_) => "permitted".to_owned(),
                    Err(d) => format!("DENIED: {d}"),
                },
            }));
        }
        out.extend(reported.iter().map(|m| {
            json!({"source": "reported-model-id", "model": m,
            "l1b": match resolution::resolve(&self.resolved.provider, m) {
                Ok(_) => "permitted".to_owned(),
                Err(d) => format!("DENIED: {d}"),
            }})
        }));
        let denied = out.iter().any(|v| {
            v["l1b"]
                .as_str()
                .map(|s| s.starts_with("DENIED"))
                .unwrap_or(false)
        });
        (out, denied)
    }

    pub fn finish(mut self) -> (Vec<Value>, Vec<Value>) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let events = self.egress_events();
        (std::mem::take(&mut self.frames), events)
    }
}

/// goose relays provider-side failures as agent text with this prefix
/// (observed live, goose 1.43.0 @ 8e78960e). Such text is NOT a model
/// completion (F-026).
pub fn is_provider_error_relay(text: &str) -> bool {
    text.trim_start().starts_with("Ran into this error:")
}

fn collect_text_chunks(update: &Value, chunks: &mut String) {
    let kind = update
        .get("sessionUpdate")
        .and_then(Value::as_str)
        .unwrap_or("");
    if kind == "agent_message_chunk" {
        if let Some(text) = update
            .get("content")
            .and_then(|c| c.get("text"))
            .and_then(Value::as_str)
        {
            chunks.push_str(text);
        }
    }
}

fn harvest_config_pair(v: &Value, provider: &mut Option<String>, model: &mut Option<String>) {
    match v {
        Value::Object(map) => {
            if let (Some(id), Some(current)) = (
                map.get("id").and_then(Value::as_str),
                map.get("currentValue").and_then(Value::as_str),
            ) {
                if id == "provider" {
                    *provider = Some(current.to_owned());
                } else if id == "model" {
                    *model = Some(current.to_owned());
                }
            }
            map.values()
                .for_each(|val| harvest_config_pair(val, provider, model));
        }
        Value::Array(items) => items
            .iter()
            .for_each(|i| harvest_config_pair(i, provider, model)),
        _ => {}
    }
}

fn harvest_model_ids(v: &Value, out: &mut Vec<String>) {
    match v {
        Value::Object(map) => {
            for (k, val) in map {
                let key = k.to_ascii_lowercase();
                if (key == "model"
                    || key == "model_id"
                    || key == "modelid"
                    || key == "current_model")
                    && val.is_string()
                {
                    out.push(val.as_str().unwrap().to_owned());
                }
                harvest_model_ids(val, out);
            }
        }
        Value::Array(items) => items.iter().for_each(|i| harvest_model_ids(i, out)),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_credential_mapping_enforces_provider_intent() {
        assert!(map_operator_credential("anthropic", "ANTHROPIC_API_KEY", "v".into()).is_ok());
        let mapped =
            map_operator_credential("anthropic", "MY_ANTHROPIC_API_KEY", "v".into()).unwrap();
        assert_eq!(mapped.key, "ANTHROPIC_API_KEY");
        // Cross-provider and unrelated names refuse.
        assert!(map_operator_credential("anthropic", "MISTRAL_API_KEY", "v".into()).is_err());
        assert!(map_operator_credential("anthropic", "SOME_RANDOM_VAR", "v".into()).is_err());
    }

    #[test]
    fn provider_error_relay_detection() {
        assert!(is_provider_error_relay("Ran into this error: 400"));
        assert!(!is_provider_error_relay("pong"));
    }
}
