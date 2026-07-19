//! Neo4j-backed wall (D-09 backend) via the denylist-clean `neo4rs` Bolt
//! driver (D-12 rider d).
//!
//! Runs the same operations as `crate::graph::InMemoryWall` through the
//! audited `crate::cypher` builders. The contract (invalidation-not-
//! deletion) is proven hermetically by `InMemoryWall` + the Cypher no-
//! delete test; this type carries the *live* leg, exercised by an
//! env-gated integration test (`HOLMES_NEO4J_URI`) that runs on any host
//! with Neo4j reachable — never faked. In-container it is blocked by the
//! org egress policy on the Neo4j image/dist CDN (recorded in STATE.md,
//! the same wall as the 0c model leg).

use crate::cypher;
use crate::graph::{FactId, WallError, WallFact};
use neo4rs::{query, Graph};

#[derive(Debug)]
pub enum Neo4jError {
    Connect(String),
    Query(String),
    /// A supersede matched no current node (already superseded / absent).
    NotSuperseded(FactId),
}

impl std::fmt::Display for Neo4jError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Neo4jError::Connect(s) => write!(f, "neo4j connect: {s}"),
            Neo4jError::Query(s) => write!(f, "neo4j query: {s}"),
            Neo4jError::NotSuperseded(id) => {
                write!(f, "supersede matched no current fact {id}")
            }
        }
    }
}

impl std::error::Error for Neo4jError {}

/// A live Neo4j-backed wall. Fact ids are the same `u64` space as the
/// in-memory wall, stored as node property `id`.
pub struct Neo4jWall {
    graph: Graph,
}

impl Neo4jWall {
    /// Connect and ensure the uniqueness constraint. `uri` is e.g.
    /// `neo4j://127.0.0.1:7687` (loopback in the supervised-backend
    /// deployment — the L1a allowlist scopes loopback).
    pub async fn connect(uri: &str, user: &str, password: &str) -> Result<Self, Neo4jError> {
        let graph = Graph::new(uri, user, password)
            .await
            .map_err(|e| Neo4jError::Connect(e.to_string()))?;
        graph
            .run(query(cypher::constraint()))
            .await
            .map_err(|e| Neo4jError::Query(e.to_string()))?;
        Ok(Self { graph })
    }

    pub async fn add_fact(
        &self,
        id: FactId,
        statement: &str,
        provenance: &[String],
        occurred_at: &str,
        ingested_at: &str,
    ) -> Result<FactId, Neo4jError> {
        if statement.trim().is_empty() {
            return Err(Neo4jError::Query(WallError::EmptyStatement.to_string()));
        }
        if provenance.is_empty() {
            return Err(Neo4jError::Query(WallError::EmptyProvenance.to_string()));
        }
        self.graph
            .run(
                query(cypher::add_fact())
                    .param("id", id.0 as i64)
                    .param("statement", statement)
                    .param("provenance", provenance.to_vec())
                    .param("occurred_at", occurred_at)
                    .param("ingested_at", ingested_at),
            )
            .await
            .map_err(|e| Neo4jError::Query(e.to_string()))?;
        Ok(id)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn supersede(
        &self,
        old: FactId,
        new: FactId,
        statement: &str,
        provenance: &[String],
        occurred_at: &str,
        ingested_at: &str,
        invalidated_at: &str,
    ) -> Result<FactId, Neo4jError> {
        let mut rows = self
            .graph
            .execute(
                query(cypher::supersede())
                    .param("old_id", old.0 as i64)
                    .param("new_id", new.0 as i64)
                    .param("statement", statement)
                    .param("provenance", provenance.to_vec())
                    .param("occurred_at", occurred_at)
                    .param("ingested_at", ingested_at)
                    .param("invalidated_at", invalidated_at),
            )
            .await
            .map_err(|e| Neo4jError::Query(e.to_string()))?;
        // Exactly one row iff a current old node matched and was invalidated.
        match rows
            .next()
            .await
            .map_err(|e| Neo4jError::Query(e.to_string()))?
        {
            Some(_) => Ok(new),
            None => Err(Neo4jError::NotSuperseded(old)),
        }
    }

    /// Count of all fact nodes (superseded included) — used by the
    /// integration test to prove nothing was deleted.
    pub async fn count_all(&self) -> Result<u64, Neo4jError> {
        let mut rows = self
            .graph
            .execute(query("MATCH (f:Fact) RETURN count(f) AS n"))
            .await
            .map_err(|e| Neo4jError::Query(e.to_string()))?;
        let row = rows
            .next()
            .await
            .map_err(|e| Neo4jError::Query(e.to_string()))?
            .ok_or_else(|| Neo4jError::Query("count returned no row".into()))?;
        let n: i64 = row.get("n").map_err(|e| Neo4jError::Query(e.to_string()))?;
        Ok(n as u64)
    }

    async fn ids_for(
        &self,
        cypher_stmt: &str,
        param: Option<(&str, String)>,
    ) -> Result<Vec<FactId>, Neo4jError> {
        let mut q = query(cypher_stmt);
        if let Some((k, v)) = param {
            q = q.param(k, v);
        }
        let mut rows = self
            .graph
            .execute(q)
            .await
            .map_err(|e| Neo4jError::Query(e.to_string()))?;
        let mut out = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .map_err(|e| Neo4jError::Query(e.to_string()))?
        {
            let node: neo4rs::Node = row.get("f").map_err(|e| Neo4jError::Query(e.to_string()))?;
            let id: i64 = node
                .get("id")
                .map_err(|e| Neo4jError::Query(e.to_string()))?;
            out.push(FactId(id as u64));
        }
        Ok(out)
    }

    pub async fn current_fact_ids(&self) -> Result<Vec<FactId>, Neo4jError> {
        self.ids_for(cypher::current_facts(), None).await
    }

    pub async fn fact_ids_as_of(&self, valid_time: &str) -> Result<Vec<FactId>, Neo4jError> {
        self.ids_for(
            cypher::facts_as_of(),
            Some(("valid_time", valid_time.to_owned())),
        )
        .await
    }

    /// Full fact by id (for the integration test's preservation assertion).
    pub async fn get(&self, id: FactId) -> Result<Option<WallFact>, Neo4jError> {
        let mut rows = self
            .graph
            .execute(query("MATCH (f:Fact {id: $id}) RETURN f").param("id", id.0 as i64))
            .await
            .map_err(|e| Neo4jError::Query(e.to_string()))?;
        let Some(row) = rows
            .next()
            .await
            .map_err(|e| Neo4jError::Query(e.to_string()))?
        else {
            return Ok(None);
        };
        let node: neo4rs::Node = row.get("f").map_err(|e| Neo4jError::Query(e.to_string()))?;
        let id_v: i64 = node
            .get("id")
            .map_err(|e| Neo4jError::Query(e.to_string()))?;
        let valid_until: Option<String> = node.get("valid_until").ok();
        let invalidated_by: Option<i64> = node.get("invalidated_by").ok();
        Ok(Some(WallFact {
            id: FactId(id_v as u64),
            statement: node.get("statement").unwrap_or_default(),
            provenance: node.get("provenance").unwrap_or_default(),
            occurred_at: node.get("occurred_at").unwrap_or_default(),
            valid_until,
            ingested_at: node.get("ingested_at").unwrap_or_default(),
            invalidated_by: invalidated_by.map(|v| FactId(v as u64)),
        }))
    }
}
