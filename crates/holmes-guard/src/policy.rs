//! The compiled policy tables — single source of truth for every guard layer.
//!
//! Roster source: spec v2.1 model tiers. Exact vendor SKUs/endpoints are
//! re-verified live at Phase RC (loop §6 "Model roster RC audit"); families
//! here are deliberately prefix-scoped so any id outside them still denies
//! (deny-by-default, AC-DL-1 §2). This module contains excluded-vendor
//! identifier literals by necessity; it is a named exemption in the AC-DL-2
//! scanner (`scan::exemptions`), applied visibly in every scan report.

/// Provider ids permitted to serve Holmes model traffic.
pub const PERMITTED_PROVIDERS: &[&str] = &[
    "anthropic",
    "google",
    "deepseek",
    "qwen",
    "mistral",
    "ollama",
];

/// Excluded vendors and vendor-reaching intermediaries (constitution #2:
/// Meta / OpenAI / xAI — direct or via routers/gateways).
pub const EXCLUDED_PROVIDERS: &[&str] = &[
    "openai",
    "azure-openai",
    "azure_openai",
    "meta",
    "meta-llama",
    "facebook",
    "xai",
    "grok",
    "openrouter",
    "litellm",
];

/// Permitted model-id families per provider, prefix-matched on the
/// normalized (trimmed, lowercased) id. The deepseek prefix pins the V4
/// line: retired alias ids do not match it and therefore deny as unknown.
pub const PERMITTED_MODEL_FAMILIES: &[(&str, &[&str])] = &[
    ("anthropic", &["claude-"]),
    ("google", &["gemini-", "gemma"]),
    ("deepseek", &["deepseek-v"]),
    ("qwen", &["qwen"]),
    ("mistral", &["mistral", "magistral", "ministral"]),
    ("ollama", &["qwen", "gemma", "magistral", "mistral"]),
];

/// A host:port pair the L1a egress proxy will carry traffic to.
pub struct PermittedHost {
    pub host: &'static str,
    pub port: u16,
}

/// Hosts the L1a proxy permits; everything else is denied at the boundary.
/// Loopback is scoped to the Ollama default port only.
pub const PERMITTED_EGRESS_HOSTS: &[PermittedHost] = &[
    PermittedHost {
        host: "api.anthropic.com",
        port: 443,
    },
    PermittedHost {
        host: "generativelanguage.googleapis.com",
        port: 443,
    },
    PermittedHost {
        host: "api.deepseek.com",
        port: 443,
    },
    PermittedHost {
        host: "dashscope.aliyuncs.com",
        port: 443,
    },
    PermittedHost {
        host: "dashscope-intl.aliyuncs.com",
        port: 443,
    },
    PermittedHost {
        host: "api.mistral.ai",
        port: 443,
    },
    PermittedHost {
        host: "127.0.0.1",
        port: 11434,
    },
    PermittedHost {
        host: "localhost",
        port: 11434,
    },
    PermittedHost {
        host: "::1",
        port: 11434,
    },
];

/// Environment variables that can select, credential, or re-route a provider
/// in stock goose. Verified against aaif-goose/goose @ 8e78960e
/// (crates/goose/src/config/providers.rs, crates/goose-providers/src/*).
/// The L2 spawn clears the environment wholesale, which strips these and
/// everything else; the list documents the surface for embedders that cannot
/// env_clear (Alfred adoption note) and validates the BYOK credential seam.
pub const PROVIDER_SELECTING_ENV_VARS: &[&str] = &[
    "GOOSE_PROVIDER",
    "GOOSE_MODEL",
    "GOOSE_FAST_MODEL",
    "GOOSE_SUBAGENT_PROVIDER",
    "GOOSE_TEST_PROVIDER",
    "GOOSE_TOOLSHIM",
    "GOOSE_TOOLSHIM_BACKEND",
    "GOOSE_TOOLSHIM_MODEL",
    "GOOSE_TOOLSHIM_OLLAMA_MODEL",
    "GOOSE_ADDITIONAL_CONFIG_FILES",
    "ANTHROPIC_API_KEY",
    "ANTHROPIC_HOST",
    "ANTHROPIC_CUSTOM_HEADERS",
    "GOOGLE_API_KEY",
    "GOOGLE_HOST",
    "GOOGLE_APPLICATION_CREDENTIALS",
    "GEMINI_OAUTH_TOKEN",
    "GEMINI_OAUTH_CLIENT_ID",
    "GEMINI_OAUTH_CLIENT_SECRET",
    "GEMINI_CLI_COMMAND",
    "DEEPSEEK_API_KEY",
    "DASHSCOPE_API_KEY",
    "MISTRAL_API_KEY",
    "OLLAMA_HOST",
    "OLLAMA_CLOUD_API_KEY",
    "OPENAI_API_KEY",
    "OPENAI_HOST",
    "OPENAI_BASE_URL",
    "OPENAI_BASE_PATH",
    "OPENAI_ORGANIZATION",
    "OPENAI_PROJECT",
    "OPENAI_CUSTOM_HEADERS",
    "AZURE_OPENAI_API_KEY",
    "AZURE_OPENAI_ENDPOINT",
    "AZURE_OPENAI_DEPLOYMENT_NAME",
    "AZURE_OPENAI_API_VERSION",
    "AZURE_OPENAI_AD_TOKEN",
    "XAI_API_KEY",
    "XAI_HOST",
    "XAI_OAUTH_TOKEN",
    "GROQ_API_KEY",
    "OPENROUTER_API_KEY",
    "OPENROUTER_HOST",
    "OPENROUTER_PARAMETERS",
    "DATABRICKS_HOST",
    "DATABRICKS_TOKEN",
    "SNOWFLAKE_HOST",
    "SNOWFLAKE_TOKEN",
    "AWS_ACCESS_KEY_ID",
    "AWS_SECRET_ACCESS_KEY",
    "AWS_BEARER_TOKEN_BEDROCK",
    "AWS_PROFILE",
    "AWS_REGION",
    "HTTP_PROXY",
    "HTTPS_PROXY",
    "http_proxy",
    "https_proxy",
    "NO_PROXY",
    "no_proxy",
    "ALL_PROXY",
];

