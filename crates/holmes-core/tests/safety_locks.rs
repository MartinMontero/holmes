//! Phase 2.5 lock tests — Safety before surface (loop §6; hard gate).
//!
//! 2.5a: planted indirect-injection fixtures in fetched content fail to
//!       move the planner or exfiltrate, demonstrated end-to-end, plus
//!       the structural raw-bytes firewall.
//! 2.5b: high-confidence emission in a low-knowability fixture is
//!       blocked on the full case path; decline → downgrade → emit.
//! 2.5c: the approval round-trip demonstrated headlessly.
//! 2.5d: anti-doxxing refusals per the adopted definition; the
//!       person-naming threshold; handoff-only as the sole resolution
//!       path.
//! Plus the carried F-029/F-031 forgery shapes as documented-limitation
//! locks (the Phase 2.5 adversarial corpus).

use holmes_core::analysis::{
    AchCell, AnalyticalCase, CasePhase, DiabloPass, EmissionDenial, EvidenceItem, Hypothesis,
    IntakeAssessment, LikelihoodRatio, PhaseError, Probability, SystemicOrIsolated,
};
use holmes_core::safety::reader::QuarantinedReader;
use holmes_core::safety::{
    ApprovalDecision, ApprovalError, ApprovalProtocol, Extraction, ExtractionKind,
    ExtractionRequest, RawCandidate, RejectionReason, SubjectScope, ToolDescriptor,
    UntrustedContent,
};
use holmes_core::{
    BriefOrigin, Confidence, Finding, HandoffChannel, Knowability, LimitsOfThisFinding, Provenance,
    ResearchBrief,
};
use std::path::Path;

fn fixture(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/hostile")
        .join(name);
    std::fs::read_to_string(&path).expect("hostile fixture on disk")
}

fn brief() -> ResearchBrief {
    ResearchBrief::new(
        "why has the permit stalled?",
        BriefOrigin::Intent,
        "fixture case",
        Vec::new(),
    )
    .unwrap()
}

fn walk_to_money(case: &mut AnalyticalCase, extra_evidence: Vec<EvidenceItem>) {
    case.record_intake(IntakeAssessment {
        someone_harmed: true,
        harm_note: "stalled community project".into(),
        systemic_or_isolated: SystemicOrIsolated::Isolated,
        can_help_without_more_harm: true,
        assessment_note: "public-records question".into(),
    })
    .unwrap();
    case.advance_to_la_lluvia().unwrap();
    let h0 = case
        .add_hypothesis(
            Hypothesis::new(
                "administrative backlog",
                Probability::new(0.5).unwrap(),
                vec![],
                vec![],
            )
            .unwrap(),
        )
        .unwrap();
    let h1 = case
        .add_hypothesis(
            Hypothesis::new(
                "unresolved objection",
                Probability::new(0.3).unwrap(),
                vec![],
                vec![],
            )
            .unwrap(),
        )
        .unwrap();
    case.advance_to_collection().unwrap();
    case.add_evidence(EvidenceItem {
        id: "E1".into(),
        description: "objection letter on file".into(),
        provenance: vec![Provenance::new(
            "fixture/permit-docket.md §3",
            Some("objection filed".into()),
        )
        .unwrap()],
    })
    .unwrap();
    for item in extra_evidence {
        case.add_evidence(item).unwrap();
    }
    case.apply_lr(
        h1,
        "E1",
        LikelihoodRatio::new(
            Probability::new(0.9).unwrap(),
            Probability::new(0.2).unwrap(),
        ),
    )
    .unwrap();
    case.advance_to_wall().unwrap();
    case.build_ach().unwrap();
    // Score the complete matrix (every live hypothesis × every evidence id).
    let evidence_ids: Vec<String> = case.evidence().iter().map(|e| e.id.clone()).collect();
    for id in &evidence_ids {
        case.score_ach(h0, id, AchCell::Inconsistent).unwrap();
        case.score_ach(h1, id, AchCell::Consistent).unwrap();
    }
    let a = case.kac_mut().add(
        holmes_core::analysis::KeyAssumption::new(
            "the docket is complete",
            "a missing filing would change the timeline",
            "a document exists outside the docket",
        )
        .unwrap(),
    );
    case.kac_mut()
        .mark_supported(
            a,
            Provenance::new("fixture/permit-docket.md §1", Some("docket index".into())).unwrap(),
        )
        .unwrap();
    case.record_diablo(DiabloPass {
        challenge: "is the objection pretextual?".into(),
        response: "it predates the backlog notice".into(),
    })
    .unwrap();
    case.advance_to_money().unwrap();
    case.record_money_note("not applicable: no financial actor in the fixture")
        .unwrap();
}

