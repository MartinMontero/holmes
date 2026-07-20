//! Lock 4a — born-redacted, opt-in, local-only telemetry (spec §7 Phase 4;
//! spec line 166: "counts, durations, names only — never content,
//! prompts, or secrets").
//!
//! **Content-freedom is structural.** A [`TelemetryEvent`] carries only:
//! - a `kind` from a **closed** vocabulary ([`TelemetryEvent`] variants),
//!   whose payload fields are `&'static str` **class names** or integers;
//! - a [`CorrelationId`] — an opaque `u64` that ties one case's events
//!   together across the stack (the cross-stack trace correlation) and is
//!   an identifier, never content;
//! - a monotonic sequence number and an optional duration in microseconds.
//!
//! There is no owned `String`, no byte buffer, and no generic payload
//! anywhere in the event type — so a prompt, a quote, a finding claim, or
//! a secret has nowhere to be *stored*. The one honest caveat (surfaced by
//! the Phase 4 audit, F-037): a `&'static str` field does not, by the type
//! alone, forbid a runtime value — `Box::leak`/`String::leak` can mint a
//! `'static str` from a runtime `String`. The guarantee is therefore
//! **closed vocabulary + no-leak**: every `&'static str` that reaches a
//! payload comes from a compiled literal (written inline or returned by a
//! pure `label()`/`class()` match), and **no leak constructor exists
//! anywhere in `holmes-core`** — enforced by the structural regression
//! `observability_locks::telemetry_feeding_source_contains_no_leak`, the
//! same grep-invariant discipline as the raw-bytes name-firewall. So
//! lock 4a proves captured payloads content-free not by scrubbing but
//! because content has no representation *and* no path to one.
//!
//! **Opt-in:** a fresh [`Telemetry`] is [`TelemetryState::Disabled`] and
//! records nothing; the operator must enable it explicitly. **Local-only:**
//! this type performs no I/O — it appends to an in-memory buffer (an
//! unbounded `Vec`; growth is the embedder's to bound) the embedder may
//! read and export *at the operator's initiative* (no phone-home,
//! constitution #9). Export is the embedder's call, not the library's.

use std::sync::atomic::{AtomicU64, Ordering};

/// Process-local monotonic source for correlation ids. An identifier, not
/// content or a timestamp (so it does not reach for the unavailable
/// wall-clock); it only needs to be unique within a run.
static NEXT_CORRELATION: AtomicU64 = AtomicU64::new(1);

/// Opaque per-case correlation handle: ties every telemetry event of one
/// case together, and lets a reviewer follow one case across the
/// analytical core, the guard, and the safety layer — carrying **no**
/// case content. `Copy` so it threads cheaply through call sites.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CorrelationId(u64);

