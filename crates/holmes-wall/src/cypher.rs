//! Cypher statement builders for the Neo4j-backed wall.
//!
//! Kept in one auditable module so the **invalidation-not-deletion**
//! invariant is checkable at the query level: [`supersede`] emits a `SET`
//! on the old node's `valid_until`/`invalidated_by` plus a `CREATE` of the
//! replacement, and **never** a `DELETE`/`DETACH DELETE`/`REMOVE`. The
//! structural test in `tests/wall_locks.rs` asserts no delete keyword
//! appears in any builder's output. Values are passed as query parameters
//! (`$name`), never string-interpolated, so statement text cannot inject
//! Cypher.

/// Parameter names a caller must bind for [`add_fact`].
pub const ADD_FACT_PARAMS: &[&str] = &[
    "id",
    "statement",
    "provenance",
    "occurred_at",
    "ingested_at",
];

/// CREATE a new current fact node.
pub fn add_fact() -> &'static str {
    "CREATE (f:Fact {id: $id, statement: $statement, provenance: $provenance, \
     occurred_at: $occurred_at, valid_until: null, ingested_at: $ingested_at, \
     invalidated_by: null}) RETURN f.id AS id"
}

/// Parameter names for [`supersede`].
pub const SUPERSEDE_PARAMS: &[&str] = &[
    "old_id",
    "new_id",
    "statement",
    "provenance",
    "occurred_at",
    "ingested_at",
    "invalidated_at",
];

/// Invalidate the old node (SET valid_until + invalidated_by) and CREATE
/// the replacement, linked by a `[:SUPERSEDES]` edge. No delete anywhere.
/// Matches only a still-current old node (`valid_until IS NULL`), so a
/// double-supersede affects nothing and returns no row.
pub fn supersede() -> &'static str {
    "MATCH (old:Fact {id: $old_id}) WHERE old.valid_until IS NULL \
     SET old.valid_until = $invalidated_at, old.invalidated_by = $new_id \
     CREATE (new:Fact {id: $new_id, statement: $statement, provenance: $provenance, \
     occurred_at: $occurred_at, valid_until: null, ingested_at: $ingested_at, \
     invalidated_by: null}) \
     CREATE (new)-[:SUPERSEDES]->(old) \
     RETURN new.id AS id"
}

/// Facts current now.
pub fn current_facts() -> &'static str {
    "MATCH (f:Fact) WHERE f.valid_until IS NULL RETURN f ORDER BY f.id"
}

/// Facts valid at a point in valid-time (half-open `[occurred_at,
/// valid_until)`). History stays queryable — superseded nodes still match.
pub fn facts_as_of() -> &'static str {
    "MATCH (f:Fact) WHERE f.occurred_at <= $valid_time \
     AND (f.valid_until IS NULL OR $valid_time < f.valid_until) \
     RETURN f ORDER BY f.id"
}

/// Everything ever recorded, superseded included.
pub fn all_facts() -> &'static str {
    "MATCH (f:Fact) RETURN f ORDER BY f.id"
}

/// Uniqueness constraint on fact id (setup; idempotent).
pub fn constraint() -> &'static str {
    "CREATE CONSTRAINT fact_id IF NOT EXISTS FOR (f:Fact) REQUIRE f.id IS UNIQUE"
}

/// Every builder's output — for the structural no-delete audit.
pub fn all_statements() -> Vec<&'static str> {
    vec![
        add_fact(),
        supersede(),
        current_facts(),
        facts_as_of(),
        all_facts(),
        constraint(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_builder_emits_a_delete_or_remove() {
        // The invalidation-not-deletion invariant, checked at the Cypher
        // layer: no destructive keyword in any generated statement.
        for stmt in all_statements() {
            let upper = stmt.to_uppercase();
            for forbidden in ["DELETE", "DETACH DELETE", "REMOVE", " DROP "] {
                assert!(
                    !upper.contains(forbidden),
                    "builder emitted forbidden `{forbidden}`: {stmt}"
                );
            }
        }
    }

    #[test]
    fn supersede_sets_validity_and_creates_replacement() {
        let s = supersede().to_uppercase();
        assert!(s.contains("SET OLD.VALID_UNTIL"));
        assert!(s.contains("SET") && s.contains("INVALIDATED_BY"));
        assert!(s.contains("CREATE (NEW:FACT"));
        assert!(
            s.contains("VALID_UNTIL IS NULL"),
            "must match only current nodes"
        );
    }

    #[test]
    fn values_are_parameterized_not_interpolated() {
        // Every builder references $-parameters; none format user values in.
        assert!(add_fact().contains("$statement"));
        assert!(supersede().contains("$new_id"));
        assert!(facts_as_of().contains("$valid_time"));
    }
}
