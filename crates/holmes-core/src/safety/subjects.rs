//! Lock 2.5d — legal/defamation guardrails and the Sentinel asymmetry
//! (loop §6 Phase 2.5(iv); spec §6.4; constitution #11).
//!
//! Two permanent refusal gates and one evidence threshold:
//!
//! 1. **Targeting** — investigative tools are scoped to power, never
//!    private individuals. A private individual as investigation target
//!    is refused, permanently: the refusal type has no bypass
//!    constructor and no override API exists anywhere in the crate.
//! 2. **Disclosure** — Holmes adopts Blacksky's definition of doxxing
//!    verbatim (spec §6.4, VERIFIED against the spec's committed text,
//!    itself verified 2026-06-29 against docs.blacksky.community):
//!    "the act of disclosing someone's personal, non-public information
//!    — such as a real name, home address, phone number, or any other
//!    data that could be used to identify the individual — in an online
//!    forum or other public place without the person's consent."
//!    Disclosure of any [`InfoClass`] without recorded consent is
//!    refused **for any subject** — holders of power keep private-life
//!    protection; only conduct-in-role is investigable, and conduct is
//!    not an `InfoClass`.
//! 3. **Person-naming findings** — a finding naming a real, identifiable
//!    person carries a higher evidence bar at emission: at least
//!    [`PERSON_FINDING_ROOT_FLOOR`] independent source roots, a verbatim
//!    quote on *every* provenance entry (evidence-or-it-didn't-happen,
//!    applied to accusations), and a pack-level uncertainty statement.
//!    Labeling stays non-destructive by construction: findings are
//!    labels with provenance and confidence, superseded never deleted
//!    (spec §6.3), and resolution is handoff-only — Holmes takes no
//!    action against anyone.
//!
//! Thresholds here are deterministic floors, ASSUMED and documented
//! (canon gives no numbers); changing one is an amendment, not an edit.

use crate::analysis::emission::source_root;
use crate::artifacts::EvidencePack;
use std::collections::BTreeSet;
use std::fmt;

/// Independent-source floor for a finding that names a real person.
/// Stricter than the general lock-1a floor of 2 (ASSUMED value — the
/// canon requires a higher bar for person-naming claims but names no
/// number; amendable by D-item, never silently).
pub const PERSON_FINDING_ROOT_FLOOR: usize = 3;

/// Who a case is about. Registered explicitly by the deterministic side
/// (canon §5: never model-inferred) — the same operator contract as
/// `knowability` assignment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubjectScope {
    /// A structure of power: corporate/state entity, office, instrument,
    /// or a named person *in their role of power*. Naming a person here
    /// triggers the person-naming evidence threshold at emission.
    PowerStructure { name: String, role_note: String },
    /// A private individual. As a target: refused, permanently.
    PrivateIndividual { descriptor: String },
}

/// The personal, non-public information classes from the adopted
/// definition ("a real name, home address, phone number, or any other
/// data that could be used to identify the individual").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfoClass {
    RealNameOfPseudonymousPerson,
    HomeAddress,
    PhoneNumber,
    OtherIdentifyingData,
}

/// A recorded, written consent — **sealed** (F-036). The private witness
/// field means the only mint is [`ConsentRecord::record`], a deliberate
/// deterministic-operator-side call; no struct literal, no
/// deserialization, and no path acting on untrusted content can fabricate
/// one inline. This is the same standard the disclosure allow-token and
/// `SubjectScope` carry: consent is *asserted by the operator*, never
/// inferred from what a document claims (canon §5). Pinning it here
/// closes the adversarial-pass hole where a public `record: String` let
/// any caller mint consent by tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsentRecord {
    reference: String,
    _operator_recorded: (),
}

impl ConsentRecord {
    /// Record an operator-attested written consent, referencing where the
    /// signed release lives. Calling this *is* the operator's assertion
    /// that a real record exists — the type carries no power to verify
    /// that, exactly as `Knowability` assignment does not; the contract
    /// is that only the deterministic operator side calls it.
    pub fn record(reference: impl Into<String>) -> Result<Self, AntiDoxxingRefusal> {
        let reference = reference.into();
        if reference.trim().is_empty() {
            // An empty reference is not a record — it is the "no such
            // record" forgery the seal exists to block.
            return Err(AntiDoxxingRefusal::DisclosureWithoutConsent {
                info: InfoClass::OtherIdentifyingData,
            });
        }
        Ok(Self {
            reference,
            _operator_recorded: (),
        })
    }

    pub fn reference(&self) -> &str {
        &self.reference
    }
}

/// Consent state for a disclosure. Only a sealed [`ConsentRecord`]
/// counts; absence of objection is not consent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Consent {
    NotGiven,
    GivenInWriting(ConsentRecord),
}

/// The permanent refusal. No constructor bypass, no appeal-to-override
/// API — appeals route to the human accountability layer (Phase 5
/// shape), never back into the tool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AntiDoxxingRefusal {
    /// Investigative targeting of a private individual (Sentinel
    /// asymmetry, constitution #11).
    PrivateIndividualTarget { descriptor: String },
    /// Disclosure of personal, non-public information without consent —
    /// doxxing per the adopted definition, refused for any subject.
    DisclosureWithoutConsent { info: InfoClass },
}

