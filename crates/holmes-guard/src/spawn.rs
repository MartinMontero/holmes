//! L2 — sanitized spawn (AC-DL-1 §3).
//!
//! The child environment is cleared wholesale and rebuilt explicitly:
//! nothing provider-selecting survives from the parent, HTTP(S)_PROXY is
//! pinned to the L1a proxy, NO_PROXY is cleared, HOME/XDG dirs point into a
//! guard-owned directory so stock user config cannot select a provider, and
//! the permitted provider/model pair is injected only after L1b resolution.
//!
//! BYOK invariant: the guard never reads a vendor credential itself. The
//! embedding application (or CI) supplies the one credential for the
//! resolved provider through [`SpawnSpec::credential`]; the shipped artifact
//! hardcodes no vendor, key, or env var requirement.

use crate::resolution::{self, Denial, ResolvedModel};
use std::collections::BTreeMap;
use std::fmt;
use std::net::SocketAddr;
use std::path::Path;
use std::process::Command;

/// Credential/endpoint variables acceptable per provider — the BYOK seam.
pub const PROVIDER_CREDENTIAL_KEYS: &[(&str, &[&str])] = &[
    ("anthropic", &["ANTHROPIC_API_KEY"]),
    ("google", &["GOOGLE_API_KEY"]),
    ("deepseek", &["DEEPSEEK_API_KEY"]),
    ("qwen", &["DASHSCOPE_API_KEY"]),
    ("mistral", &["MISTRAL_API_KEY"]),
    ("ollama", &["OLLAMA_HOST"]),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpawnDenial {
    Resolution(Denial),
    NonAbsoluteBinary(String),
    CredentialNotForProvider { provider: String, key: String },
}

impl fmt::Display for SpawnDenial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpawnDenial::Resolution(d) => write!(f, "{d}"),
            SpawnDenial::NonAbsoluteBinary(p) => {
                write!(f, "denied: goose binary path '{p}' is not absolute")
            }
            SpawnDenial::CredentialNotForProvider { provider, key } => write!(
                f,
                "denied: credential variable '{key}' is not the accepted key for provider '{provider}'"
            ),
        }
    }
}

impl std::error::Error for SpawnDenial {}

/// One caller-supplied credential (or endpoint) variable for the resolved
/// provider. Values are never logged by the guard.
#[derive(Clone)]
pub struct CredentialVar {
    pub key: String,
    pub value: String,
}

pub struct SpawnSpec<'a> {
    /// Absolute path to the verified goose binary; relative paths are denied.
    pub goose_binary: &'a Path,
    pub provider: &'a str,
    pub model: &'a str,
    /// The L1a egress proxy address every child request is forced through.
    pub proxy_addr: SocketAddr,
    /// Guard-owned home directory; HOME and XDG_* point here so stock user
    /// config (config.yaml) cannot demand a provider (§3).
    pub isolated_home: &'a Path,
    /// BYOK seam; `None` for providers that need no credential.
    pub credential: Option<CredentialVar>,
}

pub struct SanitizedSpawn {
    /// `goose acp` with a fully cleared, explicitly rebuilt environment.
    pub command: Command,
    /// Exactly the environment the child receives (env_clear + this map).
    pub env: BTreeMap<String, String>,
    pub resolved: ResolvedModel,
}

// Born-redacted by hand: environment keys only — a derived Debug would leak
// credential values into logs.
impl fmt::Debug for SanitizedSpawn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SanitizedSpawn")
            .field("program", &self.command.get_program())
            .field("resolved", &self.resolved)
            .field("env_keys", &self.env.keys().collect::<Vec<_>>())
            .finish()
    }
}

/// Build the sanitized `goose acp` command. Refuses excluded/unknown
/// provider or model ids (L1b), relative binary paths, and credentials that
/// do not belong to the resolved provider.
pub fn sanitized_spawn(spec: &SpawnSpec) -> Result<SanitizedSpawn, SpawnDenial> {
    let resolved =
        resolution::resolve(spec.provider, spec.model).map_err(SpawnDenial::Resolution)?;
    if !spec.goose_binary.is_absolute() {
        return Err(SpawnDenial::NonAbsoluteBinary(
            spec.goose_binary.display().to_string(),
        ));
    }
    if let Some(cred) = &spec.credential {
        let accepted = PROVIDER_CREDENTIAL_KEYS
            .iter()
            .find(|(p, _)| *p == resolved.provider)
            .map(|(_, keys)| keys.contains(&cred.key.as_str()))
            .unwrap_or(false);
        if !accepted {
            return Err(SpawnDenial::CredentialNotForProvider {
                provider: resolved.provider,
                key: cred.key.clone(),
            });
        }
    }

    let mut env: BTreeMap<String, String> = BTreeMap::new();
    // PATH is inherited deliberately (not provider-selecting); everything
    // else is rebuilt from scratch.
    if let Ok(path) = std::env::var("PATH") {
        env.insert("PATH".into(), path);
    }
    let home = spec.isolated_home.display().to_string();
    env.insert("HOME".into(), home.clone());
    env.insert("XDG_CONFIG_HOME".into(), format!("{home}/config"));
    env.insert("XDG_DATA_HOME".into(), format!("{home}/data"));
    env.insert("XDG_STATE_HOME".into(), format!("{home}/state"));
    let proxy = format!("http://{}", spec.proxy_addr);
    for key in ["HTTP_PROXY", "HTTPS_PROXY", "http_proxy", "https_proxy"] {
        env.insert(key.into(), proxy.clone());
    }
    // NO_PROXY / no_proxy are deliberately absent: cleared with the rest of
    // the environment, so nothing bypasses L1a.
    env.insert("GOOSE_PROVIDER".into(), resolved.provider.clone());
    env.insert("GOOSE_MODEL".into(), resolved.model.clone());
    // Headless: no OS keyring writes (the pinned build also compiles the
    // system-keyring feature out).
    env.insert("GOOSE_DISABLE_KEYRING".into(), "1".into());
    if let Some(cred) = &spec.credential {
        env.insert(cred.key.clone(), cred.value.clone());
    }

    let mut command = Command::new(spec.goose_binary);
    command.arg("acp").env_clear().envs(&env);
    Ok(SanitizedSpawn {
        command,
        env,
        resolved,
    })
}
