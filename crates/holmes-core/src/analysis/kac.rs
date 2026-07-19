//! Engine 2 — Key Assumptions Check ("¿Qué estoy asumiendo?"; spec §2.2).
//!
//! Every load-bearing assumption is listed with why it must be true and
//! the condition under which it would fail (the *Tradecraft Primer*'s DC
//! Sniper lesson: unexamined assumptions eliminated the actual
//! perpetrators). The check is bookkeeping — deterministic; the
//! assumptions themselves come from the analyst/model side.

use crate::artifacts::Provenance;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum KacError {
    EmptyStatement,
    EmptyWhy,
    EmptyFailureCondition,
    NoSuchAssumption(usize),
}

impl fmt::Display for KacError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KacError::EmptyStatement => write!(f, "rejected: empty assumption statement"),
            KacError::EmptyWhy => write!(f, "rejected: empty why-it-must-hold"),
            KacError::EmptyFailureCondition => {
                write!(
                    f,
                    "rejected: an assumption with no stated failure condition is unexamined"
                )
            }
            KacError::NoSuchAssumption(i) => write!(f, "rejected: no assumption at index {i}"),
        }
    }
}

impl std::error::Error for KacError {}

#[derive(Debug, Clone, PartialEq)]
pub enum AssumptionStatus {
    /// Listed but not yet examined against evidence.
    Unexamined,
    /// Held up under examination, with the supporting source.
    Supported { provenance: Provenance },
    /// Failed its stated failure condition — the analysis leaning on it
    /// must be revisited. Preserved, never removed.
    Failed { how: String },
}

/// One load-bearing assumption, fully examined or visibly not.
#[derive(Debug, Clone)]
pub struct KeyAssumption {
    pub statement: String,
    pub why_it_must_hold: String,
    pub failure_condition: String,
    status: AssumptionStatus,
}

impl KeyAssumption {
    pub fn new(
        statement: impl Into<String>,
        why_it_must_hold: impl Into<String>,
        failure_condition: impl Into<String>,
    ) -> Result<Self, KacError> {
        let statement = statement.into();
        let why = why_it_must_hold.into();
        let failure = failure_condition.into();
        if statement.trim().is_empty() {
            return Err(KacError::EmptyStatement);
        }
        if why.trim().is_empty() {
            return Err(KacError::EmptyWhy);
        }
        if failure.trim().is_empty() {
            return Err(KacError::EmptyFailureCondition);
        }
        Ok(Self {
            statement,
            why_it_must_hold: why,
            failure_condition: failure,
            status: AssumptionStatus::Unexamined,
        })
    }

    pub fn status(&self) -> &AssumptionStatus {
        &self.status
    }
}

/// The check itself: an append-only list with examination status.
#[derive(Debug, Clone, Default)]
pub struct KeyAssumptionsCheck {
    assumptions: Vec<KeyAssumption>,
}

impl KeyAssumptionsCheck {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, assumption: KeyAssumption) -> usize {
        self.assumptions.push(assumption);
        self.assumptions.len() - 1
    }

    pub fn mark_supported(&mut self, index: usize, provenance: Provenance) -> Result<(), KacError> {
        let a = self
            .assumptions
            .get_mut(index)
            .ok_or(KacError::NoSuchAssumption(index))?;
        a.status = AssumptionStatus::Supported { provenance };
        Ok(())
    }

    pub fn mark_failed(&mut self, index: usize, how: impl Into<String>) -> Result<(), KacError> {
        let a = self
            .assumptions
            .get_mut(index)
            .ok_or(KacError::NoSuchAssumption(index))?;
        a.status = AssumptionStatus::Failed { how: how.into() };
        Ok(())
    }

    pub fn assumptions(&self) -> &[KeyAssumption] {
        &self.assumptions
    }

    pub fn is_empty(&self) -> bool {
        self.assumptions.is_empty()
    }

    /// Indices still unexamined — visible work, never silently passed.
    pub fn unexamined(&self) -> Vec<usize> {
        self.assumptions
            .iter()
            .enumerate()
            .filter(|(_, a)| a.status == AssumptionStatus::Unexamined)
            .map(|(i, _)| i)
            .collect()
    }

    pub fn failed(&self) -> Vec<usize> {
        self.assumptions
            .iter()
            .enumerate()
            .filter(|(_, a)| matches!(a.status, AssumptionStatus::Failed { .. }))
            .map(|(i, _)| i)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unexamined_and_failed_assumptions_stay_visible() {
        let mut kac = KeyAssumptionsCheck::new();
        let a = kac.add(
            KeyAssumption::new(
                "the registry is current",
                "stale data would invalidate the entity match",
                "a filing newer than the snapshot exists",
            )
            .unwrap(),
        );
        let b = kac.add(
            KeyAssumption::new(
                "one filer, one entity",
                "the trace assumes no shell duplication",
                "two entities share the registered agent and address",
            )
            .unwrap(),
        );
        assert_eq!(kac.unexamined(), vec![a, b]);
        kac.mark_failed(b, "duplicate agent found").unwrap();
        assert_eq!(kac.failed(), vec![b]);
        assert_eq!(kac.unexamined(), vec![a]);
        assert_eq!(kac.assumptions().len(), 2, "nothing removed");
    }

    #[test]
    fn an_assumption_without_a_failure_condition_is_rejected() {
        assert_eq!(
            KeyAssumption::new("s", "w", " ").unwrap_err(),
            KacError::EmptyFailureCondition
        );
    }
}
