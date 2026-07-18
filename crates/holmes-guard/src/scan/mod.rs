//! AC-DL-2 — deterministic dependency-tree exclusion gate.
//!
//! Same input, same verdict: a full lockfile walk (direct and transitive)
//! plus a manifest/config/code-constant sweep. No network, no inference, no
//! LLM call inside this gate (triad boundary, holmes-vs-wcjbt §6.4).
//!
//! Exemptions are compiled in [`exemptions`] and applied *visibly*: every
//! exemption used appears in the report; nothing is skipped silently.

pub mod exemptions;

use crate::policy;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ViolationKind {
    /// AC-DL-2 §2: package published by / under an excluded vendor namespace.
    ExcludedNamespacePackage,
    /// AC-DL-2 §4: router/gateway whose function is reaching providers.
    RouterGatewayPackage,
    /// AC-DL-2 §3: excluded model-family identifier in a manifest, config,
    /// or code constant.
    ExcludedModelFamilyIdentifier,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Violation {
    pub kind: ViolationKind,
    pub subject: String,
    pub location: String,
}

#[derive(Debug, Default)]
pub struct ScanReport {
    pub packages_scanned: usize,
    pub files_scanned: usize,
    pub violations: Vec<Violation>,
    /// (path, reason) for every compiled exemption actually used.
    pub exemptions_applied: Vec<(String, &'static str)>,
}

impl ScanReport {
    pub fn clean(&self) -> bool {
        self.violations.is_empty()
    }
}

/// Router/gateway seed list (AC-DL-2 §4): reaching excluded vendors through
/// an intermediary is still reaching them. Seed entry litellm [CARRIED —
/// facts ledger]; openrouter added by ledgered amendment A-06. Grows only by
/// ledgered amendment.
pub const ROUTER_GATEWAY_PACKAGES: &[&str] = &["litellm", "openrouter"];

/// File extensions treated as manifests/configs/code for §3. Markdown and
/// the docs/ tree are out of scope: canon documents name excluded vendors
/// legitimately (e.g. the denylist criteria themselves).
const SCANNED_EXTENSIONS: &[&str] = &[
    "rs", "toml", "yml", "yaml", "json", "lock", "sh", "ps1", "cfg", "conf", "ini",
];

const SKIPPED_DIRS: &[&str] = &[".git", "target", "docs", "node_modules"];

/// Walk one lockfile's complete package set (§1) and check every package
/// name (§2, §4). Appends to `report`.
pub fn scan_lock_text(text: &str, location: &str, report: &mut ScanReport) {
    for line in text.lines() {
        let line = line.trim();
        if let Some(name) = line
            .strip_prefix("name = \"")
            .and_then(|rest| rest.strip_suffix('"'))
        {
            report.packages_scanned += 1;
            check_package_name(name, location, &mut report.violations);
        }
    }
}

/// §2 + §4 verdict for a single package name.
pub fn check_package_name(name: &str, location: &str, out: &mut Vec<Violation>) {
    let normalized = name.trim().to_ascii_lowercase();
    if ROUTER_GATEWAY_PACKAGES.contains(&normalized.as_str()) {
        out.push(Violation {
            kind: ViolationKind::RouterGatewayPackage,
            subject: normalized,
            location: location.to_owned(),
        });
        return;
    }
    if policy::tokenize(&normalized)
        .iter()
        .any(|t| policy::excluded_namespace_token(t))
    {
        out.push(Violation {
            kind: ViolationKind::ExcludedNamespacePackage,
            subject: normalized,
            location: location.to_owned(),
        });
    }
}

/// §3: excluded model-family identifiers in one file's text.
pub fn scan_text_for_model_ids(rel_path: &str, text: &str) -> Vec<Violation> {
    let mut out = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        for token in policy::tokenize(line) {
            if policy::excluded_model_token(&token) {
                out.push(Violation {
                    kind: ViolationKind::ExcludedModelFamilyIdentifier,
                    subject: token,
                    location: format!("{rel_path}:{}", idx + 1),
                });
            }
        }
    }
    out
}

/// Full gate: lockfile walk + tree sweep, deterministic ordering.
pub fn scan_workspace(root: &Path, lockfile: &Path) -> io::Result<ScanReport> {
    let mut report = ScanReport::default();
    let lock_text = fs::read_to_string(lockfile)?;
    scan_lock_text(&lock_text, &lockfile.display().to_string(), &mut report);

    let mut files = Vec::new();
    collect_files(root, Path::new(""), &mut files)?;
    files.sort();
    for rel in files {
        let Ok(text) = fs::read_to_string(root.join(&rel)) else {
            continue; // non-UTF-8 payloads carry no scannable identifiers
        };
        report.files_scanned += 1;
        let hits = scan_text_for_model_ids(&rel, &text);
        if hits.is_empty() {
            continue;
        }
        match exemptions::exemption_for(&rel) {
            Some(reason) => report.exemptions_applied.push((rel.clone(), reason)),
            None => report.violations.extend(hits),
        }
    }
    report.violations.sort();
    report.violations.dedup();
    report.exemptions_applied.sort();
    Ok(report)
}

fn collect_files(root: &Path, rel: &Path, out: &mut Vec<String>) -> io::Result<()> {
    for entry in fs::read_dir(root.join(rel))? {
        let entry = entry?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy().into_owned();
        let child_rel = if rel.as_os_str().is_empty() {
            name_str.clone()
        } else {
            format!("{}/{}", rel.display(), name_str)
        };
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            if SKIPPED_DIRS.contains(&name_str.as_str()) || name_str.starts_with('.') {
                // .github is config and must be scanned; other dotdirs are not.
                if name_str != ".github" {
                    continue;
                }
            }
            collect_files(root, Path::new(&child_rel), out)?;
        } else if file_type.is_file() {
            let scannable = Path::new(&name_str)
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| SCANNED_EXTENSIONS.contains(&e.to_ascii_lowercase().as_str()))
                .unwrap_or(false);
            if scannable {
                out.push(child_rel);
            }
        }
        // Symlinks are neither followed nor scanned.
    }
    Ok(())
}