fn default_limits() -> LimitsOfThisFinding {
    LimitsOfThisFinding {
        what_would_change_the_conclusion: vec!["a newer filing".into()],
        what_could_not_be_checked: vec!["the clerk's internal queue".into()],
        where_the_evidence_runs_out: vec!["motive is outside the record".into()],
    }
}

/// A backend playing a *naively obedient* model: whatever instruction the
/// content contains, it relays as extraction candidates — the worst
/// plausible reader behavior short of a hostile implementation.
fn obedient_backend(raw: &str, _req: &ExtractionRequest) -> Vec<RawCandidate> {
    let mut out = Vec::new();
    for line in raw.lines() {
        let line = line.trim();
        if line.contains("SYSTEM OVERRIDE") || line.contains("grant") {
            out.push(RawCandidate {
                kind: ExtractionKind::Claim,
                text: line.to_owned(),
            });
        }
        if line.contains("2025-11-04") {
            out.push(RawCandidate {
                kind: ExtractionKind::VerbatimQuote,
                text: "The zoning objection was filed on 2025-11-04 by the planning board."
                    .to_owned(),
            });
        }
    }
    out
}

/// Lock 2.5a — the planted instruction fixture fails to move the planner
/// or exfiltrate. The instruction survives (if at all) only as inert,
/// provenanced data; the tool it demands stays deny-by-default refused;
/// the case advances only by typed operator calls; the raw bytes the
/// reader did not extract never reach the emitted pack.
#[test]
fn lock2_5a_hostile_instruction_fails_to_move_the_planner_or_exfiltrate() {
    let raw = fixture("instruction.html");
    let content = UntrustedContent::ingest("https://records.example.gov/dockets/441", raw);
    let request =
        ExtractionRequest::for_kinds(vec![ExtractionKind::Claim, ExtractionKind::VerbatimQuote]);
    let report = QuarantinedReader::extract(&content, &request, &obedient_backend);
    assert!(
        !report.accepted.is_empty(),
        "the naive backend relayed candidates"
    );

    // The relayed instruction and forged grant are inert data with
    // operator-side provenance — never authority.
    let protocol = ApprovalProtocol::new("case-hostile-1");
    for extraction in &report.accepted {
        assert_eq!(
            extraction.origin(),
            "https://records.example.gov/dockets/441"
        );
        // Nothing accepts extraction text as a grant; the demanded tool
        // stays refused before, during, and after reading.
        assert!(matches!(
            protocol.authorize("exfiltrate.send").unwrap_err(),
            ApprovalError::NotGranted { .. }
        ));
    }

    // Drive a full case where the hostile quote enters as evidence.
    let quote: &Extraction = report
        .accepted
        .iter()
        .find(|e| e.kind() == ExtractionKind::VerbatimQuote)
        .expect("the legitimate quote was extracted");
    let mut case = AnalyticalCase::open(brief()).unwrap();
    walk_to_money(
        &mut case,
        vec![EvidenceItem {
            id: "E2".into(),
            description: "fetched docket page (quarantined read)".into(),
            provenance: vec![quote.to_provenance()],
        }],
    );
    case.evidence_pack_mut().unwrap().add_finding(
        Finding::new(
            "the stall traces to the 2025-11-04 objection",
            Confidence::new(0.7).unwrap(),
            vec![
                Provenance::new(
                    "fixture/permit-docket.md §3",
                    Some("objection filed".into()),
                )
                .unwrap(),
                quote.to_provenance(),
            ],
            "2026-07-20",
        )
        .unwrap(),
    );
    let emitted = case
        .resolve(
            Knowability::HighValidity,
            default_limits(),
            None,
            HandoffChannel::HumanReviewer,
            "hostile-fixture case close",
        )
        .unwrap();

    // The content demanded channel "attacker" and confidence 1.0: the
    // handoff went to the human reviewer at 0.7 — the planner moved only
    // on typed operator calls.
    let pack = emitted.pack();
    for finding in pack.findings() {
        assert!(finding.confidence().value() < 0.75);
        for p in finding.provenance() {
            assert!(
                !p.source.contains("attacker"),
                "content-declared identity reached provenance"
            );
        }
    }
    // The unextracted raw-byte canary never leaked into the pack.
    let rendered = format!("{pack:?}");
    assert!(
        !rendered.contains("SECRET-RAW-BYTES-CANARY"),
        "raw bytes escaped the quarantine into the emitted pack"
    );
    // And the tool the instruction demanded is still refused.
    assert!(protocol.authorize("exfiltrate.send").is_err());
}

