//! Phase 0c — headless ACP round-trip harness.
//!
//! Spawns `goose acp` (newline-delimited JSON-RPC 2.0 over stdio, ACP v1)
//! through the holmes-guard L2 sanitized seam: environment cleared, egress
//! pinned to the L1a proxy, provider/model resolved by L1b before the
//! handshake and re-checked against everything the agent reports after it.
//!
//! BYOK: any credential is read at run time from the operator's environment
//! (`--credential-env KEY`) and injected only through the guard's seam for
//! this one run; the harness hardcodes no vendor, key, or endpoint.
//!
//! Protocol shapes verified against agent-client-protocol 1.0.1 /
//! -schema 1.1.0 (the crates goose @ 8e78960e builds against).
//!
//! Exit codes: 0 round-trip complete; 2 usage; 3 guard denial;
//! 4 spawn/protocol failure; 5 timeout; 6 provider error relayed
//! (protocol + guard path complete, but the reply is goose relaying a
//! provider-side error — not a model completion).

use holmes_guard::proxy::{Decision, EgressProxy, ProxyConfig};
use holmes_guard::resolution;
use holmes_guard::spawn::{sanitized_spawn, CredentialVar, SpawnSpec, PROVIDER_CREDENTIAL_KEYS};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{ChildStdin, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::time::{Duration, Instant};

struct Args {
    goose: PathBuf,
    provider: String,
    model: String,
    credential_env: Option<String>,
    prompt: String,
    transcript: Option<PathBuf>,
    upstream: Option<String>,
    timeout_secs: u64,
}

fn usage() -> ! {
    eprintln!(
        "usage: holmes-smoke --goose /abs/path/to/goose --provider <id> --model <id> \
         [--credential-env KEY] [--prompt TEXT] [--transcript out.json] \
         [--upstream host:port] [--timeout-secs N]"
    );
    std::process::exit(2);
}

fn parse_args() -> Args {
    let mut args = Args {
        goose: PathBuf::new(),
        provider: String::new(),
        model: String::new(),
        credential_env: None,
        prompt: "Reply with the single word: pong".to_owned(),
        transcript: None,
        upstream: None,
        timeout_secs: 180,
    };
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        let mut val = || it.next().unwrap_or_else(|| usage());
        match a.as_str() {
            "--goose" => args.goose = PathBuf::from(val()),
            "--provider" => args.provider = val(),
            "--model" => args.model = val(),
            "--credential-env" => args.credential_env = Some(val()),
            "--prompt" => args.prompt = val(),
            "--transcript" => args.transcript = Some(PathBuf::from(val())),
            "--upstream" => args.upstream = Some(val()),
            "--timeout-secs" => args.timeout_secs = val().parse().unwrap_or_else(|_| usage()),
            _ => usage(),
        }
    }
    if args.goose.as_os_str().is_empty() || args.provider.is_empty() || args.model.is_empty() {
        usage();
    }
    args
}

fn send_request(
    stdin: &mut ChildStdin,
    frames: &mut Vec<Value>,
    id: u64,
    method: &str,
    params: Value,
) -> std::io::Result<()> {
    let msg = json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params});
    frames.push(msg.clone());
    stdin.write_all(msg.to_string().as_bytes())?;
    stdin.write_all(b"\n")?;
    stdin.flush()
}

fn send_error_reply(stdin: &mut ChildStdin, frames: &mut Vec<Value>, id: &Value, message: &str) {
    let msg = json!({"jsonrpc": "2.0", "id": id,
        "error": {"code": -32601, "message": message}});
    frames.push(msg.clone());
    let _ = stdin.write_all(msg.to_string().as_bytes());
    let _ = stdin.write_all(b"\n");
    let _ = stdin.flush();
}

/// Pump incoming frames until the response with `id` arrives. Notifications
/// are accumulated; agent→client requests get a polite method-not-found so
/// the agent never blocks on us.
fn wait_response(
    rx: &Receiver<Value>,
    stdin: &mut ChildStdin,
    frames: &mut Vec<Value>,
    chunks: &mut String,
    id: u64,
    deadline: Instant,
) -> Result<Value, String> {
    loop {
        let now = Instant::now();
        if now >= deadline {
            return Err(format!("timed out waiting for response id {id}"));
        }
        match rx.recv_timeout(deadline - now) {
            Ok(v) => {
                frames.push(v.clone());
                if v.get("method").is_some() {
                    if let Some(req_id) = v.get("id") {
                        // Agent→client request; not needed for the smoke.
                        let req_id = req_id.clone();
                        send_error_reply(
                            stdin,
                            frames,
                            &req_id,
                            "holmes-smoke supports no client methods",
                        );
                    } else if v["method"] == "session/update" {
                        collect_text_chunks(&v["params"]["update"], chunks);
                    }
                    continue;
                }
                if v.get("id").and_then(Value::as_u64) == Some(id) {
                    if let Some(err) = v.get("error") {
                        return Err(format!("agent returned error for id {id}: {err}"));
                    }
                    return Ok(v);
                }
            }
            Err(RecvTimeoutError::Timeout) => {
                return Err(format!("timed out waiting for response id {id}"));
            }
            Err(RecvTimeoutError::Disconnected) => {
                return Err("agent closed stdout before responding".to_owned());
            }
        }
    }
}

