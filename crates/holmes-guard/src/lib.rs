//! holmes-guard — compiled enforcement of the Holmes provider denylist.
//!
//! Three explicit layers (spec v2.1; Master Build Loop v2, Phase 0):
//! - L1a ([`proxy`]) — deny-by-default local egress proxy; permitted hosts
//!   compiled in; every Holmes-spawned session is forced through it.
//! - L1b ([`resolution`]) — provider/model-id resolution guard; excluded ids
//!   *and unknown ids* are rejected before any client is instantiated.
//! - L2 ([`spawn`]) — sanitized `goose acp` spawn: environment cleared
//!   wholesale, egress pinned to L1a, permitted provider/model injected
//!   explicitly, credential caller-supplied (BYOK — the shipped artifact
//!   requires no vendor key of its own).
//!
//! All policy lives here, in compiled Rust (AC-DL-1 §1). A UI layer may
//! *read* the lists via [`policy`]; nothing outside this crate enforces them.
//! [`scan`] implements the AC-DL-2 deterministic dependency-tree gate.
//!
//! Honest residual (AC-DL-1 §4): a hostile tool binary that ignores proxy
//! environment variables escapes this library-level boundary. Artifact/OS
//! level enforcement is an Alfred obligation, tracked in the cross-repo
//! obligations ledger. This guard is never claimed fork-proof.

pub mod policy;
pub mod proxy;
pub mod resolution;
pub mod scan;
pub mod spawn;
