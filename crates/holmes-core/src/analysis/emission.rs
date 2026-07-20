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
//! 3. **Calibration gating (Phase 2.5, lock 2.5b):** an uncalibrated
//!    likelihood cannot surface as a confident finding. The analytical
//!    core only ever assigns `CalibrationStatus::Uncalibrated` (nothing
//!    in the core mints `Calibrated`; that status arrives only when real
//!    calibration evidence exists — no fake calibration machinery is
//!    built here), so today every finding at or above
//!    [`CONFIDENT_FLOOR`] is denied with the downgrade path named. This
//!    is the loop's "calibration fallback … a safety control": the
//!    fallback IS the cap.
//! 4. **Knowability gating (Phase 2.5, lock 2.5b):** no bare high
//!    confidence in a low-`knowability` domain — a confident finding in
//!    a `LowValidity` pack additionally requires the prominent
//!    uncertainty statement (canon §4/§5; the decline-or-downgrade rule).
//!
//! Non-empty provenance and confidence ∈ [0, 1] are already
//! unrepresentable at `Finding` construction (Phase 0); the gate
//! re-states them as its contract rather than trusting callers.

use crate::analysis::hypothesis::CalibrationStatus;
use crate::artifacts::{Confidence, EvidencePack, Finding, Knowability};
use std::collections::BTreeSet;
use std::fmt;

/// The confidence at or above which a finding counts as "confident" for
/// the Phase 2.5 gates. ASSUMED (canon names the rule, not the number;
/// 0.75 = 3:1 odds); documented here and in every denial; amendable by
/// D-item, never silently.
pub const CONFIDENT_FLOOR: f64 = 0.75;

/// Where [`downgrade_uncalibrated`] caps a confident-but-uncalibrated
/// finding: strictly below [`CONFIDENT_FLOOR`], visibly not at it.
pub const CALIBRATION_CAP: f64 = 0.7;

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
    /// Lock 2.5b: a confident finding whose judgment source carries no
    /// calibration evidence. The remedy is named in the denial: cap the
    /// confidence (see [`downgrade_uncalibrated`]) or supply real
    /// calibration evidence — there is no third path.
    UncalibratedConfidence {
        finding_index: usize,
        confidence: f64,
    },
    /// Lock 2.5b: bare high confidence in a low-knowability domain — the
    /// pack lacks the prominent uncertainty statement canon §4 requires.
    BareHighConfidenceInLowValidity {
        finding_index: usize,
        confidence: f64,
    },
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
                 heuristic: distinct normalized source roots; an empty root never counts)"
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
            EmissionDenial::UncalibratedConfidence {
                finding_index,
                confidence,
            } => write!(
                f,
                "emission denied: finding {finding_index} carries confidence {confidence} >= \
                 {CONFIDENT_FLOOR} with no calibration evidence; downgrade to <= \
                 {CALIBRATION_CAP} (invalidation-not-deletion; see downgrade_uncalibrated) or \
                 supply real calibration evidence"
            ),
            EmissionDenial::BareHighConfidenceInLowValidity {
                finding_index,
                confidence,
            } => write!(
                f,
                "emission denied: finding {finding_index} carries confidence {confidence} >= \
                 {CONFIDENT_FLOOR} in a low-knowability domain with no prominent uncertainty \
                 statement (canon Upgrade B: decline, downgrade, or attach the statement)"
            ),
        }
    }
}

impl std::error::Error for EmissionDenial {}

impl EmissionDenial {
    /// A stable, content-free class label for telemetry (Phase 4): the
    /// *kind* of denial, never the finding, roots, or reason text.
    pub fn class(&self) -> &'static str {
        match self {
            EmissionDenial::Uncorroborated { .. } => "uncorroborated",
            EmissionDenial::KnowabilityUnassigned => "knowability_unassigned",
            EmissionDenial::LimitsMissing => "limits_missing",
            EmissionDenial::NoCurrentFindings => "no_current_findings",
            EmissionDenial::UncalibratedConfidence { .. } => "uncalibrated_confidence",
            EmissionDenial::BareHighConfidenceInLowValidity { .. } => {
                "bare_high_confidence_low_validity"
            }
        }
    }
}

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
/// to the lexically normalized path. Deterministic; documented as a
/// heuristic floor.
pub fn source_root(source: &str) -> String {
    // F-031: strip tab/CR/LF anywhere (interior whitespace must not mint
    // a new host) and fold `\` into `/` — backslash spellings of one
    // authority or path reach the same place.
    let s: String = source
        .trim()
        .to_ascii_lowercase()
        .chars()
        .filter(|c| !matches!(c, '\t' | '\r' | '\n'))
        .map(|c| if c == '\\' { '/' } else { c })
        .collect();
    if let Some(rest) = s.split_once("://").map(|(_, r)| r) {
        let authority = rest.split(['/', '?', '#']).next().unwrap_or(rest);
        if authority.is_empty() {
            // No authority (`file:///path`, slash-count decorations): the
            // path is the identity — same rules as a file source.
            return path_root(rest);
        }
        // F-029: drop userinfo (`alice@example.org` and `bob@example.org`
        // are one host — decoration must not fabricate independence; the
        // L1a proxy's s4 rule rejects userinfo outright, this heuristic
        // strips it).
        let host = authority.rsplit('@').next().unwrap_or(authority);
        return host_root(host);
    }
    path_root(&s)
}

