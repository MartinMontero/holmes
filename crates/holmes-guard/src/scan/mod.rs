//! AC-DL-2 — deterministic dependency-tree exclusion gate (v3).
//!
//! Same input, same verdict: every dependency lockfile present is walked as
//! a resolved transitive closure (§1), package names checked against a
//! checked-in, documented seed denylist (§2), and any hit fails the gate
//! naming the package, the lockfile, and the dependency path that pulled it
//! in (§3). Source files are additionally swept for excluded model-family
//! identifiers. No network, no inference, no LLM call inside this gate
//! (triad boundary, holmes-vs-wcjbt §6.4).
//!
//! AC-DL-2 is **provider exclusion**, deliberately distinct from the ambient
//! supply-chain CVE gate (Syft SBOM + OSV-Scanner + Grype, no Trivy) — §6.
//! Both run in CI; neither substitutes for the other. `litellm` should trip
//! both, a useful cross-check.
//!
//! Exemptions are compiled in [`exemptions`] and applied *visibly*: every
//! exemption used appears in the report; nothing is skipped silently.
//!
//! Fails closed: an input the gate cannot read (I/O error, not UTF-8 and no
//! UTF-16 BOM, NUL bytes) or a lockfile it cannot parse is itself a
//! violation ([`ViolationKind::UnreadableInput`]), never a silent skip.
//! Noise rules for the matchers live in [`matching`] and can only remove a
//! hit, never add one.

pub mod exemptions;
pub mod matching;
pub mod recipes;

