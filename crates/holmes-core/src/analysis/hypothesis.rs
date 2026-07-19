//! Engine 1 — abduction + likelihood-ratio updating (spec §2.2).
//!
//! Hypothesis objects carry a prior, predicted-*present* evidence,
//! predicted-*absent* evidence ("the dog that didn't bark"), and a running
//! log-likelihood updated as data arrives. The scorer takes **both**
//! conditionals explicitly — P(E|H) and P(E|¬H) — so the prosecutor's
//! fallacy (reading P(E|H) as P(H|E)) is unrepresentable at the type
//! level: a likelihood ratio cannot be constructed from one conditional.
//! Strong inference (Platt, *Science*, 1964): updates exist to
//! *discriminate* between live hypotheses.

use std::fmt;

/// A probability validated into (0, 1] at construction. Zero is excluded:
/// a zero conditional makes the ratio degenerate, and "impossible" is a
/// hypothesis-elimination decision, not an update.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Probability(f64);

#[derive(Debug, Clone, PartialEq)]
pub enum LrError {
    ProbabilityOutOfRange(f64),
    EmptyStatement,
    NoSuchEvidence(String),
}

impl fmt::Display for LrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LrError::ProbabilityOutOfRange(v) => {
                write!(f, "rejected: probability {v} outside (0, 1]")
            }
            LrError::EmptyStatement => write!(f, "rejected: empty hypothesis statement"),
            LrError::NoSuchEvidence(id) => write!(f, "rejected: unknown evidence id '{id}'"),
        }
    }
}

impl std::error::Error for LrError {}

