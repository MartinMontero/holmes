//! Phase 4 lock tests — Observability & hardening (loop §6 Phase 4).
//!
//! 4a: born-redacted telemetry is provably content-free on captured
//!     payloads — driven over a full observed case whose content and a
//!     planted secret are known canaries; the cross-stack correlation
//!     ties the case's events together.
//! Permission manifest: deny-by-default; read-only runs free, write asks
//!     first; the investigative class is absent from the beta (D-14(a)).

use holmes_core::analysis::{
    AchCell, AnalyticalCase, DiabloPass, EvidenceItem, Hypothesis, IntakeAssessment,
    LikelihoodRatio, Probability, SystemicOrIsolated,
};
use holmes_core::observability::{
    telemetry::TelemetryEvent, PermissionDecision, PermissionManifest,
};
use holmes_core::{
    BriefOrigin, Confidence, Finding, HandoffChannel, Knowability, LimitsOfThisFinding, Provenance,
    ResearchBrief,
};

/// Distinctive canaries that appear in the case's *content* — the brief
/// question, a finding claim, a provenance quote — and a planted secret.
/// Lock 4a proves none of these ever reaches a telemetry payload.
const CONTENT_CANARY: &str = "ZEBRA-COMMUNITY-GARDEN-PERMIT-CONTENT";
const SECRET_CANARY: &str = "sk-ant-SECRETCANARY-do-not-log";

fn observed_case_to_handoff() -> AnalyticalCase {
    let mut brief = ResearchBrief::new(
        format!("why did the permit stall? {CONTENT_CANARY}"),
        BriefOrigin::Intent,
        format!("scope: {CONTENT_CANARY}"),
        Vec::new(),
    )
    .unwrap();
    // A secret riding on the brief's firewalled field must never surface.
    brief.stated_confidence = Some(Confidence::new(0.95).unwrap());

    let mut case = AnalyticalCase::open_observed(brief).unwrap();
    case.record_intake(IntakeAssessment {
        someone_harmed: true,
        harm_note: format!("harm note {CONTENT_CANARY}"),
        systemic_or_isolated: SystemicOrIsolated::Isolated,
        can_help_without_more_harm: true,
        assessment_note: format!("assessment {SECRET_CANARY}"),
    })
    .unwrap();
    case.advance_to_la_lluvia().unwrap();
    let h0 = case
        .add_hypothesis(
            Hypothesis::new(
                format!("backlog {CONTENT_CANARY}"),
                Probability::new(0.5).unwrap(),
                vec![],
                vec![],
            )
            .unwrap(),
        )
        .unwrap();
    let h1 = case
        .add_hypothesis(
            Hypothesis::new("objection", Probability::new(0.3).unwrap(), vec![], vec![]).unwrap(),
        )
        .unwrap();
    case.advance_to_collection().unwrap();
    case.add_evidence(EvidenceItem {
        id: "E1".into(),
        description: format!("evidence {CONTENT_CANARY}"),
        provenance: vec![Provenance::new(
            "fixture/permit-docket.md §3",
            Some(format!("quote {CONTENT_CANARY}")),
        )
        .unwrap()],
    })
    .unwrap();
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
    case.score_ach(h0, "E1", AchCell::Inconsistent).unwrap();
    case.score_ach(h1, "E1", AchCell::Consistent).unwrap();
    let a = case.kac_mut().add(
        holmes_core::analysis::KeyAssumption::new(
            format!("assumption {CONTENT_CANARY}"),
            "matters",
            "could be false",
        )
        .unwrap(),
    );
    case.kac_mut()
        .mark_supported(
            a,
            Provenance::new("fixture/permit-docket.md §1", Some("docket".into())).unwrap(),
        )
        .unwrap();
    case.record_diablo(DiabloPass {
        challenge: format!("challenge {CONTENT_CANARY}"),
        response: "predates".into(),
    })
    .unwrap();
    case.advance_to_money().unwrap();
    case.record_money_note(format!("money note {CONTENT_CANARY}"))
        .unwrap();
    case.evidence_pack_mut().unwrap().add_finding(
        Finding::new(
            format!("finding claim {CONTENT_CANARY}"),
            Confidence::new(0.7).unwrap(),
            vec![
                Provenance::new("fixture/permit-docket.md §3", Some("objection".into())).unwrap(),
                Provenance::new("https://records.example.gov/permits/441", None).unwrap(),
            ],
            "2026-07-20",
        )
        .unwrap(),
    );
    case.resolve(
        Knowability::HighValidity,
        LimitsOfThisFinding {
            what_would_change_the_conclusion: vec![format!("limit {CONTENT_CANARY}")],
            ..Default::default()
        },
        None,
        HandoffChannel::HumanReviewer,
        format!("close {CONTENT_CANARY}"),
    )
    .unwrap();
    case
}

