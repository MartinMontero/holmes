//! §6.2 hand-off artifact types (holmes-vs-wcjbt.md), as committed canon —
//! types and validation only (loop §6 Phase 0, lock 0d).
//!
//! Field shapes mirror the §6.2 YAML schemas exactly; where canon leaves a
//! field free-text this module keeps it a `String` rather than inventing
//! structure canon does not carry.

use std::fmt;

/// Validation failures for artifact construction. Every rejection is typed;
/// nothing is accepted with a warning (no warning-as-pass anywhere).
#[derive(Debug, Clone, PartialEq)]
pub enum ArtifactError {
    /// §6.4 invariant 5: a finding without non-empty provenance is rejected.
    EmptyProvenance,
    /// §6.4 invariant 5: confidence must lie in [0, 1].
    ConfidenceOutOfRange(f64),
    EmptyClaim,
    EmptyQuestion,
    EmptySource,
    EmptyValidFrom,
    /// Superseding must reference an existing finding index.
    NoSuchFinding(usize),
    /// A closed (handed-off) case file accepts no further mutation.
    CaseAlreadyHandedOff,
}

impl fmt::Display for ArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArtifactError::EmptyProvenance => {
                write!(f, "rejected: finding carries no provenance (invariant 5)")
            }
            ArtifactError::ConfidenceOutOfRange(v) => {
                write!(f, "rejected: confidence {v} outside [0, 1] (invariant 5)")
            }
            ArtifactError::EmptyClaim => write!(f, "rejected: empty claim"),
            ArtifactError::EmptyQuestion => write!(f, "rejected: empty question"),
            ArtifactError::EmptySource => write!(f, "rejected: provenance with empty source"),
            ArtifactError::EmptyValidFrom => write!(f, "rejected: empty valid_from date"),
            ArtifactError::NoSuchFinding(i) => write!(f, "rejected: no finding at index {i}"),
            ArtifactError::CaseAlreadyHandedOff => {
                write!(
                    f,
                    "rejected: case file already handed off (append-only after close)"
                )
            }
        }
    }
}

impl std::error::Error for ArtifactError {}

/// Opaque WCJBT catalog reference (`catalog_ref` in §6.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogRef(pub String);

/// `research_brief.origin` (§6.2: `enum[intent, build_time]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BriefOrigin {
    /// From WCJBT intent elicitation.
    Intent,
    /// From an Alfred build-time sourcing question (Loop D).
    BuildTime,
}

/// `research_brief` (§6.2) — producer: WCJBT or Alfred; consumer: Holmes.
/// The *input* artifact of the embedding contract.
#[derive(Debug, Clone)]
pub struct ResearchBrief {
    pub question: String,
    pub origin: BriefOrigin,
    pub scope: String,
    /// Loop B: search the catalog first.
    pub catalog_seed: Vec<CatalogRef>,
    /// The builder's self-reported certainty, traveling with the brief
    /// (epistemic canon §3). **Recorded but firewalled**: nothing in the
    /// analytical core reads this field — high confidence never lowers a
    /// verification requirement (illusion of validity). The firewall is
    /// regression-tested structurally, like the vendor denylist.
    pub stated_confidence: Option<Confidence>,
}

impl ResearchBrief {
    pub fn new(
        question: impl Into<String>,
        origin: BriefOrigin,
        scope: impl Into<String>,
        catalog_seed: Vec<CatalogRef>,
    ) -> Result<Self, ArtifactError> {
        let question = question.into();
        if question.trim().is_empty() {
            return Err(ArtifactError::EmptyQuestion);
        }
        Ok(Self {
            question,
            origin,
            scope: scope.into(),
            catalog_seed,
            stated_confidence: None,
        })
    }
}

