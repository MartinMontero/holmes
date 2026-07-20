//! The Six-Phase Case Method as a typed state machine (spec §2.3).
//!
//! Intake → La Lluvia → Collection → The Wall → Following the Money →
//! Resolution & Handoff. Transitions are forward-only and each demands
//! typed evidence that the phase's work happened; a case that fails
//! intake is Declined — terminal and preserved. The machine is pure
//! bookkeeping: every judgment it records arrives from outside; every
//! rule it enforces is compiled.

use super::ach::{AchCell, AchError, AchMatrix, AchVerdict};
use super::emission::{self, EmissionDenial, EmittedEvidencePack};
use super::hypothesis::{CalibrationStatus, Hypothesis, HypothesisId, LikelihoodRatio};
use super::kac::KeyAssumptionsCheck;
use crate::artifacts::{
    ArtifactError, CaseFile, HandoffChannel, Knowability, LimitsOfThisFinding, Provenance,
    ResearchBrief,
};
use crate::observability::telemetry::{CorrelationId, Telemetry, TelemetryEvent};
use crate::safety::subjects::{self, AntiDoxxingRefusal, DefamationDenial, SubjectScope};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CasePhase {
    /// Phase 1 — acceptance criteria: was someone harmed? systemic or
    /// isolated? can Holmes help *without creating more harm*?
    Intake,
    /// Phase 2 — hypothesis generation, including negative evidence.
    LaLluvia,
    /// Phase 3 — collection: documents state of mind.
    Collection,
    /// Phase 4 — the wall: ACH + Key Assumptions Check + el diablo.
    TheWall,
    /// Phase 5 — following the money.
    FollowingTheMoney,
    /// Phase 6 — terminal: emission-gated pack handed to a human channel.
    ResolutionHandoff,
    /// Terminal: intake refused the case. Preserved with the reason.
    Declined,
}

impl CasePhase {
    /// The phase's stable, content-free name (Phase 4 telemetry uses it as
    /// the `PhaseAdvanced` label; `Display` renders the same string).
    pub fn label(&self) -> &'static str {
        match self {
            CasePhase::Intake => "Intake",
            CasePhase::LaLluvia => "La Lluvia",
            CasePhase::Collection => "Collection",
            CasePhase::TheWall => "The Wall",
            CasePhase::FollowingTheMoney => "Following the Money",
            CasePhase::ResolutionHandoff => "Resolution & Handoff",
            CasePhase::Declined => "Declined",
        }
    }
}

impl fmt::Display for CasePhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemicOrIsolated {
    Systemic,
    Isolated,
}

/// The intake acceptance record (spec §2.3 Phase 1).
#[derive(Debug, Clone)]
pub struct IntakeAssessment {
    pub someone_harmed: bool,
    pub harm_note: String,
    pub systemic_or_isolated: SystemicOrIsolated,
    /// The controlling criterion: help without creating more harm.
    pub can_help_without_more_harm: bool,
    pub assessment_note: String,
}

/// One collected evidence item — provenanced or it does not enter
/// ("documents state of mind": assume a document exists for every claim).
#[derive(Debug, Clone)]
pub struct EvidenceItem {
    pub id: String,
    pub description: String,
    pub provenance: Vec<Provenance>,
}

