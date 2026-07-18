//! Compiled AC-DL-2 exemptions: paths permitted to contain excluded
//! identifiers, each with its reason. The scan report lists every exemption
//! it actually used — visible, never silent. Grows only by ledgered change.

pub const EXEMPT_PATHS: &[(&str, &str)] = &[
    (
        "crates/holmes-guard/src/policy.rs",
        "the compiled denylist itself: excluded-identifier pattern definitions",
    ),
    (
        "crates/holmes-guard/src/scan/",
        "scanner pattern and exemption definitions",
    ),
    (
        "crates/holmes-guard/tests/",
        "planted negative-control fixtures (AC-DL-2 §5) and denial assertions",
    ),
];

/// Exemption reason for a repo-relative path (forward-slash separated), if any.
pub fn exemption_for(rel_path: &str) -> Option<&'static str> {
    EXEMPT_PATHS
        .iter()
        .find(|(p, _)| rel_path == *p || rel_path.starts_with(p))
        .map(|(_, reason)| *reason)
}