/// Lock 4a — telemetry captured over a full observed case is content-free:
/// no case content and no planted secret reaches any payload, and the
/// events are all from the closed vocabulary under one correlation id.
#[test]
fn lock4a_telemetry_is_content_free_on_captured_payloads() {
    let case = observed_case_to_handoff();
    let records = case.telemetry().records();
    assert!(records.len() >= 6, "an observed case should emit a stream");

    // Every event is tied to the case's single correlation id (cross-stack
    // trace correlation).
    let corr = case.correlation();
    assert!(records.iter().all(|r| r.correlation == corr));

    // The captured payloads — rendered every way an exporter might — carry
    // neither the content canary nor the secret canary.
    for r in records {
        let rendered = format!("{r:?}");
        assert!(
            !rendered.contains(CONTENT_CANARY),
            "telemetry payload leaked case content: {rendered}"
        );
        assert!(
            !rendered.contains(SECRET_CANARY),
            "telemetry payload leaked a secret: {rendered}"
        );
        assert!(
            !rendered.contains("sk-ant"),
            "telemetry payload leaked a key-shaped token: {rendered}"
        );
        // The event's label is a compiled vocabulary word.
        assert!(!r.event.label().is_empty());
    }

    // The stream tells the case's shape (a real trace) without its content.
    let labels: Vec<&str> = records.iter().map(|r| r.event.label()).collect();
    assert!(labels.contains(&"case_opened"));
    assert!(labels.contains(&"phase_advanced"));
    assert!(labels.contains(&"pack_emitted"));
    assert!(labels.contains(&"handoff_recorded"));
}

/// Lock 4a — opt-in: a case opened un-observed records nothing, even
/// through a full run.
#[test]
fn lock4a_unobserved_case_records_nothing() {
    let brief = ResearchBrief::new("q", BriefOrigin::Intent, "s", Vec::new()).unwrap();
    let mut case = AnalyticalCase::open(brief).unwrap();
    case.record_intake(IntakeAssessment {
        someone_harmed: false,
        harm_note: String::new(),
        systemic_or_isolated: SystemicOrIsolated::Isolated,
        can_help_without_more_harm: true,
        assessment_note: "ok".into(),
    })
    .unwrap();
    case.advance_to_la_lluvia().unwrap();
    assert!(
        case.telemetry().records().is_empty(),
        "opt-in default must record nothing"
    );
}

/// A declined case still emits a content-free decline event (the trace
/// records that a refusal happened, never why in content terms).
#[test]
fn lock4a_declined_case_emits_content_free_decline() {
    let brief = ResearchBrief::new("q", BriefOrigin::Intent, "s", Vec::new()).unwrap();
    let mut case = AnalyticalCase::open_observed(brief).unwrap();
    case.record_intake(IntakeAssessment {
        someone_harmed: true,
        harm_note: format!("harm {CONTENT_CANARY}"),
        systemic_or_isolated: SystemicOrIsolated::Isolated,
        can_help_without_more_harm: false,
        assessment_note: format!("would expose {SECRET_CANARY}"),
    })
    .unwrap();
    let declined = case
        .telemetry()
        .records()
        .iter()
        .any(|r| matches!(r.event, TelemetryEvent::CaseDeclined { .. }));
    assert!(declined, "decline should be traced");
    for r in case.telemetry().records() {
        let rendered = format!("{r:?}");
        assert!(!rendered.contains(CONTENT_CANARY));
        assert!(!rendered.contains(SECRET_CANARY));
    }
}

/// F-037 lock (Phase 4 audit): the telemetry content-free guarantee is
/// "closed vocabulary + no-leak". The closed vocabulary is a type fact;
/// the no-leak half is enforced here — no `leak` constructor exists
/// anywhere in `holmes-core/src`, so no runtime `String` can be minted
/// into a `&'static str` telemetry field. Same grep-invariant discipline
/// as the raw-bytes name-firewall. If a future edit needs a leak, this
/// test forces a deliberate, reviewed exception rather than a silent hole.
#[test]
fn telemetry_feeding_source_contains_no_leak() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut checked = 0usize;
    let mut stack = vec![dir];
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d).expect("read src dir") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                checked += 1;
                let src = std::fs::read_to_string(&path).expect("read source");
                // Strip nothing fancy: any `.leak(` or `Box::leak`/`str::leak`
                // call could mint a 'static str from runtime data. The doc
                // comments that *name* the loophole use `leak` without a
                // trailing `(`, so they do not trip this.
                assert!(
                    !src.contains(".leak(") && !src.contains("::leak("),
                    "no-leak invariant breached (F-037): {} calls a leak constructor",
                    path.display()
                );
            }
        }
    }
    assert!(checked >= 10, "no-leak scan saw too few files ({checked})");
}

/// D-14(a) lock: the beta (default) build compiles with the investigative
/// feature OFF — the Phase-3 surface is *absent* from the beta artifact,
/// not dark-flagged. When Phase 3 lands behind `--features investigative`,
/// this proves the default build still excludes it.
#[test]
fn beta_build_compiles_with_investigative_off() {
    // The constant-ness is the point: this test binary was built with the
    // default feature set, and the beta must not carry Phase-3 code. (A
    // future `--features investigative` build is legitimate for Phase 3,
    // so this stays a per-build runtime check, not a compile_error.)
    #[allow(clippy::assertions_on_constants)]
    {
        assert!(
            !cfg!(feature = "investigative"),
            "the analytical open-beta build must compile with investigative OFF (D-14(a))"
        );
    }
}

/// The deny-by-default permission manifest, finalized for the analytical
/// surface: read-only runs free, write asks first, unlisted is denied,
/// and the investigative class is absent (D-14(a)).
#[test]
fn permission_manifest_is_deny_by_default_and_investigative_free() {
    let m = PermissionManifest::analytical_beta();
    assert_eq!(
        m.decide("read_untrusted_content"),
        PermissionDecision::RunFree
    );
    assert!(matches!(
        m.decide("wall_write"),
        PermissionDecision::AskFirst { .. }
    ));
    // Every Phase-3 capability is denied by absence.
    for phase3 in ["records_search", "osint_fetch", "code_exec"] {
        assert!(matches!(
            m.decide(phase3),
            PermissionDecision::Denied { .. }
        ));
    }
    assert!(!m.declares_investigative());
}
