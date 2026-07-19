//! Phase 0c — headless ACP round-trip harness (see lib.rs for the
//! guarded-session plumbing shared with `holmes-case`).
//!
//! Exit codes: 0 round-trip complete; 2 usage; 3 guard denial;
//! 4 spawn/protocol failure; 5 timeout; 6 provider error relayed
//! (protocol + guard path complete, but the reply is goose relaying a
//! provider-side error — not a model completion; F-026).

use holmes_smoke::{is_provider_error_relay, AcpSession, SessionError};
use serde_json::json;
use std::path::PathBuf;
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

fn exit_for(e: &SessionError) -> i32 {
    match e {
        SessionError::GuardDenial(_) | SessionError::CredentialDenial(_) => 3,
        SessionError::Spawn(_) | SessionError::Protocol(_) => 4,
        SessionError::Timeout(_) => 5,
    }
}

fn main() {
    let args = parse_args();
    let deadline = Instant::now() + Duration::from_secs(args.timeout_secs);

    let mut session = match AcpSession::start(
        &args.goose,
        &args.provider,
        &args.model,
        args.credential_env.as_deref(),
        args.upstream.clone(),
        deadline,
    ) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("holmes-smoke: {e}");
            std::process::exit(exit_for(&e));
        }
    };

    let prompt_result = session.prompt(&args.prompt, deadline);
    let (post_handshake, post_handshake_denied) = session.post_handshake_l1b();
    let resolved = session.resolved.clone();
    let stderr_path = session.stderr_path.clone();
    let goose_display = args.goose.display().to_string();
    let (frames, events) = session.finish();

    let (response_text, run_err) = match prompt_result {
        Ok(t) => (t, None),
        Err(e) => (String::new(), Some(e)),
    };

    let provider_error_relayed = is_provider_error_relay(&response_text);
    let (verdict, exit_code) = match (&run_err, response_text.is_empty(), post_handshake_denied) {
        (None, false, false) if provider_error_relayed => ("PROVIDER ERROR RELAYED", 6),
        (None, false, false) => ("ROUND-TRIP COMPLETE", 0),
        (None, true, false) => ("PROTOCOL OK BUT EMPTY RESPONSE", 4),
        (None, _, true) => ("POST-HANDSHAKE L1B DENIAL", 3),
        (Some(e), _, _) => {
            eprintln!("holmes-smoke: {e}");
            ("FAILED", exit_for(e))
        }
    };

    let transcript = json!({
        "harness": "holmes-smoke 0.2.0",
        "goose_binary": goose_display,
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
        transcript["response_text"].as_str().unwrap_or("").len(),
        transcript["egress_events"]
            .as_array()
            .map(|a| a.len())
            .unwrap_or(0)
    );
    std::process::exit(exit_code);
}