/// `knowability` — assigned to every evidence pack *before* the confidence
/// score (epistemic canon §3, Upgrade B; schema amendment A-07 to §6.2):
/// is this question the kind that *can* be resolved, or fundamentally
/// uncertain? Shares vocabulary with WCJBT's `intuition_validity` so domain
/// classification flows coherently from intent into verification.
/// Assignment is **deterministic, never model-inferred** (canon §5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Knowability {
    /// Stable, learnable regularities; resolvable with evidence.
    HighValidity,
    /// Slow/noisy/absent feedback, novelty, irreducible uncertainty.
    LowValidity,
}

/// "Limits of this finding" — the structured boundary statement every
/// emitted evidence pack carries (epistemic canon §3; A-07). Not hedging:
/// explicit boundary-marking.
#[derive(Debug, Clone, Default)]
pub struct LimitsOfThisFinding {
    /// What new evidence would change the conclusion.
    pub what_would_change_the_conclusion: Vec<String>,
    /// What could not be checked (with why, where known).
    pub what_could_not_be_checked: Vec<String>,
    /// Where the evidence runs out.
    pub where_the_evidence_runs_out: Vec<String>,
}

impl LimitsOfThisFinding {
    /// An all-empty limits statement is no statement at all.
    pub fn is_empty(&self) -> bool {
        self.what_would_change_the_conclusion.is_empty()
            && self.what_could_not_be_checked.is_empty()
            && self.where_the_evidence_runs_out.is_empty()
    }
}

/// A finding's confidence, validated into [0, 1] at construction
/// (§6.2 `confidence: float # 0..1`; §6.4 invariant 5 rejects otherwise).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Confidence(f64);

