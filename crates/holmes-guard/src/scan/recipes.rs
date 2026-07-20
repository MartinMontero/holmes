//! Lock 1d — recipe safety scan (loop §6 Phase 1; roadmap: smuggling
//! scanner moved here from Phase 4).
//!
//! Detects invisible/deceptive Unicode ("ASCII smuggling") in recipe
//! files: zero-width characters, bidirectional-override controls, the
//! Unicode tag block, variation selectors, and kin — the channels used to
//! hide instructions from a human reviewer while a model still reads
//! them. The scan **fails closed and never auto-strips**: silently
//! rewriting an attacked file would hide the attack; a visible refusal
//! surfaces it.
//!
//! The codepoint set below is a documented floor (like the AC-DL-2 seed
//! list); it grows by ledgered amendment, never silently.

use std::fmt;
use std::path::{Path, PathBuf};

/// One smuggling hit: where, what, and why it is flagged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmugglingHit {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub codepoint: u32,
    pub class: &'static str,
}

impl fmt::Display for SmugglingHit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}: U+{:04X} ({})",
            self.file.display(),
            self.line,
            self.column,
            self.codepoint,
            self.class
        )
    }
}

/// Classify a character as an invisible/deceptive-Unicode smuggling
/// vector, or `None` for ordinary text (all printable ASCII and normal
/// letters in any script pass).
pub fn smuggling_class(c: char) -> Option<&'static str> {
    let cp = c as u32;
    match cp {
        0x00AD => Some("soft hyphen"),
        0x034F => Some("combining grapheme joiner"),
        0x061C => Some("arabic letter mark"),
        // Hangul fillers render blank — invisible placeholders (F-035).
        0x115F..=0x1160 => Some("hangul filler (blank)"),
        // Khmer inherent-vowel signs, deprecated and invisible (F-035).
        0x17B4..=0x17B5 => Some("khmer inherent vowel (invisible)"),
        0x180E => Some("mongolian vowel separator"),
        0x200B..=0x200F => Some("zero-width / direction mark"),
        0x202A..=0x202E => Some("bidi embedding/override control"),
        // 0x2065 is the reserved gap between the invisible operators and
        // the bidi isolates; fold it in so no member of the block leaks.
        0x2060..=0x2065 => Some("invisible operator / word joiner"),
        0x2066..=0x2069 => Some("bidi isolate control"),
        // Deprecated format controls (symmetric-swapping / shaping /
        // national digit shapes) — the gap above the bidi isolates (F-035).
        0x206A..=0x206F => Some("deprecated format control"),
        0x3164 => Some("hangul filler (blank)"),
        0xFE00..=0xFE0F => Some("variation selector"),
        0xFEFF => Some("zero-width no-break space (BOM in text)"),
        0xFFA0 => Some("halfwidth hangul filler (blank)"),
        // Interlinear annotation anchors — deceptive hidden-text framing.
        0xFFF9..=0xFFFB => Some("interlinear annotation control"),
        // Musical-symbol begin/end format controls (invisible) (F-035).
        0x1D173..=0x1D17A => Some("musical format control (invisible)"),
        0xE0000..=0xE007F => Some("unicode tag block (ascii smuggling)"),
        0xE0100..=0xE01EF => Some("variation selector supplement"),
        _ => None,
    }
}

/// Scan one text for smuggling characters.
pub fn scan_text(file: &Path, text: &str) -> Vec<SmugglingHit> {
    let mut hits = Vec::new();
    for (line_idx, line) in text.lines().enumerate() {
        for (col_idx, c) in line.chars().enumerate() {
            if let Some(class) = smuggling_class(c) {
                hits.push(SmugglingHit {
                    file: file.to_path_buf(),
                    line: line_idx + 1,
                    column: col_idx + 1,
                    codepoint: c as u32,
                    class,
                });
            }
        }
    }
    hits
}

/// Recipe file extensions the scan covers.
fn is_recipe_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("yaml") | Some("yml") | Some("md")
    )
}

/// Scan a recipe directory tree (or a single file). Missing paths are an
/// error — "I checked and found nothing" must be distinguishable from
/// "I didn't check".
pub fn scan_path(root: &Path) -> Result<(usize, Vec<SmugglingHit>), std::io::Error> {
    let mut files_scanned = 0usize;
    let mut hits = Vec::new();
    if root.is_file() {
        files_scanned += 1;
        hits.extend(scan_text(root, &std::fs::read_to_string(root)?));
        return Ok((files_scanned, hits));
    }
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir)? {
            let path = entry?.path();
            if path.is_dir() {
                stack.push(path);
            } else if is_recipe_file(&path) {
                files_scanned += 1;
                hits.extend(scan_text(&path, &std::fs::read_to_string(&path)?));
            }
        }
    }
    Ok((files_scanned, hits))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_text_passes_including_non_ascii_letters() {
        let hits = scan_text(
            Path::new("t.yaml"),
            "title: la lluvia\ninstructions: ¿Qué estoy asumiendo? — generate hypotheses\n",
        );
        assert!(hits.is_empty(), "accented/Spanish text is not smuggling");
    }

    #[test]
    fn zero_width_bidi_and_tag_characters_are_flagged() {
        let smuggled = format!(
            "step: benign\u{200B} text\u{202E} here {}{}",
            '\u{E0041}', '\u{E0049}'
        );
        let hits = scan_text(Path::new("t.yaml"), &smuggled);
        let classes: Vec<&str> = hits.iter().map(|h| h.class).collect();
        assert!(classes.iter().any(|c| c.contains("zero-width")));
        assert!(classes.iter().any(|c| c.contains("bidi")));
        assert!(classes.iter().any(|c| c.contains("tag block")));
        assert_eq!(hits.len(), 4);
        assert_eq!(hits[0].line, 1);
    }
}