/// Host rules (userinfo already removed): shed the port — the
/// default-port spelling of one host is the same host, and collapsing
/// non-default ports only tightens the floor — trim trailing dots on both
/// sides of the port strip (F-029's FQDN form, incl. `host.:443` /
/// `host:443.`), then drop a leading `www.`.
///
/// Granularity note (documented, deliberate): subdomains of one
/// registrable domain (`a.example.org` vs `b.example.org`) DO count as
/// distinct roots — collapsing to registrable domains needs a
/// public-suffix list, a dependency this floor heuristic does not take.
/// Likewise carried (F-031, Phase 2.5 adversarial corpus): IPv4 numeric
/// forms, IPv6 compression, IDN/punycode, and percent-encoded hosts are
/// not canonicalized here.
fn host_root(host: &str) -> String {
    let host = host.trim_end_matches('.');
    let host = strip_port(host);
    let host = host.trim_end_matches('.');
    host.strip_prefix("www.").unwrap_or(host).to_owned()
}

/// Shed a trailing `:port`. `all()` is true on empty, so the empty-port
/// form (`host:`) sheds too; `[::1]:8080` splits at the final `:` (the
/// port), while `[::1]` alone does not match (suffix `1]` is not digits).
fn strip_port(s: &str) -> &str {
    match s.rsplit_once(':') {
        Some((head, port)) if port.chars().all(|c| c.is_ascii_digit()) => head,
        _ => s,
    }
}

/// Path rules for file+section sources: cut the section marker (space,
/// `#`, or `§` — attached or not), shed trailing `,`/`;` and `:line`
/// decorations, then normalize lexically (`//` and `.` segments dropped,
/// `..` resolved, no leading or trailing `/`). A host-like first segment
/// (`example.org/page` — scheme-less URL citation) collapses to the host
/// rules so one site cannot mint one root per page. Absolute-vs-relative
/// spellings of one file remain distinct — this floor has no repo-root
/// knowledge; carried with the other F-031 shapes to the Phase 2.5
/// adversarial corpus.
fn path_root(s: &str) -> String {
    let head = s.split([' ', '#', '§']).next().unwrap_or(s);
    let head = strip_port(head.trim_end_matches([',', ';']));
    let mut segments: Vec<&str> = Vec::new();
    for segment in head.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                // Unmatched `..` (no segment to pop) is kept: a
                // parent-relative citation stays visibly distinct rather
                // than silently collapsing.
                if segments.pop().is_none() {
                    segments.push("..");
                }
            }
            _ => segments.push(segment),
        }
    }
    if segments.len() > 1 && segments[0].contains('.') {
        return host_root(segments[0]);
    }
    segments.join("/")
}

