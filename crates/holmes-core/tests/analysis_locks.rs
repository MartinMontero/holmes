//! Phase 1 lock tests.
//!
//! Lock 1a: the emission gate (corroboration + Upgrade B) enforced on the
//! full case path. Also here: the canon §5 `stated_confidence` firewall as
//! a structural regression test (the same discipline as the vendor
//! denylist), and the full six-phase walk an embedder would drive.

use holmes_core::analysis::{
    AchCell, AnalyticalCase, DiabloPass, EmissionDenial, EvidenceItem, Hypothesis,
    IntakeAssessment, LikelihoodRatio, PhaseError, Probability, SystemicOrIsolated,
};
use holmes_core::{
    BriefOrigin, Confidence, Finding, HandoffChannel, Knowability, LimitsOfThisFinding, Provenance,
    ResearchBrief,
};

fn brief() -> ResearchBrief {
    let mut b = ResearchBrief::new(
        "why has the community garden permit stalled for nine months?",
        BriefOrigin::Intent,
        "fixture case: permit timeline and actors",
        Vec::new(),
    )
    .unwrap();
    // Recorded — and, per the firewall, read by nothing below.
    b.stated_confidence = Some(Confidence::new(0.95).unwrap());
    b
}

fn walk_to_money(case: &mut AnalyticalCase) {
    case.record_intake(IntakeAssessment {
        someone_harmed: true,
        harm_note: "community project stalled; growers lost a season".into(),
        systemic_or_isolated: SystemicOrIsolated::Isolated,
        can_help_without_more_harm: true,
        assessment_note: "public-records question; no private individual targeted".into(),
    })
    .unwrap();
    case.advance_to_la_lluvia().unwrap();
    let h0 = case
        .add_hypothesis(
            Hypothesis::new(
                "administrative backlog",
                Probability::new(0.5).unwrap(),
                vec!["a backlog notice exists".into()],
                vec!["no objection letter exists".into()],
            )
            .unwrap(),
        )
        .unwrap();
    let h1 = case
        .add_hypothesis(
            Hypothesis::new(
                "an unresolved zoning objection",
                Probability::new(0.3).unwrap(),
                vec!["an objection letter exists".into()],
                vec![],
            )
            .unwrap(),
        )
        .unwrap();
    case.advance_to_collection().unwrap();
    case.add_evidence(EvidenceItem {
        id: "E1".into(),
        description: "objection letter on file, dated 2025-11-04".into(),
        provenance: vec![Provenance::new(
            "fixture/permit-docket.md §3",
            Some("objection filed".into()),
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
            "the docket is complete",
            "a missing filing would change the timeline",
            "a document exists that is not in the docket",
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
        challenge: "could the objection be pretextual cover for the backlog?".into(),
        response: "the objection predates the backlog notice by four months".into(),
    })
    .unwrap();
    case.advance_to_money().unwrap();
    case.record_money_note(
        "not applicable: no financial actor in the fixture; recorded per the \
         explicit-not-applicable rule",
    )
    .unwrap();
}

fn corroborated_finding() -> Finding {
    Finding::new(
        "the stall traces to the 2025-11-04 zoning objection",
        Confidence::new(0.8).unwrap(),
        vec![
            Provenance::new(
                "fixture/permit-docket.md §3",
                Some("objection filed".into()),
            )
            .unwrap(),
            Provenance::new("https://records.example.gov/permits/441", None).unwrap(),
        ],
        "2026-07-19",
    )
    .unwrap()
}

#[test]
fn lock1a_uncorroborated_findings_cannot_leave_the_case() {
    let mut case = AnalyticalCase::open(brief()).unwrap();
    walk_to_money(&mut case);
    case.evidence_pack_mut().unwrap().add_finding(
        Finding::new(
            "single-source claim",
            Confidence::new(0.9).unwrap(),
            vec![Provenance::new("fixture/permit-docket.md §3", None).unwrap()],
            "2026-07-19",
        )
        .unwrap(),
    );
    let denial = case
        .resolve(
            Knowability::HighValidity,
            LimitsOfThisFinding {
                what_could_not_be_checked: vec!["clerk's internal queue".into()],
                ..Default::default()
            },
            HandoffChannel::HumanReviewer,
            "fixture close",
        )
        .unwrap_err();
    assert!(matches!(
        denial,
        PhaseError::Emission(EmissionDenial::Uncorroborated { .. })
    ));
    // The denial left the case open and un-handed-off.
    assert_ne!(
        *case.phase(),
        holmes_core::analysis::CasePhase::ResolutionHandoff
    );
}

#[test]
fn lock1a_corroborated_case_emits_and_hands_off() {
    let mut case = AnalyticalCase::open(brief()).unwrap();
    walk_to_money(&mut case);
    case.evidence_pack_mut()
        .unwrap()
        .add_finding(corroborated_finding());
    let emitted = case
        .resolve(
            Knowability::HighValidity,
            LimitsOfThisFinding {
                what_would_change_the_conclusion: vec![
                    "a docket filing dated after the objection's withdrawal".into(),
                ],
                what_could_not_be_checked: vec!["the clerk's internal queue".into()],
                where_the_evidence_runs_out: vec![
                    "motive behind the objection is outside the record".into(),
                ],
            },
            HandoffChannel::HumanReviewer,
            "fixture close",
        )
        .unwrap();
    assert_eq!(
        *case.phase(),
        holmes_core::analysis::CasePhase::ResolutionHandoff
    );
    let pack = emitted.pack();
    assert_eq!(pack.knowability, Some(Knowability::HighValidity));
    assert!(!pack.competing_hypotheses.is_empty());
    assert!(!pack.key_assumptions.is_empty());
    // The ACH verdict discriminated, so no tie flag was raised.
    assert!(pack.risk_flags.iter().all(|r| !r.contains("tie at top")));
}

#[test]
fn six_phase_walk_enforces_order_and_prerequisites() {
    let mut case = AnalyticalCase::open(brief()).unwrap();
    // One live hypothesis is not a field of alternatives.
    case.record_intake(IntakeAssessment {
        someone_harmed: false,
        harm_note: String::new(),
        systemic_or_isolated: SystemicOrIsolated::Isolated,
        can_help_without_more_harm: true,
        assessment_note: "ok".into(),
    })
    .unwrap();
    case.advance_to_la_lluvia().unwrap();
    case.add_hypothesis(
        Hypothesis::new(
            "only one idea",
            Probability::new(0.5).unwrap(),
            vec![],
            vec![],
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        case.advance_to_collection().unwrap_err(),
        PhaseError::NeedAtLeastTwoLiveHypotheses(1)
    );
}

/// Canon §5: "`stated_confidence` must never reach the verification-routing
/// logic … This boundary carries a regression test, the same way the vendor
/// denylist does." Structural enforcement: no source file in the analysis
/// module references the field at all.
#[test]
fn stated_confidence_firewall_is_structural() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/analysis");
    let mut checked = 0usize;
    let mut stack = vec![dir];
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d).expect("read analysis dir") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                checked += 1;
                let src = std::fs::read_to_string(&path).expect("read source");
                assert!(
                    !src.contains("stated_confidence"),
                    "firewall breach: {} references stated_confidence",
                    path.display()
                );
            }
        }
    }
    assert!(checked >= 6, "firewall scan saw too few files ({checked})");
}
