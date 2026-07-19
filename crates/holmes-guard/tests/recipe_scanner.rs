//! Lock 1d integration tests — the recipe safety scan with both controls,
//! run against the real repo files (not synthetic strings), mirroring the
//! CI wiring.

use holmes_guard::scan::recipes::scan_path;
use std::path::Path;

fn repo_root() -> &'static Path {
    // crates/holmes-guard -> repo root
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
}

/// Positive control: the planted fixture (zero-width + bidi override +
/// tag-block payload) fires the scan.
#[test]
fn d4_positive_control_planted_recipe_fires() {
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/planted_recipe_smuggled.yaml");
    let (files, hits) = scan_path(&fixture).expect("fixture readable");
    assert_eq!(files, 1);
    assert!(
        hits.len() >= 3,
        "planted zero-width, bidi, and tag payloads must all fire; got {hits:?}"
    );
    let classes: Vec<&str> = hits.iter().map(|h| h.class).collect();
    assert!(classes.iter().any(|c| c.contains("zero-width")));
    assert!(classes.iter().any(|c| c.contains("bidi")));
    assert!(classes.iter().any(|c| c.contains("tag block")));
}

/// Negative control: the real recipes/ tree is clean — and non-ASCII
/// *letters* (the Spanish subsystem names) do not false-positive.
#[test]
fn d5_negative_control_real_recipes_are_clean() {
    let recipes = repo_root().join("recipes");
    let (files, hits) = scan_path(&recipes).expect("recipes dir readable");
    assert!(files >= 2, "expected the committed recipes to be scanned");
    assert!(hits.is_empty(), "real recipes must be clean: {hits:?}");
}

/// Checked-and-absent vs didn't-check: a missing path is an error, never
/// an empty CLEAN.
#[test]
fn d6_missing_path_errors_rather_than_passing() {
    assert!(scan_path(Path::new("no/such/dir-anywhere")).is_err());
}