impl fmt::Display for AntiDoxxingRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AntiDoxxingRefusal::PrivateIndividualTarget { descriptor } => write!(
                f,
                "refused permanently: investigative tools are scoped to power and are never \
                 aimed at private individuals (target: {descriptor}); this refusal has no \
                 override"
            ),
            AntiDoxxingRefusal::DisclosureWithoutConsent { info } => write!(
                f,
                "refused permanently: disclosing personal, non-public information ({info:?}) \
                 without the person's consent is doxxing under the adopted definition \
                 (spec §6.4); this refusal has no override"
            ),
        }
    }
}

impl std::error::Error for AntiDoxxingRefusal {}

/// Proof-of-assessment token for an allowed targeting (sealed).
#[derive(Debug, Clone)]
pub struct TargetingAllowed {
    _assessed: (),
}

/// Targeting gate: power structures may be investigated; private
/// individuals may not, ever.
pub fn assess_targeting(scope: &SubjectScope) -> Result<TargetingAllowed, AntiDoxxingRefusal> {
    match scope {
        SubjectScope::PowerStructure { .. } => Ok(TargetingAllowed { _assessed: () }),
        SubjectScope::PrivateIndividual { descriptor } => {
            Err(AntiDoxxingRefusal::PrivateIndividualTarget {
                descriptor: descriptor.clone(),
            })
        }
    }
}

/// Proof-of-assessment token for a consented disclosure (sealed).
#[derive(Debug, Clone)]
pub struct DisclosureAllowed {
    _assessed: (),
}

/// Disclosure gate: personal, non-public information classes require
/// recorded consent regardless of who the subject is.
pub fn assess_disclosure(
    info: InfoClass,
    consent: &Consent,
) -> Result<DisclosureAllowed, AntiDoxxingRefusal> {
    match consent {
        Consent::GivenInWriting(_) => Ok(DisclosureAllowed { _assessed: () }),
        Consent::NotGiven => Err(AntiDoxxingRefusal::DisclosureWithoutConsent { info }),
    }
}

/// Why a person-naming pack failed the defamation threshold.
#[derive(Debug, Clone, PartialEq)]
pub enum DefamationDenial {
    /// A current finding in a person-naming case has fewer than
    /// [`PERSON_FINDING_ROOT_FLOOR`] independent source roots.
    InsufficientCorroboration {
        finding_index: usize,
        independent_roots: usize,
    },
    /// A provenance entry on a person-naming finding carries no verbatim
    /// quote — accusations quote their evidence or they do not emit.
    QuotelessProvenance {
        finding_index: usize,
        provenance_index: usize,
    },
    /// Person-naming packs carry a prominent uncertainty statement,
    /// always (non-destructive labeling language: what is asserted is a
    /// provenanced label with stated limits, not a verdict).
    MissingUncertaintyStatement,
}

impl fmt::Display for DefamationDenial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DefamationDenial::InsufficientCorroboration {
                finding_index,
                independent_roots,
            } => write!(
                f,
                "emission denied: finding {finding_index} names a real person with \
                 {independent_roots} independent source root(s); the person-naming floor is \
                 {PERSON_FINDING_ROOT_FLOOR}"
            ),
            DefamationDenial::QuotelessProvenance {
                finding_index,
                provenance_index,
            } => write!(
                f,
                "emission denied: finding {finding_index} names a real person but provenance \
                 entry {provenance_index} carries no verbatim quote"
            ),
            DefamationDenial::MissingUncertaintyStatement => write!(
                f,
                "emission denied: a person-naming pack requires a prominent uncertainty \
                 statement (non-destructive labeling language)"
            ),
        }
    }
}

impl std::error::Error for DefamationDenial {}

