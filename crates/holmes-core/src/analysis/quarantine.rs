//! Engine 3 — First Principles with input quarantine (spec §2.2; Feynman:
//! "the first principle is that you must not fool yourself").
//!
//! When pattern-matching fails, the problem is rebuilt upward from only
//! verifiable facts. The quarantine is structural: a first-principles
//! rebuild accepts only [`VerifiedFact`] values, and a `VerifiedFact`
//! cannot be constructed without provenance — so an unverifiable input
//! has no type-level path into the rebuild. Quarantined inputs are kept
//! and listed (visible, never silently dropped), but they cannot
//! contaminate the rebuilt chain.

use crate::artifacts::{ArtifactError, Provenance};

/// A fact admissible to a first-principles rebuild: statement +
/// non-optional provenance. The only constructor requires both.
#[derive(Debug, Clone)]
pub struct VerifiedFact {
    pub statement: String,
    pub provenance: Provenance,
}

impl VerifiedFact {
    pub fn new(
        statement: impl Into<String>,
        provenance: Provenance,
    ) -> Result<Self, ArtifactError> {
        let statement = statement.into();
        if statement.trim().is_empty() {
            return Err(ArtifactError::EmptyClaim);
        }
        Ok(Self {
            statement,
            provenance,
        })
    }
}

/// An input that failed verification, held aside with the reason. Kept
/// for the record; structurally unable to enter the rebuild.
#[derive(Debug, Clone)]
pub struct QuarantinedInput {
    pub content: String,
    pub reason: String,
}

/// The rebuild: verified facts in, derivation steps recorded on top.
#[derive(Debug, Clone, Default)]
pub struct FirstPrinciplesRebuild {
    facts: Vec<VerifiedFact>,
    quarantined: Vec<QuarantinedInput>,
    derivations: Vec<String>,
}

impl FirstPrinciplesRebuild {
    pub fn new() -> Self {
        Self::default()
    }

    /// Admit a verified fact — the only way content enters the rebuild.
    pub fn admit(&mut self, fact: VerifiedFact) {
        self.facts.push(fact);
    }

    /// Set an unverifiable input aside, visibly.
    pub fn quarantine(&mut self, content: impl Into<String>, reason: impl Into<String>) {
        self.quarantined.push(QuarantinedInput {
            content: content.into(),
            reason: reason.into(),
        });
    }

    /// Record one derivation step over the admitted facts.
    pub fn derive(&mut self, step: impl Into<String>) {
        self.derivations.push(step.into());
    }

    pub fn facts(&self) -> &[VerifiedFact] {
        &self.facts
    }

    pub fn quarantined(&self) -> &[QuarantinedInput] {
        &self.quarantined
    }

    pub fn derivations(&self) -> &[String] {
        &self.derivations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_provenanced_facts_enter_the_rebuild() {
        let mut fp = FirstPrinciplesRebuild::new();
        fp.admit(
            VerifiedFact::new(
                "the permit application was filed 2026-03-02",
                Provenance::new("fixture/permit-log.md §1", Some("filed 2026-03-02".into()))
                    .unwrap(),
            )
            .unwrap(),
        );
        fp.quarantine(
            "a neighbor says the clerk is corrupt",
            "single-source hearsay, no document",
        );
        fp.derive("timeline starts at the verified filing date, not the rumor");
        assert_eq!(fp.facts().len(), 1);
        assert_eq!(
            fp.quarantined().len(),
            1,
            "quarantine is visible, not a drop"
        );
        // The type-level guarantee: there is no API admitting a
        // QuarantinedInput (or any unprovenanced value) into facts().
    }
}