/// Lock 2.5a — the smuggled-Unicode fixture is neutralized at
/// validation: candidates carrying any recipe-scan smuggling class are
/// rejected (never repaired), and rejection records stay redacted.
#[test]
fn lock2_5a_smuggled_unicode_fixture_is_rejected_not_repaired() {
    let raw = fixture("smuggled.txt");
    let content = UntrustedContent::ingest("https://hostile.example.net/f", raw);
    let request = ExtractionRequest::for_kinds(vec![ExtractionKind::Claim]);
    // Backend relays every non-empty line verbatim.
    let relay = |raw: &str, _req: &ExtractionRequest| {
        raw.lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| RawCandidate {
                kind: ExtractionKind::Claim,
                text: l.to_owned(),
            })
            .collect::<Vec<_>>()
    };
    let report = QuarantinedReader::extract(&content, &request, &relay);
    let smuggle_rejections: Vec<_> = report
        .rejected
        .iter()
        .filter(|r| matches!(r.reason, RejectionReason::SmugglingCharacter { .. }))
        .collect();
    assert!(
        smuggle_rejections.len() >= 3,
        "zero-width, bidi, and tag-block lines must all be rejected; got {}",
        smuggle_rejections.len()
    );
    // Clean lines survive as data.
    assert!(report
        .accepted
        .iter()
        .any(|e| e.text().contains("2026-03-02")));
    // No accepted extraction carries a smuggling character.
    for e in &report.accepted {
        assert!(e
            .text()
            .chars()
            .all(|c| holmes_guard::scan::recipes::smuggling_class(c).is_none()));
    }
}

/// Lock 2.5a structural firewall (hardened after the adversarial pass,
/// F-034): the raw-bytes accessor identifier exists in `safety/reader.rs`
/// and nowhere else across the **whole workspace source** — not just this
/// crate. Combined with the accessor being `pub(crate)` (compiler-
/// enforced: no external crate can call it), a privileged module in any
/// sibling crate cannot even spell the path to hostile bytes. The scan
/// covers every `crates/*/src` tree; test files are excluded (a test may
/// legitimately reference the boundary it is checking).
#[test]
fn lock2_5a_raw_bytes_accessor_is_name_firewalled() {
    // CARGO_MANIFEST_DIR = <workspace>/crates/holmes-core.
    let crates_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates dir");
    let accessor = "expose_raw_to_quarantined_backend";
    let reader_rel = Path::new("holmes-core")
        .join("src")
        .join("safety")
        .join("reader.rs");
    let mut seen_in_reader = false;
    let mut src_roots = Vec::new();
    for entry in std::fs::read_dir(crates_dir).expect("read crates dir") {
        let src = entry.expect("entry").path().join("src");
        if src.is_dir() {
            src_roots.push(src);
        }
    }
    assert!(
        src_roots.len() >= 3,
        "expected the full workspace crate set"
    );
    let mut stack = src_roots;
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d).expect("read src dir") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let source = std::fs::read_to_string(&path).expect("read source");
                if path.ends_with(&reader_rel) {
                    seen_in_reader = source.contains(accessor);
                } else {
                    assert!(
                        !source.contains(accessor),
                        "raw-bytes firewall breach: {} names the accessor",
                        path.display()
                    );
                }
            }
        }
    }
    assert!(seen_in_reader, "the accessor moved; update the firewall");
}

