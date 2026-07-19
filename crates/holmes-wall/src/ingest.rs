//! Lock 2c — ingestion quality (spec §7 Phase 2 lock 2c: "Tier-2
//! ingestion quality tested, not assumed — documented suite ... with
//! failure-rate evidence").
//!
//! The model side (extraction) proposes episodes — candidate facts pulled
//! from source text. This module is the **deterministic scorer**: given
//! the source and the proposed episodes, it measures, without any model
//! call, (1) schema conformance (well-formed, non-empty statement +
//! provenance + a date), (2) provenance grounding (every cited source span
//! actually occurs in the source — no fabricated citation), and (3)
//! fabrication flags (a statement whose key tokens do not appear in the
//! cited span is surfaced, not silently accepted). Quality is a
//! *failure-rate report*, not a pass/fail — the evidence 2c asks for.

use std::fmt;

/// One extracted candidate fact.
#[derive(Debug, Clone)]
pub struct Episode {
    pub statement: String,
    /// The exact source substring the extractor cited as support.
    pub cited_span: String,
    pub occurred_at: String,
}

/// Per-episode verdict from the deterministic scorer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EpisodeVerdict {
    /// Well-formed and its citation occurs verbatim in the source.
    Grounded,
    /// Missing statement / span / date.
    Malformed(&'static str),
    /// Citation text does not occur in the source (fabricated support).
    UngroundedCitation,
    /// Citation occurs, but the statement's salient tokens are absent from
    /// it — the claim outruns its cited evidence.
    ClaimExceedsCitation,
}

/// The failure-rate report for one ingestion batch.
#[derive(Debug, Clone)]
pub struct IngestionQualityReport {
    pub total: usize,
    pub grounded: usize,
    pub malformed: usize,
    pub ungrounded_citation: usize,
    pub claim_exceeds_citation: usize,
    pub verdicts: Vec<EpisodeVerdict>,
}

impl IngestionQualityReport {
    /// Fraction of episodes fully grounded (0.0 when empty).
    pub fn grounded_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.grounded as f64 / self.total as f64
        }
    }

    pub fn failure_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.total - self.grounded) as f64 / self.total as f64
        }
    }
}

impl fmt::Display for IngestionQualityReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ingestion: {}/{} grounded ({:.0}% ), malformed={}, ungrounded_citation={}, \
             claim_exceeds_citation={}",
            self.grounded,
            self.total,
            self.grounded_rate() * 100.0,
            self.malformed,
            self.ungrounded_citation,
            self.claim_exceeds_citation,
        )
    }
}

/// Normalize for token comparison: lowercase, split on non-alphanumerics,
/// drop very short/common tokens so scoring keys on salient content words.
fn salient_tokens(s: &str) -> Vec<String> {
    const STOP: &[&str] = &[
        "the", "a", "an", "of", "to", "in", "on", "and", "or", "is", "was", "for", "by", "at",
        "as", "that", "this", "with", "from", "has", "have",
    ];
    s.to_ascii_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| t.len() >= 3 && !STOP.contains(t))
        .map(str::to_owned)
        .collect()
}

/// Score one episode against the source text.
pub fn score_episode(source: &str, ep: &Episode) -> EpisodeVerdict {
    if ep.statement.trim().is_empty() {
        return EpisodeVerdict::Malformed("empty statement");
    }
    if ep.cited_span.trim().is_empty() {
        return EpisodeVerdict::Malformed("empty citation span");
    }
    if ep.occurred_at.trim().is_empty() {
        return EpisodeVerdict::Malformed("empty occurred_at");
    }
    // Provenance grounding: the cited span must occur in the source
    // (whitespace-normalized, case-insensitive).
    let norm = |s: &str| {
        s.split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_ascii_lowercase()
    };
    let source_n = norm(source);
    let span_n = norm(&ep.cited_span);
    if !source_n.contains(&span_n) {
        return EpisodeVerdict::UngroundedCitation;
    }
    // Claim grounding: a majority of the statement's salient tokens should
    // appear in the cited span (the claim must not outrun its evidence).
    let claim_tokens = salient_tokens(&ep.statement);
    if !claim_tokens.is_empty() {
        let span_tokens = salient_tokens(&ep.cited_span);
        let covered = claim_tokens
            .iter()
            .filter(|t| span_tokens.contains(t))
            .count();
        if (covered as f64) < 0.5 * claim_tokens.len() as f64 {
            return EpisodeVerdict::ClaimExceedsCitation;
        }
    }
    EpisodeVerdict::Grounded
}

