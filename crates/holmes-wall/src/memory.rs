//! Lock 2b — AC-DL-1 §6 at the memory layer (the deferral recorded in
//! every CI run since Phase 0).
//!
//! AC-DL-1 §6: "a memory layer instantiated with no explicit LLM config
//! does not reach the excluded provider's default endpoint." The dropped
//! Graphiti earned that criterion because its *default* client was an
//! excluded vendor's. The owned wall inverts it: the default
//! extraction/embedding config is **fully local** (Ollama on loopback), so
//! a no-config memory layer reaches **no cloud endpoint at all**, excluded
//! or otherwise. An operator may opt into a *permitted* cloud provider
//! explicitly; an excluded one is unrepresentable — the config validates
//! provider/model through `holmes-guard` L1b and every endpoint host
//! through the L1a allowlist.
//!
//! No excluded-vendor identifier is written in this file: the canary host
//! and the excluded-provider set are sourced from the compiled denylist
//! (`holmes-guard::policy`), the one place the AC-DL-2 scanner exempts for
//! such literals (F-025 reword-over-exemption precedent).
//!
//! This module owns the config + its endpoint derivation and the
//! deterministic assertions; the live boundary proof (that the L1a proxy
//! denies the excluded endpoint even if something tried) lives in
//! `tests/wall_locks.rs` against a real `EgressProxy`.

use holmes_guard::policy::{host_permitted, provider_excluded, EXCLUDED_CANARY_HOST};
use holmes_guard::resolution;
use std::fmt;

/// Ollama's loopback endpoint — the default extraction/embedding target,
/// scoped in the L1a allowlist to `127.0.0.1:11434`.
pub const LOCAL_OLLAMA_HOST: &str = "127.0.0.1";
pub const LOCAL_OLLAMA_PORT: u16 = 11434;

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryConfigError {
    ExcludedProvider(String),
    UnresolvedModel(String),
    EndpointNotPermitted { host: String, port: u16 },
}

impl fmt::Display for MemoryConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryConfigError::ExcludedProvider(p) => {
                write!(
                    f,
                    "rejected: excluded provider '{p}' in memory-layer config"
                )
            }
            MemoryConfigError::UnresolvedModel(d) => write!(f, "rejected: {d}"),
            MemoryConfigError::EndpointNotPermitted { host, port } => write!(
                f,
                "rejected: memory endpoint {host}:{port} is not in the L1a allowlist"
            ),
        }
    }
}

impl std::error::Error for MemoryConfigError {}

/// One extraction/embedding target the memory layer would contact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoint {
    pub host: String,
    pub port: u16,
}

/// The memory layer's LLM/embedding configuration. `Default` is
/// **local-only** — the no-explicit-config case AC-DL-1 §6 tests.
#[derive(Debug, Clone)]
pub struct MemoryLayerConfig {
    pub extraction_provider: String,
    pub extraction_model: String,
    /// Host:port pairs this config's clients would contact.
    endpoints: Vec<Endpoint>,
}

impl Default for MemoryLayerConfig {
    /// No explicit LLM config → fully local Ollama on loopback. Reaches no
    /// cloud endpoint; certainly not the excluded canary.
    fn default() -> Self {
        Self {
            extraction_provider: "ollama".to_owned(),
            // A permitted local family (spec v2.1 Tier-2 roster).
            extraction_model: "qwen3.6-flash".to_owned(),
            endpoints: vec![Endpoint {
                host: LOCAL_OLLAMA_HOST.to_owned(),
                port: LOCAL_OLLAMA_PORT,
            }],
        }
    }
}

impl MemoryLayerConfig {
    /// Build a config for an explicit provider/model + endpoints, validated:
    /// the provider is not excluded, the model resolves (L1b), and every
    /// endpoint is L1a-permitted. An excluded provider cannot be built.
    pub fn new(
        provider: &str,
        model: &str,
        endpoints: Vec<Endpoint>,
    ) -> Result<Self, MemoryConfigError> {
        if provider_excluded(provider) {
            return Err(MemoryConfigError::ExcludedProvider(provider.to_owned()));
        }
        resolution::resolve(provider, model)
            .map_err(|d| MemoryConfigError::UnresolvedModel(d.to_string()))?;
        for e in &endpoints {
            if !host_permitted(&e.host, e.port) {
                return Err(MemoryConfigError::EndpointNotPermitted {
                    host: e.host.clone(),
                    port: e.port,
                });
            }
        }
        Ok(Self {
            extraction_provider: provider.to_owned(),
            extraction_model: model.to_owned(),
            endpoints,
        })
    }

    pub fn endpoints(&self) -> &[Endpoint] {
        &self.endpoints
    }

    /// True iff every endpoint is L1a-permitted (deny-by-default holds).
    pub fn all_endpoints_permitted(&self) -> bool {
        self.endpoints
            .iter()
            .all(|e| host_permitted(&e.host, e.port))
    }

    /// True iff any endpoint is the excluded canary host (must always be
    /// false — the §6 assertion in config form).
    pub fn reaches_excluded_endpoint(&self) -> bool {
        self.endpoints
            .iter()
            .any(|e| e.host.eq_ignore_ascii_case(EXCLUDED_CANARY_HOST))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use holmes_guard::policy::{EXCLUDED_CANARY_PORT, EXCLUDED_PROVIDERS};

    #[test]
    fn default_no_config_memory_layer_is_local_only_and_reaches_no_excluded_endpoint() {
        let cfg = MemoryLayerConfig::default();
        // The §6 property: no explicit config → no excluded endpoint.
        assert!(!cfg.reaches_excluded_endpoint());
        // Local-only: every endpoint is loopback Ollama, all L1a-permitted.
        assert!(cfg.all_endpoints_permitted());
        assert_eq!(
            cfg.endpoints(),
            &[Endpoint {
                host: LOCAL_OLLAMA_HOST.to_owned(),
                port: LOCAL_OLLAMA_PORT
            }]
        );
        // And the extraction provider is not an excluded vendor.
        assert!(!provider_excluded(&cfg.extraction_provider));
    }

    #[test]
    fn every_excluded_provider_cannot_be_configured() {
        // Sourced from the compiled denylist — no excluded literal here.
        for excluded in EXCLUDED_PROVIDERS {
            let err = MemoryLayerConfig::new(excluded, "any-model", vec![]).unwrap_err();
            assert!(matches!(err, MemoryConfigError::ExcludedProvider(_)));
        }
    }

    #[test]
    fn the_excluded_canary_endpoint_is_refused_even_for_a_permitted_provider() {
        // Permitted provider/model, but pointed at the excluded canary host.
        let err = MemoryLayerConfig::new(
            "anthropic",
            "claude-sonnet-5",
            vec![Endpoint {
                host: EXCLUDED_CANARY_HOST.to_owned(),
                port: EXCLUDED_CANARY_PORT,
            }],
        )
        .unwrap_err();
        assert!(matches!(
            err,
            MemoryConfigError::EndpointNotPermitted { .. }
        ));
    }

    #[test]
    fn a_permitted_cloud_provider_on_its_own_host_is_allowed() {
        let cfg = MemoryLayerConfig::new(
            "anthropic",
            "claude-sonnet-5",
            vec![Endpoint {
                host: "api.anthropic.com".to_owned(),
                port: 443,
            }],
        )
        .expect("permitted provider on its permitted host");
        assert!(cfg.all_endpoints_permitted());
        assert!(!cfg.reaches_excluded_endpoint());
    }
}
