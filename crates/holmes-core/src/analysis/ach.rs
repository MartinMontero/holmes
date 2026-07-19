//! Engine 2 — Analysis of Competing Hypotheses (Heuer; spec §2.2).
//!
//! The matrix of hypotheses × evidence with each cell scored
//! consistent / inconsistent / not-applicable. The verdict selects the
//! hypothesis with the **fewest inconsistencies** — the focus is
//! disproving alternatives, not proving a favorite — and reports which
//! evidence is *diagnostic* (scores differently across hypotheses;
//! evidence consistent with everything discriminates nothing).
//!
//! Cell judgments arrive from the model side (recipe/subagent); the
//! bookkeeping and verdict here are deterministic. A verdict on an
//! incomplete matrix is refused, not warned.

use super::hypothesis::HypothesisId;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AchCell {
    Consistent,
    Inconsistent,
    NotApplicable,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AchError {
    UnknownHypothesis(usize),
    UnknownEvidence(String),
    /// Verdict refused: `missing` cells are unscored.
    IncompleteMatrix {
        missing: usize,
    },
    /// A matrix needs at least two hypotheses to discriminate between
    /// (strong inference has nothing to do with a field of one).
    FewerThanTwoHypotheses,
    NoEvidence,
}

impl fmt::Display for AchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AchError::UnknownHypothesis(i) => write!(f, "rejected: unknown hypothesis id {i}"),
            AchError::UnknownEvidence(e) => write!(f, "rejected: unknown evidence id '{e}'"),
            AchError::IncompleteMatrix { missing } => {
                write!(f, "verdict refused: {missing} unscored matrix cell(s)")
            }
            AchError::FewerThanTwoHypotheses => {
                write!(f, "rejected: ACH needs at least two hypotheses")
            }
            AchError::NoEvidence => write!(f, "rejected: ACH needs at least one evidence item"),
        }
    }
}

impl std::error::Error for AchError {}

/// The deterministic ACH verdict.
#[derive(Debug, Clone)]
pub struct AchVerdict {
    /// (hypothesis, inconsistency count), fewest inconsistencies first;
    /// insertion order breaks ties (ties are also reported explicitly).
    pub ranking: Vec<(HypothesisId, usize)>,
    /// True when the top inconsistency count is shared — the matrix does
    /// not discriminate a single leader and saying otherwise would be
    /// false precision.
    pub tie_at_top: bool,
    /// Evidence ids that scored differently across hypotheses
    /// (diagnostic); evidence consistent with every hypothesis is listed
    /// nowhere here by design.
    pub diagnostic_evidence: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AchMatrix {
    hypotheses: Vec<HypothesisId>,
    evidence: Vec<String>,
    cells: BTreeMap<(HypothesisId, String), AchCell>,
}

impl AchMatrix {
    pub fn new(hypotheses: Vec<HypothesisId>, evidence: Vec<String>) -> Result<Self, AchError> {
        if hypotheses.len() < 2 {
            return Err(AchError::FewerThanTwoHypotheses);
        }
        if evidence.is_empty() {
            return Err(AchError::NoEvidence);
        }
        Ok(Self {
            hypotheses,
            evidence,
            cells: BTreeMap::new(),
        })
    }

    pub fn score(
        &mut self,
        hypothesis: HypothesisId,
        evidence_id: &str,
        cell: AchCell,
    ) -> Result<(), AchError> {
        if !self.hypotheses.contains(&hypothesis) {
            return Err(AchError::UnknownHypothesis(hypothesis.0));
        }
        if !self.evidence.iter().any(|e| e == evidence_id) {
            return Err(AchError::UnknownEvidence(evidence_id.to_owned()));
        }
        self.cells
            .insert((hypothesis, evidence_id.to_owned()), cell);
        Ok(())
    }

    pub fn unscored_cells(&self) -> usize {
        self.hypotheses.len() * self.evidence.len() - self.cells.len()
    }

    /// The fewest-inconsistencies verdict; refused while any cell is
    /// unscored.
    pub fn verdict(&self) -> Result<AchVerdict, AchError> {
        let missing = self.unscored_cells();
        if missing > 0 {
            return Err(AchError::IncompleteMatrix { missing });
        }

        let mut ranking: Vec<(HypothesisId, usize)> = self
            .hypotheses
            .iter()
            .map(|h| {
                let inconsistencies = self
                    .evidence
                    .iter()
                    .filter(|e| self.cells.get(&(*h, (*e).clone())) == Some(&AchCell::Inconsistent))
                    .count();
                (*h, inconsistencies)
            })
            .collect();
        ranking.sort_by_key(|(_, n)| *n);
        let tie_at_top = ranking.len() >= 2 && ranking[0].1 == ranking[1].1;

        let diagnostic_evidence = self
            .evidence
            .iter()
            .filter(|e| {
                let mut seen = Vec::new();
                for h in &self.hypotheses {
                    if let Some(c) = self.cells.get(&(*h, (*e).clone())) {
                        if *c != AchCell::NotApplicable && !seen.contains(c) {
                            seen.push(*c);
                        }
                    }
                }
                seen.len() > 1
            })
            .cloned()
            .collect();

        Ok(AchVerdict {
            ranking,
            tie_at_top,
            diagnostic_evidence,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix() -> AchMatrix {
        AchMatrix::new(
            vec![HypothesisId(0), HypothesisId(1)],
            vec!["E1".into(), "E2".into()],
        )
        .unwrap()
    }

    #[test]
    fn verdict_refused_on_incomplete_matrix() {
        let m = matrix();
        assert_eq!(
            m.verdict().unwrap_err(),
            AchError::IncompleteMatrix { missing: 4 }
        );
    }

    #[test]
    fn fewest_inconsistencies_wins_and_diagnosticity_reported() {
        let mut m = matrix();
        m.score(HypothesisId(0), "E1", AchCell::Consistent).unwrap();
        m.score(HypothesisId(1), "E1", AchCell::Inconsistent)
            .unwrap();
        m.score(HypothesisId(0), "E2", AchCell::Consistent).unwrap();
        m.score(HypothesisId(1), "E2", AchCell::Consistent).unwrap();
        let v = m.verdict().unwrap();
        assert_eq!(v.ranking[0], (HypothesisId(0), 0));
        assert_eq!(v.ranking[1], (HypothesisId(1), 1));
        assert!(!v.tie_at_top);
        // E1 discriminates; E2 (consistent with everything) does not.
        assert_eq!(v.diagnostic_evidence, vec!["E1".to_owned()]);
    }

    #[test]
    fn a_tie_at_the_top_is_reported_not_hidden() {
        let mut m = matrix();
        for e in ["E1", "E2"] {
            m.score(HypothesisId(0), e, AchCell::Consistent).unwrap();
            m.score(HypothesisId(1), e, AchCell::Consistent).unwrap();
        }
        let v = m.verdict().unwrap();
        assert!(v.tie_at_top, "an undiscriminating matrix must say so");
        assert!(v.diagnostic_evidence.is_empty());
    }

    #[test]
    fn needs_two_hypotheses_and_some_evidence() {
        assert_eq!(
            AchMatrix::new(vec![HypothesisId(0)], vec!["E1".into()]).unwrap_err(),
            AchError::FewerThanTwoHypotheses
        );
        assert_eq!(
            AchMatrix::new(vec![HypothesisId(0), HypothesisId(1)], vec![]).unwrap_err(),
            AchError::NoEvidence
        );
    }
}