use crate::policy;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ViolationKind {
    /// §2: package published by / under an excluded vendor namespace.
    ExcludedNamespacePackage,
    /// §2: router/gateway whose function is reaching providers.
    RouterGatewayPackage,
    /// §2 (second half): excluded model-family id in a manifest/config/code.
    ExcludedModelFamilyIdentifier,
    /// An input the gate could not read or parse. `subject` is the reason.
    /// Fail-closed: a scan that could not see an input cannot pass it.
    UnreadableInput,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Violation {
    pub kind: ViolationKind,
    pub subject: String,
    pub location: String,
    /// §3: root → … → offender. Length 1 = direct/root entry. Empty when the
    /// lockfile format carries no dependency edges (see `traced`).
    pub dependency_path: Vec<String>,
    /// True when the lockfile format exposed edges and the path was traced.
    pub traced: bool,
}

impl Violation {
    /// Human-readable §3 provenance line.
    pub fn path_note(&self) -> String {
        if self.kind == ViolationKind::UnreadableInput {
            "input could not be read or parsed; the gate fails closed".to_owned()
        } else if !self.traced {
            format!(
                "listed in {} (format carries no dependency edges — path not traced)",
                self.location
            )
        } else if self.dependency_path.len() > 1 {
            format!("pulled in via {}", self.dependency_path.join(" -> "))
        } else {
            "direct/root dependency".to_owned()
        }
    }
}

#[derive(Debug, Default)]
pub struct ScanReport {
    pub packages_scanned: usize,
    pub files_scanned: usize,
    pub lockfiles_walked: Vec<String>,
    pub violations: Vec<Violation>,
    /// (path, reason) for every compiled exemption actually used.
    pub exemptions_applied: Vec<(String, &'static str)>,
}

impl ScanReport {
    pub fn clean(&self) -> bool {
        self.violations.is_empty()
    }
}

// ── §2 — the checked-in package seed denylist (documented rationale) ──────

/// How a seed token matches a package-name token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeedMatch {
    /// Token equals the seed exactly.
    Exact,
    /// Seed appears anywhere in the token (families that concatenate),
    /// after [`matching::mask_word_noise`] removes word-boundary noise
    /// (`tagptr` is tag + ptr, not GPT).
    Substring,
    /// The seed is an ordinary word as well as a vendor name, so it counts
    /// only in the vendor's own position: the whole name, the npm scope
    /// (`@meta/x`), or the leading token of the scope or name followed by
    /// one of `products` (`meta-llama-*`). `pest_meta`, `ptr_meta` and
    /// `import-meta-resolve` use the word, not the vendor.
    Vendor { products: &'static [&'static str] },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeedClass {
    Namespace,
    Router,
}

/// One documented seed. The list is version-controlled and grows only by
/// ledgered amendment (AC-DL-2 §2).
#[derive(Debug, Clone, Copy)]
pub struct Seed {
    pub token: &'static str,
    pub matching: SeedMatch,
    pub class: SeedClass,
    pub rationale: &'static str,
}

/// AC-DL-2 §2 seed denylist — every entry carries its rationale in-file.
/// `ollama` is carved out of the `llama` substring match in [`seed_hit`]
/// because it is a permitted provider.
pub const PACKAGE_SEEDS: &[Seed] = &[
    Seed {
        token: "openai",
        matching: SeedMatch::Exact,
        class: SeedClass::Namespace,
        rationale: "OpenAI first-party SDK / namespace (excluded vendor)",
    },
    Seed {
        token: "tiktoken",
        matching: SeedMatch::Exact,
        class: SeedClass::Namespace,
        rationale: "OpenAI tokenizer library (v3 §2 seed)",
    },
    Seed {
        token: "llama",
        matching: SeedMatch::Substring,
        class: SeedClass::Namespace,
        rationale:
            "Meta Llama family — covers llama-stack, llama-* (v3 §2 seed); ollama carved out",
    },
    Seed {
        token: "meta",
        matching: SeedMatch::Vendor {
            products: &["llama", "ai"],
        },
        class: SeedClass::Namespace,
        rationale: "Meta vendor namespace (vendor position only: the English word is common)",
    },
    Seed {
        token: "facebook",
        matching: SeedMatch::Exact,
        class: SeedClass::Namespace,
        rationale: "Meta / Facebook vendor namespace",
    },
    Seed {
        token: "gpt",
        matching: SeedMatch::Substring,
        class: SeedClass::Namespace,
        rationale: "OpenAI GPT model-family identifier surfacing as a package token",
    },
    Seed {
        token: "grok",
        matching: SeedMatch::Substring,
        class: SeedClass::Namespace,
        rationale: "xAI Grok family (v3 §2 grok* seed)",
    },
    Seed {
        token: "xai",
        matching: SeedMatch::Exact,
        class: SeedClass::Namespace,
        rationale: "xAI vendor namespace (v3 §2 xai* seed)",
    },
    Seed {
        token: "litellm",
        matching: SeedMatch::Exact,
        class: SeedClass::Router,
        rationale:
            "routes to excluded providers; GHSA-69fq-xp46-6x23 linkage [DIRECTIONAL] (v3 §2 seed)",
    },
    Seed {
        token: "openrouter",
        matching: SeedMatch::Exact,
        class: SeedClass::Router,
        rationale: "multi-vendor router reaching excluded vendors — ledgered amendment A-06",
    },
];

fn seed_hit(token: &str, seed: &Seed) -> bool {
    if token == "ollama" {
        return false; // permitted provider — never a namespace hit
    }
    match seed.matching {
        SeedMatch::Exact => token == seed.token,
        SeedMatch::Substring => matching::mask_word_noise(token).contains(seed.token),
        // Judged on the whole name in `vendor_position_hit`.
        SeedMatch::Vendor { .. } => false,
    }
}

/// `SeedMatch::Vendor`: the seed is the whole name, the npm scope, or the
/// leading token of the scope or of the name followed by a product token.
fn vendor_position_hit(normalized: &str, vendor: &str, products: &[&str]) -> bool {
    let (scope, rest) = match normalized.strip_prefix('@').and_then(|s| s.split_once('/')) {
        Some((scope, rest)) => (Some(scope), rest),
        None => (None, normalized),
    };
    let leads = |part: &str| {
        let t = policy::tokenize(part);
        match t.as_slice() {
            [only] => only == vendor,
            [first, second, ..] => first == vendor && products.contains(&second.as_str()),
            [] => false,
        }
    };
    scope.is_some_and(|s| leads(s)) || leads(rest)
}

/// §2 verdict for a single package name. Router class takes precedence in
/// labelling. Token-scoped: `litellm-proxy` fires (token `litellm`),
/// `litellmish` does not (single token embeds but does not equal the seed).
pub fn check_package_name(name: &str, location: &str, out: &mut Vec<Violation>) {
    let normalized = name.trim().to_ascii_lowercase();
    let tokens = policy::tokenize(&normalized);
    let mut namespace_hit = false;
    for seed in PACKAGE_SEEDS {
        let hit = match seed.matching {
            SeedMatch::Vendor { products } => {
                vendor_position_hit(&normalized, seed.token, products)
            }
            _ => tokens.iter().any(|token| seed_hit(token, seed)),
        };
        if hit {
            match seed.class {
                SeedClass::Router => {
                    out.push(Violation {
                        kind: ViolationKind::RouterGatewayPackage,
                        subject: normalized.clone(),
                        location: location.to_owned(),
                        dependency_path: vec![normalized.clone()],
                        traced: false,
                    });
                    return;
                }
                SeedClass::Namespace => namespace_hit = true,
            }
        }
    }
    if namespace_hit {
        out.push(Violation {
            kind: ViolationKind::ExcludedNamespacePackage,
            subject: normalized.clone(),
            location: location.to_owned(),
            dependency_path: vec![normalized],
            traced: false,
        });
    }
}

// ── §1 — lockfile discovery and per-ecosystem parsing ─────────────────────

/// Canonical lockfile names discovered across ecosystems (§1). Discovery is
/// exact-name to avoid sweeping planted test fixtures (which use
/// non-canonical names).
pub const LOCKFILE_NAMES: &[&str] = &[
    "Cargo.lock",
    "package-lock.json",
    "pnpm-lock.yaml",
    "yarn.lock",
    "uv.lock",
    "poetry.lock",
    "requirements.txt",
];

const SCANNED_EXTENSIONS: &[&str] = &[
    "rs", "toml", "yml", "yaml", "json", "lock", "sh", "ps1", "cfg", "conf", "ini",
];

const SKIPPED_DIRS: &[&str] = &[".git", "target", "docs", "node_modules"];

/// A resolved package with its direct dependency edges (empty when the
/// format carries no edges).
#[derive(Debug, Clone)]
struct LockPackage {
    name: String,
    deps: Vec<String>,
}

/// One parsed lockfile. `problem` is set when the text has content the
/// parser did not recognise — the gate then fails closed on that file
/// instead of passing it with zero packages.
struct ParsedLock {
    packages: Vec<LockPackage>,
    edges: bool,
    problem: Option<&'static str>,
}

/// Parse one lockfile, format inferred from its filename.
fn parse_lockfile(path: &Path, text: &str) -> ParsedLock {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    let has_content = text.lines().any(|l| {
        let t = l.trim();
        !t.is_empty() && !t.starts_with('#')
    });
    let (packages, edges, problem) = if name.ends_with(".json") {
        let p = parse_npm(text);
        let bad = p.is_empty() && has_content && !text.contains("\"packages\"");
        (
            p,
            false,
            bad.then_some(
                "npm lockfile has no \"packages\" map (lockfileVersion 1 layout is not read)",
            ),
        )
    } else if name == "yarn.lock" {
        let p = parse_yarn(text);
        let bad = p.is_empty() && has_content;
        (
            p,
            false,
            bad.then_some("yarn lockfile: no package entries recognised"),
        )
    } else if name.ends_with(".yaml") || name.ends_with(".yml") {
        let p = parse_pnpm(text);
        let bad = p.is_empty() && has_content && !text.contains("lockfileVersion");
        (
            p,
            false,
            bad.then_some("pnpm lockfile: no lockfileVersion and no package entries"),
        )
    } else if name.ends_with(".txt") || name.contains("requirements") {
        (parse_pip(text), false, None)
    } else {
        // Cargo.lock / uv.lock / poetry.lock and other TOML [[package]] lockfiles.
        let p = parse_toml(text);
        let empty_ok = text.lines().any(|l| l.trim() == "package = []");
        let problem = if p.iter().any(|pkg| pkg.name.is_empty()) {
            Some("TOML lockfile: a [[package]] entry has no parsable name")
        } else if p.is_empty() && has_content && !empty_ok {
            Some("TOML lockfile: no [[package]] entries recognised")
        } else {
            None
        };
        (p, true, problem)
    };
    ParsedLock {
        packages,
        edges,
        problem,
    }
}

fn quoted_strings(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '"' {
            let mut s = String::new();
            for d in chars.by_ref() {
                if d == '"' {
                    break;
                }
                s.push(d);
            }
            out.push(s);
        }
    }
    out
}

