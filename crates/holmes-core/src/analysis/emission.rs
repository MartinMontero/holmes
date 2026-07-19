//! Lock 1a — the emission gate.
//!
//! An evidence pack leaves Holmes only through this gate, which enforces
//! deterministically — no model call, and no input from the brief's
//! self-reported certainty (the canon §5 firewall; that field's name is
//! deliberately not written in this module so the structural firewall
//! test can assert its absence):
//!
//! 1. **Corroboration:** every current finding carries ≥ 2 provenance
//!    entries from *independent* sources (loop §6 lock 1a). Independence
//!    is judged by a documented heuristic — distinct normalized source
//!    roots — which is a deterministic *floor*, not a proof of true
//!    independence (two domains can share an owner); the heuristic is
//!    named in every denial so reviewers know what was checked.
//! 2. **Upgrade B schema (A-07):** `knowability` assigned and a non-empty
//!    `limits_of_this_finding` present.
//!
//! Non-empty provenance and confidence ∈ [0, 1] are already
//! unrepresentable at `Finding` construction (Phase 0); the gate
//! re-states them as its contract rather than trusting callers.

use crate::artifacts::EvidencePack;
use std::collections::BTreeSet;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum EmissionDenial {
    /// A current finding lacks ≥2 independent source roots. Carries the
    /// finding index, its distinct-root count, and the roots seen.
    Uncorroborated {
        finding_index: usize,
        independent_roots: usize,
        roots: Vec<String>,
    },
    /// `knowability` was never assigned (it must be set deterministically
    /// before any confidence talk — canon §3).
    KnowabilityUnassigned,
    /// The limits statement is absent or empty — a pack with no stated
    /// boundaries is not emittable (Upgrade B).
    LimitsMissing,
    /// Nothing to emit: a pack with no current findings is a report of
    /// work not done, not evidence.
    NoCurrentFindings,
}

impl fmt::Display for EmissionDenial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmissionDenial::Uncorroborated {
                finding_index,
                independent_roots,
                roots,
            } => write!(
                f,
                "emission denied: finding {finding_index} has {independent_roots} independent \
                 source root(s) {roots:?}; the corroboration gate requires >= 2 (independence \
                 heuristic: distinct normalized source roots)"
            ),
            EmissionDenial::KnowabilityUnassigned => {
                write!(f, "emission denied: knowability unassigned (A-07)")
            }
            EmissionDenial::LimitsMissing => write!(
                f,
                "emission denied: limits_of_this_finding absent or empty (A-07)"
            ),
            EmissionDenial::NoCurrentFindings => {
                write!(f, "emission denied: no current findings")
            }
        }
    }
}

impl std::error::Error for EmissionDenial {}

/// Proof-of-gate wrapper: the only way to obtain one is [`emit`], so an
/// `EmittedEvidencePack` *is* the record that the gate ran and passed.
#[derive(Debug, Clone)]
pub struct EmittedEvidencePack {
    pack: EvidencePack,
}

impl EmittedEvidencePack {
    pub fn pack(&self) -> &EvidencePack {
        &self.pack
    }
}

/// Normalize one provenance source to its independence "root".
/// URL-shaped sources (`scheme://host/...`) reduce to the host without a
/// leading `www.`; file+section sources (`path §n`, `path#frag`) reduce
/// to the path. Deterministic; documented as a heuristic floor.
pub fn source_root(source: &str) -> String {
    let s = source.trim().to_ascii_lowercase();
    if let Some(rest) = s.split_once("://").map(|(_, r)| r) {
        let host = rest.split(['/', '?', '#']).next().unwrap_or(rest);
        return host.strip_prefix("www.").unwrap_or(host).to_owned();
    }
    s.split([' ', '#'])
        .next()
        .unwrap_or(&s)
        .trim_end_matches('/')
        .to_owned()
}

