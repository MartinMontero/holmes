//! The Wall — Holmes's bi-temporal case memory (loop §6 Phase 2).
//!
//! Per D-12 (Graphiti dropped — its base excluded-vendor SDK + phone-home
//! deps violate the vendor/telemetry gates, F-027), this is the *owned
//! subset*: the load-bearing properties of a temporal knowledge graph —
//! bi-temporal validity, invalidation-not-deletion, provenance on every
//! fact — built directly on **Neo4j Community Edition** (D-09) via the
//! denylist-clean `neo4rs` Bolt driver (D-12 rider d). No Graphiti, no
//! excluded-vendor SDK, no phone-home; permitted clients only.
//!
//! Layers:
//! - [`graph`] — the bi-temporal model + `InMemoryWall` reference store
//!   (lock 2a contract, proven hermetically).
//! - [`cypher`] — audited Cypher builders (no delete keyword, lock 2a).
//! - [`neo4j`] — the live Neo4j-backed store (env-gated integration).
//! - [`memory`] — the memory-layer config whose default reaches no cloud
//!   endpoint (lock 2b, AC-DL-1 §6).
//! - [`supervise`] — supervised-backend lifecycle (lock 2d).
//! - [`provenance`] — weight-provenance verify-before-load (lock 2e).

pub mod cypher;
pub mod graph;
pub mod ingest;
pub mod memory;
pub mod neo4j;
pub mod provenance;
pub mod supervise;

pub use graph::{FactId, InMemoryWall, Wall, WallError, WallFact};
pub use ingest::{score_batch, score_episode, Episode, EpisodeVerdict, IngestionQualityReport};
pub use memory::{Endpoint, MemoryConfigError, MemoryLayerConfig};
pub use provenance::{verify_weights, VerifiedWeights, WeightManifest};
pub use supervise::{BackendSpec, SuperviseError, SupervisedBackend};
