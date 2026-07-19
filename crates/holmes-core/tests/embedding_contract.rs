//! Lock 0d — the Alfred-shaped consumer test.
//!
//! This test uses `holmes-core` exactly as the embedding application
//! would: public API only, no internals. It proves the contract *links*
//! and that its invariants hold at the boundary an embedder actually
//! touches: research_brief in → evidence accumulates under validation →
//! case_file out through a handoff-only close, with the provider seam
//! readable for an onboarding UI and enforcing at resolution.

use holmes_core::provider;
use holmes_core::{
    ArtifactError, BriefOrigin, CaseFile, CaseStatus, CatalogRef, Confidence, Finding,
    HandoffChannel, Provenance, ResearchBrief,
};

/// The full Loop-D shape: Alfred hits a build-time sourcing question,
/// emits a research_brief, receives a traceable case_file back.
#[test]
fn alfred_loop_d_brief_in_case_file_out() {
    let brief = ResearchBrief::new(
        "is dependency X actually maintained?",
        BriefOrigin::BuildTime,
        "release cadence and maintainer activity over the last 24 months",
        vec![CatalogRef("wcjbt:dependency-x".into())],
    )
    .expect("a well-formed brief constructs");

    let mut case = CaseFile::open(brief).expect("case opens from the brief");
    assert_eq!(case.status(), CaseStatus::Open);
    assert_eq!(
        case.evidence().question(),
        "is dependency X actually maintained?",
        "the evidence pack inherits the brief's question — traceable end-to-end"
    );

    let finding = Finding::new(
        "latest release shipped 2026-06-30",
        Confidence::new(0.9).unwrap(),
        vec![Provenance::new(
            "https://example.org/x/releases",
            Some("v4.2.0 — 2026-06-30".into()),
        )
        .unwrap()],
        "2026-07-19",
    )
    .unwrap();
    let idx = case.evidence_mut().unwrap().add_finding(finding);

    // Supersession preserves; nothing is deleted.
    let corrected = Finding::new(
        "latest release shipped 2026-07-02 (v4.2.1 hotfix)",
        Confidence::new(0.95).unwrap(),
        vec![Provenance::new(
            "https://example.org/x/releases",
            Some("v4.2.1 — 2026-07-02".into()),
        )
        .unwrap()],
        "2026-07-19",
    )
    .unwrap();
    case.evidence_mut()
        .unwrap()
        .supersede_finding(idx, corrected)
        .unwrap();
    assert_eq!(case.evidence().findings().len(), 2);
    assert!(!case.evidence().findings()[idx].is_current());

    // Handoff-only close; the case file is immutable evidence afterwards.
    case.hand_off(HandoffChannel::HumanReviewer, "interim block review")
        .unwrap();
    assert_eq!(case.status(), CaseStatus::HandedOff);
    assert!(matches!(
        case.evidence_mut(),
        Err(ArtifactError::CaseAlreadyHandedOff)
    ));
}

/// Invariant 5 at the embedder boundary: the contract makes invalid
/// findings unrepresentable rather than warning about them.
#[test]
fn invalid_findings_are_rejected_not_warned() {
    assert!(matches!(
        Finding::new(
            "unsourced claim",
            Confidence::new(0.5).unwrap(),
            Vec::new(),
            "2026-07-19"
        ),
        Err(ArtifactError::EmptyProvenance)
    ));
    assert!(matches!(
        Confidence::new(1.5),
        Err(ArtifactError::ConfidenceOutOfRange(_))
    ));
}

/// The provider seam an onboarding UI drives: rosters are readable,
/// resolution enforces. The env/Ollama pair goose itself reported in the
/// Session-2 handshake is the seam's first consumer; excluded and unknown
/// ids deny at the same seam.
#[test]
fn provider_seam_reads_and_resolves_as_the_ui_would() {
    assert!(provider::PERMITTED_PROVIDERS.contains(&"anthropic"));
    assert!(provider::PERMITTED_PROVIDERS.contains(&"ollama"));
    assert!(!provider::PERMITTED_PROVIDERS.is_empty());
    assert!(provider::PROVIDER_CREDENTIAL_KEYS
        .iter()
        .any(|(p, _)| *p == "anthropic"));

    assert!(provider::resolve("ollama", "gemma3:1b").is_ok());
    assert!(provider::resolve("anthropic", "claude-sonnet-5").is_ok());
    assert!(provider::resolve("unknown-provider", "any-model").is_err());
    assert!(provider::resolve("ollama", "some-unknown-family").is_err());
}
