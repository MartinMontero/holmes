//! Phase 4 — the deny-by-default permission manifest, finalized (spec §7
//! Phase 4: "deny-by-default permission manifest"; spec line 166:
//! "read-only tools run free, every write/shell action asks first").
//!
//! The manifest is the compiled declaration of *which capabilities exist
//! and their default posture*. It is deny-by-default in the strongest
//! sense: a capability the manifest does not name is refused — the
//! `decide` fallthrough is `Denied`, so adding a capability is a
//! deliberate source edit, never an omission that silently permits.
//!
//! Postures follow the spec's read/write split:
//! - **read-only → run free** (no approval; the analytical core reasons
//!   over operator-supplied material and never reaches outward);
//! - **write / shell / network → ask first** — routed to the Phase 2.5
//!   tool-approval protocol (`safety::approval`), so nothing outward-
//!   facing fires without an operator grant.
//!
//! **D-14(a): the investigative capability class is absent from the beta
//! manifest.** [`PermissionManifest::analytical_beta`] declares no
//! `Investigative` capability, so every Phase-3 capability resolves to
//! `Denied` by absence — *compiled out, not dark-flagged*. When Phase 3
//! builds (post-beta, under the `investigative` feature), it adds those
//! capabilities to a *different* manifest gated behind that feature; the
//! beta artifact never carries them. [`INVESTIGATIVE_ABSENT`] and the
//! lock test assert this holds.

/// True in the beta build: no investigative capability is compiled into
/// the analytical manifest (D-14(a)). The lock test enforces it.
pub const INVESTIGATIVE_ABSENT: bool = true;

/// Compile-time guard: flipping the flag above fails the build here, so
/// the D-14(a) invariant cannot be silently loosened (a const assertion,
/// not a runtime one — the constant-ness is the point).
const _: () = assert!(INVESTIGATIVE_ABSENT);

/// The class of a capability — determines its default posture and, for
/// the investigative class, its absence from the beta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityClass {
    /// Reads operator-supplied or local state; reaches nothing outward.
    ReadOnly,
    /// Mutates local state (e.g. writing a fact to the Wall).
    Write,
    /// Executes a subprocess.
    Shell,
    /// Reaches a network endpoint.
    Network,
    /// Investigative collection (public records, OSINT, link analysis).
    /// **Never present in the beta manifest** (D-14(a)); listed only so
    /// the type can name what is deliberately excluded.
    Investigative,
}

/// The default posture the manifest assigns a capability's class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityPosture {
    /// Runs without approval (read-only).
    RunFree,
    /// Requires an operator grant via the Phase 2.5 approval protocol.
    AskFirst,
}

impl CapabilityClass {
    /// The spec's read/write split as a total function.
    fn posture(self) -> CapabilityPosture {
        match self {
            CapabilityClass::ReadOnly => CapabilityPosture::RunFree,
            CapabilityClass::Write
            | CapabilityClass::Shell
            | CapabilityClass::Network
            | CapabilityClass::Investigative => CapabilityPosture::AskFirst,
        }
    }
}

/// The manifest's decision for a requested capability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionDecision {
    /// Read-only: run without approval.
    RunFree,
    /// Ask first: route to the approval protocol for this capability class.
    AskFirst { class: CapabilityClass },
    /// Deny-by-default: the manifest does not name this capability.
    Denied { capability: String },
}

/// One declared capability: a stable name and its class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capability {
    pub name: &'static str,
    pub class: CapabilityClass,
}

/// The finalized, deny-by-default capability manifest.
#[derive(Debug, Clone)]
pub struct PermissionManifest {
    capabilities: Vec<Capability>,
}

impl PermissionManifest {
    /// The analytical-surface (beta) manifest — the only manifest the beta
    /// artifact compiles (D-14(a)). Read-only analytical capabilities run
    /// free; the one local mutation (writing a fact to the Wall) asks
    /// first. **No `Investigative` capability appears here.**
    pub fn analytical_beta() -> Self {
        Self {
            capabilities: vec![
                // Read-only analytical capabilities — run free.
                Capability {
                    name: "read_untrusted_content",
                    class: CapabilityClass::ReadOnly,
                },
                Capability {
                    name: "emit_evidence_pack",
                    class: CapabilityClass::ReadOnly,
                },
                Capability {
                    name: "hand_off_case",
                    class: CapabilityClass::ReadOnly,
                },
                Capability {
                    name: "wall_read",
                    class: CapabilityClass::ReadOnly,
                },
                // Local mutation — ask first (routed to the approval
                // protocol even though it never reaches outward).
                Capability {
                    name: "wall_write",
                    class: CapabilityClass::Write,
                },
            ],
        }
    }

    pub fn capabilities(&self) -> &[Capability] {
        &self.capabilities
    }

    /// Decide a requested capability. Deny-by-default: an unnamed
    /// capability is refused (the fallthrough), so investigative and any
    /// other unlisted capability cannot run in the beta.
    pub fn decide(&self, capability: &str) -> PermissionDecision {
        match self.capabilities.iter().find(|c| c.name == capability) {
            None => PermissionDecision::Denied {
                capability: capability.to_owned(),
            },
            Some(c) => match c.class.posture() {
                CapabilityPosture::RunFree => PermissionDecision::RunFree,
                CapabilityPosture::AskFirst => PermissionDecision::AskFirst { class: c.class },
            },
        }
    }

    /// Whether this manifest declares any investigative capability. The
    /// beta manifest must return `false` (D-14(a)).
    pub fn declares_investigative(&self) -> bool {
        self.capabilities
            .iter()
            .any(|c| c.class == CapabilityClass::Investigative)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deny_by_default_refuses_unlisted_capabilities() {
        let m = PermissionManifest::analytical_beta();
        // Investigative / Phase-3 capabilities are absent → Denied.
        for phase3 in [
            "records_search",
            "osint_fetch",
            "code_exec",
            "link_analysis",
        ] {
            assert_eq!(
                m.decide(phase3),
                PermissionDecision::Denied {
                    capability: phase3.to_owned()
                },
                "unlisted capability {phase3} must be denied"
            );
        }
    }

    #[test]
    fn read_only_runs_free_and_write_asks_first() {
        let m = PermissionManifest::analytical_beta();
        assert_eq!(
            m.decide("read_untrusted_content"),
            PermissionDecision::RunFree
        );
        assert_eq!(m.decide("emit_evidence_pack"), PermissionDecision::RunFree);
        assert_eq!(m.decide("hand_off_case"), PermissionDecision::RunFree);
        assert_eq!(m.decide("wall_read"), PermissionDecision::RunFree);
        assert_eq!(
            m.decide("wall_write"),
            PermissionDecision::AskFirst {
                class: CapabilityClass::Write
            }
        );
    }

    /// D-14(a) lock: the beta manifest declares no investigative
    /// capability — absent, not dark-flagged.
    #[test]
    fn beta_manifest_carries_no_investigative_capability() {
        let m = PermissionManifest::analytical_beta();
        assert!(!m.declares_investigative());
        assert!(m
            .capabilities()
            .iter()
            .all(|c| c.class != CapabilityClass::Investigative));
    }
}
