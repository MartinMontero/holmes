//! Lock 2a structural — the invalidation-not-deletion invariant asserted
//! across the whole Cypher surface (belt-and-suspenders with the in-module
//! test): scan `src/cypher.rs` for any destructive keyword outside the
//! audit machinery itself.

use std::path::Path;

#[test]
fn cypher_module_contains_no_destructive_operations() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/cypher.rs");
    let src = std::fs::read_to_string(&path).expect("read cypher.rs");

    // Only string-literal Cypher matters; scan lines that are builder
    // return strings (contain a quote and a Cypher verb), skipping the
    // test module and the audit list that names the forbidden words.
    let mut in_tests = false;
    for line in src.lines() {
        if line.trim_start().starts_with("#[cfg(test)]") {
            in_tests = true;
        }
        if in_tests {
            continue;
        }
        let l = line.to_uppercase();
        // Cypher string literals in this module start with a MATCH/CREATE.
        if l.contains('"') && (l.contains("MATCH (") || l.contains("CREATE (")) {
            for forbidden in ["DELETE", "DETACH DELETE", " REMOVE ", " DROP "] {
                assert!(
                    !l.contains(forbidden),
                    "destructive `{forbidden}` in a Cypher builder: {line}"
                );
            }
        }
    }
}
