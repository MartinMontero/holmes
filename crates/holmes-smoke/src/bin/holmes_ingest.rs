//! Lock 2c evidence — live ingestion-quality run.
//!
//! Drives one guarded ACP session: the model extracts episodes (candidate
//! facts + verbatim citations) from fixture source text; the deterministic
//! `holmes_wall::ingest` scorer then measures the failure rate — grounded
//! vs ungrounded-citation vs claim-exceeds-citation vs malformed — with no
//! model call in the scoring. Emits a born-redacted transcript.
//!
//! Note on tiers: spec lock 2c wants the *Tier-2 local (Ollama)* model;
//! Ollama's model egress is org-blocked in-container (established Phase 0),
//! so this run uses the Tier-1 smoke model to exercise the
//! extraction→scoring pipeline end-to-end. The Tier-2 failure-rate numbers
//! are environment-gated and carried; the scorer is identical for both.
//!
//! Exit: 0 scored; 2 usage; 3 guard denial; 4 protocol/parse; 5 timeout;
//! 6 provider error relayed.

use holmes_smoke::{is_provider_error_relay, AcpSession, SessionError};
use holmes_wall::ingest::{score_batch, Episode};
use serde_json::json;
use std::path::PathBuf;
use std::time::{Duration, Instant};

const SOURCE: &str = "The community-garden permit application (docket 441) was filed on \
    2025-10-01. An objection letter from an adjacent property owner, dated 2025-11-04, is \
    in the permit docket. A clerk's-office bulletin announced a processing backlog covering \
    March through June 2026. The enforcement log shows no violation or enforcement action \
    against the parcel. The application record was last updated on 2025-11-06.";

struct Args {
    goose: PathBuf,
    provider: String,
    model: String,
    credential_env: Option<String>,
    transcript: Option<PathBuf>,
    timeout_secs: u64,
}

fn usage() -> ! {
    eprintln!(
        "usage: holmes-ingest --goose /abs/goose --provider <id> --model <id> \
         [--credential-env KEY] [--transcript out.json] [--timeout-secs N]"
    );
    std::process::exit(2);
}

fn parse_args() -> Args {
    let mut a = Args {
        goose: PathBuf::new(),
        provider: String::new(),
        model: String::new(),
        credential_env: None,
        transcript: None,
        timeout_secs: 240,
    };
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        let mut val = || it.next().unwrap_or_else(|| usage());
        match arg.as_str() {
            "--goose" => a.goose = PathBuf::from(val()),
            "--provider" => a.provider = val(),
            "--model" => a.model = val(),
            "--credential-env" => a.credential_env = Some(val()),
            "--transcript" => a.transcript = Some(PathBuf::from(val())),
            "--timeout-secs" => a.timeout_secs = val().parse().unwrap_or_else(|_| usage()),
            _ => usage(),
        }
    }
    if a.goose.as_os_str().is_empty() || a.provider.is_empty() || a.model.is_empty() {
        usage();
    }
    a
}

fn fail(code: i32, verdict: &str, detail: &str) -> ! {
    eprintln!("holmes-ingest: {verdict}: {detail}");
    std::process::exit(code);
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
        None,
        deadline,
    ) {
        Ok(s) => s,
        Err(e) => fail(exit_for(&e), "SESSION START FAILURE", &e.to_string()),
    };

    let prompt = format!(
        "Extract every factual claim you can support from the SOURCE below, as episodes. \
         For each, quote the EXACT verbatim substring of the SOURCE that supports it (do not \
         paraphrase the citation), and give the date it concerns. Invent nothing beyond the \
         SOURCE. Reply one line per episode, exactly:\n\
         EP: <claim> || CITE: <verbatim source substring> || DATE: <yyyy-mm-dd>\n\n\
         SOURCE:\n{SOURCE}"
    );
    let reply = match session.prompt(&prompt, deadline) {
        Ok(t) if is_provider_error_relay(&t) => fail(6, "PROVIDER ERROR RELAYED", t.trim()),
        Ok(t) => t,
        Err(e) => fail(exit_for(&e), "SESSION FAILURE", &e.to_string()),
    };

    let mut episodes = Vec::new();
    for line in reply.lines() {
        let t = line.trim();
        let Some(rest) = t.strip_prefix("EP:") else {
            continue;
        };
        let parts: Vec<&str> = rest.split("||").collect();
        if parts.len() != 3 {
            continue;
        }
        let claim = parts[0].trim().to_owned();
        let cite = parts[1]
            .trim()
            .trim_start_matches("CITE:")
            .trim()
            .to_owned();
        let date = parts[2]
            .trim()
            .trim_start_matches("DATE:")
            .trim()
            .to_owned();
        episodes.push(Episode {
            statement: claim,
            cited_span: cite,
            occurred_at: date,
        });
    }
    if episodes.is_empty() {
        fail(
            4,
            "PARSE FAILURE",
            &format!("no EP: lines parsed from: {reply}"),
        );
    }

    let report = score_batch(SOURCE, &episodes);
    let (post_handshake, post_denied) = session.post_handshake_l1b();
    let resolved = session.resolved.clone();
    let (frames, events) = session.finish();
    if post_denied {
        fail(3, "POST-HANDSHAKE L1B DENIAL", "see transcript");
    }

    let transcript = json!({
        "harness": "holmes-ingest 0.1.0 (lock 2c)",
        "tier_note": "Tier-1 smoke model exercising the extraction->scoring pipeline; \
                      Tier-2 (Ollama) failure-rate numbers are environment-gated (model \
                      egress blocked in-container).",
        "resolved": {"provider": resolved.provider, "model": resolved.model},
        "episodes_extracted": episodes.len(),
        "quality": {
            "grounded": report.grounded,
            "malformed": report.malformed,
            "ungrounded_citation": report.ungrounded_citation,
            "claim_exceeds_citation": report.claim_exceeds_citation,
            "grounded_rate": report.grounded_rate(),
            "failure_rate": report.failure_rate(),
        },
        "episodes": episodes.iter().zip(report.verdicts.iter()).map(|(e, v)| json!({
            "statement": e.statement, "cited_span": e.cited_span,
            "occurred_at": e.occurred_at, "verdict": format!("{v:?}"),
        })).collect::<Vec<_>>(),
        "post_handshake_l1b": post_handshake,
        "egress_events": events,
        "frame_count": frames.len(),
        "verdict": "INGESTION SCORED",
    });
    if let Some(path) = &args.transcript {
        std::fs::write(path, serde_json::to_string_pretty(&transcript).unwrap())
            .expect("write transcript");
        println!("transcript written to {}", path.display());
    }
    println!("holmes-ingest: {report}; egress_events={}", events.len());
    std::process::exit(0);
}