/// Normalize a hostname for allowlist comparison: trim, lowercase, strip one
/// trailing dot and any IPv6 brackets. Comparison is exact — no wildcard or
/// subdomain matching, so lookalike and suffix hosts deny.
pub fn normalize_host(host: &str) -> String {
    let h = host.trim().trim_start_matches('[').trim_end_matches(']');
    let h = h.strip_suffix('.').unwrap_or(h);
    h.to_ascii_lowercase()
}

/// True when the (normalized) host:port pair is in the compiled allowlist.
pub fn host_permitted(host: &str, port: u16) -> bool {
    let host = normalize_host(host);
    PERMITTED_EGRESS_HOSTS
        .iter()
        .any(|p| p.host == host && p.port == port)
}

pub fn provider_permitted(provider: &str) -> bool {
    let p = provider.trim().to_ascii_lowercase();
    PERMITTED_PROVIDERS.contains(&p.as_str())
}

pub fn provider_excluded(provider: &str) -> bool {
    let p = provider.trim().to_ascii_lowercase();
    EXCLUDED_PROVIDERS.contains(&p.as_str())
}

/// True when a single lowercase token names an excluded model family
/// (llama*, gpt-*/o-series, grok*) or an excluded vendor id.
pub fn excluded_model_token(token: &str) -> bool {
    token.starts_with("llama")
        || token.contains("gpt")
        || token.starts_with("grok")
        || token == "openai"
        || token == "xai"
        || is_o_series(token)
}

/// True when a single lowercase token names an excluded vendor namespace
/// (package-name granularity; o-series excluded here — package names are not
/// model ids, and single-letter+digit names are common and innocent).
pub fn excluded_namespace_token(token: &str) -> bool {
    token.starts_with("llama")
        || token.contains("gpt")
        || token.starts_with("grok")
        || token == "openai"
        || token == "tiktoken"
        || token == "xai"
        || token == "meta"
        || token == "facebook"
}

/// OpenAI o-series reasoning ids: "o" followed only by 1–3 digits.
fn is_o_series(token: &str) -> bool {
    let Some(rest) = token.strip_prefix('o') else {
        return false;
    };
    !rest.is_empty() && rest.len() <= 3 && rest.chars().all(|c| c.is_ascii_digit())
}

/// Split an identifier into comparable lowercase tokens on any
/// non-alphanumeric boundary.
pub fn tokenize(id: &str) -> Vec<String> {
    id.to_ascii_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(str::to_owned)
        .collect()
}

/// True when a model id contains an excluded family or vendor token.
pub fn model_family_excluded(model: &str) -> bool {
    tokenize(model).iter().any(|t| excluded_model_token(t))
}

/// True when the (normalized) model id falls inside the provider's
/// permitted family prefixes.
pub fn model_permitted_for_provider(provider: &str, model: &str) -> bool {
    let p = provider.trim().to_ascii_lowercase();
    let m = model.trim().to_ascii_lowercase();
    PERMITTED_MODEL_FAMILIES
        .iter()
        .find(|(prov, _)| *prov == p)
        .map(|(_, families)| families.iter().any(|f| m.starts_with(f)))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_normalization_is_exact_and_case_insensitive() {
        assert!(host_permitted("Api.Anthropic.Com", 443));
        assert!(host_permitted("api.anthropic.com.", 443));
        assert!(host_permitted("[::1]", 11434));
        // Suffix/lookalike hosts must not match.
        assert!(!host_permitted("api.anthropic.com.evil.example", 443));
        assert!(!host_permitted("evil-api.anthropic.com", 443));
        // Port-scoped: permitted host on the wrong port denies.
        assert!(!host_permitted("api.anthropic.com", 80));
        assert!(!host_permitted("127.0.0.1", 8080));
    }

    #[test]
    fn o_series_matching_is_tight() {
        assert!(is_o_series("o1"));
        assert!(is_o_series("o3"));
        assert!(is_o_series("o200"));
        assert!(!is_o_series("o"));
        assert!(!is_o_series("o3b"));
        assert!(!is_o_series("ollama"));
        assert!(!is_o_series("oauth2"));
    }

    #[test]
    fn ollama_and_groq_tokens_are_not_false_positives() {
        assert!(!excluded_model_token("ollama"));
        assert!(!excluded_namespace_token("ollama"));
        assert!(!excluded_model_token("groq"));
        assert!(!excluded_namespace_token("metadata"));
    }
}