impl Confidence {
    pub fn new(value: f64) -> Result<Self, ArtifactError> {
        if !(0.0..=1.0).contains(&value) || value.is_nan() {
            return Err(ArtifactError::ConfidenceOutOfRange(value));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

/// One provenance entry (§6.2 `provenance: [url] # named sources, verbatim
/// quotes`): a named source plus, where available, the verbatim quote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    /// Named primary source — URL or file+section locator. Never empty.
    pub source: String,
    /// Verbatim quote from the source, when one exists.
    pub quote: Option<String>,
}

impl Provenance {
    pub fn new(source: impl Into<String>, quote: Option<String>) -> Result<Self, ArtifactError> {
        let source = source.into();
        if source.trim().is_empty() {
            return Err(ArtifactError::EmptySource);
        }
        Ok(Self { source, quote })
    }
}

/// One evidence-pack finding (§6.2): claim + confidence + provenance +
/// bi-temporal validity. `valid_until` is `None` while current; superseding
/// sets it and preserves the finding — superseded, never deleted.
#[derive(Debug, Clone)]
pub struct Finding {
    claim: String,
    confidence: Confidence,
    provenance: Vec<Provenance>,
    valid_from: String,
    valid_until: Option<String>,
}

impl Finding {
    /// Constructing a finding *is* the §6.4-invariant-5 gate: empty
    /// provenance or out-of-range confidence cannot exist as a value.
    pub fn new(
        claim: impl Into<String>,
        confidence: Confidence,
        provenance: Vec<Provenance>,
        valid_from: impl Into<String>,
    ) -> Result<Self, ArtifactError> {
        let claim = claim.into();
        if claim.trim().is_empty() {
            return Err(ArtifactError::EmptyClaim);
        }
        if provenance.is_empty() {
            return Err(ArtifactError::EmptyProvenance);
        }
        let valid_from = valid_from.into();
        if valid_from.trim().is_empty() {
            return Err(ArtifactError::EmptyValidFrom);
        }
        Ok(Self {
            claim,
            confidence,
            provenance,
            valid_from,
            valid_until: None,
        })
    }

    pub fn claim(&self) -> &str {
        &self.claim
    }

    pub fn confidence(&self) -> Confidence {
        self.confidence
    }

    pub fn provenance(&self) -> &[Provenance] {
        &self.provenance
    }

    pub fn valid_from(&self) -> &str {
        &self.valid_from
    }

    /// `None` while current; `Some(date)` once superseded or expired.
    pub fn valid_until(&self) -> Option<&str> {
        self.valid_until.as_deref()
    }

    pub fn is_current(&self) -> bool {
        self.valid_until.is_none()
    }
}

/// `evidence_pack` (§6.2) — producer: Holmes; consumers: WCJBT, Alfred,
/// builder. Findings are append-only: superseding flags the old entry
/// invalid-from a date and appends the replacement; no removal API exists.
#[derive(Debug, Clone)]
pub struct EvidencePack {
    question: String,
    findings: Vec<Finding>,
    pub competing_hypotheses: Vec<String>,
    pub key_assumptions: Vec<String>,
    pub risk_flags: Vec<String>,
    /// §6.2: options/risks only — never a build directive. Holmes supplies
    /// verifiable evidence; the builder (and only the builder) decides.
    pub recommendation: Option<String>,
    /// A-07 (Upgrade B): domain classification, set deterministically and
    /// *before* any confidence score. Required at emission (lock 1a gate).
    pub knowability: Option<Knowability>,
    /// A-07 (Upgrade B): the structured boundary statement. Required and
    /// non-empty at emission (lock 1a gate).
    pub limits_of_this_finding: Option<LimitsOfThisFinding>,
}

impl EvidencePack {
    pub fn new(question: impl Into<String>) -> Result<Self, ArtifactError> {
        let question = question.into();
        if question.trim().is_empty() {
            return Err(ArtifactError::EmptyQuestion);
        }
        Ok(Self {
            question,
            findings: Vec::new(),
            competing_hypotheses: Vec::new(),
            key_assumptions: Vec::new(),
            risk_flags: Vec::new(),
            recommendation: None,
            knowability: None,
            limits_of_this_finding: None,
        })
    }

    pub fn question(&self) -> &str {
        &self.question
    }

    /// Every finding ever recorded, superseded entries included — the
    /// non-destructive-truth guarantee is that this list only grows.
    pub fn findings(&self) -> &[Finding] {
        &self.findings
    }

    pub fn current_findings(&self) -> impl Iterator<Item = &Finding> {
        self.findings.iter().filter(|f| f.is_current())
    }

    /// Returns the index of the appended finding.
    pub fn add_finding(&mut self, finding: Finding) -> usize {
        self.findings.push(finding);
        self.findings.len() - 1
    }

    /// Invalidation-not-deletion: the finding at `index` is flagged
    /// invalid from the replacement's `valid_from` and *kept*; the
    /// replacement is appended. Returns the replacement's index.
    pub fn supersede_finding(
        &mut self,
        index: usize,
        replacement: Finding,
    ) -> Result<usize, ArtifactError> {
        if index >= self.findings.len() {
            return Err(ArtifactError::NoSuchFinding(index));
        }
        self.findings[index].valid_until = Some(replacement.valid_from.clone());
        Ok(self.add_finding(replacement))
    }
}

/// Where a closed case file routes (spec §2 Phase 6, "Resolution & Handoff
/// — STRICT": Holmes has no authority; it routes complete, traceable case
/// files to a journalist, lawyer, or community channel). Until the
/// accountability assembly exists, the human reviewer is the interim block
/// (constitution #12).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandoffChannel {
    Journalist,
    Lawyer,
    CommunityChannel,
    /// Interim block: the human reviewer (constitution #12).
    HumanReviewer,
}

/// The recorded handoff of a closed case.
#[derive(Debug, Clone)]
pub struct Handoff {
    pub channel: HandoffChannel,
    pub note: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseStatus {
    Open,
    HandedOff,
}

/// `case_file` (§6.2) — the fully traceable container: the brief that
/// opened the case, the evidence produced, and — as the *only* terminal
/// state — a handoff to a human channel. There is no execute/apply/act
/// API: Holmes takes no autonomous action at resolution.
#[derive(Debug, Clone)]
pub struct CaseFile {
    brief: ResearchBrief,
    evidence: EvidencePack,
    handoff: Option<Handoff>,
}

impl CaseFile {
    /// Open a case from a research brief; the evidence pack inherits the
    /// brief's question so the chain stays traceable end-to-end.
    pub fn open(brief: ResearchBrief) -> Result<Self, ArtifactError> {
        let evidence = EvidencePack::new(brief.question.clone())?;
        Ok(Self {
            brief,
            evidence,
            handoff: None,
        })
    }