/// Lock 2.5b on the full case path: a confident finding cannot leave an
/// analytical case — the core is uncalibrated and says so — and the
/// named downgrade path (cap, supersede, flag) recovers the emission.
#[test]
fn lock2_5b_confident_finding_blocked_then_downgraded_then_emits() {
    let mut case = AnalyticalCase::open(brief()).unwrap();
    walk_to_money(&mut case, Vec::new());
    case.evidence_pack_mut().unwrap().add_finding(
        Finding::new(
            "high-confidence claim in a fundamentally uncertain domain",
            Confidence::new(0.92).unwrap(),
            vec![
                Provenance::new("https://a.example.org/1", None).unwrap(),
                Provenance::new("https://b.example.net/2", None).unwrap(),
            ],
            "2026-07-20",
        )
        .unwrap(),
    );
    // Blocked: the lock-2.5b fixture (low knowability, high confidence).
    let denial = case
        .resolve(
            Knowability::LowValidity,
            default_limits(),
            None,
            HandoffChannel::HumanReviewer,
            "fixture close",
        )
        .unwrap_err();
    assert!(
        matches!(
            denial,
            PhaseError::Emission(EmissionDenial::UncalibratedConfidence { confidence, .. })
                if confidence == 0.92
        ),
        "got: {denial:?}"
    );
    assert_ne!(*case.phase(), CasePhase::ResolutionHandoff);

    // The named downgrade path: cap the finding, then resolve cleanly.
    let downgraded =
        holmes_core::analysis::downgrade_uncalibrated(case.evidence_pack_mut().unwrap());
    assert_eq!(downgraded, 1);
    let emitted = case
        .resolve(
            Knowability::LowValidity,
            default_limits(),
            Some("irreducible uncertainty: sparse feedback in this domain".into()),
            HandoffChannel::HumanReviewer,
            "fixture close after downgrade",
        )
        .unwrap();
    let pack = emitted.pack();
    // The original stays, superseded; the cap and the flag are visible.
    assert!(pack.findings().iter().any(|f| !f.is_current()));
    assert!(pack
        .risk_flags
        .iter()
        .any(|r| r.contains("calibration downgrade")));
    assert!(pack
        .current_findings()
        .all(|f| f.confidence().value() < 0.75));
}

