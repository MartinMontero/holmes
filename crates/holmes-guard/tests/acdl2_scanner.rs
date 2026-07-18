//! AC-DL-2 (v3) — all seven criteria, positive and negative controls firing.
//! Control convention per v3 §4/§5: **positive control = the gate fires on
//! bad input**; **negative control = a permitted-adjacent package does not
//! trip**. Hermetic and deterministic; no network.
//! This file is an AC-DL-2 scan exemption (planted fixtures name excluded
//! identifiers on purpose).

use holmes_guard::scan::{
    check_package_name, discover_lockfiles, scan_lock_text, scan_text_for_model_ids,
    scan_workspace, scan_workspace_full, ScanReport, ViolationKind,
};
use std::path::{Path, PathBuf};

const PLANTED_LOCK: &str = include_str!("fixtures/planted.lock");
const PLANTED_TRANSITIVE: &str = include_str!("fixtures/planted_transitive.lock");
const PLANTED_NPM: &str = include_str!("fixtures/planted_npm.json");
const PLANTED_PIP: &str = include_str!("fixtures/planted_python.txt");

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

#[test]
fn c1_full_lock_walk_is_deterministic_and_complete() {
    let mut a = ScanReport::default();
    scan_lock_text(PLANTED_LOCK, "planted.lock", &mut a);
    assert_eq!(a.packages_scanned, 7, "every [[package]] must be walked");

    let mut b = ScanReport::default();
    scan_lock_text(PLANTED_LOCK, "planted.lock", &mut b);
    assert_eq!(
        format!("{:?}", a.violations),
        format!("{:?}", b.violations),
        "same input must yield the same verdict"
    );
}

#[test]
fn c1_walks_lockfiles_across_ecosystems() {
    // §1: Node and Python lockfiles are walked, not just Rust.
    let mut npm = ScanReport::default();
    scan_lock_text(PLANTED_NPM, "fixtures/planted_npm.json", &mut npm);
    let npm_subjects: Vec<_> = npm.violations.iter().map(|v| v.subject.as_str()).collect();
    assert!(
        npm_subjects.contains(&"openai"),
        "npm openai not flagged: {npm_subjects:?}"
    );
    assert!(
        npm_subjects.contains(&"litellm-proxy"),
        "npm litellm-proxy not flagged"
    );
    assert!(
        !npm_subjects.contains(&"react"),
        "permitted npm dep wrongly flagged"
    );

    let mut pip = ScanReport::default();
    scan_lock_text(PLANTED_PIP, "fixtures/planted_python.txt", &mut pip);
    let pip_subjects: Vec<_> = pip.violations.iter().map(|v| v.subject.as_str()).collect();
    for planted in ["openai", "tiktoken", "litellm"] {
        assert!(
            pip_subjects.contains(&planted),
            "pip {planted} not flagged: {pip_subjects:?}"
        );
    }
    assert!(
        !pip_subjects.contains(&"graphiti-core"),
        "permitted pip dep wrongly flagged"
    );
    assert!(
        !pip_subjects.contains(&"qwen-agent"),
        "permitted pip dep wrongly flagged"
    );
}

#[test]
fn c1_discovery_finds_the_real_cargo_lock_only() {
    // Discovery is exact-name, so planted non-canonical fixtures are never
    // swept into a real-tree scan.
    let found = discover_lockfiles(&repo_root()).expect("discover");
    assert!(
        found.iter().any(|p| p.ends_with("Cargo.lock")),
        "Cargo.lock not discovered: {found:?}"
    );
    assert!(
        !found
            .iter()
            .any(|p| p.to_string_lossy().contains("planted")),
        "a planted fixture was discovered: {found:?}"
    );
}

#[test]
fn c2_excluded_vendor_namespaces_rejected_wherever_they_appear() {
    let mut out = Vec::new();
    check_package_name("async-openai", "fixture", &mut out);
    check_package_name("tiktoken-rs", "fixture", &mut out);
    check_package_name("llama-cpp-2", "fixture", &mut out);
    check_package_name("llama-stack", "fixture", &mut out);
    check_package_name("meta-llama-tools", "fixture", &mut out);
    assert_eq!(out.len(), 5, "namespace hits: {out:?}");
    assert!(out
        .iter()
        .all(|v| v.kind == ViolationKind::ExcludedNamespacePackage));

    // Innocent lookalikes pass: token-scoped, not substring-of-whole-name.
    let mut clean = Vec::new();
    for name in [
        "ollama-rs",
        "metadata",
        "serde",
        "groq-client",
        "litellmish",
    ] {
        check_package_name(name, "fixture", &mut clean);
    }
    assert!(clean.is_empty(), "false positives: {clean:?}");
}

#[test]
fn c2_seed_list_covers_every_v3_documented_entry() {
    // The checked-in seed denylist must carry the v3 §2 entries with rationale.
    use holmes_guard::scan::PACKAGE_SEEDS;
    for required in [
        "openai",
        "tiktoken",
        "llama",
        "xai",
        "grok",
        "litellm",
        "openrouter",
    ] {
        let seed = PACKAGE_SEEDS.iter().find(|s| s.token == required);
        let seed = seed.unwrap_or_else(|| panic!("seed '{required}' missing from PACKAGE_SEEDS"));
        assert!(
            !seed.rationale.is_empty(),
            "seed '{required}' lacks a documented rationale"
        );
    }
}

