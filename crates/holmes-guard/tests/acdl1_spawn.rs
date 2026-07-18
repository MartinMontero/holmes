//! AC-DL-1 §3 — config/env-override refusal at the L2 sanitized spawn.
//! Hermetic: nothing is actually spawned; the sanitized command and its
//! exact environment map are inspected.
//! This file is an AC-DL-2 scan exemption (poisons the env with excluded
//! vendor variables as fixtures).

use holmes_guard::policy;
use holmes_guard::resolution::Denial;
use holmes_guard::spawn::{sanitized_spawn, CredentialVar, SpawnDenial, SpawnSpec};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::Path;

fn proxy_addr() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 39999)
}

fn spec<'a>(provider: &'a str, model: &'a str, home: &'a Path) -> SpawnSpec<'a> {
    SpawnSpec {
        goose_binary: Path::new("/opt/holmes/goose"),
        provider,
        model,
        proxy_addr: proxy_addr(),
        isolated_home: home,
        credential: None,
    }
}

#[test]
fn s3_env_demanding_excluded_provider_is_refused() {
    // The refusal itself, asserted: a request for an excluded provider —
    // whatever carried it (env var, config file, caller) — must error, not
    // warn.
    let home = std::env::temp_dir();
    match sanitized_spawn(&spec("openai", "gpt-4o", &home)) {
        Err(SpawnDenial::Resolution(Denial::ExcludedProvider(_))) => {}
        other => panic!("excluded provider must refuse, got {other:?}"),
    }
    match sanitized_spawn(&spec("ollama", "llama3.3:70b", &home)) {
        Err(SpawnDenial::Resolution(Denial::ExcludedModelFamily(_))) => {}
        other => panic!("excluded model family must refuse, got {other:?}"),
    }
}

#[test]
fn s3_poisoned_parent_environment_is_stripped_wholesale() {
    // Poison the parent with provider-selecting variables; none may survive.
    std::env::set_var("OPENAI_API_KEY", "sk-planted-fixture");
    std::env::set_var("GOOSE_PROVIDER", "openai");
    std::env::set_var("GOOSE_MODEL", "gpt-4o");
    std::env::set_var("XAI_API_KEY", "planted");
    std::env::set_var("OPENROUTER_API_KEY", "planted");
    std::env::set_var("NO_PROXY", "*");
    std::env::set_var("ANTHROPIC_API_KEY", "sk-parent-key-must-not-leak");

    let home = std::env::temp_dir().join("holmes-guard-test-home");
    let sanitized = sanitized_spawn(&spec("anthropic", "claude-sonnet-5", &home))
        .expect("permitted pair must pass");

    // The only provider-selecting keys present are the two the guard itself
    // injects, and they hold the resolved permitted values — not the
    // parent's demands.
    for key in policy::PROVIDER_SELECTING_ENV_VARS {
        match *key {
            "GOOSE_PROVIDER" => assert_eq!(sanitized.env.get(*key).unwrap(), "anthropic"),
            "GOOSE_MODEL" => assert_eq!(sanitized.env.get(*key).unwrap(), "claude-sonnet-5"),
            "HTTP_PROXY" | "HTTPS_PROXY" | "http_proxy" | "https_proxy" => {
                assert_eq!(sanitized.env.get(*key).unwrap(), &format!("http://{}", proxy_addr()));
            }
            _ => assert!(
                !sanitized.env.contains_key(*key),
                "provider-selecting var {key} leaked into the child env"
            ),
        }
    }
    // NO_PROXY cleared; egress pinned to L1a; config isolated to the
    // guard-owned home so no stock config.yaml can demand a provider.
    assert!(!sanitized.env.contains_key("NO_PROXY"));
    assert!(!sanitized.env.contains_key("no_proxy"));
    assert_eq!(
        sanitized.env.get("HOME").unwrap(),
        &home.display().to_string()
    );
    assert!(sanitized.env.get("XDG_CONFIG_HOME").unwrap().starts_with(&home.display().to_string()));
    assert_eq!(sanitized.env.get("GOOSE_DISABLE_KEYRING").unwrap(), "1");

    for key in [
        "OPENAI_API_KEY",
        "XAI_API_KEY",
        "OPENROUTER_API_KEY",
        "ANTHROPIC_API_KEY",
        "NO_PROXY",
    ] {
        std::env::remove_var(key);
    }
    std::env::remove_var("GOOSE_PROVIDER");
    std::env::remove_var("GOOSE_MODEL");
}

#[test]
fn s3_relative_binary_path_is_refused() {
    let home = std::env::temp_dir();
    let mut s = spec("anthropic", "claude-sonnet-5", &home);
    s.goose_binary = Path::new("goose");
    match sanitized_spawn(&s) {
        Err(SpawnDenial::NonAbsoluteBinary(_)) => {}
        other => panic!("relative binary path must refuse, got {other:?}"),
    }
}

#[test]
fn byok_credential_seam_accepts_only_the_resolved_providers_key() {
    let home = std::env::temp_dir();

    // The right key for the right provider passes and is injected.
    let mut s = spec("anthropic", "claude-sonnet-5", &home);
    s.credential = Some(CredentialVar {
        key: "ANTHROPIC_API_KEY".into(),
        value: "user-supplied".into(),
    });
    let ok = sanitized_spawn(&s).expect("matching credential must pass");
    assert_eq!(ok.env.get("ANTHROPIC_API_KEY").unwrap(), "user-supplied");

    // A key for a different provider is refused — even a permitted one.
    let mut s = spec("ollama", "gemma3:1b", &home);
    s.credential = Some(CredentialVar {
        key: "ANTHROPIC_API_KEY".into(),
        value: "user-supplied".into(),
    });
    match sanitized_spawn(&s) {
        Err(SpawnDenial::CredentialNotForProvider { .. }) => {}
        other => panic!("cross-provider credential must refuse, got {other:?}"),
    }

    // An excluded vendor's key is refused outright.
    let mut s = spec("anthropic", "claude-sonnet-5", &home);
    s.credential = Some(CredentialVar {
        key: "OPENAI_API_KEY".into(),
        value: "planted".into(),
    });
    assert!(sanitized_spawn(&s).is_err());
}

#[test]
fn sanitized_command_is_goose_acp_on_the_absolute_path() {
    let home = std::env::temp_dir();
    let sanitized =
        sanitized_spawn(&spec("anthropic", "claude-sonnet-5", &home)).expect("must pass");
    assert_eq!(
        sanitized.command.get_program().to_string_lossy(),
        "/opt/holmes/goose"
    );
    let args: Vec<_> = sanitized.command.get_args().collect();
    assert_eq!(args, ["acp"]);
}
