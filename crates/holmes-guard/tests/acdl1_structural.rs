//! AC-DL-1 §1 — ownership: all policy logic lives in the compiled Rust core.
//!
//! Scope, stated honestly: this test enforces one *necessary* condition of
//! §1 — that no TypeScript/JavaScript source exists in the repo's own tree,
//! so the "if a UI layer needs the list it reads it from the crate, never
//! enforces it" rule cannot be quietly violated in TS/JS. It does not, and
//! cannot, prove the full ownership claim: policy authored in another
//! language (Python, shell, a `build.rs`, WASM) would be invisible to it.
//! The full guarantee rests on there being exactly one policy crate and code
//! review; this is the mechanical tripwire for the most likely regression
//! (an Alfred-style TS guard reappearing).
//!
//! Build artifacts (`target`, `.git`) are skipped; everything else in the
//! tree is scanned. Symlinks are flagged, not followed — a symlinked
//! `policy.js` is a hit, not a path to walk past.

use std::path::{Path, PathBuf};

const FORBIDDEN_EXTENSIONS: &[&str] = &["ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs"];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn has_forbidden_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| FORBIDDEN_EXTENSIONS.contains(&e.to_ascii_lowercase().as_str()))
        .unwrap_or(false)
}

fn walk(dir: &Path, hits: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("read_dir") {
        let entry = entry.expect("dir entry");
        let name = entry.file_name().to_string_lossy().into_owned();
        let ftype = entry.file_type().expect("file type");
        if ftype.is_symlink() {
            // Do not follow; flag if the link name itself looks like TS/JS.
            if has_forbidden_extension(&entry.path()) {
                hits.push(entry.path());
            }
        } else if ftype.is_dir() {
            // Skip build/VCS artifacts only. node_modules is deliberately NOT
            // skipped: a vendored JS tree is exactly the kind of policy-in-JS
            // reappearance §1 forbids, and there is none in this repo.
            if name == ".git" || name == "target" {
                continue;
            }
            walk(&entry.path(), hits);
        } else if ftype.is_file() && has_forbidden_extension(&entry.path()) {
            hits.push(entry.path());
        }
    }
}

#[test]
fn s1_no_typescript_or_javascript_anywhere_in_the_repo() {
    let mut hits = Vec::new();
    walk(&repo_root(), &mut hits);
    assert!(
        hits.is_empty(),
        "TS/JS source present — §1's necessary condition (no policy-capable \
         TS/JS in the tree) is violated: {hits:?}"
    );
}