/// Score a batch and produce the failure-rate report.
pub fn score_batch(source: &str, episodes: &[Episode]) -> IngestionQualityReport {
    let verdicts: Vec<EpisodeVerdict> = episodes.iter().map(|e| score_episode(source, e)).collect();
    let mut r = IngestionQualityReport {
        total: episodes.len(),
        grounded: 0,
        malformed: 0,
        ungrounded_citation: 0,
        claim_exceeds_citation: 0,
        verdicts: verdicts.clone(),
    };
    for v in &verdicts {
        match v {
            EpisodeVerdict::Grounded => r.grounded += 1,
            EpisodeVerdict::Malformed(_) => r.malformed += 1,
            EpisodeVerdict::UngroundedCitation => r.ungrounded_citation += 1,
            EpisodeVerdict::ClaimExceedsCitation => r.claim_exceeds_citation += 1,
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = "The permit application was filed on 2025-10-01. An objection letter \
        from an adjacent owner, dated 2025-11-04, is in the docket. No enforcement action \
        has been taken against the parcel.";

    #[test]
    fn a_grounded_episode_scores_grounded() {
        let ep = Episode {
            statement: "an objection letter dated 2025-11-04 is in the docket".into(),
            cited_span:
                "An objection letter from an adjacent owner, dated 2025-11-04, is in the docket"
                    .into(),
            occurred_at: "2025-11-04".into(),
        };
        assert_eq!(score_episode(SOURCE, &ep), EpisodeVerdict::Grounded);
    }

    #[test]
    fn a_fabricated_citation_is_caught() {
        let ep = Episode {
            statement: "the mayor personally intervened".into(),
            cited_span: "the mayor ordered the permit frozen".into(), // not in source
            occurred_at: "2025-12-01".into(),
        };
        assert_eq!(
            score_episode(SOURCE, &ep),
            EpisodeVerdict::UngroundedCitation
        );
    }

    #[test]
    fn a_claim_exceeding_its_citation_is_flagged() {
        let ep = Episode {
            statement: "corrupt officials conspired to block the community garden permanently"
                .into(),
            cited_span: "The permit application was filed on 2025-10-01".into(), // real span, unrelated claim
            occurred_at: "2025-10-01".into(),
        };
        assert_eq!(
            score_episode(SOURCE, &ep),
            EpisodeVerdict::ClaimExceedsCitation
        );
    }

    #[test]
    fn malformed_episodes_are_flagged_not_counted_grounded() {
        let ep = Episode {
            statement: "".into(),
            cited_span: "x".into(),
            occurred_at: "2025".into(),
        };
        assert!(matches!(
            score_episode(SOURCE, &ep),
            EpisodeVerdict::Malformed(_)
        ));
    }

    #[test]
    fn batch_report_computes_failure_rate() {
        let good = Episode {
            statement: "objection letter dated 2025-11-04 is in the docket".into(),
            cited_span:
                "An objection letter from an adjacent owner, dated 2025-11-04, is in the docket"
                    .into(),
            occurred_at: "2025-11-04".into(),
        };
        let bad = Episode {
            statement: "the clerk was bribed".into(),
            cited_span: "the clerk was bribed by the developer".into(), // not in source
            occurred_at: "2025-12-01".into(),
        };
        let r = score_batch(SOURCE, &[good, bad]);
        assert_eq!(r.total, 2);
        assert_eq!(r.grounded, 1);
        assert_eq!(r.ungrounded_citation, 1);
        assert!((r.failure_rate() - 0.5).abs() < 1e-9);
    }
}
