//! Phase 2.5 — Safety before surface (loop §6; hard gate).
//!
//! The three properties Holmes most needs — injection resistance on
//! hostile fetched content, calibrated confidence, legal/defamation
//! guardrails — built *before* the collection surface they protect.
//! Everything here is deterministic and model-free (no LLM inference
//! inside a gate, canon §5); the model-shaped seams are traits whose
//! signatures *are* the confinement.
//!
//! - [`reader`] — the dual-model injection defense (loop §6 2.5(i)):
//!   a quarantined reader over fetched/untrusted content; the privileged
//!   side never sees raw hostile bytes; extractions are typed, bounded,
//!   schema-validated values with no executable authority.
//! - [`approval`] — the tool-approval protocol (loop §6 2.5(iii)):
//!   deny-by-default per-case tool grants; the operator sees a preview
//!   and approves the tool set before anything fires; approvals logged
//!   born-redacted. Rendering is Alfred's surface (obligation recorded);
//!   the protocol and its blocking behavior live here, testable headless.
//! - [`subjects`] — legal/defamation guardrails and the Sentinel
//!   asymmetry (loop §6 2.5(iv)): anti-doxxing refusals per Blacksky's
//!   definition (spec §6.4, adopted verbatim), targeting scoped to power,
//!   and the person-naming evidence threshold at emission.
//!
//! Resolution stays handoff-only: nothing in this module (or anywhere in
//! `holmes-core`) publishes, posts, executes, or otherwise acts — the
//! sole terminal transition remains `CaseFile::hand_off`, structurally
//! re-verified by the Phase 2.5 lock suite.

pub mod approval;
pub mod reader;
pub mod subjects;

pub use approval::{
    ApprovalDecision, ApprovalError, ApprovalProtocol, ApprovalRequest, RequestId, ToolDescriptor,
    ToolGrant,
};
pub use reader::{
    Extraction, ExtractionKind, ExtractionReport, ExtractionRequest, RawCandidate, ReaderBackend,
    RejectionReason, UntrustedContent,
};
pub use subjects::{
    AntiDoxxingRefusal, Consent, ConsentRecord, DefamationDenial, InfoClass, SubjectScope,
    PERSON_FINDING_ROOT_FLOOR,
};