/// TOML `[[package]]` blocks with `name`, `version`, and `dependencies`.
/// Dependency entries are `"name"` or `"name version (source)"` — the first
/// whitespace token is the name.
fn parse_toml(text: &str) -> Vec<LockPackage> {
    let mut pkgs = Vec::new();
    let mut cur: Option<LockPackage> = None;
    let mut in_deps = false;
    for line in text.lines() {
        let t = line.trim();
        if t == "[[package]]" {
            if let Some(p) = cur.take() {
                pkgs.push(p);
            }
            cur = Some(LockPackage {
                name: String::new(),
                deps: Vec::new(),
            });
            in_deps = false;
        } else if t.starts_with('[') {
            in_deps = false; // any other table ends the deps array
        } else if let Some(p) = cur.as_mut() {
            if let Some(n) = t
                .strip_prefix("name = \"")
                .and_then(|r| r.strip_suffix('"'))
            {
                p.name = n.to_string();
                in_deps = false;
            } else if t.starts_with("dependencies = [") {
                for dep in quoted_strings(t) {
                    if let Some(first) = dep.split_whitespace().next() {
                        p.deps.push(first.to_string());
                    }
                }
                in_deps = !t.contains(']');
            } else if in_deps {
                if t.starts_with(']') {
                    in_deps = false;
                } else {
                    for dep in quoted_strings(t) {
                        if let Some(first) = dep.split_whitespace().next() {
                            p.deps.push(first.to_string());
                        }
                    }
                }
            }
        }
    }
    if let Some(p) = cur.take() {
        pkgs.push(p);
    }
    pkgs
}

