//! Phase 1 — the analytical core (loop §6 Phase 1; spec §2.2/§2.3/§4.2).
//!
//! The deterministic half of the three engines and the six-phase case
//! method. Model-side creativity (hypothesis brainstorming, cell
//! judgments, the devil's-advocate challenge) arrives from goose
//! recipes/subagents as *inputs*; everything here — likelihood-ratio
//! arithmetic, ACH bookkeeping and verdicts, assumption tracking, phase
//! transitions, and the emission gate — is compiled, rule-based, and
//! model-free. No LLM inference runs inside a deterministic gate, and the
//! brief's self-reported-certainty field is never read here (the canon §5
//! firewall, regression-tested structurally in `tests/analysis_locks.rs`,
//! which is also why this module never writes that field's identifier).

pub mod ach;
pub mod emission;
pub mod hypothesis;
pub mod kac;
pub mod quarantine;
pub mod state;

pub use ach::{AchCell, AchError, AchMatrix, AchVerdict};
pub use emission::{EmissionDenial, EmittedEvidencePack};
pub use hypothesis::{
    CalibrationStatus, Hypothesis, HypothesisId, HypothesisStatus, LikelihoodRatio, LrError,
    LrUpdate, Probability,
};
pub use kac::{AssumptionStatus, KeyAssumption, KeyAssumptionsCheck};
pub use quarantine::{FirstPrinciplesRebuild, QuarantinedInput, VerifiedFact};
pub use state::{
    AnalyticalCase, CasePhase, DiabloPass, EvidenceItem, IntakeAssessment, PhaseError,
    SystemicOrIsolated,
};