/// Run the lock-1a + lock-2.5b gates over a pack. `calibration` is the
/// case's judgment-source calibration status — the analytical core only
/// ever supplies `Uncalibrated` (it has no mint for `Calibrated`), so
/// callers cannot talk the gate into leniency. On success the pack is
/// cloned into the emitted wrapper; the source pack stays untouched
/// (append-only history remains with the case).
pub fn emit(
    pack: &EvidencePack,
    calibration: CalibrationStatus,
) -> Result<EmittedEvidencePack, EmissionDenial> {
    let knowability = match pack.knowability {
        None => return Err(EmissionDenial::KnowabilityUnassigned),
        Some(k) => k,
    };
    match &pack.limits_of_this_finding {
        None => return Err(EmissionDenial::LimitsMissing),
        Some(l) if l.is_empty() => return Err(EmissionDenial::LimitsMissing),
        Some(_) => {}
    }
    let uncertainty_present = pack
        .uncertainty_statement
        .as_deref()
        .is_some_and(|s| !s.trim().is_empty());
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
        // F-031: a source that normalizes to an empty root (fragment-only,
        // bare scheme) corroborates nothing; the denial still lists it so
        // reviewers see what was cited.
        let independent_roots = roots.iter().filter(|r| !r.is_empty()).count();
        if independent_roots < 2 {
            return Err(EmissionDenial::Uncorroborated {
                finding_index: index,
                independent_roots,
                roots: roots.into_iter().collect(),
            });
        }
        let confidence = finding.confidence().value();
        if confidence >= CONFIDENT_FLOOR {
            // Lock 2.5b, rule 1: no confident finding without calibration
            // evidence. Checked before the knowability rule so the denial
            // names the binding constraint.
            if calibration == CalibrationStatus::Uncalibrated {
                return Err(EmissionDenial::UncalibratedConfidence {
                    finding_index: index,
                    confidence,
                });
            }
            // Lock 2.5b, rule 2: no bare high confidence in a
            // low-knowability domain (canon Upgrade B).
            if knowability == Knowability::LowValidity && !uncertainty_present {
                return Err(EmissionDenial::BareHighConfidenceInLowValidity {
                    finding_index: index,
                    confidence,
                });
            }
        }
    }
    if !any_current {
        return Err(EmissionDenial::NoCurrentFindings);
    }
    Ok(EmittedEvidencePack { pack: pack.clone() })
}