/// Run the lock-1a gate over a pack. On success the pack is cloned into
/// the emitted wrapper; the source pack stays untouched (append-only
/// history remains with the case).
pub fn emit(pack: &EvidencePack) -> Result<EmittedEvidencePack, EmissionDenial> {
    if pack.knowability.is_none() {
        return Err(EmissionDenial::KnowabilityUnassigned);
    }
    match &pack.limits_of_this_finding {
        None => return Err(EmissionDenial::LimitsMissing),
        Some(l) if l.is_empty() => return Err(EmissionDenial::LimitsMissing),
        Some(_) => {}
    }
    let mut any_current = false;
    for (index, finding) in pack.findings().iter().enumerate() {
        if !finding.is_current() {
            continue;
        }
        any_current = true;
        let roots: BTreeSet<String> = finding
            .provenance()
            .iter()
            .map(|p| source_root(&p.source))
            .collect();
        if roots.len() < 2 {
            return Err(EmissionDenial::Uncorroborated {
                finding_index: index,
                independent_roots: roots.len(),
                roots: roots.into_iter().collect(),
            });
        }
    }
    if !any_current {
        return Err(EmissionDenial::NoCurrentFindings);
    }
    Ok(EmittedEvidencePack { pack: pack.clone() })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::{Confidence, Finding, Knowability, LimitsOfThisFinding, Provenance};

    fn pack_with(provenance: Vec<Provenance>) -> EvidencePack {
        let mut pack = EvidencePack::new("q").unwrap();
        pack.knowability = Some(Knowability::HighValidity);
        pack.limits_of_this_finding = Some(LimitsOfThisFinding {
            what_would_change_the_conclusion: vec!["a newer filing".into()],
            ..Default::default()
        });
        pack.add_finding(
            Finding::new(
                "claim",
                Confidence::new(0.8).unwrap(),
                provenance,
                "2026-07-19",
            )
            .unwrap(),
        );
        pack
    }

    #[test]
    fn source_roots_normalize_hosts_and_paths() {
        assert_eq!(source_root("https://www.example.org/a/b"), "example.org");
        assert_eq!(
            source_root("https://records.example.gov/x?y=1"),
            "records.example.gov"
        );
        assert_eq!(
            source_root("docs/holmes-spec-v2.md §2"),
            "docs/holmes-spec-v2.md"
        );
        assert_eq!(source_root("Docs/File.md#frag"), "docs/file.md");
    }

    #[test]
    fn lock1a_single_root_is_denied_two_independent_pass() {
        let same_root = pack_with(vec![
            Provenance::new("https://example.org/page-1", None).unwrap(),
            Provenance::new("https://www.example.org/page-2", None).unwrap(),
        ]);
        assert!(matches!(
            emit(&same_root).unwrap_err(),
            EmissionDenial::Uncorroborated {
                independent_roots: 1,
                ..
            }
        ));

        let independent = pack_with(vec![
            Provenance::new("https://example.org/page-1", None).unwrap(),
            Provenance::new("https://registry.example.gov/entity/9", None).unwrap(),
        ]);
        assert!(emit(&independent).is_ok());
    }

    #[test]
    fn lock1a_upgrade_b_fields_required() {
        let mut p = pack_with(vec![
            Provenance::new("https://a.example/1", None).unwrap(),
            Provenance::new("https://b.example/2", None).unwrap(),
        ]);
        p.knowability = None;
        assert_eq!(emit(&p).unwrap_err(), EmissionDenial::KnowabilityUnassigned);
        p.knowability = Some(Knowability::LowValidity);
        p.limits_of_this_finding = Some(LimitsOfThisFinding::default());
        assert_eq!(emit(&p).unwrap_err(), EmissionDenial::LimitsMissing);
    }

    #[test]
    fn empty_packs_do_not_emit() {
        let mut pack = EvidencePack::new("q").unwrap();
        pack.knowability = Some(Knowability::HighValidity);
        pack.limits_of_this_finding = Some(LimitsOfThisFinding {
            where_the_evidence_runs_out: vec!["no findings yet".into()],
            ..Default::default()
        });
        assert_eq!(emit(&pack).unwrap_err(), EmissionDenial::NoCurrentFindings);
    }
}