/// npm `package-lock.json` — names from `"node_modules/<pkg>"` keys (v2/v3
/// lockfileVersion). Scoped/nested paths reduce to the final package segment.
fn parse_npm(text: &str) -> Vec<LockPackage> {
    let mut names = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("\"node_modules/") {
            if let Some(end) = rest.find('"') {
                let seg = &rest[..end];
                let name = seg.rsplit("/node_modules/").next().unwrap_or(seg);
                if !name.is_empty() {
                    names.push(name.to_string());
                }
            }
        }
    }
    dedup_packages(names)
}

/// pnpm `pnpm-lock.yaml` — package keys like `/<name>@<ver>:` or the v6
/// `'/<name>@<ver>':` form.
fn parse_pnpm(text: &str) -> Vec<LockPackage> {
    let mut names = Vec::new();
    for line in text.lines() {
        let t = line.trim().trim_start_matches('\'').trim_start_matches('/');
        if let Some(at) = t.find('@') {
            let candidate = &t[..at];
            if !candidate.is_empty() && (t.ends_with(':') || t.contains("@")) {
                names.push(candidate.trim_end_matches(':').to_string());
            }
        }
    }
    dedup_packages(names)
}

/// yarn `yarn.lock` — header lines `"<name>@range", <name>@range:`.
fn parse_yarn(text: &str) -> Vec<LockPackage> {
    let mut names = Vec::new();
    for line in text.lines() {
        if line.starts_with(' ') || line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        let header = line.trim_end_matches(':');
        for spec in header.split(',') {
            let spec = spec.trim().trim_matches('"');
            // Strip the @range, respecting a leading @scope.
            let name = if let Some(scoped) = spec.strip_prefix('@') {
                scoped
                    .find('@')
                    .map(|i| format!("@{}", &scoped[..i]))
                    .unwrap_or_else(|| spec.to_string())
            } else {
                spec.split('@').next().unwrap_or(spec).to_string()
            };
            if !name.is_empty() {
                names.push(name);
            }
        }
    }
    dedup_packages(names)
}