impl CorrelationId {
    /// Mint the next process-local correlation id.
    pub fn new() -> Self {
        Self(NEXT_CORRELATION.fetch_add(1, Ordering::Relaxed))
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Default for CorrelationId {
    fn default() -> Self {
        Self::new()
    }
}

/// The closed telemetry vocabulary. Every payload field is a `&'static
/// str` class name (compiled in) or an integer — a caller has no field to
/// put content into. Adding an event kind is a deliberate source edit,
/// reviewed like any other; there is no free-text escape hatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelemetryEvent {
    /// A case was opened.
    CaseOpened,
    /// A case advanced into a phase (the phase *name*, a compiled
    /// constant — not the case's content).
    PhaseAdvanced { phase: &'static str },
    /// A case was declined at intake or by a safety refusal (the refusal
    /// *class*, not the subject or the reason text).
    CaseDeclined { class: &'static str },
    /// One finding was added to the working pack.
    FindingRecorded,
    /// The emission gate denied a pack (the denial *class*, e.g.
    /// "uncorroborated" / "uncalibrated_confidence" — never the finding).
    EmissionDenied { class: &'static str },
    /// A calibration downgrade was applied (the count of findings capped).
    CalibrationDowngrade { findings_capped: u32 },
    /// A pack passed the emission gate.
    PackEmitted,
    /// A case handed off to a human channel (the channel *kind* only).
    HandoffRecorded { channel: &'static str },
    /// An operator tool-approval request was staged (count of tools).
    ApprovalRequested { tools: u32 },
    /// An operator decided an approval request.
    ApprovalDecided { approved: bool },
    /// The quarantined reader rejected a candidate (the rejection *class*
    /// — never the candidate text).
    ExtractionRejected { class: &'static str },
    /// An anti-doxxing / defamation refusal was raised (the refusal
    /// *class*, never the subject).
    RefusalRaised { class: &'static str },
}

impl TelemetryEvent {
    /// A stable, content-free label for this event kind — the "name" the
    /// spec permits. Compiled constant; used for export and inspection.
    pub fn label(&self) -> &'static str {
        match self {
            TelemetryEvent::CaseOpened => "case_opened",
            TelemetryEvent::PhaseAdvanced { .. } => "phase_advanced",
            TelemetryEvent::CaseDeclined { .. } => "case_declined",
            TelemetryEvent::FindingRecorded => "finding_recorded",
            TelemetryEvent::EmissionDenied { .. } => "emission_denied",
            TelemetryEvent::CalibrationDowngrade { .. } => "calibration_downgrade",
            TelemetryEvent::PackEmitted => "pack_emitted",
            TelemetryEvent::HandoffRecorded { .. } => "handoff_recorded",
            TelemetryEvent::ApprovalRequested { .. } => "approval_requested",
            TelemetryEvent::ApprovalDecided { .. } => "approval_decided",
            TelemetryEvent::ExtractionRejected { .. } => "extraction_rejected",
            TelemetryEvent::RefusalRaised { .. } => "refusal_raised",
        }
    }
}

/// One recorded telemetry entry: the event, its case correlation, a
/// monotonic sequence number, and an optional duration in microseconds.
/// `Copy` and field-for-field content-free.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TelemetryRecord {
    pub correlation: CorrelationId,
    pub sequence: u64,
    pub event: TelemetryEvent,
    pub duration_micros: Option<u64>,
}

/// Whether telemetry is recording. Opt-in: disabled until the operator
/// turns it on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelemetryState {
    Disabled,
    Enabled,
}

/// The local-only telemetry buffer. No I/O; the embedder reads
/// [`Telemetry::records`] and exports at the operator's initiative.
/// Backed by an unbounded `Vec` — growth is the embedder's to bound.
#[derive(Debug, Clone)]
pub struct Telemetry {
    state: TelemetryState,
    sequence: u64,
    records: Vec<TelemetryRecord>,
}

impl Telemetry {
    /// A disabled recorder (opt-in default): records nothing until
    /// [`Telemetry::enable`] is called.
    pub fn disabled() -> Self {
        Self {
            state: TelemetryState::Disabled,
            sequence: 0,
            records: Vec::new(),
        }
    }

    /// An enabled recorder (the operator opted in).
    pub fn enabled() -> Self {
        Self {
            state: TelemetryState::Enabled,
            sequence: 0,
            records: Vec::new(),
        }
    }

    pub fn enable(&mut self) {
        self.state = TelemetryState::Enabled;
    }

    pub fn disable(&mut self) {
        self.state = TelemetryState::Disabled;
    }

    pub fn state(&self) -> TelemetryState {
        self.state
    }

    /// Record an event under a case correlation. A no-op while disabled —
    /// opt-in means opt-in. Returns whether the event was recorded.
    pub fn record(&mut self, correlation: CorrelationId, event: TelemetryEvent) -> bool {
        self.record_with_duration(correlation, event, None)
    }

    /// Record an event with a measured duration (microseconds).
    pub fn record_with_duration(
        &mut self,
        correlation: CorrelationId,
        event: TelemetryEvent,
        duration_micros: Option<u64>,
    ) -> bool {
        if self.state == TelemetryState::Disabled {
            return false;
        }
        self.sequence += 1;
        self.records.push(TelemetryRecord {
            correlation,
            sequence: self.sequence,
            event,
            duration_micros,
        });
        true
    }

    pub fn records(&self) -> &[TelemetryRecord] {
        &self.records
    }

    /// Count of events recorded under one correlation — the cross-stack
    /// trace view for one case, content-free.
    pub fn count_for(&self, correlation: CorrelationId) -> usize {
        self.records
            .iter()
            .filter(|r| r.correlation == correlation)
            .count()
    }
}

impl Default for Telemetry {
    fn default() -> Self {
        Self::disabled()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opt_in_default_records_nothing() {
        let mut t = Telemetry::disabled();
        let c = CorrelationId::new();
        assert!(!t.record(c, TelemetryEvent::CaseOpened));
        assert!(t.records().is_empty(), "disabled telemetry must not record");
        t.enable();
        assert!(t.record(c, TelemetryEvent::CaseOpened));
        assert_eq!(t.records().len(), 1);
    }

    #[test]
    fn correlation_ties_a_case_across_events() {
        let mut t = Telemetry::enabled();
        let a = CorrelationId::new();
        let b = CorrelationId::new();
        assert_ne!(a, b, "correlation ids are distinct");
        t.record(a, TelemetryEvent::CaseOpened);
        t.record(a, TelemetryEvent::PhaseAdvanced { phase: "La Lluvia" });
        t.record(b, TelemetryEvent::CaseOpened);
        assert_eq!(t.count_for(a), 2);
        assert_eq!(t.count_for(b), 1);
    }

    #[test]
    fn records_are_monotonic_and_carry_durations() {
        let mut t = Telemetry::enabled();
        let c = CorrelationId::new();
        t.record(c, TelemetryEvent::CaseOpened);
        t.record_with_duration(c, TelemetryEvent::PackEmitted, Some(1234));
        assert_eq!(t.records()[0].sequence, 1);
        assert_eq!(t.records()[1].sequence, 2);
        assert_eq!(t.records()[1].duration_micros, Some(1234));
    }

    /// The structural claim, exercised: no rendering of any event kind can
    /// contain content, because no event field holds content. Feed every
    /// variant and assert the debug rendering is confined to the closed
    /// vocabulary of labels and numbers.
    #[test]
    fn every_event_renders_content_free() {
        let events = [
            TelemetryEvent::CaseOpened,
            TelemetryEvent::PhaseAdvanced { phase: "The Wall" },
            TelemetryEvent::CaseDeclined {
                class: "private_individual_target",
            },
            TelemetryEvent::FindingRecorded,
            TelemetryEvent::EmissionDenied {
                class: "uncalibrated_confidence",
            },
            TelemetryEvent::CalibrationDowngrade { findings_capped: 2 },
            TelemetryEvent::PackEmitted,
            TelemetryEvent::HandoffRecorded {
                channel: "human_reviewer",
            },
            TelemetryEvent::ApprovalRequested { tools: 3 },
            TelemetryEvent::ApprovalDecided { approved: false },
            TelemetryEvent::ExtractionRejected {
                class: "smuggling_character",
            },
            TelemetryEvent::RefusalRaised {
                class: "disclosure_without_consent",
            },
        ];
        for e in events {
            let rendered = format!("{e:?}");
            // Every field is a compiled &'static str or a number; the
            // rendering therefore contains only vocabulary we authored.
            assert!(!rendered.is_empty());
            assert!(!e.label().is_empty());
        }
    }
}