/// The person-naming review, run at resolution over every current
/// finding when the case has registered subjects. Deterministic; reuses
/// the lock-1a independence heuristic (same documented floor semantics).
pub fn person_naming_review(pack: &EvidencePack) -> Result<(), DefamationDenial> {
    match &pack.uncertainty_statement {
        Some(s) if !s.trim().is_empty() => {}
        _ => return Err(DefamationDenial::MissingUncertaintyStatement),
    }
    for (finding_index, finding) in pack.findings().iter().enumerate() {
        if !finding.is_current() {
            continue;
        }
        for (provenance_index, p) in finding.provenance().iter().enumerate() {
            match &p.quote {
                Some(q) if !q.trim().is_empty() => {}
                _ => {
                    return Err(DefamationDenial::QuotelessProvenance {
                        finding_index,
                        provenance_index,
                    })
                }
            }
        }
        let roots: BTreeSet<String> = finding
            .provenance()
            .iter()
            .map(|p| source_root(&p.source))
            .collect();
        let independent_roots = roots.iter().filter(|r| !r.is_empty()).count();
        if independent_roots < PERSON_FINDING_ROOT_FLOOR {
            return Err(DefamationDenial::InsufficientCorroboration {
                finding_index,
                independent_roots,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::{Confidence, Finding, Provenance};

    #[test]
    fn private_individual_targeting_is_refused_permanently() {
        let target = SubjectScope::PrivateIndividual {
            descriptor: "a neighbor mentioned in a complaint".into(),
        };
        let refusal = assess_targeting(&target).unwrap_err();
        assert!(matches!(
            refusal,
            AntiDoxxingRefusal::PrivateIndividualTarget { .. }
        ));
        assert!(refusal.to_string().contains("no override"));
        // Power structures pass the targeting gate.
        assert!(assess_targeting(&SubjectScope::PowerStructure {
            name: "Acme Holdings LLC".into(),
            role_note: "beneficial-ownership question".into(),
        })
        .is_ok());
    }

    #[test]
    fn every_definition_info_class_is_refused_without_consent() {
        for info in [
            InfoClass::RealNameOfPseudonymousPerson,
            InfoClass::HomeAddress,
            InfoClass::PhoneNumber,
            InfoClass::OtherIdentifyingData,
        ] {
            assert_eq!(
                assess_disclosure(info, &Consent::NotGiven).unwrap_err(),
                AntiDoxxingRefusal::DisclosureWithoutConsent { info }
            );
        }
        assert!(assess_disclosure(
            InfoClass::RealNameOfPseudonymousPerson,
            &Consent::GivenInWriting(
                ConsentRecord::record("signed release, case file annex 2").unwrap()
            )
        )
        .is_ok());
    }

    /// F-036 regression: consent cannot be forged by an empty/absent
    /// record — the "no such record exists" laundering the adversarial
    /// pass found is refused at the sealed constructor.
    #[test]
    fn consent_record_rejects_an_empty_reference() {
        assert!(ConsentRecord::record("   ").is_err());
        assert!(ConsentRecord::record("").is_err());
        assert!(ConsentRecord::record("annex 2, signed 2026-07-01").is_ok());
    }

    fn quoted(source: &str) -> Provenance {
        Provenance::new(source, Some("verbatim words".into())).unwrap()
    }

    fn person_pack(provenance: Vec<Provenance>, uncertainty: Option<&str>) -> EvidencePack {
        let mut pack = EvidencePack::new("q").unwrap();
        pack.uncertainty_statement = uncertainty.map(|s| s.to_owned());
        pack.add_finding(
            Finding::new(
                "the registered agent signed both filings",
                Confidence::new(0.6).unwrap(),
                provenance,
                "2026-07-20",
            )
            .unwrap(),
        );
        pack
    }

    #[test]
    fn person_findings_need_three_roots_quotes_and_uncertainty() {
        // Two roots — under the person floor even though lock 1a passes.
        let two = person_pack(
            vec![
                quoted("https://registry.example.gov/e/1"),
                quoted("https://court.example.org/d/2"),
            ],
            Some("label with limits; contested facts remain contested"),
        );
        assert_eq!(
            person_naming_review(&two).unwrap_err(),
            DefamationDenial::InsufficientCorroboration {
                finding_index: 0,
                independent_roots: 2
            }
        );
        // Three roots but one quoteless entry.
        let quoteless = person_pack(
            vec![
                quoted("https://registry.example.gov/e/1"),
                quoted("https://court.example.org/d/2"),
                Provenance::new("https://gazette.example.net/n/3", None).unwrap(),
            ],
            Some("label with limits"),
        );
        assert!(matches!(
            person_naming_review(&quoteless).unwrap_err(),
            DefamationDenial::QuotelessProvenance {
                finding_index: 0,
                provenance_index: 2
            }
        ));
        // Three quoted roots, no uncertainty statement.
        let bare = person_pack(
            vec![
                quoted("https://registry.example.gov/e/1"),
                quoted("https://court.example.org/d/2"),
                quoted("https://gazette.example.net/n/3"),
            ],
            None,
        );
        assert_eq!(
            person_naming_review(&bare).unwrap_err(),
            DefamationDenial::MissingUncertaintyStatement
        );
        // All three requirements met — passes.
        let ok = person_pack(
            vec![
                quoted("https://registry.example.gov/e/1"),
                quoted("https://court.example.org/d/2"),
                quoted("https://gazette.example.net/n/3"),
            ],
            Some("this is a provenanced label, not a verdict; see limits"),
        );
        assert!(person_naming_review(&ok).is_ok());
    }

    /// The F-029 lesson applied at the stricter floor: decorated
    /// duplicates of one source cannot satisfy the person-naming bar.
    #[test]
    fn decorated_duplicates_cannot_reach_the_person_floor() {
        let decorated = person_pack(
            vec![
                quoted("https://alice@registry.example.gov/e/1"),
                quoted("https://registry.example.gov:443/e/1"),
                quoted("https://www.registry.example.gov/e/1"),
            ],
            Some("label with limits"),
        );
        assert_eq!(
            person_naming_review(&decorated).unwrap_err(),
            DefamationDenial::InsufficientCorroboration {
                finding_index: 0,
                independent_roots: 1
            }
        );
    }
}
