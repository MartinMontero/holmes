//! AC-DL-1 §1 — ownership: all policy logic lives in the compiled Rust core.
//! The enforceable structural half: no TypeScript/JavaScript exists anywhere
//! in this repository, so no policy logic *can* live there.

use std::path::{Path, PathBuf};

const FORBIDDEN_EXTENSIONS: &[&str] = &["ts", "tsx", "js", "jsx", "mjs", "cjs"];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn walk(dir: &Path, hits: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("read_dir") {
        let entry = entry.expect("dir entry");
        let name = entry.file_name().to_string_lossy().into_owned();
        let ftype = entry.file_type().expect("file type");
        if ftype.is_dir() {
            if name == ".git" || name == "target" || name == "node_modules" {
                continue;
            }
            walk(&entry.path(), hits);
        } else if ftype.is_file() {
            let is_forbidden = entry
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| FORBIDDEN_EXTENSIONS.contains(&e.to_ascii_lowercase().as_str()))
                .unwrap_or(false);
            if is_forbidden {
                hits.push(entry.path());
            }
        }
    }
}

#[test]
fn s1_no_typescript_or_javascript_anywhere_in_the_repo() {
    let mut hits = Vec::new();
    walk(&repo_root(), &mut hits);
    assert!(
        hits.is_empty(),
        "policy-capable TS/JS files present (AC-DL-1 §1 forbids policy outside the Rust core): {hits:?}"
    );
}
