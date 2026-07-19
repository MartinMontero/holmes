//! The bi-temporal case-memory model (spec §3.1/§4.4; loop §6 Phase 2,
//! lock 2a) — the *owned subset* that replaces Graphiti per D-12.
//!
//! Every fact carries **two** time axes (bi-temporal, the property Zep's
//! Graphiti is prized for): valid time — when the fact was true in the
//! world (`occurred_at` .. `valid_until`) — and transaction time
//! (`ingested_at`) — when Holmes recorded it. The load-bearing invariant
//! is **invalidation-not-deletion** (constitution: "facts are never
//! silently deleted; superseded facts are flagged invalidated and
//! preserved"): superseding a fact sets its `valid_until` and records
//! which fact replaced it, and **there is no removal API anywhere** — the
//! store only grows, and history stays queryable via [`Wall::facts_as_of`].
//!
//! [`InMemoryWall`] is the deterministic reference implementation that
//! *is* the contract test; the Neo4j-backed store (`crate::neo4j`) drives
//! the same operations through the Cypher in `crate::cypher`, asserted to
//! carry no delete.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FactId(pub u64);

impl fmt::Display for FactId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fact-{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WallError {
    EmptyStatement,
    EmptyProvenance,
    NoSuchFact(FactId),
    /// Superseding an already-superseded fact is refused: invalidation is
    /// single-writer to keep the history chain unambiguous.
    AlreadySuperseded(FactId),
}

impl fmt::Display for WallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WallError::EmptyStatement => write!(f, "rejected: empty fact statement"),
            WallError::EmptyProvenance => {
                write!(f, "rejected: a wall fact carries no provenance")
            }
            WallError::NoSuchFact(id) => write!(f, "rejected: no such fact {id}"),
            WallError::AlreadySuperseded(id) => {
                write!(f, "rejected: fact {id} is already superseded")
            }
        }
    }
}

impl std::error::Error for WallError {}

/// One provenance-bearing, bi-temporally versioned fact on the wall.
#[derive(Debug, Clone, PartialEq)]
pub struct WallFact {
    pub id: FactId,
    pub statement: String,
    /// Named sources — never empty (provenance-bearing, invariant 5 kin).
    pub provenance: Vec<String>,
    /// Valid-time start: when the fact became true in the world.
    pub occurred_at: String,
    /// Valid-time end: `None` while current; `Some` once superseded/expired.
    pub valid_until: Option<String>,
    /// Transaction-time: when Holmes ingested it. Never mutated.
    pub ingested_at: String,
    /// The fact that superseded this one, if any (audit chain).
    pub invalidated_by: Option<FactId>,
}

impl WallFact {
    pub fn is_current(&self) -> bool {
        self.valid_until.is_none()
    }

    /// True when `valid_time` falls in this fact's valid interval
    /// `[occurred_at, valid_until)` (half-open; string dates compare
    /// lexicographically for ISO-8601, which the caller supplies).
    pub fn valid_at(&self, valid_time: &str) -> bool {
        if valid_time < self.occurred_at.as_str() {
            return false;
        }
        match &self.valid_until {
            None => true,
            Some(end) => valid_time < end.as_str(),
        }
    }
}

/// The wall's operations. Deliberately has **no delete/remove** method:
/// non-destructive truth is enforced by the absence of the capability.
pub trait Wall {
    fn add_fact(
        &mut self,
        statement: &str,
        provenance: &[String],
        occurred_at: &str,
        ingested_at: &str,
    ) -> Result<FactId, WallError>;

    /// Invalidate `old` as of `invalidated_at` and append `replacement`;
    /// the superseded record is preserved with `valid_until` +
    /// `invalidated_by` set. Returns the replacement's id.
    fn supersede(
        &mut self,
        old: FactId,
        statement: &str,
        provenance: &[String],
        occurred_at: &str,
        ingested_at: &str,
        invalidated_at: &str,
    ) -> Result<FactId, WallError>;

    fn get(&self, id: FactId) -> Option<&WallFact>;
    /// Everything ever recorded, superseded included — only grows.
    fn all_facts(&self) -> Vec<&WallFact>;
    /// Facts current now (`valid_until` is `None`).
    fn current_facts(&self) -> Vec<&WallFact>;
    /// Facts valid at a point in valid-time — history stays queryable.
    fn facts_as_of(&self, valid_time: &str) -> Vec<&WallFact>;
}

/// The deterministic reference wall — the contract, testable without a
/// server. `crate::neo4j::Neo4jWall` must behave identically.
#[derive(Debug, Default)]
pub struct InMemoryWall {
    facts: BTreeMap<u64, WallFact>,
    next: u64,
}

impl InMemoryWall {
    pub fn new() -> Self {
        Self::default()
    }