/// pip `requirements*.txt` — `<name>[extras]==<ver>` / `>=` / `~=` lines.
fn parse_pip(text: &str) -> Vec<LockPackage> {
    let mut names = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') || t.starts_with('-') {
            continue;
        }
        let name = t
            .split(['=', '>', '<', '~', '!', '[', ';', ' '])
            .next()
            .unwrap_or("")
            .trim();
        if !name.is_empty() {
            names.push(name.to_string());
        }
    }
    dedup_packages(names)
}

fn dedup_packages(names: Vec<String>) -> Vec<LockPackage> {
    let mut seen = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    for n in names {
        if seen.insert(n.clone()) {
            out.push(LockPackage {
                name: n,
                deps: Vec::new(),
            });
        }
    }
    out
}

/// §3 — trace root → … → offender using reverse edges. Shortest path from a
/// root (a package nothing depends on) down to the offender.
fn trace_dependency_path(offender: &str, packages: &[LockPackage]) -> Vec<String> {
    let mut parents: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut has_parent: BTreeMap<&str, bool> = BTreeMap::new();
    for p in packages {
        has_parent.entry(p.name.as_str()).or_insert(false);
        for d in &p.deps {
            parents.entry(d.as_str()).or_default().push(p.name.as_str());
            has_parent.insert(d.as_str(), true);
        }
    }
    // BFS upward from the offender to the nearest root.
    let mut queue: std::collections::VecDeque<Vec<&str>> = std::collections::VecDeque::new();
    let mut visited: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    queue.push_back(vec![offender]);
    visited.insert(offender);
    while let Some(chain) = queue.pop_front() {
        let head = *chain.first().unwrap();
        let is_root = !has_parent.get(head).copied().unwrap_or(false);
        if is_root {
            return chain.iter().map(|s| s.to_string()).collect();
        }
        if let Some(ps) = parents.get(head) {
            for &parent in ps {
                if visited.insert(parent) {
                    let mut next = vec![parent];
                    next.extend_from_slice(&chain);
                    queue.push_back(next);
                }
            }
        }
    }
    vec![offender.to_string()]
}

/// Walk one lockfile: parse packages, check each name, attach dependency
/// paths (§3). Appends to `report`.
pub fn scan_lockfile(path: &Path, text: &str, report: &mut ScanReport) {
    let ParsedLock {
        packages,
        edges,
        problem,
    } = parse_lockfile(path, text);
    let location = path.display().to_string();
    report.lockfiles_walked.push(location.clone());
    if let Some(why) = problem {
        report
            .violations
            .push(unreadable(location.clone(), why.to_owned()));
    }
    for pkg in packages.iter().filter(|p| !p.name.is_empty()) {
        report.packages_scanned += 1;
        let mut hits = Vec::new();
        check_package_name(&pkg.name, &location, &mut hits);
        for mut v in hits {
            if edges {
                v.dependency_path = trace_dependency_path(&pkg.name, &packages);
                v.traced = true;
            }
            report.violations.push(v);
        }
    }
}

/// Back-compat helper retained for tests: walk a TOML lockfile's text.
pub fn scan_lock_text(text: &str, location: &str, report: &mut ScanReport) {
    scan_lockfile(Path::new(location), text, report);
}

// ── §2 (second half) — source-file model-family identifier sweep ──────────

/// Excluded model-family identifiers in one file's text. Families come from
/// `policy::excluded_model_token`; [`matching`] removes word-boundary noise
/// and requires a model context for bare o-series tokens.
pub fn scan_text_for_model_ids(rel_path: &str, text: &str) -> Vec<Violation> {
    let mut out = Vec::new();
    let mut state = matching::SweepState::default();
    for (idx, line) in text.lines().enumerate() {
        for subject in matching::line_hits(line, &mut state) {
            out.push(Violation {
                kind: ViolationKind::ExcludedModelFamilyIdentifier,
                subject,
                location: format!("{rel_path}:{}", idx + 1),
                dependency_path: Vec::new(),
                traced: false,
            });
        }
    }
    out
}