    pub fn brief(&self) -> &ResearchBrief {
        &self.brief
    }

    pub fn evidence(&self) -> &EvidencePack {
        &self.evidence
    }

    /// Mutable evidence access while the case is open; refused after
    /// handoff (a closed case file is immutable evidence).
    pub fn evidence_mut(&mut self) -> Result<&mut EvidencePack, ArtifactError> {
        if self.handoff.is_some() {
            return Err(ArtifactError::CaseAlreadyHandedOff);
        }
        Ok(&mut self.evidence)
    }

    pub fn status(&self) -> CaseStatus {
        if self.handoff.is_some() {
            CaseStatus::HandedOff
        } else {
            CaseStatus::Open
        }
    }

    pub fn handoff_record(&self) -> Option<&Handoff> {
        self.handoff.as_ref()
    }

    /// The single terminal transition: route the case to a human channel.
    /// Idempotence is refused — a case hands off exactly once.
    pub fn hand_off(
        &mut self,
        channel: HandoffChannel,
        note: impl Into<String>,
    ) -> Result<(), ArtifactError> {
        if self.handoff.is_some() {
            return Err(ArtifactError::CaseAlreadyHandedOff);
        }
        self.handoff = Some(Handoff {
            channel,
            note: note.into(),
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn provenance() -> Vec<Provenance> {
        vec![Provenance::new("docs/holmes-spec-v2.md §2", Some("quoted".into())).unwrap()]
    }

    #[test]
    fn invariant_5_unprovenanced_findings_cannot_exist() {
        let c = Confidence::new(0.9).unwrap();
        assert_eq!(
            Finding::new("claim", c, Vec::new(), "2026-07-19").unwrap_err(),
            ArtifactError::EmptyProvenance
        );
    }

    #[test]
    fn invariant_5_confidence_is_bounded() {
        assert!(Confidence::new(0.0).is_ok());
        assert!(Confidence::new(1.0).is_ok());
        assert_eq!(
            Confidence::new(1.2).unwrap_err(),
            ArtifactError::ConfidenceOutOfRange(1.2)
        );
        assert!(Confidence::new(-0.1).is_err());
        assert!(Confidence::new(f64::NAN).is_err());
    }

    #[test]
    fn superseding_preserves_the_old_finding_flagged() {
        let c = Confidence::new(0.8).unwrap();
        let mut pack = EvidencePack::new("q").unwrap();
        let first = pack.add_finding(Finding::new("v1", c, provenance(), "2026-07-01").unwrap());
        let second = pack
            .supersede_finding(
                first,
                Finding::new("v2", c, provenance(), "2026-07-19").unwrap(),
            )
            .unwrap();
        assert_eq!(pack.findings().len(), 2, "nothing was deleted");
        assert_eq!(pack.findings()[first].valid_until(), Some("2026-07-19"));
        assert!(!pack.findings()[first].is_current());
        assert!(pack.findings()[second].is_current());
        assert_eq!(pack.current_findings().count(), 1);
    }

    #[test]
    fn handoff_is_terminal_and_single() {
        let brief = ResearchBrief::new("q", BriefOrigin::BuildTime, "scope", Vec::new()).unwrap();
        let mut case = CaseFile::open(brief).unwrap();
        assert_eq!(case.status(), CaseStatus::Open);
        case.hand_off(HandoffChannel::HumanReviewer, "interim block")
            .unwrap();
        assert_eq!(case.status(), CaseStatus::HandedOff);
        assert_eq!(
            case.hand_off(HandoffChannel::Journalist, "again")
                .unwrap_err(),
            ArtifactError::CaseAlreadyHandedOff
        );
        assert_eq!(
            case.evidence_mut().unwrap_err(),
            ArtifactError::CaseAlreadyHandedOff
        );
    }
}
