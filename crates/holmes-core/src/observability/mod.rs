//! Phase 4 — Observability & hardening (loop §6 Phase 4; spec §7 Phase 4).
//!
//! The analytical surface only (D-14(a): the investigative surface is
//! *absent* from the beta, not dark-flagged — there is no Phase 3 code to
//! observe). Everything here is deterministic, local-only, and holds no
//! I/O of its own:
//!
//! - [`telemetry`] — born-redacted, opt-in, local-only telemetry
//!   (counts, durations, names — never content, prompts, or secrets),
//!   with a [`telemetry::CorrelationId`] that ties one case's events
//!   across the stack (cross-stack trace correlation) without carrying a
//!   byte of case content. Content-freedom is **structural**: the event
//!   type has no owned-content field, so there is nothing to leak (lock
//!   4a).
//! - [`permission`] — the deny-by-default permission manifest, finalized
//!   for the analytical surface: read-only capabilities run free, write/
//!   shell/network capabilities are ask-first (routed to the Phase 2.5
//!   approval protocol), and an unknown capability is refused outright.
//!   The investigative capability class is **absent** from the beta
//!   manifest (D-14(a)).
//!
//! The recipe safety scanner (Phase 1 lock 1d, extended in Phase 2.5
//! F-035) is the third Phase-4 element; it lives in `holmes-guard` and
//! its regression stays green — re-verified in the sweep, not re-homed.

pub mod permission;
pub mod telemetry;

pub use permission::{
    CapabilityClass, CapabilityPosture, PermissionDecision, PermissionManifest,
    INVESTIGATIVE_ABSENT,
};
pub use telemetry::{CorrelationId, Telemetry, TelemetryEvent, TelemetryState};