fn unreadable(location: String, reason: String) -> Violation {
    Violation {
        kind: ViolationKind::UnreadableInput,
        subject: reason,
        location,
        dependency_path: Vec::new(),
        traced: false,
    }
}

/// Read one input as text, or say why it cannot be read. UTF-8 (BOM
/// optional) and UTF-16 with a BOM decode. Anything else, and text holding
/// NUL bytes (binary, or UTF-16 without a BOM, where every identifier would
/// be split apart), is unreadable: the caller fails closed on it.
fn read_input(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|e| format!("cannot read: {e}"))?;
    let text = if let Some(rest) = bytes.strip_prefix(b"\xEF\xBB\xBF") {
        String::from_utf8(rest.to_vec()).map_err(|_| "not valid UTF-8".to_owned())?
    } else if let Some(rest) = bytes.strip_prefix(b"\xFF\xFE") {
        decode_utf16(rest, u16::from_le_bytes)?
    } else if let Some(rest) = bytes.strip_prefix(b"\xFE\xFF") {
        decode_utf16(rest, u16::from_be_bytes)?
    } else {
        String::from_utf8(bytes).map_err(|_| "not valid UTF-8 and no UTF-16 BOM".to_owned())?
    };
    if text.contains('\0') {
        return Err("contains NUL bytes (binary, or UTF-16 without a BOM)".to_owned());
    }
    Ok(text)
}

fn decode_utf16(bytes: &[u8], unit: fn([u8; 2]) -> u16) -> Result<String, String> {
    if bytes.len() % 2 != 0 {
        return Err("UTF-16 BOM but odd byte length".to_owned());
    }
    char::decode_utf16(bytes.chunks_exact(2).map(|c| unit([c[0], c[1]])))
        .collect::<Result<String, _>>()
        .map_err(|_| "UTF-16 BOM but invalid UTF-16".to_owned())
}

// ── the full gate ─────────────────────────────────────────────────────────

/// Full gate: discover and walk every lockfile present (§1), plus the source
/// sweep. `explicit_lockfiles`, when non-empty, replaces discovery (used by
/// controls to feed a planted fixture).
pub fn scan_workspace_full(root: &Path, explicit_lockfiles: &[PathBuf]) -> io::Result<ScanReport> {
    let mut report = ScanReport::default();

    let lockfiles: Vec<PathBuf> = if explicit_lockfiles.is_empty() {
        discover_lockfiles(root)?
    } else {
        explicit_lockfiles.to_vec()
    };
    for lf in &lockfiles {
        match read_input(lf) {
            Ok(text) => scan_lockfile(lf, &text, &mut report),
            Err(why) => report
                .violations
                .push(unreadable(lf.display().to_string(), why)),
        }
    }

    let mut files = Vec::new();
    collect_files(root, Path::new(""), &mut files)?;
    files.sort();
    for rel in files {
        let text = match read_input(&root.join(&rel)) {
            Ok(text) => text,
            Err(why) => {
                report.violations.push(unreadable(rel, why));
                continue;
            }
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
    report.lockfiles_walked.sort();
    Ok(report)
}

/// Convenience: scan a single explicit lockfile plus the source sweep
/// (retained for existing callers/tests).
pub fn scan_workspace(root: &Path, lockfile: &Path) -> io::Result<ScanReport> {
    scan_workspace_full(root, &[lockfile.to_path_buf()])
}

/// Discover every canonical lockfile in the tree (§1).
pub fn discover_lockfiles(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut names = Vec::new();
    collect_files(root, Path::new(""), &mut names)?;
    let mut out: Vec<PathBuf> = names
        .into_iter()
        .filter(|rel| {
            Path::new(rel)
                .file_name()
                .and_then(|n| n.to_str())
                .map(|n| LOCKFILE_NAMES.contains(&n))
                .unwrap_or(false)
        })
        .map(|rel| root.join(rel))
        .collect();
    out.sort();
    Ok(out)
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
            let is_lockfile = LOCKFILE_NAMES.contains(&name_str.as_str());
            if scannable || is_lockfile {
                out.push(child_rel);
            }
        }
        // Symlinks are neither followed nor scanned.
    }
    Ok(())
}