/// One recorded devil's-advocate pass ("el diablo").
#[derive(Debug, Clone)]
pub struct DiabloPass {
    pub challenge: String,
    pub response: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PhaseError {
    WrongPhase {
        expected: &'static str,
        actual: String,
    },
    CaseDeclined,
    IntakeNotRecorded,
    /// Strong inference needs a field of alternatives.
    NeedAtLeastTwoLiveHypotheses(usize),
    NoEvidenceCollected,
    EvidenceNeedsProvenance,
    UnknownEvidence(String),
    AchNotBuilt,
    KacEmpty,
    KacUnexamined(Vec<usize>),
    NoDiabloPass,
    MoneyPhaseUnrecorded,
    Ach(AchError),
    Emission(EmissionDenial),
    Artifact(ArtifactError),
    /// Phase 2.5: the Sentinel-asymmetry refusal (permanent; registering
    /// a private-individual target also declines the case terminally).
    AntiDoxxing(AntiDoxxingRefusal),
    /// Phase 2.5: the person-naming evidence threshold failed at
    /// resolution.
    Defamation(DefamationDenial),
}

impl fmt::Display for PhaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhaseError::WrongPhase { expected, actual } => {
                write!(
                    f,
                    "refused: requires phase '{expected}', case is in '{actual}'"
                )
            }
            PhaseError::CaseDeclined => write!(f, "refused: case was declined at intake"),
            PhaseError::IntakeNotRecorded => {
                write!(f, "refused: intake assessment not recorded")
            }
            PhaseError::NeedAtLeastTwoLiveHypotheses(n) => write!(
                f,
                "refused: strong inference needs >= 2 live hypotheses, have {n}"
            ),
            PhaseError::NoEvidenceCollected => write!(f, "refused: no evidence collected"),
            PhaseError::EvidenceNeedsProvenance => {
                write!(f, "refused: evidence without provenance does not enter")
            }
            PhaseError::UnknownEvidence(id) => write!(f, "refused: unknown evidence id '{id}'"),
            PhaseError::AchNotBuilt => write!(f, "refused: ACH matrix not built"),
            PhaseError::KacEmpty => {
                write!(
                    f,
                    "refused: no key assumptions listed (¿Qué estoy asumiendo?)"
                )
            }
            PhaseError::KacUnexamined(idx) => {
                write!(f, "refused: key assumptions unexamined at indices {idx:?}")
            }
            PhaseError::NoDiabloPass => {
                write!(f, "refused: no devil's-advocate pass on record")
            }
            PhaseError::MoneyPhaseUnrecorded => write!(
                f,
                "refused: following-the-money phase has no record (an explicit, reasoned \
                 not-applicable counts; silence does not)"
            ),
            PhaseError::Ach(e) => write!(f, "{e}"),
            PhaseError::Emission(e) => write!(f, "{e}"),
            PhaseError::Artifact(e) => write!(f, "{e}"),
            PhaseError::AntiDoxxing(e) => write!(f, "{e}"),
            PhaseError::Defamation(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for PhaseError {}

impl From<AchError> for PhaseError {
    fn from(e: AchError) -> Self {
        PhaseError::Ach(e)
    }
}

impl From<EmissionDenial> for PhaseError {
    fn from(e: EmissionDenial) -> Self {
        PhaseError::Emission(e)
    }
}

impl From<ArtifactError> for PhaseError {
    fn from(e: ArtifactError) -> Self {
        PhaseError::Artifact(e)
    }
}

impl From<AntiDoxxingRefusal> for PhaseError {
    fn from(e: AntiDoxxingRefusal) -> Self {
        PhaseError::AntiDoxxing(e)
    }
}

impl From<DefamationDenial> for PhaseError {
    fn from(e: DefamationDenial) -> Self {
        PhaseError::Defamation(e)
    }
}

/// The analytical case: the §6.2 case file plus the working analytical
/// state the six phases accumulate.
#[derive(Debug)]
pub struct AnalyticalCase {
    file: CaseFile,
    phase: CasePhase,
    declined_reason: Option<String>,
    intake: Option<IntakeAssessment>,
    hypotheses: Vec<Hypothesis>,
    evidence: Vec<EvidenceItem>,
    ach: Option<AchMatrix>,
    ach_verdict: Option<AchVerdict>,
    kac: KeyAssumptionsCheck,
    diablo: Vec<DiabloPass>,
    money_notes: Vec<String>,
    /// Phase 2.5: the case's declared investigation subjects (canon §5:
    /// declared deterministically by the operator side, never
    /// model-inferred — the same contract as `knowability` assignment).
    subjects: Vec<SubjectScope>,
    /// Phase 4: born-redacted, opt-in telemetry and this case's
    /// correlation id. Disabled by default (records nothing); the
    /// operator enables it. Every recorded event is content-free by
    /// construction (see `observability::telemetry`).
    telemetry: Telemetry,
    correlation: CorrelationId,
}

impl AnalyticalCase {
    /// Open a case with telemetry **disabled** (the opt-in default): the
    /// analytical machine behaves identically and records nothing.
    pub fn open(brief: ResearchBrief) -> Result<Self, ArtifactError> {
        Self::open_with_telemetry(brief, Telemetry::disabled())
    }

    /// Open a case with telemetry **enabled** (the operator opted in);
    /// records `CaseOpened` under a fresh correlation id.
    pub fn open_observed(brief: ResearchBrief) -> Result<Self, ArtifactError> {
        Self::open_with_telemetry(brief, Telemetry::enabled())
    }

    fn open_with_telemetry(
        brief: ResearchBrief,
        telemetry: Telemetry,
    ) -> Result<Self, ArtifactError> {
        let mut case = Self {
            file: CaseFile::open(brief)?,
            phase: CasePhase::Intake,
            declined_reason: None,
            intake: None,
            hypotheses: Vec::new(),
            evidence: Vec::new(),
            ach: None,
            ach_verdict: None,
            kac: KeyAssumptionsCheck::new(),
            diablo: Vec::new(),
            money_notes: Vec::new(),
            subjects: Vec::new(),
            telemetry,
            correlation: CorrelationId::new(),
        };
        // No-op while disabled; the first recorded event when observed.
        case.telemetry
            .record(case.correlation, TelemetryEvent::CaseOpened);
        Ok(case)
    }

    /// Phase 4: opt into born-redacted telemetry mid-case. Events
    /// attempted while disabled were not recorded (opt-in); from here,
    /// content-free events are captured under this case's correlation id.
    pub fn enable_telemetry(&mut self) {
        self.telemetry.enable();
    }

    /// The case's telemetry recorder (read-only) — the embedder exports
    /// at the operator's initiative; the library never phones home.
    pub fn telemetry(&self) -> &Telemetry {
        &self.telemetry
    }

    /// This case's cross-stack correlation id.
    pub fn correlation(&self) -> CorrelationId {
        self.correlation
    }

    /// Phase 2.5: register a declared investigation subject, any phase
    /// before resolution. A private individual as subject is refused by
    /// the Sentinel asymmetry AND declines the case terminally — the
    /// refusal is permanent, and a case aimed at a private person does
    /// not continue under a corrected target.
    pub fn register_subject(&mut self, scope: SubjectScope) -> Result<(), PhaseError> {
        if self.phase == CasePhase::Declined {
            return Err(PhaseError::CaseDeclined);
        }
        if self.phase == CasePhase::ResolutionHandoff {
            return Err(PhaseError::WrongPhase {
                expected: "any phase before resolution",
                actual: self.phase.to_string(),
            });
        }
        match subjects::assess_targeting(&scope) {
            Ok(_) => {
                self.subjects.push(scope);
                Ok(())
            }
            Err(refusal) => {
                let class = refusal.class();
                self.telemetry
                    .record(self.correlation, TelemetryEvent::RefusalRaised { class });
                self.telemetry
                    .record(self.correlation, TelemetryEvent::CaseDeclined { class });
                self.declined_reason = Some(refusal.to_string());
                self.phase = CasePhase::Declined;
                Err(PhaseError::AntiDoxxing(refusal))
            }
        }
    }

    pub fn subjects(&self) -> &[SubjectScope] {
        &self.subjects
    }

    pub fn phase(&self) -> &CasePhase {
        &self.phase
    }

    pub fn file(&self) -> &CaseFile {
        &self.file
    }

    pub fn declined_reason(&self) -> Option<&str> {
        self.declined_reason.as_deref()
    }

    fn require_phase(&self, expected: CasePhase, name: &'static str) -> Result<(), PhaseError> {
        if self.phase == CasePhase::Declined {
            return Err(PhaseError::CaseDeclined);
        }
        if self.phase != expected {
            return Err(PhaseError::WrongPhase {
                expected: name,
                actual: self.phase.to_string(),
            });
        }
        Ok(())
    }

    // ---- Phase 1: Intake -------------------------------------------------

    /// Record the intake assessment. Failing the help-without-more-harm
    /// criterion declines the case on the spot (terminal, preserved).
    pub fn record_intake(&mut self, assessment: IntakeAssessment) -> Result<(), PhaseError> {
        self.require_phase(CasePhase::Intake, "Intake")?;
        if !assessment.can_help_without_more_harm {
            self.declined_reason = Some(format!(
                "intake declined: cannot help without creating more harm — {}",
                assessment.assessment_note
            ));
            self.intake = Some(assessment);
            self.phase = CasePhase::Declined;
            self.telemetry.record(
                self.correlation,
                TelemetryEvent::CaseDeclined {
                    class: "intake_harm_criterion",
                },
            );
            return Ok(());
        }
        self.intake = Some(assessment);
        Ok(())
    }

    pub fn advance_to_la_lluvia(&mut self) -> Result<(), PhaseError> {
        self.require_phase(CasePhase::Intake, "Intake")?;
        if self.intake.is_none() {
            return Err(PhaseError::IntakeNotRecorded);
        }
        self.phase = CasePhase::LaLluvia;
        self.telemetry.record(
            self.correlation,
            TelemetryEvent::PhaseAdvanced {
                phase: self.phase.label(),
            },
        );
        Ok(())
    }

    // ---- Phase 2: La Lluvia ----------------------------------------------

    pub fn add_hypothesis(&mut self, hypothesis: Hypothesis) -> Result<HypothesisId, PhaseError> {
        self.require_phase(CasePhase::LaLluvia, "La Lluvia")?;
        self.hypotheses.push(hypothesis);
        Ok(HypothesisId(self.hypotheses.len() - 1))
    }

    pub fn hypotheses(&self) -> &[Hypothesis] {
        &self.hypotheses
    }

    pub fn advance_to_collection(&mut self) -> Result<(), PhaseError> {
        self.require_phase(CasePhase::LaLluvia, "La Lluvia")?;
        let live = self.hypotheses.iter().filter(|h| h.is_live()).count();
        if live < 2 {
            return Err(PhaseError::NeedAtLeastTwoLiveHypotheses(live));
        }
        self.phase = CasePhase::Collection;
        self.telemetry.record(
            self.correlation,
            TelemetryEvent::PhaseAdvanced {
                phase: self.phase.label(),
            },
        );
        Ok(())
    }

    // ---- Phase 3: Collection ---------------------------------------------

    pub fn add_evidence(&mut self, item: EvidenceItem) -> Result<(), PhaseError> {
        self.require_phase(CasePhase::Collection, "Collection")?;
        if item.provenance.is_empty() {
            return Err(PhaseError::EvidenceNeedsProvenance);
        }
        self.evidence.push(item);
        Ok(())
    }

    pub fn evidence(&self) -> &[EvidenceItem] {
        &self.evidence
    }

    /// Apply a likelihood-ratio update to one hypothesis for one collected
    /// evidence item (Engine 1 over Engine-3-grade inputs).
    pub fn apply_lr(
        &mut self,
        hypothesis: HypothesisId,
        evidence_id: &str,
        lr: LikelihoodRatio,
    ) -> Result<(), PhaseError> {
        if !self.evidence.iter().any(|e| e.id == evidence_id) {
            return Err(PhaseError::UnknownEvidence(evidence_id.to_owned()));
        }
        let h = self
            .hypotheses
            .get_mut(hypothesis.0)
            .ok_or(PhaseError::Ach(AchError::UnknownHypothesis(hypothesis.0)))?;
        h.apply_update(evidence_id, lr);
        Ok(())
    }

    pub fn advance_to_wall(&mut self) -> Result<(), PhaseError> {
        self.require_phase(CasePhase::Collection, "Collection")?;
        if self.evidence.is_empty() {
            return Err(PhaseError::NoEvidenceCollected);
        }
        self.phase = CasePhase::TheWall;
        self.telemetry.record(
            self.correlation,
            TelemetryEvent::PhaseAdvanced {
                phase: self.phase.label(),
            },
        );
        Ok(())
    }

    // ---- Phase 4: The Wall -----------------------------------------------

    /// Build the ACH matrix over the live hypotheses × collected evidence.
    pub fn build_ach(&mut self) -> Result<(), PhaseError> {
        self.require_phase(CasePhase::TheWall, "The Wall")?;
        let live: Vec<HypothesisId> = self
            .hypotheses
            .iter()
            .enumerate()
            .filter(|(_, h)| h.is_live())
            .map(|(i, _)| HypothesisId(i))
            .collect();
        let evidence_ids: Vec<String> = self.evidence.iter().map(|e| e.id.clone()).collect();
        self.ach = Some(AchMatrix::new(live, evidence_ids)?);
        Ok(())
    }

    pub fn score_ach(
        &mut self,
        hypothesis: HypothesisId,
        evidence_id: &str,
        cell: AchCell,
    ) -> Result<(), PhaseError> {
        self.require_phase(CasePhase::TheWall, "The Wall")?;
        let ach = self.ach.as_mut().ok_or(PhaseError::AchNotBuilt)?;
        ach.score(hypothesis, evidence_id, cell)?;
        Ok(())
    }

    pub fn kac_mut(&mut self) -> &mut KeyAssumptionsCheck {
        &mut self.kac
    }

    pub fn kac(&self) -> &KeyAssumptionsCheck {
        &self.kac
    }

    pub fn record_diablo(&mut self, pass: DiabloPass) -> Result<(), PhaseError> {
        self.require_phase(CasePhase::TheWall, "The Wall")?;
        self.diablo.push(pass);
        Ok(())
    }

    pub fn diablo_passes(&self) -> &[DiabloPass] {
        &self.diablo
    }

    pub fn ach_verdict(&self) -> Option<&AchVerdict> {
        self.ach_verdict.as_ref()
    }

    /// Leave the wall: complete ACH verdict computed, every key
    /// assumption examined, and at least one el-diablo pass on record.
    pub fn advance_to_money(&mut self) -> Result<(), PhaseError> {
        self.require_phase(CasePhase::TheWall, "The Wall")?;
        let ach = self.ach.as_ref().ok_or(PhaseError::AchNotBuilt)?;
        let verdict = ach.verdict()?;
        if self.kac.is_empty() {
            return Err(PhaseError::KacEmpty);
        }
        let unexamined = self.kac.unexamined();
        if !unexamined.is_empty() {
            return Err(PhaseError::KacUnexamined(unexamined));
        }
        if self.diablo.is_empty() {
            return Err(PhaseError::NoDiabloPass);
        }
        self.ach_verdict = Some(verdict);
        self.phase = CasePhase::FollowingTheMoney;
        self.telemetry.record(
            self.correlation,
            TelemetryEvent::PhaseAdvanced {
                phase: self.phase.label(),
            },
        );
        Ok(())
    }

    // ---- Phase 5: Following the Money ------------------------------------

    /// Record the money-trail work — or an explicit, reasoned
    /// not-applicable. Silence is not an option; skipping is.
    pub fn record_money_note(&mut self, note: impl Into<String>) -> Result<(), PhaseError> {
        self.require_phase(CasePhase::FollowingTheMoney, "Following the Money")?;
        self.money_notes.push(note.into());
        Ok(())
    }

    pub fn money_notes(&self) -> &[String] {
        &self.money_notes
    }

    // ---- Phase 6: Resolution & Handoff -----------------------------------

    /// Mutable access to the case file's evidence pack for authoring
    /// findings, open phases only (delegates the handed-off check).
    pub fn evidence_pack_mut(&mut self) -> Result<&mut crate::artifacts::EvidencePack, PhaseError> {
        Ok(self.file.evidence_mut()?)
    }

    /// The single terminal transition: assemble the deterministic pack
    /// annotations, run the person-naming review (when subjects are
    /// registered) and the lock-1a + lock-2.5b emission gates, and hand
    /// off. The calibration status passed to emission is always
    /// `Uncalibrated` — the analytical core has no calibration evidence
    /// and no API to claim any (lock 2.5b). On any denial the case stays
    /// where it is, un-handed-off.
    pub fn resolve(
        &mut self,
        knowability: Knowability,
        limits: LimitsOfThisFinding,
        uncertainty_statement: Option<String>,
        channel: HandoffChannel,
        note: impl Into<String>,
    ) -> Result<EmittedEvidencePack, PhaseError> {
        self.require_phase(CasePhase::FollowingTheMoney, "Following the Money")?;
        if self.money_notes.is_empty() {
            return Err(PhaseError::MoneyPhaseUnrecorded);
        }
        let verdict = self.ach_verdict.clone().ok_or(PhaseError::AchNotBuilt)?;

        // Deterministic pack assembly from the case record.
        let competing: Vec<String> = self
            .hypotheses
            .iter()
            .map(|h| {
                if h.is_live() {
                    h.statement.clone()
                } else {
                    format!("[eliminated] {}", h.statement)
                }
            })
            .collect();
        let mut risk_flags: Vec<String> = self
            .kac
            .failed()
            .into_iter()
            .map(|i| {
                format!(
                    "failed key assumption: {}",
                    self.kac.assumptions()[i].statement
                )
            })
            .collect();
        if verdict.tie_at_top {
            risk_flags.push(
                "ACH tie at top: the matrix does not discriminate a single leading hypothesis"
                    .to_owned(),
            );
        }
        let assumptions: Vec<String> = self
            .kac
            .assumptions()
            .iter()
            .map(|a| a.statement.clone())
            .collect();
        {
            let pack = self.file.evidence_mut()?;
            pack.competing_hypotheses = competing;
            pack.key_assumptions = assumptions;
            pack.risk_flags.extend(risk_flags);
            pack.knowability = Some(knowability);
            pack.limits_of_this_finding = Some(limits);
            pack.uncertainty_statement = uncertainty_statement;
        }

        // Phase 2.5: a case with declared subjects names real entities —
        // every current finding meets the person-naming threshold.
        if !self.subjects.is_empty() {
            if let Err(denial) = subjects::person_naming_review(self.file.evidence()) {
                let class = denial.class();
                self.telemetry
                    .record(self.correlation, TelemetryEvent::EmissionDenied { class });
                return Err(denial.into());
            }
        }
        let emitted = match emission::emit(self.file.evidence(), CalibrationStatus::Uncalibrated) {
            Ok(e) => e,
            Err(denial) => {
                let class = denial.class();
                self.telemetry
                    .record(self.correlation, TelemetryEvent::EmissionDenied { class });
                return Err(denial.into());
            }
        };
        self.file.hand_off(channel, note)?;
        self.phase = CasePhase::ResolutionHandoff;
        // Phase 4 telemetry: the terminal transition, content-free.
        let channel_label = match channel {
            HandoffChannel::Journalist => "journalist",
            HandoffChannel::Lawyer => "lawyer",
            HandoffChannel::CommunityChannel => "community_channel",
            HandoffChannel::HumanReviewer => "human_reviewer",
        };
        self.telemetry
            .record(self.correlation, TelemetryEvent::PackEmitted);
        self.telemetry.record(
            self.correlation,
            TelemetryEvent::HandoffRecorded {
                channel: channel_label,
            },
        );
        Ok(emitted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::BriefOrigin;

    fn brief() -> ResearchBrief {
        ResearchBrief::new("q", BriefOrigin::BuildTime, "scope", Vec::new()).unwrap()
    }

    #[test]
    fn intake_harm_criterion_declines_terminally() {
        let mut case = AnalyticalCase::open(brief()).unwrap();
        case.record_intake(IntakeAssessment {
            someone_harmed: true,
            harm_note: "n".into(),
            systemic_or_isolated: SystemicOrIsolated::Isolated,
            can_help_without_more_harm: false,
            assessment_note: "investigation would expose the reporter".into(),
        })
        .unwrap();
        assert_eq!(*case.phase(), CasePhase::Declined);
        assert!(case.declined_reason().unwrap().contains("more harm"));
        assert_eq!(
            case.advance_to_la_lluvia().unwrap_err(),
            PhaseError::CaseDeclined
        );
    }

    #[test]
    fn phases_cannot_be_skipped() {
        let mut case = AnalyticalCase::open(brief()).unwrap();
        assert!(matches!(
            case.advance_to_collection().unwrap_err(),
            PhaseError::WrongPhase { .. }
        ));
        assert_eq!(
            case.advance_to_la_lluvia().unwrap_err(),
            PhaseError::IntakeNotRecorded
        );
    }
}
