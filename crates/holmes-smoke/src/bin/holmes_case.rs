//! Lock 1b — one full analytical case, end-to-end, on the smoke model.
//!
//! Drives the six-phase case method over a single guarded ACP session:
//! the model side supplies hypothesis brainstorming (la lluvia), ACH cell
//! judgments, one likelihood pair, and the el-diablo challenge — exactly
//! the judgments the recipes describe; the deterministic side (this
//! binary + holmes-core) owns intake, evidence fixtures, all bookkeeping,
//! the ACH verdict, the KAC record, knowability (rule-assigned, never
//! model-assigned), the emission gate, and the handoff.
//!
//! The case is a self-contained fixture (a stalled community-garden
//! permit); every fact lives in the prompts and every provenance entry
//! cites fixture documents — no live records are claimed.
//!
//! Exit codes: 0 case complete + handed off; 2 usage; 3 guard/gate
//! denial; 4 protocol or parse failure; 5 timeout; 6 provider error
//! relayed.

use holmes_core::analysis::{
    AchCell, AnalyticalCase, DiabloPass, EvidenceItem, Hypothesis, IntakeAssessment, KeyAssumption,
    LikelihoodRatio, Probability, SystemicOrIsolated,
};
use holmes_core::{
    BriefOrigin, Confidence, Finding, HandoffChannel, Knowability, LimitsOfThisFinding, Provenance,
    ResearchBrief,
};
use holmes_smoke::{is_provider_error_relay, AcpSession, SessionError};
use serde_json::json;
use std::path::PathBuf;
use std::time::{Duration, Instant};

const QUESTION: &str =
    "why has the fixture community-garden permit (docket 441) stalled for nine months?";

const MATERIALS: &str = "FIXTURE MATERIALS (the only facts in evidence):\n\
    E1: an objection letter from an adjacent property owner, dated 2025-11-04, \
    is in the permit docket [fixture/permit-docket.md section 3].\n\
    E2: a clerk's-office bulletin announces a processing backlog covering \
    March through June 2026 [fixture/clerk-bulletin.md section 2].\n\
    E3: the enforcement log shows no violation or enforcement action against \
    the parcel [fixture/enforcement-log.md section 1].\n\
    The permit application itself was filed 2025-10-01 and last updated \
    2025-11-06 [fixture/permit-docket.md section 1].";

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
        "usage: holmes-case --goose /abs/path/to/goose --provider <id> --model <id> \
         [--credential-env KEY] [--transcript out.json] [--timeout-secs N]"
    );
    std::process::exit(2);
}

fn parse_args() -> Args {
    let mut args = Args {
        goose: PathBuf::new(),
        provider: String::new(),
        model: String::new(),
        credential_env: None,
        transcript: None,
        timeout_secs: 420,
    };
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        let mut val = || it.next().unwrap_or_else(|| usage());
        match a.as_str() {
            "--goose" => args.goose = PathBuf::from(val()),
            "--provider" => args.provider = val(),
            "--model" => args.model = val(),
            "--credential-env" => args.credential_env = Some(val()),
            "--transcript" => args.transcript = Some(PathBuf::from(val())),
            "--timeout-secs" => args.timeout_secs = val().parse().unwrap_or_else(|_| usage()),
            _ => usage(),
        }
    }
    if args.goose.as_os_str().is_empty() || args.provider.is_empty() || args.model.is_empty() {
        usage();
    }
    args
}

fn fail(code: i32, verdict: &str, detail: &str) -> ! {
    eprintln!("holmes-case: {verdict}: {detail}");
    std::process::exit(code);
}

fn exit_for(e: &SessionError) -> i32 {
    match e {
        SessionError::GuardDenial(_) | SessionError::CredentialDenial(_) => 3,
        SessionError::Spawn(_) | SessionError::Protocol(_) => 4,
        SessionError::Timeout(_) => 5,
    }
}

