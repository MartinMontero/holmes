//! Phase 0 lock 0d — the Holmes embedding contract.
//!
//! `holmes-core` + `holmes-guard` are the crates Alfred embeds (spec v2.1
//! delivery, amendment A-03). This crate is the *contract* leg: the §6.2
//! hand-off artifact types — [`ResearchBrief`] in; [`EvidencePack`] /
//! [`CaseFile`] out — with validation only. The analytical engines that
//! *populate* these artifacts are Phase 1; nothing here calls a model,
//! touches the network, or performs any action.
//!
//! Contract invariants carried in the type system (holmes-vs-wcjbt §6.4):
//! - Holmes never authors the blueprint: this crate exports no
//!   blueprint/constitution/spec artifact type (structural test
//!   `no_blueprint_exports`), and no API authors application code.
//! - Every finding is provenance-bearing: a [`Finding`] cannot be
//!   constructed with empty provenance or a confidence outside [0, 1].
//! - Non-destructive truth: superseding a finding preserves the old one
//!   flagged invalid-from; there is no removal API anywhere.
//! - Resolution is handoff-only: a [`CaseFile`] closes by routing to a
//!   human channel ([`HandoffChannel`]); no autonomous-action API exists.
//!
//! The Phase 1 schema amendment (epistemic canon Upgrade B: `knowability` +
//! `limits_of_this_finding` on evidence packs) lands with the analytical
//! core, recorded as an A-## to §6.2 — deliberately not pre-implemented
//! here (types follow canon as committed, never ahead of it).

pub mod analysis;
pub mod artifacts;
pub mod observability;
pub mod safety;

pub use artifacts::{
    ArtifactError, BriefOrigin, CaseFile, CaseStatus, CatalogRef, Confidence, EvidencePack,
    Finding, Handoff, HandoffChannel, Knowability, LimitsOfThisFinding, Provenance, ResearchBrief,
};

/// The provider-selection seam (loop §6 Phase 0: "provider selection
/// through a seam Alfred's onboarding UI can drive").
///
/// Alfred's UI may *read* the permitted rosters to render a picker and may
/// call [`provider::resolve`] to validate a selection — but enforcement
/// lives entirely in `holmes-guard` compiled policy (a UI layer may read
/// the list, never enforce it). The env/Ollama path exercised by
/// `holmes-smoke` is this seam's first consumer.
pub mod provider {
    pub use holmes_guard::policy::{PERMITTED_MODEL_FAMILIES, PERMITTED_PROVIDERS};
    pub use holmes_guard::resolution::{resolve, Denial, ResolvedModel};
    pub use holmes_guard::spawn::PROVIDER_CREDENTIAL_KEYS;
}