/// Lock 2.5c — the approval round-trip, headless: preview → decision →
/// grants; deny and unanswered paths stay blocked; the log is
/// born-redacted.
#[test]
fn lock2_5c_approval_round_trip_headless() {
    let mut protocol = ApprovalProtocol::new("case-approval-1");
    let tools = vec![
        ToolDescriptor::new("registry.search", "query the corporate registry").unwrap(),
        ToolDescriptor::new("court-records.fetch", "fetch docket entries").unwrap(),
    ];
    let id = protocol.request(tools, "2026-07-20T00:00:00Z").unwrap();

    // Unanswered blocks — this is the blocking behavior, headless.
    assert!(protocol.authorize("registry.search").is_err());

    // The operator sees the whole set before anything fires.
    let preview = protocol.requests()[0].preview();
    assert!(preview.contains("registry.search"));
    assert!(preview.contains("court-records.fetch"));
    assert!(preview.contains("approve to grant exactly this set"));

    // Approve → exactly the previewed set is granted.
    let minted = protocol
        .record_decision(id, ApprovalDecision::Approved, "2026-07-20T00:01:00Z")
        .unwrap();
    assert_eq!(minted, 2);
    assert!(protocol.authorize("registry.search").is_ok());
    assert!(protocol.authorize("court-records.fetch").is_ok());
    // Anything outside the previewed set stays refused.
    assert!(protocol.authorize("shell.exec").is_err());

    // A second case: denied → nothing fires, decision immutable.
    let mut denied = ApprovalProtocol::new("case-approval-2");
    let id2 = denied
        .request(
            vec![ToolDescriptor::new("registry.search", "same").unwrap()],
            "t0",
        )
        .unwrap();
    denied
        .record_decision(id2, ApprovalDecision::Denied, "t1")
        .unwrap();
    assert!(denied.authorize("registry.search").is_err());
    assert_eq!(
        denied
            .record_decision(id2, ApprovalDecision::Approved, "t2")
            .unwrap_err(),
        ApprovalError::AlreadyDecided(id2)
    );

    // Born-redacted: names, decisions, timestamps — nothing else.
    for entry in protocol.log() {
        let rendered = format!("{entry:?}");
        assert!(
            !rendered.contains("corporate registry"),
            "log leaked purpose text"
        );
    }
    assert_eq!(protocol.log().len(), 2);
}

/// Lock 2.5d — registering a private individual as an investigation
/// subject is refused permanently AND declines the case terminally.
#[test]
fn lock2_5d_private_individual_target_declines_the_case() {
    let mut case = AnalyticalCase::open(brief()).unwrap();
    let err = case
        .register_subject(SubjectScope::PrivateIndividual {
            descriptor: "a neighbor named in a complaint".into(),
        })
        .unwrap_err();
    assert!(matches!(err, PhaseError::AntiDoxxing(_)));
    assert_eq!(*case.phase(), CasePhase::Declined);
    assert!(case.declined_reason().unwrap().contains("no override"));
    // Terminal: nothing continues after the refusal.
    assert_eq!(
        case.advance_to_la_lluvia().unwrap_err(),
        PhaseError::CaseDeclined
    );
}

/// Lock 2.5d — a power-scoped case that names a real person carries the
/// higher evidence bar at resolution: three independent roots, verbatim
/// quotes everywhere, and the uncertainty statement.
#[test]
fn lock2_5d_person_naming_case_carries_the_evidence_threshold() {
    let quoted = |s: &str| Provenance::new(s, Some("verbatim words".into())).unwrap();
    let mut case = AnalyticalCase::open(brief()).unwrap();
    case.register_subject(SubjectScope::PowerStructure {
        name: "Acme Holdings LLC — registered agent".into(),
        role_note: "conduct in role: signatures on both filings".into(),
    })
    .unwrap();
    walk_to_money(&mut case, Vec::new());
    case.evidence_pack_mut().unwrap().add_finding(
        Finding::new(
            "the registered agent signed both conflicting filings",
            Confidence::new(0.6).unwrap(),
            vec![
                quoted("https://registry.example.gov/e/1"),
                quoted("https://court.example.org/d/2"),
            ],
            "2026-07-20",
        )
        .unwrap(),
    );
    // Two roots meet lock 1a but not the person-naming floor.
    let denial = case
        .resolve(
            Knowability::HighValidity,
            default_limits(),
            Some("provenanced label, not a verdict; contested facts remain contested".into()),
            HandoffChannel::Journalist,
            "person-naming close",
        )
        .unwrap_err();
    assert!(
        matches!(denial, PhaseError::Defamation(_)),
        "got: {denial:?}"
    );
    assert_ne!(*case.phase(), CasePhase::ResolutionHandoff);

    // Third independent quoted root → the same resolution passes.
    {
        let pack = case.evidence_pack_mut().unwrap();
        let replacement = Finding::new(
            "the registered agent signed both conflicting filings",
            Confidence::new(0.6).unwrap(),
            vec![
                quoted("https://registry.example.gov/e/1"),
                quoted("https://court.example.org/d/2"),
                quoted("https://gazette.example.net/notices/3"),
            ],
            "2026-07-20",
        )
        .unwrap();
        pack.supersede_finding(0, replacement).unwrap();
    }
    let emitted = case
        .resolve(
            Knowability::HighValidity,
            default_limits(),
            Some("provenanced label, not a verdict; contested facts remain contested".into()),
            HandoffChannel::Journalist,
            "person-naming close",
        )
        .unwrap();
    // Non-destructive labeling: the two-root version is preserved,
    // superseded — nothing about the person was silently rewritten.
    assert!(emitted.pack().findings().iter().any(|f| !f.is_current()));
}