fn model_reply(session: &mut AcpSession, prompt: &str, deadline: Instant) -> String {
    match session.prompt(prompt, deadline) {
        Ok(text) => {
            if is_provider_error_relay(&text) {
                fail(6, "PROVIDER ERROR RELAYED", text.trim());
            }
            text
        }
        Err(e) => fail(exit_for(&e), "SESSION FAILURE", &e.to_string()),
    }
}

fn main() {
    let args = parse_args();
    let deadline = Instant::now() + Duration::from_secs(args.timeout_secs);

    // ---- deterministic: open the case, record intake -------------------
    let brief = ResearchBrief::new(
        QUESTION,
        BriefOrigin::Intent,
        "self-contained fixture; all facts embedded in prompts; no live records",
        Vec::new(),
    )
    .expect("fixture brief");
    let mut case = AnalyticalCase::open(brief).expect("open case");
    case.record_intake(IntakeAssessment {
        someone_harmed: true,
        harm_note: "fixture: community growers lost a season to the stall".into(),
        systemic_or_isolated: SystemicOrIsolated::Isolated,
        can_help_without_more_harm: true,
        assessment_note: "fixture documents only; no private individual targeted".into(),
    })
    .expect("intake");
    case.advance_to_la_lluvia().expect("to la lluvia");

    // ---- guarded session ----------------------------------------------
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

    // ---- Phase 2: La Lluvia (model; per recipes/la-lluvia.yaml) --------
    let lluvia_prompt = format!(
        "You are running the La Lluvia phase of a Holmes case.\n{MATERIALS}\n\
         CASE QUESTION: {QUESTION}\n\
         Generate at least three distinct candidate hypotheses. Do not rank or discard any. \
         Make no factual claims beyond the materials. Reply with one line per hypothesis, \
         exactly in this format and nothing else:\n\
         H: <statement> | PRESENT: <evidence you would expect to find if true> | \
         ABSENT: <evidence you would expect to be absent if true>"
    );
    let lluvia_reply = model_reply(&mut session, &lluvia_prompt, deadline);
    let mut hypothesis_ids = Vec::new();
    let mut hypothesis_statements = Vec::new();
    let lluvia_lines: Vec<&str> = lluvia_reply
        .lines()
        .filter(|l| l.trim_start().starts_with("H:"))
        .collect();
    if lluvia_lines.len() < 3 {
        fail(
            4,
            "PARSE FAILURE",
            &format!(
                "la lluvia returned {} parseable H: lines (need >= 3)",
                lluvia_lines.len()
            ),
        );
    }
    let prior = Probability::new(1.0 / lluvia_lines.len() as f64).expect("uniform prior");
    for line in &lluvia_lines {
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() != 3 {
            fail(
                4,
                "PARSE FAILURE",
                &format!("malformed hypothesis line: {line}"),
            );
        }
        let statement = parts[0]
            .trim_start()
            .trim_start_matches("H:")
            .trim()
            .to_owned();
        let present = parts[1]
            .trim()
            .trim_start_matches("PRESENT:")
            .trim()
            .to_owned();
        let absent = parts[2]
            .trim()
            .trim_start_matches("ABSENT:")
            .trim()
            .to_owned();
        let h = Hypothesis::new(statement.clone(), prior, vec![present], vec![absent])
            .unwrap_or_else(|e| fail(4, "PARSE FAILURE", &e.to_string()));
        let id = case.add_hypothesis(h).expect("add hypothesis");
        hypothesis_ids.push(id);
        hypothesis_statements.push(statement);
    }
    case.advance_to_collection().expect("to collection");

    // ---- Phase 3: Collection (deterministic fixtures) ------------------
    let fixture_evidence = [
        (
            "E1",
            "objection letter dated 2025-11-04 in the docket",
            vec![
                (
                    "fixture/permit-docket.md §3",
                    Some("objection filed 2025-11-04"),
                ),
                ("https://records.fixture.example/permits/441", None),
            ],
        ),
        (
            "E2",
            "clerk backlog bulletin covering 2026-03..2026-06",
            vec![
                ("fixture/clerk-bulletin.md §2", Some("backlog notice")),
                ("https://records.fixture.example/bulletins/2026-03", None),
            ],
        ),
        (
            "E3",
            "no enforcement action on the parcel (absence evidence)",
            vec![
                (
                    "fixture/enforcement-log.md §1",
                    Some("no entries for parcel"),
                ),
                (
                    "https://records.fixture.example/enforcement/parcel-441",
                    None,
                ),
            ],
        ),
    ];
    for (id, desc, prov) in &fixture_evidence {
        let provenance = prov
            .iter()
            .map(|(s, q)| Provenance::new(*s, q.map(str::to_owned)).expect("fixture provenance"))
            .collect();
        case.add_evidence(EvidenceItem {
            id: (*id).into(),
            description: (*desc).into(),
            provenance,
        })
        .expect("add evidence");
    }

    // One likelihood pair from the model for E1 against H1 vs the field.
    let hypothesis_list: String = hypothesis_statements
        .iter()
        .enumerate()
        .map(|(i, s)| format!("H{}: {s}\n", i + 1))
        .collect();
    let lr_prompt = format!(
        "Same case. Hypotheses:\n{hypothesis_list}\
         Considering ONLY evidence E1 (the 2025-11-04 objection letter): estimate the \
         probability of observing E1 if H1 is true, and if H1 is false (some other \
         hypothesis true). Values in (0,1], two decimals. Reply exactly one line:\n\
         LR: P_IF_TRUE=<value> P_IF_FALSE=<value>"
    );
    let lr_reply = model_reply(&mut session, &lr_prompt, deadline);
    let lr_line = lr_reply
        .lines()
        .find(|l| l.trim_start().starts_with("LR:"))
        .unwrap_or_else(|| fail(4, "PARSE FAILURE", &format!("no LR: line in: {lr_reply}")));
    let mut p_true = None;
    let mut p_false = None;
    for token in lr_line.split_whitespace() {
        if let Some(v) = token.strip_prefix("P_IF_TRUE=") {
            p_true = v.trim_end_matches(',').parse::<f64>().ok();
        }
        if let Some(v) = token.strip_prefix("P_IF_FALSE=") {
            p_false = v.trim_end_matches(',').parse::<f64>().ok();
        }
    }
    let (Some(pt), Some(pf)) = (p_true, p_false) else {
        fail(
            4,
            "PARSE FAILURE",
            &format!("unparseable LR line: {lr_line}"),
        );
    };
    let lr = LikelihoodRatio::new(
        Probability::new(pt).unwrap_or_else(|e| fail(4, "PARSE FAILURE", &e.to_string())),
        Probability::new(pf).unwrap_or_else(|e| fail(4, "PARSE FAILURE", &e.to_string())),
    );
    case.apply_lr(hypothesis_ids[0], "E1", lr)
        .expect("apply lr");
    case.advance_to_wall().expect("to wall");

    // ---- Phase 4: The Wall — ACH (model cells), KAC, el diablo ---------
    case.build_ach().expect("build ach");
    let ach_prompt = format!(
        "Same case. Hypotheses:\n{hypothesis_list}\
         Evidence: E1 (objection letter), E2 (backlog bulletin), E3 (no enforcement \
         action). Score every hypothesis-evidence cell as C (consistent), \
         I (inconsistent), or NA (not applicable), seeking disconfirmation. Reply with \
         exactly one line per cell, format:\nACH: H<number> E<number> <C|I|NA>"
    );
    let ach_reply = model_reply(&mut session, &ach_prompt, deadline);
    let mut cells_scored = 0usize;
    for line in ach_reply.lines() {
        let t = line.trim();
        let Some(rest) = t.strip_prefix("ACH:") else {
            continue;
        };
        let toks: Vec<&str> = rest.split_whitespace().collect();
        if toks.len() != 3 {
            fail(4, "PARSE FAILURE", &format!("malformed ACH line: {line}"));
        }
        let h_idx: usize = toks[0]
            .trim_start_matches('H')
            .parse::<usize>()
            .ok()
            .and_then(|n| n.checked_sub(1))
            .unwrap_or_else(|| fail(4, "PARSE FAILURE", &format!("bad hypothesis ref: {line}")));
        let e_id = toks[1].to_owned();
        let cell = match toks[2] {
            "C" => AchCell::Consistent,
            "I" => AchCell::Inconsistent,
            "NA" => AchCell::NotApplicable,
            other => fail(
                4,
                "PARSE FAILURE",
                &format!("bad cell '{other}' in: {line}"),
            ),
        };
        if h_idx >= hypothesis_ids.len() {
            fail(
                4,
                "PARSE FAILURE",
                &format!("hypothesis out of range: {line}"),
            );
        }
        case.score_ach(hypothesis_ids[h_idx], &e_id, cell)
            .unwrap_or_else(|e| fail(4, "PARSE FAILURE", &e.to_string()));
        cells_scored += 1;
    }
    let expected_cells = hypothesis_ids.len() * fixture_evidence.len();
    if cells_scored < expected_cells {
        fail(
            4,
            "PARSE FAILURE",
            &format!("ACH returned {cells_scored} cells; matrix needs {expected_cells}"),
        );
    }

    let a0 = case.kac_mut().add(
        KeyAssumption::new(
            "the fixture docket is the complete record",
            "an undisclosed filing would change the timeline",
            "a document exists outside the three fixture sources",
        )
        .expect("assumption"),
    );
    case.kac_mut()
        .mark_supported(
            a0,
            Provenance::new("fixture/permit-docket.md §1", Some("docket index".into()))
                .expect("provenance"),
        )
        .expect("mark supported");

    let diablo_prompt = format!(
        "You are running the El Diablo pass (devil's advocate) of the same case.\n\
         Hypotheses:\n{hypothesis_list}\
         Attack the CURRENTLY LEADING hypothesis (assume it is H1) as hard as the fixture \
         materials allow. Do not invent facts. Reply in exactly two lines:\n\
         CHALLENGE: <your strongest contrary argument>\n\
         WOULD-DAMAGE: <the single piece of evidence that would most damage H1 if it existed>"
    );
    let diablo_reply = model_reply(&mut session, &diablo_prompt, deadline);
    let challenge = diablo_reply
        .lines()
        .find(|l| l.trim_start().starts_with("CHALLENGE:"))
        .map(|l| l.trim().trim_start_matches("CHALLENGE:").trim().to_owned())
        .unwrap_or_else(|| fail(4, "PARSE FAILURE", "no CHALLENGE: line"));
    let would_damage = diablo_reply
        .lines()
        .find(|l| l.trim_start().starts_with("WOULD-DAMAGE:"))
        .map(|l| {
            l.trim()
                .trim_start_matches("WOULD-DAMAGE:")
                .trim()
                .to_owned()
        })
        .unwrap_or_else(|| fail(4, "PARSE FAILURE", "no WOULD-DAMAGE: line"));
    case.record_diablo(DiabloPass {
        challenge,
        response: format!(
            "recorded for the wall (verdict not edited); named damager carried into the \
             limits statement: {would_damage}"
        ),
    })
    .expect("record diablo");

    case.advance_to_money().expect("to money");
    let verdict = case.ach_verdict().expect("verdict").clone();

    // ---- Phase 5: Following the Money (explicit not-applicable) --------
    case.record_money_note(
        "not applicable: the fixture names no financial actor or flow; recorded \
         explicitly per the phase rule",
    )
    .expect("money note");

    // ---- Phase 6: findings + emission gate + handoff -------------------
    let leader = verdict.ranking[0];
    let leader_statement = &hypothesis_statements[leader.0 .0];
    let finding = Finding::new(
        format!(
            "fixture-case ACH verdict: fewest inconsistencies ({}) for: {leader_statement}{}",
            leader.1,
            if verdict.tie_at_top {
                " [tie at top]"
            } else {
                ""
            }
        ),
        Confidence::new(0.7).expect("confidence"),
        vec![
            Provenance::new(
                "fixture/permit-docket.md §3",
                Some("objection filed".into()),
            )
            .expect("p1"),
            Provenance::new("https://records.fixture.example/permits/441", None).expect("p2"),
        ],
        "2026-07-19",
    )
    .expect("finding");
    case.evidence_pack_mut().expect("pack").add_finding(finding);

    // knowability is RULE-assigned (canon §5: deterministic, never
    // model-inferred): a documented-records fixture domain is HighValidity.
    let knowability = Knowability::HighValidity;
    let limits = LimitsOfThisFinding {
        what_would_change_the_conclusion: vec![format!(
            "el-diablo's named damager, if it existed: {would_damage}"
        )],
        what_could_not_be_checked: vec![
            "nothing outside the three fixture documents was consulted (self-contained case)"
                .into(),
        ],
        where_the_evidence_runs_out: vec![
            "motive behind the objection is outside the fixture record".into(),
        ],
    };

    let emitted = match case.resolve(
        knowability,
        limits,
        HandoffChannel::HumanReviewer,
        "lock 1b fixture case close",
    ) {
        Ok(e) => e,
        Err(e) => fail(3, "EMISSION/PHASE DENIAL", &e.to_string()),
    };

    // ---- transcript -----------------------------------------------------
    let (post_handshake, post_denied) = session.post_handshake_l1b();
    let resolved = session.resolved.clone();
    let (frames, events) = session.finish();
    if post_denied {
        fail(3, "POST-HANDSHAKE L1B DENIAL", "see transcript");
    }

    let transcript = json!({
        "harness": "holmes-case 0.1.0 (lock 1b)",
        "resolved": {"provider": resolved.provider, "model": resolved.model},
        "question": QUESTION,
        "phases": {
            "intake": "accepted (fixture; help-without-more-harm satisfied)",
            "la_lluvia": {
                "raw": lluvia_reply,
                "hypotheses": hypothesis_statements,
            },
            "collection": {
                "evidence": fixture_evidence.iter().map(|(id, d, _)| json!({"id": id, "description": d})).collect::<Vec<_>>(),
                "lr_raw": lr_reply,
                "lr_applied": {"hypothesis": "H1", "evidence": "E1",
                                "log_ratio": lr.log_ratio(),
                                "calibration": "Uncalibrated (gating lands in Phase 2.5)"},
            },
            "the_wall": {
                "ach_raw": ach_reply,
                "ach_ranking": verdict.ranking.iter().map(|(h, n)| json!({"hypothesis": hypothesis_statements[h.0], "inconsistencies": n})).collect::<Vec<_>>(),
                "tie_at_top": verdict.tie_at_top,
                "diagnostic_evidence": verdict.diagnostic_evidence,
                "kac": case.kac().assumptions().iter().map(|a| json!({"statement": a.statement})).collect::<Vec<_>>(),
                "diablo_raw": diablo_reply,
            },
            "following_the_money": case.money_notes(),
            "resolution": {
                "emission": "PASSED (corroboration + knowability + limits)",
                "knowability": "HighValidity (rule-assigned)",
                "handoff": "HumanReviewer (interim block)",
                "pack_findings": emitted.pack().findings().len(),
            },
        },
        "post_handshake_l1b": post_handshake,
        "egress_events": events,
        "frame_count": frames.len(),
        "frames": frames,
        "verdict": "CASE COMPLETE",
    });
    if let Some(path) = &args.transcript {
        std::fs::write(path, serde_json::to_string_pretty(&transcript).unwrap())
            .expect("write transcript");
        println!("transcript written to {}", path.display());
    }
    println!(
        "holmes-case: CASE COMPLETE; hypotheses={}; ach_cells={}; egress_events={}; phase={}",
        hypothesis_statements.len(),
        cells_scored,
        transcript["egress_events"]
            .as_array()
            .map(|a| a.len())
            .unwrap_or(0),
        case.phase()
    );
    std::process::exit(0);
}