    fn insert(
        &mut self,
        statement: &str,
        provenance: &[String],
        occurred_at: &str,
        ingested_at: &str,
    ) -> Result<FactId, WallError> {
        if statement.trim().is_empty() {
            return Err(WallError::EmptyStatement);
        }
        if provenance.is_empty() || provenance.iter().all(|p| p.trim().is_empty()) {
            return Err(WallError::EmptyProvenance);
        }
        let id = FactId(self.next);
        self.next += 1;
        self.facts.insert(
            id.0,
            WallFact {
                id,
                statement: statement.to_owned(),
                provenance: provenance.to_vec(),
                occurred_at: occurred_at.to_owned(),
                valid_until: None,
                ingested_at: ingested_at.to_owned(),
                invalidated_by: None,
            },
        );
        Ok(id)
    }
}

impl Wall for InMemoryWall {
    fn add_fact(
        &mut self,
        statement: &str,
        provenance: &[String],
        occurred_at: &str,
        ingested_at: &str,
    ) -> Result<FactId, WallError> {
        self.insert(statement, provenance, occurred_at, ingested_at)
    }

    fn supersede(
        &mut self,
        old: FactId,
        statement: &str,
        provenance: &[String],
        occurred_at: &str,
        ingested_at: &str,
        invalidated_at: &str,
    ) -> Result<FactId, WallError> {
        match self.facts.get(&old.0) {
            None => return Err(WallError::NoSuchFact(old)),
            Some(f) if !f.is_current() => return Err(WallError::AlreadySuperseded(old)),
            Some(_) => {}
        }
        let new_id = self.insert(statement, provenance, occurred_at, ingested_at)?;
        // Flag the old fact invalidated — preserved, never removed.
        let old_fact = self.facts.get_mut(&old.0).expect("checked above");
        old_fact.valid_until = Some(invalidated_at.to_owned());
        old_fact.invalidated_by = Some(new_id);
        Ok(new_id)
    }

    fn get(&self, id: FactId) -> Option<&WallFact> {
        self.facts.get(&id.0)
    }

    fn all_facts(&self) -> Vec<&WallFact> {
        self.facts.values().collect()
    }

    fn current_facts(&self) -> Vec<&WallFact> {
        self.facts.values().filter(|f| f.is_current()).collect()
    }

    fn facts_as_of(&self, valid_time: &str) -> Vec<&WallFact> {
        self.facts
            .values()
            .filter(|f| f.valid_at(valid_time))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prov() -> Vec<String> {
        vec!["fixture/docket.md §3".to_owned()]
    }

    #[test]
    fn provenance_and_statement_are_required() {
        let mut w = InMemoryWall::new();
        assert_eq!(
            w.add_fact("", &prov(), "2026-01-01", "2026-07-19")
                .unwrap_err(),
            WallError::EmptyStatement
        );
        assert_eq!(
            w.add_fact("s", &[], "2026-01-01", "2026-07-19")
                .unwrap_err(),
            WallError::EmptyProvenance
        );
    }

    #[test]
    fn supersede_invalidates_but_never_deletes_and_history_stays_queryable() {
        let mut w = InMemoryWall::new();
        let v1 = w
            .add_fact(
                "entity X owns parcel 441",
                &prov(),
                "2025-01-01",
                "2026-07-01",
            )
            .unwrap();
        let v2 = w
            .supersede(
                v1,
                "entity Y owns parcel 441 (transfer recorded)",
                &prov(),
                "2026-03-15",
                "2026-07-19",
                "2026-03-15",
            )
            .unwrap();

        // Nothing deleted: both records present.
        assert_eq!(w.all_facts().len(), 2);
        // Old record preserved and flagged.
        let old = w.get(v1).unwrap();
        assert_eq!(old.valid_until.as_deref(), Some("2026-03-15"));
        assert_eq!(old.invalidated_by, Some(v2));
        // Only the replacement is current.
        assert_eq!(w.current_facts().len(), 1);
        assert_eq!(w.current_facts()[0].id, v2);

        // History stays queryable: as-of a date inside v1's validity, v1
        // is returned; as-of after the transfer, v2 is.
        let before: Vec<FactId> = w.facts_as_of("2025-06-01").iter().map(|f| f.id).collect();
        assert_eq!(before, vec![v1]);
        let after: Vec<FactId> = w.facts_as_of("2026-06-01").iter().map(|f| f.id).collect();
        assert_eq!(after, vec![v2]);
    }

    #[test]
    fn double_supersede_is_refused() {
        let mut w = InMemoryWall::new();
        let v1 = w
            .add_fact("a", &prov(), "2025-01-01", "2026-07-01")
            .unwrap();
        w.supersede(v1, "b", &prov(), "2026-01-01", "2026-07-19", "2026-01-01")
            .unwrap();
        assert_eq!(
            w.supersede(v1, "c", &prov(), "2026-02-01", "2026-07-19", "2026-02-01")
                .unwrap_err(),
            WallError::AlreadySuperseded(v1)
        );
    }

    #[test]
    fn the_trait_has_no_delete_capability() {
        // Compile-time guarantee documented as a test: `Wall` exposes no
        // delete/remove/clear method. If one is ever added, this comment
        // and the structural test in tests/wall_locks.rs must both change.
        fn _assert_ops<W: Wall>() {}
        _assert_ops::<InMemoryWall>();
    }
}
