//! Lock 0d structural check — holmes-vs-wcjbt §6.4 invariant 1: "Holmes
//! never authors the blueprint … no Holmes output artifact has type
//! blueprint/constitution/spec; Holmes emits only evidence_pack/case_file."
//!
//! Honest scope (necessary condition, not proof): this scans
//! `crates/holmes-core/src` for public items whose *names* claim a
//! blueprint-authoring artifact. It cannot prove no future code path
//! authors one — that stays guarded by review plus this test growing with
//! the crate.

use std::fs;
use std::path::Path;

/// Public-item names that would constitute a blueprint-type export.
/// "spec"/"constitution" are matched as whole identifiers; "blueprint"
/// anywhere in an identifier.
fn names_a_blueprint_artifact(ident: &str) -> bool {
    let id = ident.to_ascii_lowercase();
    id.contains("blueprint") || id == "spec" || id == "constitution" || id == "buildplan"
}

fn public_item_idents(source: &str) -> Vec<String> {
    let mut idents = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim_start();
        for keyword in [
            "pub struct ",
            "pub enum ",
            "pub type ",
            "pub trait ",
            "pub fn ",
        ] {
            if let Some(rest) = trimmed.strip_prefix(keyword) {
                let ident: String = rest
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .collect();
                if !ident.is_empty() {
                    idents.push(ident);
                }
            }
        }
    }
    idents
}

#[test]
fn no_blueprint_type_exports_in_the_public_api() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut checked_files = 0usize;
    let mut offenders = Vec::new();

    let mut stack = vec![src_dir];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).expect("read src dir") {
            let path = entry.expect("dir entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                checked_files += 1;
                let source = fs::read_to_string(&path).expect("read source file");
                for ident in public_item_idents(&source) {
                    if names_a_blueprint_artifact(&ident) {
                        offenders.push(format!("{}: pub item `{ident}`", path.display()));
                    }
                }
            }
        }
    }

    assert!(checked_files > 0, "structural scan saw no source files");
    assert!(
        offenders.is_empty(),
        "blueprint-type exports found (invariant 1): {offenders:?}"
    );
}

#[test]
fn the_scan_itself_catches_offending_names() {
    // Fidelity check on the scanner (F-020 lesson: a structural test must
    // demonstrate it can fire, not just that it currently passes).
    let planted = "pub struct Blueprint {\n}\npub fn spec() {}\npub enum Verdict {}\n";
    let idents = public_item_idents(planted);
    assert!(idents.iter().any(|i| names_a_blueprint_artifact(i)));
    assert!(!names_a_blueprint_artifact("Verdict"));
    assert!(!names_a_blueprint_artifact("SpawnSpec"),
        "identifiers merely containing 'spec' as a fragment are out of scope; whole-ident match only");
}