#[test]
fn c3_excluded_model_family_ids_in_configs_and_code_fail() {
    for text in [
        "default_model: llama-3.3-70b\n",
        "const FALLBACK: &str = \"gpt-4o\";\n",
        "model: grok-4\n",
        "reasoning = \"o3\"\n",
    ] {
        assert!(
            !scan_text_for_model_ids("cfg", text).is_empty(),
            "missed: {text:?}"
        );
    }
    for text in [
        "model: gemma3:1b\n",
        "model: qwen3.5-27b\n",
        "provider: ollama\n",
        "model: claude-sonnet-5\n",
        "endpoint: o3b-service\n",
    ] {
        assert!(
            scan_text_for_model_ids("clean", text).is_empty(),
            "false positive: {text:?}"
        );
    }
}

#[test]
fn c3_transitive_violation_names_the_dependency_path() {
    // §3: a transitively-introduced excluded package is traceable to its parent.
    let mut report = ScanReport::default();
    scan_lock_text(PLANTED_TRANSITIVE, "planted_transitive.lock", &mut report);
    let v = report
        .violations
        .iter()
        .find(|v| v.subject == "async-openai")
        .expect("async-openai must be flagged");
    assert!(v.traced, "TOML lockfile edges must be traced");
    assert_eq!(
        v.dependency_path,
        vec!["holmes-app", "middleware-lib", "async-openai"],
        "dependency path must trace root -> middle -> offender"
    );
    assert!(v
        .path_note()
        .contains("holmes-app -> middleware-lib -> async-openai"));
}

#[test]
fn c4_router_gateway_seed_list_fires_including_variants() {
    // §2 router seeds, incl. real-world variant/transitive spellings.
    let mut out = Vec::new();
    for name in [
        "litellm",
        "openrouter",
        "litellm-proxy",
        "litellm-proxy-extras",
        "openrouter-py",
        "litellm_router",
    ] {
        check_package_name(name, "fixture", &mut out);
    }
    assert_eq!(out.len(), 6, "router hits: {out:?}");
    assert!(out
        .iter()
        .all(|v| v.kind == ViolationKind::RouterGatewayPackage));
}

#[test]
fn c4_positive_control_planted_lockfile_fires_the_gate() {
    // v3 convention: POSITIVE control = the gate fires on bad input.
    let mut report = ScanReport::default();
    scan_lock_text(PLANTED_LOCK, "planted.lock", &mut report);
    assert!(!report.clean(), "planted excluded deps must fail the gate");
    let subjects: Vec<_> = report
        .violations
        .iter()
        .map(|v| v.subject.as_str())
        .collect();
    for planted in [
        "async-openai",
        "tiktoken-rs",
        "litellm",
        "openrouter",
        "llama-cpp-2",
    ] {
        assert!(subjects.contains(&planted), "{planted} not flagged");
    }
    assert!(
        !subjects.contains(&"serde"),
        "clean package wrongly flagged"
    );
}

#[test]
fn c5_negative_control_permitted_adjacent_packages_do_not_trip() {
    // v3 §5: permitted-but-adjacent packages must NOT trip the matcher.
    let mut out = Vec::new();
    for name in [
        "ollama",
        "ollama-rs",
        "anthropic-sdk",
        "async-anthropic",
        "google-generativeai",
        "google-cloud-aiplatform",
        "qwen-agent",
        "serde",
        "tokio",
    ] {
        check_package_name(name, "fixture", &mut out);
    }
    assert!(out.is_empty(), "matcher is over-broad: {out:?}");
}

#[test]
fn c5_negative_control_real_workspace_passes_untouched() {
    let root = repo_root();
    let report = scan_workspace_full(&root, &[]).expect("scan workspace");
    assert!(
        report
            .lockfiles_walked
            .iter()
            .any(|l| l.ends_with("Cargo.lock")),
        "real Cargo.lock not walked: {:?}",
        report.lockfiles_walked
    );
    assert!(report.packages_scanned >= 2, "lockfile walk looks empty");
    assert!(report.files_scanned > 5, "tree sweep looks empty");
    assert!(
        report.clean(),
        "the permitted stack must pass; violations: {:?}",
        report.violations
    );
    // Exemptions visible, never silent.
    assert!(
        report
            .exemptions_applied
            .iter()
            .any(|(p, _)| p == "crates/holmes-guard/src/policy.rs"),
        "expected the compiled-denylist exemption to be reported; got {:?}",
        report.exemptions_applied
    );
}

#[test]
fn c6_denylist_is_distinct_from_the_cve_gate() {
    // §6: AC-DL-2 is provider exclusion, not the CVE scan. litellm is exactly
    // the cross-check case — it must trip AC-DL-2 here regardless of any CVE
    // status. (The CVE gate — Syft/OSV/Grype — is a separate CI concern.)
    let mut out = Vec::new();
    check_package_name("litellm", "fixture", &mut out);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].kind, ViolationKind::RouterGatewayPackage);
}

#[test]
fn c7_ci_wiring_present_with_both_controls_in_one_workflow() {
    let workflow = repo_root().join(".github/workflows/acdl-gate.yml");
    let text = std::fs::read_to_string(&workflow)
        .unwrap_or_else(|e| panic!("joint-gate workflow missing at {}: {e}", workflow.display()));
    for needle in [
        "cargo test --release",
        "--bin acdl2-scan",
        "planted.lock",
        "positive control",
        "negative control",
        "SCHEDULED",
        "schedule",
        "pull_request",
    ] {
        assert!(
            text.contains(needle),
            "workflow lacks required element {needle:?}"
        );
    }
}

#[test]
fn scan_workspace_single_lockfile_helper_still_works() {
    let root = repo_root();
    let report = scan_workspace(&root, &fixtures().join("planted.lock")).expect("scan");
    assert!(
        !report.clean(),
        "planted fixture via single-lockfile helper must fail"
    );
}