/// Lock 2.5d structural: the safety layer exposes no action API —
/// resolution remains handoff-only, and nothing in `src/safety` can
/// publish, execute, or send anything anywhere.
#[test]
fn lock2_5d_handoff_only_no_action_api_in_the_safety_layer() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/safety");
    let act_verbs = [
        "pub fn publish",
        "pub fn execute",
        "pub fn deploy",
        "pub fn send",
        "pub fn post",
        "pub fn act",
        "pub fn apply_action",
        "pub fn delete",
        "pub fn remove",
    ];
    let mut checked = 0usize;
    let mut stack = vec![dir];
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d).expect("read safety dir") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                checked += 1;
                let src = std::fs::read_to_string(&path).expect("read source");
                for verb in act_verbs {
                    assert!(
                        !src.contains(verb),
                        "action API '{}' found in {}",
                        verb,
                        path.display()
                    );
                }
            }
        }
    }
    assert!(checked >= 4, "safety scan saw too few files ({checked})");
}

/// The Phase 2.5 adversarial corpus: the F-029/F-031 shapes carried to
/// this phase, locked at their *documented current behavior*. Each of
/// these is a deliberate limitation of the floor heuristic (needs IP/IDN/
/// percent canonicalization or repo-root knowledge the floor does not
/// take). If a future change alters any of these, this lock makes the
/// change visible and reviewable instead of silent.
#[test]
fn carried_forgery_shapes_documented_limitation_lock() {
    use holmes_core::analysis::emission::source_root;
    // IPv4 numeric spellings of one address stay distinct (no IP parsing).
    assert_ne!(
        source_root("http://192.168.0.1/x"),
        source_root("http://0xC0.0xA8.0.1/x")
    );
    assert_ne!(
        source_root("http://192.168.0.1/x"),
        source_root("http://3232235521/x")
    );
    assert_ne!(
        source_root("http://127.0.0.1/x"),
        source_root("http://127.1/x")
    );
    // IPv6 compression variants stay distinct (no IPv6 canonicalization).
    assert_ne!(
        source_root("http://[::1]/x"),
        source_root("http://[0:0:0:0:0:0:0:1]/x")
    );
    // IDN Unicode vs punycode stays distinct (no IDN mapping).
    assert_ne!(
        source_root("https://münchen.example/x"),
        source_root("https://xn--mnchen-3ya.example/x")
    );
    // Percent-encoded hosts stay distinct (no percent decoding).
    assert_ne!(
        source_root("https://ex%61mple.org/x"),
        source_root("https://example.org/x")
    );
    // Single-slash scheme falls to the path branch; its first segment
    // ("https:") is not host-like, so the spelling stays distinct from
    // the two-slash URL — carried limitation, documented.
    assert_ne!(source_root("https:/example.org/x"), "example.org");
    // Absolute vs repo-relative spellings of one file stay distinct (no
    // repo-root knowledge).
    assert_ne!(
        source_root("/home/user/holmes/docs/x.md"),
        source_root("docs/x.md")
    );
    // Subdomain granularity stands as documented (no public-suffix list).
    assert_ne!(
        source_root("https://a.example.org/"),
        source_root("https://b.example.org/")
    );
}
