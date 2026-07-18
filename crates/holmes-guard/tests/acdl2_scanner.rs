//! AC-DL-2 — all seven criteria, positive and negative controls firing.
//! Hermetic and deterministic; no network.
//! This file is an AC-DL-2 scan exemption (planted fixtures name excluded
//! identifiers on purpose).

use holmes_guard::scan::{
    check_package_name, scan_lock_text, scan_text_for_model_ids, scan_workspace, ScanReport,
    ViolationKind,
};
use std::path::{Path, PathBuf};

const PLANTED_LOCK: &str = include_str!("fixtures/planted.lock");

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

#[test]
fn c1_full_lock_walk_is_deterministic_and_complete() {
    let mut a = ScanReport::default();
    scan_lock_text(PLANTED_LOCK, "planted.lock", &mut a);
    // Every [[package]] name in the fixture is walked — direct and
    // transitive alike; the lockfile is already the flattened graph.
    assert_eq!(a.packages_scanned, 7);

    let mut b = ScanReport::default();
    scan_lock_text(PLANTED_LOCK, "planted.lock", &mut b);
    assert_eq!(
        format!("{:?}", a.violations),
        format!("{:?}", b.violations),
        "same input must yield the same verdict"
    );
}

#[test]
fn c2_excluded_vendor_namespaces_rejected_wherever_they_appear() {
    let mut out = Vec::new();
    check_package_name("async-openai", "fixture", &mut out);
    check_package_name("tiktoken-rs", "fixture", &mut out);
    check_package_name("llama-cpp-2", "fixture", &mut out);
    check_package_name("meta-llama-tools", "fixture", &mut out);
    assert_eq!(out.len(), 4);
    assert!(out
        .iter()
        .all(|v| v.kind == ViolationKind::ExcludedNamespacePackage));

    // Innocent lookalikes pass: the check is token-scoped, not substring.
    let mut clean = Vec::new();
    check_package_name("ollama-rs", "fixture", &mut clean);
    check_package_name("metadata", "fixture", &mut clean);
    check_package_name("serde", "fixture", &mut clean);
    check_package_name("groq-client", "fixture", &mut clean);
    assert!(clean.is_empty(), "false positives: {clean:?}");
}

#[test]
fn c3_excluded_model_family_ids_in_configs_and_code_fail() {
    let hits = scan_text_for_model_ids("config/models.yaml", "default_model: llama-3.3-70b\n");
    assert!(!hits.is_empty());

    let hits = scan_text_for_model_ids("src/settings.rs", "const FALLBACK: &str = \"gpt-4o\";\n");
    assert!(!hits.is_empty());

    let hits = scan_text_for_model_ids("goose.yaml", "model: grok-4\n");
    assert!(!hits.is_empty());

    let hits = scan_text_for_model_ids("cfg.toml", "reasoning = \"o3\"\n");
    assert!(!hits.is_empty());

    // Permitted roster and near-miss tokens stay clean.
    for text in [
        "model: gemma3:1b\n",
        "model: qwen3.5-27b\n",
        "provider: ollama\n",
        "model: claude-sonnet-5\n",
        "endpoint: o3b-service\n",
        "package.metadata.docs\n",
    ] {
        let hits = scan_text_for_model_ids("clean.yaml", text);
        assert!(hits.is_empty(), "false positive on {text:?}: {hits:?}");
    }
}

#[test]
fn c4_router_gateway_seed_list_fires() {
    let mut out = Vec::new();
    check_package_name("litellm", "fixture", &mut out);
    check_package_name("openrouter", "fixture", &mut out);
    assert_eq!(out.len(), 2);
    assert!(out
        .iter()
        .all(|v| v.kind == ViolationKind::RouterGatewayPackage));
}

#[test]
fn c5_negative_control_planted_lock_fails_the_gate() {
    let mut report = ScanReport::default();
    scan_lock_text(PLANTED_LOCK, "planted.lock", &mut report);
    assert!(!report.clean(), "planted excluded deps must fail the gate");
    let subjects: Vec<_> = report.violations.iter().map(|v| v.subject.as_str()).collect();
    for planted in ["async-openai", "tiktoken-rs", "litellm", "openrouter", "llama-cpp-2"] {
        assert!(subjects.contains(&planted), "{planted} not flagged");
    }
    assert!(!subjects.contains(&"serde"), "clean package wrongly flagged");
    assert!(
        !subjects.contains(&"holmes-guard"),
        "clean package wrongly flagged"
    );
}

#[test]
fn c6_positive_control_real_workspace_passes_untouched() {
    let root = repo_root();
    let report = scan_workspace(&root, &root.join("Cargo.lock")).expect("scan workspace");
    assert!(report.packages_scanned >= 2, "lockfile walk looks empty");
    assert!(report.files_scanned > 5, "tree sweep looks empty");
    assert!(
        report.clean(),
        "the permitted stack must pass; violations: {:?}",
        report.violations
    );
    // Exemptions are visible, never silent: the policy module (which defines
    // the excluded identifiers) must appear as an applied exemption.
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
fn c7_ci_wiring_present_with_both_controls_in_one_workflow() {
    let workflow = repo_root().join(".github/workflows/acdl-gate.yml");
    let text = std::fs::read_to_string(&workflow)
        .unwrap_or_else(|e| panic!("joint-gate workflow missing at {}: {e}", workflow.display()));
    for needle in [
        "cargo test --release",
        "--bin acdl2-scan",
        "planted.lock",
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