/// The named downgrade path for [`EmissionDenial::UncalibratedConfidence`]:
/// every confident current finding is superseded (invalidation-not-
/// deletion — the original stays, flagged) by a copy capped at
/// [`CALIBRATION_CAP`], and each downgrade is recorded visibly as a risk
/// flag. Returns how many findings were downgraded. Deterministic; the
/// caller decides whether downgrading is analytically honest — this
/// helper only makes it mechanical and auditable.
pub fn downgrade_uncalibrated(pack: &mut EvidencePack) -> usize {
    let confident: Vec<(usize, f64)> = pack
        .findings()
        .iter()
        .enumerate()
        .filter(|(_, f)| f.is_current() && f.confidence().value() >= CONFIDENT_FLOOR)
        .map(|(i, f)| (i, f.confidence().value()))
        .collect();
    for &(index, original) in &confident {
        let source = &pack.findings()[index];
        let capped = Finding::new(
            source.claim().to_owned(),
            Confidence::new(CALIBRATION_CAP).expect("cap is in range"),
            source.provenance().to_vec(),
            source.valid_from().to_owned(),
        )
        .expect("copy of a valid finding is valid");
        pack.supersede_finding(index, capped)
            .expect("index enumerated from this pack");
        pack.risk_flags.push(format!(
            "calibration downgrade: finding {index} capped {original} -> {CALIBRATION_CAP} \
             (uncalibrated likelihood; Phase 2.5 gate)"
        ));
    }
    confident.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::{Confidence, Finding, Knowability, LimitsOfThisFinding, Provenance};

    // Fixture confidence sits below CONFIDENT_FLOOR so the lock-1a tests
    // keep exercising corroboration, not the 2.5b calibration gate (which
    // has its own tests below).
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
                Confidence::new(0.7).unwrap(),
                provenance,
                "2026-07-19",
            )
            .unwrap(),
        );
        pack
    }

    fn independent_provenance() -> Vec<Provenance> {
        vec![
            Provenance::new("https://example.org/page-1", None).unwrap(),
            Provenance::new("https://registry.example.gov/entity/9", None).unwrap(),
        ]
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

    /// F-029 regression: decorated duplicates of ONE source must not
    /// satisfy the ≥2-independent floor.
    #[test]
    fn userinfo_cannot_fabricate_independent_roots() {
        // Userinfo variation collapses to one root.
        assert_eq!(source_root("https://alice@example.org/a"), "example.org");
        assert_eq!(source_root("https://bob@example.org/b"), "example.org");
        // Trailing-dot FQDN form collapses too.
        assert_eq!(source_root("https://example.org./c"), "example.org");
        assert_eq!(
            source_root("https://user:pw@www.example.org./d"),
            "example.org"
        );
        // Documented granularity: distinct subdomains remain distinct
        // roots (no public-suffix collapse in this floor heuristic).
        assert_ne!(
            source_root("https://a.example.org/"),
            source_root("https://b.example.org/")
        );
    }

    /// F-031 regression: the decoration shapes confirmed by the
    /// adversarial pass on the F-029 fix — ports, whitespace, backslash
    /// and slash-count spellings, scheme-less URLs, and file-path
    /// decorations — must not mint extra roots.
    #[test]
    fn port_path_and_scheme_decorations_cannot_fabricate_independent_roots() {
        // Port spellings of one host are one root: default, non-default,
        // empty, and combined with the trailing-dot FQDN form.
        assert_eq!(source_root("https://example.org:443/a"), "example.org");
        assert_eq!(source_root("http://example.org:80/a"), "example.org");
        assert_eq!(source_root("https://example.org:/a"), "example.org");
        assert_eq!(source_root("https://example.org:8080/a"), "example.org");
        assert_eq!(source_root("https://example.org.:443/a"), "example.org");
        assert_eq!(source_root("https://example.org../a"), "example.org");
        // Interior tab and backslash / slash-count spellings.
        assert_eq!(source_root("https://exa\tmple.org/x"), "example.org");
        assert_eq!(source_root("https:\\\\example.org\\a"), "example.org");
        assert_eq!(source_root("https:///example.org/a"), "example.org");
        // A scheme-less citation of one site collapses to the host — not
        // one root per page…
        assert_eq!(source_root("example.org/page-1"), "example.org");
        assert_eq!(source_root("www.example.org/page-2"), "example.org");
        // …while real file paths keep path identity under every spelling.
        assert_eq!(
            source_root("./docs/holmes-spec-v2.md §2"),
            "docs/holmes-spec-v2.md"
        );
        assert_eq!(
            source_root("docs//holmes-spec-v2.md §2"),
            "docs/holmes-spec-v2.md"
        );
        assert_eq!(
            source_root("docs/./holmes-spec-v2.md"),
            "docs/holmes-spec-v2.md"
        );
        assert_eq!(
            source_root("docs/../docs/holmes-spec-v2.md"),
            "docs/holmes-spec-v2.md"
        );
        assert_eq!(
            source_root("docs\\holmes-spec-v2.md §2"),
            "docs/holmes-spec-v2.md"
        );
        // Section-marker decorations: attached §, trailing comma, :line.
        assert_eq!(
            source_root("docs/holmes-spec-v2.md§2"),
            "docs/holmes-spec-v2.md"
        );
        assert_eq!(
            source_root("docs/holmes-spec-v2.md, §2"),
            "docs/holmes-spec-v2.md"
        );
        assert_eq!(
            source_root("docs/holmes-spec-v2.md:12"),
            "docs/holmes-spec-v2.md"
        );
        // IPv6: the port sheds, the bracketed literal survives intact.
        assert_eq!(source_root("http://[::1]:8080/x"), "[::1]");
        assert_eq!(source_root("http://[::1]/x"), "[::1]");
        // Documented granularity (carried to the Phase 2.5 corpus):
        // absolute vs repo-relative spellings of one file stay distinct —
        // this floor has no repo-root knowledge.
        assert_ne!(
            source_root("/home/user/holmes/docs/x.md"),
            source_root("docs/x.md")
        );
    }

    /// F-031: a source that normalizes to an empty root (fragment-only,
    /// bare scheme) contributes nothing to the corroboration count.
    #[test]
    fn empty_roots_do_not_count_toward_corroboration() {
        assert_eq!(source_root("#appendix"), "");
        let pack = pack_with(vec![
            Provenance::new("docs/holmes-spec-v2.md §1", None).unwrap(),
            Provenance::new("#appendix", None).unwrap(),
        ]);
        assert!(matches!(
            emit(&pack, CalibrationStatus::Uncalibrated).unwrap_err(),
            EmissionDenial::Uncorroborated {
                independent_roots: 1,
                ..
            }
        ));
    }

    #[test]
    fn lock1a_single_root_is_denied_two_independent_pass() {
        let same_root = pack_with(vec![
            Provenance::new("https://example.org/page-1", None).unwrap(),
            Provenance::new("https://www.example.org/page-2", None).unwrap(),
        ]);
        assert!(matches!(
            emit(&same_root, CalibrationStatus::Uncalibrated).unwrap_err(),
            EmissionDenial::Uncorroborated {
                independent_roots: 1,
                ..
            }
        ));

        let independent = pack_with(vec![
            Provenance::new("https://example.org/page-1", None).unwrap(),
            Provenance::new("https://registry.example.gov/entity/9", None).unwrap(),
        ]);
        assert!(emit(&independent, CalibrationStatus::Uncalibrated).is_ok());
    }

    #[test]
    fn lock1a_upgrade_b_fields_required() {
        let mut p = pack_with(vec![
            Provenance::new("https://a.example/1", None).unwrap(),
            Provenance::new("https://b.example/2", None).unwrap(),
        ]);
        p.knowability = None;
        assert_eq!(
            emit(&p, CalibrationStatus::Uncalibrated).unwrap_err(),
            EmissionDenial::KnowabilityUnassigned
        );
        p.knowability = Some(Knowability::LowValidity);
        p.limits_of_this_finding = Some(LimitsOfThisFinding::default());
        assert_eq!(
            emit(&p, CalibrationStatus::Uncalibrated).unwrap_err(),
            EmissionDenial::LimitsMissing
        );
    }

    /// Lock 2.5b, rule 1: an uncalibrated likelihood cannot surface as a
    /// confident finding — and the named downgrade path works, preserving
    /// the original finding flagged (invalidation-not-deletion).
    #[test]
    fn lock2_5b_uncalibrated_confidence_is_denied_and_downgrade_recovers() {
        let mut pack = pack_with(independent_provenance());
        pack.add_finding(
            Finding::new(
                "confident claim",
                Confidence::new(0.9).unwrap(),
                independent_provenance(),
                "2026-07-20",
            )
            .unwrap(),
        );
        assert_eq!(
            emit(&pack, CalibrationStatus::Uncalibrated).unwrap_err(),
            EmissionDenial::UncalibratedConfidence {
                finding_index: 1,
                confidence: 0.9
            }
        );
        // Boundary: exactly the floor is confident.
        let mut at_floor = pack_with(independent_provenance());
        at_floor.add_finding(
            Finding::new(
                "at-floor claim",
                Confidence::new(CONFIDENT_FLOOR).unwrap(),
                independent_provenance(),
                "2026-07-20",
            )
            .unwrap(),
        );
        assert!(matches!(
            emit(&at_floor, CalibrationStatus::Uncalibrated).unwrap_err(),
            EmissionDenial::UncalibratedConfidence { .. }
        ));
        // The downgrade path: cap, keep the original superseded, flag it.
        let downgraded = downgrade_uncalibrated(&mut pack);
        assert_eq!(downgraded, 1);
        assert_eq!(pack.findings().len(), 3, "original kept, capped appended");
        assert!(!pack.findings()[1].is_current());
        assert_eq!(
            pack.findings()[2].confidence().value(),
            CALIBRATION_CAP,
            "capped strictly below the floor"
        );
        assert!(pack
            .risk_flags
            .iter()
            .any(|r| r.contains("calibration downgrade")));
        assert!(emit(&pack, CalibrationStatus::Uncalibrated).is_ok());
    }

    /// Lock 2.5b, rule 2 (the lock's own fixture): high-confidence
    /// emission in a low-knowability domain is blocked without the
    /// prominent uncertainty statement, and passes with it. Exercised
    /// with `Calibrated` so rule 1 does not mask it — the core itself
    /// never mints that status (test-only construction).
    #[test]
    fn lock2_5b_low_knowability_blocks_bare_high_confidence() {
        let mut pack = pack_with(independent_provenance());
        pack.knowability = Some(Knowability::LowValidity);
        pack.add_finding(
            Finding::new(
                "confident claim in an unknowable domain",
                Confidence::new(0.9).unwrap(),
                independent_provenance(),
                "2026-07-20",
            )
            .unwrap(),
        );
        assert_eq!(
            emit(&pack, CalibrationStatus::Calibrated).unwrap_err(),
            EmissionDenial::BareHighConfidenceInLowValidity {
                finding_index: 1,
                confidence: 0.9
            }
        );
        pack.uncertainty_statement =
            Some("irreducible uncertainty: feedback in this domain is absent".into());
        assert!(emit(&pack, CalibrationStatus::Calibrated).is_ok());
        // A whitespace-only statement is no statement.
        pack.uncertainty_statement = Some("   ".into());
        assert!(matches!(
            emit(&pack, CalibrationStatus::Calibrated).unwrap_err(),
            EmissionDenial::BareHighConfidenceInLowValidity { .. }
        ));
        // High-validity domains do not require the statement (calibrated).
        pack.knowability = Some(Knowability::HighValidity);
        pack.uncertainty_statement = None;
        assert!(emit(&pack, CalibrationStatus::Calibrated).is_ok());
    }

    #[test]
    fn empty_packs_do_not_emit() {
        let mut pack = EvidencePack::new("q").unwrap();
        pack.knowability = Some(Knowability::HighValidity);
        pack.limits_of_this_finding = Some(LimitsOfThisFinding {
            where_the_evidence_runs_out: vec!["no findings yet".into()],
            ..Default::default()
        });
        assert_eq!(
            emit(&pack, CalibrationStatus::Uncalibrated).unwrap_err(),
            EmissionDenial::NoCurrentFindings
        );
    }
}
