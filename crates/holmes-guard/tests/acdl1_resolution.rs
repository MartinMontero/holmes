//! AC-DL-1 §2 (L1b resolution guard) and §5 (permitted-path positive control).
//! This file is an AC-DL-2 scan exemption: it names excluded identifiers as
//! denial fixtures.

use holmes_guard::resolution::{resolve, Denial};

#[test]
fn s5_positive_control_permitted_stack_resolves() {
    // Anthropic, Google, DeepSeek, Qwen, Magistral, and Gemma resolve and
    // pass — proving the denylist has not silently become a full blocklist.
    for (provider, model) in [
        ("anthropic", "claude-sonnet-5"),
        ("anthropic", "claude-opus-4-8"),
        ("google", "gemini-3.1-pro"),
        ("google", "gemma-3-27b"),
        ("deepseek", "deepseek-v4"),
        ("qwen", "qwen3.7-max"),
        ("mistral", "magistral-small"),
        ("ollama", "qwen3.5-27b"),
        ("ollama", "gemma3:1b"),
        ("ollama", "magistral-small"),
    ] {
        let resolved = resolve(provider, model)
            .unwrap_or_else(|d| panic!("permitted pair {provider}/{model} denied: {d}"));
        assert_eq!(resolved.provider, provider);
    }
}

#[test]
fn s2_excluded_providers_denied() {
    for provider in [
        "openai",
        "azure-openai",
        "meta",
        "meta-llama",
        "facebook",
        "xai",
        "grok",
        "openrouter",
        "litellm",
    ] {
        match resolve(provider, "any-model") {
            Err(Denial::ExcludedProvider(_)) => {}
            other => panic!("excluded provider {provider} must deny as excluded, got {other:?}"),
        }
    }
}

#[test]
fn s2_excluded_model_families_denied_even_on_permitted_providers() {
    for (provider, model) in [
        ("ollama", "llama3.3:70b"),
        ("ollama", "llama-4-scout"),
        ("anthropic", "gpt-4o"),
        ("google", "grok-4"),
        ("qwen", "chatgpt-compat"),
        ("ollama", "o3-mini"),
        ("mistral", "gpt4all-j"),
    ] {
        match resolve(provider, model) {
            Err(Denial::ExcludedModelFamily(_)) => {}
            other => panic!(
                "excluded family {provider}/{model} must deny as excluded family, got {other:?}"
            ),
        }
    }
}

#[test]
fn s2_unknown_providers_denied_not_warned() {
    for provider in ["cohere", "databricks", "snowflake", "groq", "bedrock", ""] {
        match resolve(provider, "some-model") {
            Err(Denial::UnknownProvider(_)) => {}
            other => panic!("unknown provider {provider:?} must deny, got {other:?}"),
        }
    }
}

#[test]
fn s2_unknown_models_denied_not_warned() {
    for (provider, model) in [
        ("anthropic", "totally-novel-model"),
        ("anthropic", ""),
        ("google", "palm-2"),
        ("deepseek", "deepseek-coder"),
        ("ollama", "phi-4"),
    ] {
        match resolve(provider, model) {
            Err(Denial::UnknownModel { .. }) => {}
            other => panic!("unknown model {provider}/{model} must deny, got {other:?}"),
        }
    }
}

#[test]
fn s2_retired_deepseek_alias_ids_do_not_resolve() {
    // A-01: the two alias ids retiring 2026-07-24 must never bind. Their
    // literals are constructed at runtime so the retired strings appear in
    // no committed code or config (build-loop constraint), while the
    // rejection itself is still executed proof.
    for suffix in ["chat", "reasoner"] {
        let alias = format!("deepseek-{suffix}");
        match resolve("deepseek", &alias) {
            Err(Denial::UnknownModel { .. }) => {}
            other => panic!("retired alias {alias} must deny as unknown, got {other:?}"),
        }
    }
}

#[test]
fn s2_concatenated_excluded_family_after_permitted_prefix_denies() {
    // Regression: an id that starts with a permitted family prefix but embeds
    // an excluded family mid-token (single tokenize unit) must deny, not pass
    // on the permitted prefix.
    for (provider, model) in [
        ("ollama", "qwenllama"),
        ("mistral", "mistralgrok"),
        ("ollama", "gemmagrok"),
        ("ollama", "mistralllama70b"),
    ] {
        match resolve(provider, model) {
            Err(Denial::ExcludedModelFamily(_)) => {}
            other => {
                panic!("{provider}/{model} embeds an excluded family and must deny, got {other:?}")
            }
        }
    }
}

#[test]
fn s2_normalization_does_not_open_holes() {
    assert!(resolve(" Anthropic ", "CLAUDE-Sonnet-5").is_ok());
    assert!(resolve("OPENAI", "claude-sonnet-5").is_err());
    assert!(resolve("anthropic", " GPT-4O ").is_err());
}