impl Probability {
    pub fn new(value: f64) -> Result<Self, LrError> {
        if !(value > 0.0 && value <= 1.0) || value.is_nan() {
            return Err(LrError::ProbabilityOutOfRange(value));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Calibration status carried on every likelihood-ratio score (loop §6
/// Phase 1: "LR scores carry a calibration status now; **gating lands in
/// Phase 2.5**"). Phase 1 only records the status honestly — nothing here
/// blocks on it yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CalibrationStatus {
    /// No calibration evidence exists for the judgment source. The Phase 1
    /// default — and the only status the analytical core assigns itself.
    #[default]
    Uncalibrated,
    /// Reserved for Phase 2.5's calibration machinery; nothing in Phase 1
    /// constructs it.
    Calibrated,
}

/// A likelihood ratio: how much more probable this evidence is under the
/// hypothesis than under its alternatives. Requires both conditionals.
#[derive(Debug, Clone, Copy)]
pub struct LikelihoodRatio {
    p_e_given_h: Probability,
    p_e_given_not_h: Probability,
}

impl LikelihoodRatio {
    pub fn new(p_e_given_h: Probability, p_e_given_not_h: Probability) -> Self {
        Self {
            p_e_given_h,
            p_e_given_not_h,
        }
    }

    /// Natural-log likelihood ratio: ln(P(E|H) / P(E|¬H)).
    pub fn log_ratio(&self) -> f64 {
        (self.p_e_given_h.value() / self.p_e_given_not_h.value()).ln()
    }

    pub fn p_e_given_h(&self) -> Probability {
        self.p_e_given_h
    }

    pub fn p_e_given_not_h(&self) -> Probability {
        self.p_e_given_not_h
    }
}

/// One applied update, preserved for traceability — the running score is
/// never a bare number with no history.
#[derive(Debug, Clone)]
pub struct LrUpdate {
    pub evidence_id: String,
    pub p_e_given_h: f64,
    pub p_e_given_not_h: f64,
    pub log_ratio: f64,
    pub calibration: CalibrationStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HypothesisStatus {
    Live,
    /// Eliminated hypotheses are preserved with the reason — invalidated,
    /// never deleted (non-destructive truth).
    Eliminated {
        reason: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HypothesisId(pub usize);

/// A structured hypothesis object (spec §2.2 Engine 1).
#[derive(Debug, Clone)]
pub struct Hypothesis {
    pub statement: String,
    pub prior: Probability,
    /// Evidence this hypothesis predicts should be found.
    pub predicted_present: Vec<String>,
    /// Evidence this hypothesis predicts should be *absent* — the dog
    /// that didn't bark.
    pub predicted_absent: Vec<String>,
    log_likelihood: f64,
    updates: Vec<LrUpdate>,
    status: HypothesisStatus,
}

impl Hypothesis {
    pub fn new(
        statement: impl Into<String>,
        prior: Probability,
        predicted_present: Vec<String>,
        predicted_absent: Vec<String>,
    ) -> Result<Self, LrError> {
        let statement = statement.into();
        if statement.trim().is_empty() {
            return Err(LrError::EmptyStatement);
        }
        Ok(Self {
            statement,
            prior,
            predicted_present,
            predicted_absent,
            log_likelihood: 0.0,
            updates: Vec::new(),
            status: HypothesisStatus::Live,
        })
    }

    /// Apply one likelihood-ratio update for a piece of evidence. The
    /// update is appended to the trace and the running score adjusted —
    /// deterministic arithmetic on caller-supplied judgments.
    pub fn apply_update(&mut self, evidence_id: impl Into<String>, lr: LikelihoodRatio) {
        let log_ratio = lr.log_ratio();
        self.updates.push(LrUpdate {
            evidence_id: evidence_id.into(),
            p_e_given_h: lr.p_e_given_h().value(),
            p_e_given_not_h: lr.p_e_given_not_h().value(),
            log_ratio,
            calibration: CalibrationStatus::Uncalibrated,
        });
        self.log_likelihood += log_ratio;
    }

    pub fn log_likelihood(&self) -> f64 {
        self.log_likelihood
    }

    pub fn updates(&self) -> &[LrUpdate] {
        &self.updates
    }

    pub fn status(&self) -> &HypothesisStatus {
        &self.status
    }

    pub fn is_live(&self) -> bool {
        self.status == HypothesisStatus::Live
    }

    /// Eliminate with a reason; the object and its trace are preserved.
    pub fn eliminate(&mut self, reason: impl Into<String>) {
        self.status = HypothesisStatus::Eliminated {
            reason: reason.into(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probability_bounds_are_enforced() {
        assert!(Probability::new(0.5).is_ok());
        assert!(Probability::new(1.0).is_ok());
        assert!(Probability::new(0.0).is_err());
        assert!(Probability::new(-0.1).is_err());
        assert!(Probability::new(1.1).is_err());
        assert!(Probability::new(f64::NAN).is_err());
    }

    #[test]
    fn lr_requires_both_conditionals_and_updates_trace() {
        let lr = LikelihoodRatio::new(
            Probability::new(0.8).unwrap(),
            Probability::new(0.2).unwrap(),
        );
        assert!(lr.log_ratio() > 0.0);
        let mut h = Hypothesis::new(
            "the permit stalled for administrative reasons",
            Probability::new(0.5).unwrap(),
            vec!["a routine-backlog notice exists".into()],
            vec!["no enforcement action exists".into()],
        )
        .unwrap();
        h.apply_update("E1", lr);
        assert_eq!(h.updates().len(), 1);
        assert!((h.log_likelihood() - lr.log_ratio()).abs() < 1e-12);
        assert_eq!(h.updates()[0].calibration, CalibrationStatus::Uncalibrated);
    }

    #[test]
    fn elimination_preserves_the_object_and_trace() {
        let mut h = Hypothesis::new("h", Probability::new(0.3).unwrap(), vec![], vec![]).unwrap();
        h.apply_update(
            "E1",
            LikelihoodRatio::new(
                Probability::new(0.1).unwrap(),
                Probability::new(0.9).unwrap(),
            ),
        );
        h.eliminate("contradicted by E1");
        assert!(!h.is_live());
        assert_eq!(h.updates().len(), 1, "trace survives elimination");
    }
}