fn collect_text_chunks(update: &Value, chunks: &mut String) {
    let kind = update
        .get("sessionUpdate")
        .and_then(Value::as_str)
        .unwrap_or("");
    if kind == "agent_message_chunk" || kind == "agent_thought_chunk" {
        if let Some(text) = update
            .get("content")
            .and_then(|c| c.get("text"))
            .and_then(Value::as_str)
        {
            if kind == "agent_message_chunk" {
                chunks.push_str(text);
            }
        }
    }
}

/// Harvest the agent's *own* post-handshake provider/model from ACP
/// `configOptions` entries (`{"id": "provider"|"model", "currentValue": ..}`
/// — shape observed live from goose 1.43.0 `session/new`).
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

/// Recursively harvest every string sitting under a "model"-ish key so the
/// post-handshake check can validate all model ids the agent reports.
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

fn main() {
    let args = parse_args();

    // L1b, pre-handshake.
    let resolved = match resolution::resolve(&args.provider, &args.model) {
        Ok(r) => r,
        Err(d) => {
            eprintln!("holmes-smoke: pre-handshake L1b denial: {d}");
            std::process::exit(3);
        }
    };

    let proxy = EgressProxy::spawn(ProxyConfig {
        upstream: args.upstream.clone(),
    })
    .expect("spawn L1a proxy");

    let home = std::env::temp_dir().join(format!("holmes-smoke-{}", std::process::id()));
    std::fs::create_dir_all(&home).expect("create isolated home");

    // BYOK: `--credential-env` names the *operator's* variable holding the
    // credential value (platform env stores often reserve the vendor's own
    // key name, so operators store under e.g. MY_<ACCEPTED_KEY>). The value
    // is injected through the L2 seam under the provider's accepted key —
    // and only when the operator's name embeds that accepted key, so a
    // cross-provider credential still refuses here (journey 9), before the
    // seam re-checks the injected key itself.
    let credential = args.credential_env.as_ref().map(|operator_key| {
        let value = std::env::var(operator_key).unwrap_or_else(|_| {
            eprintln!(
                "holmes-smoke: --credential-env {operator_key} is not set in the environment"
            );
            std::process::exit(2);
        });
        let accepted = PROVIDER_CREDENTIAL_KEYS
            .iter()
            .find(|(p, _)| *p == resolved.provider)
            .and_then(|(_, keys)| {
                keys.iter().find(|accepted| {
                    operator_key == **accepted || operator_key.ends_with(&format!("_{accepted}"))
                })
            });
        let Some(accepted_key) = accepted else {
            eprintln!(
                "holmes-smoke: credential denial: operator variable '{operator_key}' does not \
                 name a credential for provider '{}'",
                resolved.provider
            );
            std::process::exit(3);
        };
        CredentialVar {
            key: (*accepted_key).to_owned(),
            value,
        }
    });

    let sanitized = match sanitized_spawn(&SpawnSpec {
        goose_binary: &args.goose,
        provider: &args.provider,
        model: &args.model,
        proxy_addr: proxy.addr(),
        isolated_home: &home,
        credential,
    }) {
        Ok(s) => s,
        Err(d) => {
            eprintln!("holmes-smoke: L2 spawn denial: {d}");
            std::process::exit(3);
        }
    };

    let mut command = sanitized.command;
    let stderr_path = home.join("goose-stderr.log");
    let stderr_file = std::fs::File::create(&stderr_path).expect("stderr log");
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(stderr_file);
    let mut child = match command.spawn() {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "holmes-smoke: failed to spawn {}: {e}",
                args.goose.display()
            );
            std::process::exit(4);
        }
    };
    let mut stdin = child.stdin.take().expect("child stdin");
    let stdout = child.stdout.take().expect("child stdout");

    let (tx, rx) = mpsc::channel::<Value>();
    std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            let Ok(line) = line else { break };
            if line.trim().is_empty() {
                continue;
            }
            match serde_json::from_str::<Value>(&line) {
                Ok(v) => {
                    if tx.send(v).is_err() {
                        break;
                    }
                }
                Err(_) => continue, // non-protocol noise on stdout
            }
        }
    });

    let deadline = Instant::now() + Duration::from_secs(args.timeout_secs);
    let mut frames: Vec<Value> = Vec::new();
    let mut response_text = String::new();

    let run = (|| -> Result<Value, String> {
        send_request(
            &mut stdin,
            &mut frames,
            1,
            "initialize",
            json!({
                "protocolVersion": 1,
                "clientCapabilities": {},
                "clientInfo": {"name": "holmes-smoke", "version": "0.1.0"}
            }),
        )
        .map_err(|e| format!("write initialize: {e}"))?;
        let init = wait_response(
            &rx,
            &mut stdin,
            &mut frames,
            &mut response_text,
            1,
            deadline,
        )?;

        send_request(
            &mut stdin,
            &mut frames,
            2,
            "session/new",
            json!({
                "cwd": home.display().to_string(),
                "mcpServers": []
            }),
        )
        .map_err(|e| format!("write session/new: {e}"))?;
        let new_session = wait_response(
            &rx,
            &mut stdin,
            &mut frames,
            &mut response_text,
            2,
            deadline,
        )?;
        let session_id = new_session["result"]["sessionId"]
            .as_str()
            .ok_or("session/new returned no sessionId")?
            .to_owned();

        send_request(
            &mut stdin,
            &mut frames,
            3,
            "session/prompt",
            json!({
                "sessionId": session_id,
                "prompt": [{"type": "text", "text": args.prompt}]
            }),
        )
        .map_err(|e| format!("write session/prompt: {e}"))?;
        let prompt_resp = wait_response(
            &rx,
            &mut stdin,
            &mut frames,
            &mut response_text,
            3,
            deadline,
        )?;

        Ok(json!({"initialize": init, "prompt": prompt_resp}))
    })();

    // Post-handshake: validate the agent's own reported provider/model pair
    // (ACP configOptions) plus every model id it mentioned anywhere.
    let (mut agent_provider, mut agent_model) = (None, None);
    let mut reported_models = Vec::new();
    for frame in &frames {
        harvest_config_pair(frame, &mut agent_provider, &mut agent_model);
        harvest_model_ids(frame, &mut reported_models);
    }
    reported_models.sort();
    reported_models.dedup();
    let mut post_handshake: Vec<Value> = Vec::new();
    if agent_provider.is_some() || agent_model.is_some() {
        let p = agent_provider.as_deref().unwrap_or(&resolved.provider);
        let m = agent_model.as_deref().unwrap_or(&resolved.model);
        let verdict = resolution::resolve(p, m);
        post_handshake.push(json!({
            "source": "configOptions",
            "provider": p,
            "model": m,
            "l1b": match verdict { Ok(_) => "permitted".to_owned(), Err(d) => format!("DENIED: {d}") },
        }));
    }
    post_handshake.extend(reported_models.iter().map(|m| {
        let verdict = resolution::resolve(&resolved.provider, m);
        json!({"source": "reported-model-id", "model": m,
               "l1b": match verdict { Ok(_) => "permitted".to_owned(), Err(d) => format!("DENIED: {d}") }})
    }));
    let post_handshake_denied = post_handshake.iter().any(|v| {
        v["l1b"]
            .as_str()
            .map(|s| s.starts_with("DENIED"))
            .unwrap_or(false)
    });

    let events: Vec<Value> = proxy
        .events()
        .iter()
        .map(|e| {
            json!({"host": e.host, "port": e.port,
                   "decision": match e.decision { Decision::Allowed => "allowed", Decision::Denied => "denied" }})
        })
        .collect();

    let _ = child.kill();
    let _ = child.wait();

    // goose relays provider-side failures as agent text prefixed
    // "Ran into this error:" (observed live, goose 1.43.0 @ 8e78960e, vs a
    // real 400). Such a reply proves the protocol + guard path but is NOT a
    // model completion — claiming ROUND-TRIP COMPLETE on it would be a
    // fabricated model leg (F-026).
    let provider_error_relayed = response_text
        .trim_start()
        .starts_with("Ran into this error:");
    let (verdict, exit_code) = match (&run, response_text.is_empty(), post_handshake_denied) {
        (Ok(_), false, false) if provider_error_relayed => ("PROVIDER ERROR RELAYED", 6),
        (Ok(_), false, false) => ("ROUND-TRIP COMPLETE", 0),
        (Ok(_), true, false) => ("PROTOCOL OK BUT EMPTY RESPONSE", 4),
        (Ok(_), _, true) => ("POST-HANDSHAKE L1B DENIAL", 3),
        (Err(e), _, _) => {
            eprintln!("holmes-smoke: {e}");
            let code = if e.contains("timed out") { 5 } else { 4 };
            ("FAILED", code)
        }
    };

    let transcript = json!({
        "harness": "holmes-smoke 0.1.0",
        "goose_binary": args.goose.display().to_string(),
        "resolved": {"provider": resolved.provider, "model": resolved.model},
        "pre_handshake_l1b": "permitted",
        "post_handshake_l1b": post_handshake,
        "egress_events": events,
        "frames": frames,
        "response_text": response_text,
        "stderr_log": stderr_path.display().to_string(),
        "verdict": verdict,
    });
    if let Some(path) = &args.transcript {
        std::fs::write(path, serde_json::to_string_pretty(&transcript).unwrap())
            .expect("write transcript");
        println!("transcript written to {}", path.display());
    }
    println!(
        "holmes-smoke: {verdict}; model={}; response_bytes={}; egress_events={}",
        resolved.model,
        response_text.len(),
        events.len()
    );
    std::process::exit(exit_code);
}
