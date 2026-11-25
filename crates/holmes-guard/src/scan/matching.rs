// SPDX-License-Identifier: AGPL-3.0-or-later
//! Noise rules for the AC-DL-2 source sweep and package-name check (W0,
//! BuilderOS lessons strategy: "The Holmes scanner is noisy").
//!
//! Which families are excluded is decided only by [`crate::policy`]
//! (`excluded_model_token`, `is_o_series`) and, for package names, by the
//! seed table in [`super`]. This module adds no family and no vendor: it
//! only marks occurrences that are not a vendor identifier, so it can
//! *remove* a hit, never invent one. Each rule names the false positive it
//! exists for (audit D4, 2026-11-18, `machine\d4\_notes.md`):
//!
//! - `gpt` with letters of the same word on both sides (`tagptr`,
//!   `pendingPtyCount`), unless the letters before it end in `chat`
//!   (ChatGPT compounds keep firing). Trade-off, stated: an all-lowercase
//!   compound such as `autogptplugins` no longer fires; camelCase,
//!   kebab-case and snake_case spellings of it still do.
//! - `llama` inside the permitted provider name `ollama` at the start of a
//!   word (`OllamaClient`), or inside a Spanish form of `llamar`, "to call"
//!   (`llamada`, `llamar`). The bare word `llama` always fires.
//! - the base64 payload of a Subresource-Integrity hash (`sha512-...`),
//!   where any three letters can occur by chance. Hex hashes cannot spell a
//!   family name and need no rule.
//! - a bare o-series token (`o1`, `O7`) fires only in a model context: a
//!   model word (`model`, `reasoning`, ...) within three tokens before it,
//!   an o-series product suffix after it (`-mini`, `-pro`, a dated
//!   snapshot), another hit within three tokens before it, or nesting under
//!   a model-keyed YAML/JSON/Python block.
//!
//! ASCII-only, like `policy::tokenize`.

use crate::policy;
use std::borrow::Cow;

/// Spanish forms of `llamar` ("to call") that contain `llama` but are not
/// the Llama family. `llama` and `llamas` are deliberately absent: they
/// are also the model name and its plural.
pub const SPANISH_LLAMAR_FORMS: &[&str] = &[
    "llamaba",
    "llamabais",
    "llamaban",
    "llamabas",
    "llamad",
    "llamada",
    "llamadas",
    "llamado",
    "llamados",
    "llamamos",
    "llaman",
    "llamando",
    "llamar",
    "llamara",
    "llamaran",
    "llamaras",
    "llamare",
    "llamaremos",
    "llamaria",
    "llamarian",
    "llamarla",
    "llamarlas",
    "llamarle",
    "llamarles",
    "llamarlo",
    "llamarlos",
    "llamarme",
    "llamarnos",
    "llamaron",
    "llamarse",
    "llamarte",
];

/// Words that make a bare o-series token a model id when they precede it
/// or key the block it sits in.
pub const MODEL_CONTEXT_WORDS: &[&str] = &["model", "models", "reasoning", "engine", "llm", "llms"];

/// O-series product suffixes (`o3-mini`, `o1-preview`, `o3-pro`,
/// `o3-deep-research`). A four-digit `20xx` year (dated snapshot) also
/// counts.
pub const O_SERIES_SUFFIXES: &[&str] = &["mini", "preview", "pro", "deep"];

/// How many tokens back a model word or a previous hit gives context.
const CONTEXT_WINDOW: usize = 3;

/// Shortest base64 run after `shaNNN-` treated as an SRI payload. SHA-1 is
/// 28 characters; anything shorter is checked as ordinary text.
const SRI_MIN_PAYLOAD: usize = 20;

/// True when byte `i` starts a word inside an identifier: the start, after
/// a non-letter, or a lower-to-Upper camelCase step.
fn word_starts_at(w: &[u8], i: usize) -> bool {
    i == 0
        || !w[i - 1].is_ascii_alphabetic()
        || (w[i].is_ascii_uppercase() && w[i - 1].is_ascii_lowercase())
}

/// The letters-only word around `[start, end)`.
fn word_bounds(w: &[u8], start: usize, end: usize) -> (usize, usize) {
    let mut s = start;
    while !word_starts_at(w, s) {
        s -= 1;
    }
    let mut e = end;
    while e < w.len() && w[e].is_ascii_alphabetic() && !word_starts_at(w, e) {
        e += 1;
    }
    (s, e)
}

fn occurrences(hay: &[u8], needle: &[u8]) -> Vec<usize> {
    if hay.len() < needle.len() {
        return Vec::new();
    }
    (0..=hay.len() - needle.len())
        .filter(|&i| &hay[i..i + needle.len()] == needle)
        .collect()
}

fn gpt_is_noise(orig: &[u8], lower: &[u8], i: usize) -> bool {
    let end = i + 3;
    let letter_before = i > 0 && orig[i - 1].is_ascii_alphabetic() && !word_starts_at(orig, i);
    let letter_after =
        end < orig.len() && orig[end].is_ascii_alphabetic() && !word_starts_at(orig, end);
    if !(letter_before && letter_after) {
        return false;
    }
    let (word_start, _) = word_bounds(orig, i, end);
    !lower[word_start..i].ends_with(b"chat")
}

fn llama_is_noise(orig: &[u8], lower: &[u8], i: usize) -> bool {
    if i >= 1 && lower[i - 1] == b'o' && word_starts_at(orig, i - 1) {
        return true; // `ollama`, the permitted provider
    }
    let (s, e) = word_bounds(orig, i, i + 5);
    SPANISH_LLAMAR_FORMS
        .iter()
        .any(|form| form.as_bytes() == &lower[s..e])
}

/// One identifier token, lowercased, with every noise occurrence of `gpt`
/// or `llama` masked out. Feed the result to the policy predicates: a token
/// whose only family occurrences were noise no longer matches.
pub fn mask_word_noise(token: &str) -> String {
    let orig = token.as_bytes();
    let lower = token.to_ascii_lowercase().into_bytes();
    let mut masked = lower.clone();
    for i in occurrences(&lower, b"gpt") {
        if gpt_is_noise(orig, &lower, i) {
            masked[i..i + 3].fill(b'_');
        }
    }
    for i in occurrences(&lower, b"llama") {
        if llama_is_noise(orig, &lower, i) {
            masked[i..i + 5].fill(b'_');
        }
    }
    String::from_utf8(masked).unwrap_or_default()
}

/// Blank the base64 payload of every SRI hash (`sha1-`, `sha256-`,
/// `sha384-`, `sha512-`) on a line.
fn strip_sri_payloads(line: &str) -> Cow<'_, str> {
    const PREFIXES: [&str; 4] = ["sha1-", "sha256-", "sha384-", "sha512-"];
    let b = line.as_bytes();
    let mut out: Option<Vec<u8>> = None;
    let mut i = 0;
    while i < b.len() {
        let at_boundary = i == 0 || !b[i - 1].is_ascii_alphanumeric();
        let prefix = if at_boundary {
            PREFIXES.iter().find(|p| b[i..].starts_with(p.as_bytes()))
        } else {
            None
        };
        if let Some(p) = prefix {
            let start = i + p.len();
            let mut end = start;
            while end < b.len() && (b[end].is_ascii_alphanumeric() || b"+/=".contains(&b[end])) {
                end += 1;
            }
            if end - start >= SRI_MIN_PAYLOAD {
                out.get_or_insert_with(|| b.to_vec())[start..end].fill(b' ');
                i = end;
                continue;
            }
        }
        i += 1;
    }
    match out {
        // Only ASCII bytes were replaced, so the text stays valid UTF-8.
        Some(v) => Cow::Owned(String::from_utf8(v).unwrap_or_default()),
        None => Cow::Borrowed(line),
    }
}

/// Byte ranges of the ASCII-alphanumeric runs on a line.
fn alnum_runs(b: &[u8]) -> Vec<(usize, usize)> {
    let mut runs = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i].is_ascii_alphanumeric() {
            let s = i;
            while i < b.len() && b[i].is_ascii_alphanumeric() {
                i += 1;
            }
            runs.push((s, i));
        } else {
            i += 1;
        }
    }
    runs
}

/// True when any letters-only word of `token` is a model context word
/// (`defaultModel` -> `model`; `REASONING` -> `reasoning`).
fn has_model_word(token: &str) -> bool {
    let w = token.as_bytes();
    let mut i = 0;
    while i < w.len() {
        if !w[i].is_ascii_alphabetic() {
            i += 1;
            continue;
        }
        let (s, e) = word_bounds(w, i, i + 1);
        let word = token[s..e].to_ascii_lowercase();
        if MODEL_CONTEXT_WORDS.contains(&word.as_str()) {
            return true;
        }
        i = e;
    }
    false
}

/// Per-file sweep state: the indentation of an open model-keyed block.
#[derive(Debug, Default)]
pub struct SweepState {
    model_block: Option<usize>,
}

impl SweepState {
    /// Close a block this line dedents out of; report whether the line sits
    /// inside an open model block.
    fn enter_line(&mut self, line: &str) -> bool {
        let body = line.trim_start();
        if body.trim_end().is_empty() {
            return self.model_block.is_some();
        }
        let indent = line.len() - body.len();
        if self.model_block.is_some_and(|open| indent <= open) {
            self.model_block = None;
        }
        self.model_block.is_some()
    }

    /// Open a block when the line ends in `:`, `[`, `{` or `(` and the key
    /// right before its first `:` or `=` holds a model word (`models:`,
    /// `"models": [`, `MODELS = [`). A line with no separator never opens
    /// one: `fn model_graph() {` is code, not a model list (RustGIS
    /// `crates/engine/src/tests.rs:37013`).
    fn leave_line(&mut self, line: &str) {
        if self.model_block.is_some() {
            return; // the outermost model block stays the boundary
        }
        let body = line.trim();
        if !matches!(body.as_bytes().last(), Some(b':' | b'[' | b'{' | b'(')) {
            return;
        }
        let Some(sep) = body.find([':', '=']) else {
            return;
        };
        let before = body[..sep].trim_end();
        let key_start = before
            .char_indices()
            .rev()
            .find(|&(_, c)| !(c.is_ascii_alphanumeric() || "_-.\"'".contains(c)))
            .map_or(0, |(i, c)| i + c.len_utf8());
        let key = &before[key_start..];
        if alnum_runs(key.as_bytes())
            .iter()
            .any(|&(s, e)| has_model_word(&key[s..e]))
        {
            self.model_block = Some(line.len() - line.trim_start().len());
        }
    }
}

fn o_series_in_context(
    line: &str,
    runs: &[(usize, usize)],
    k: usize,
    last_hit: Option<usize>,
    in_block: bool,
) -> bool {
    if in_block {
        return true;
    }
    if let Some(&(ns, ne)) = runs.get(k + 1) {
        let gap = &line[runs[k].1..ns];
        let next = line[ns..ne].to_ascii_lowercase();
        let dated =
            next.len() == 4 && next.starts_with("20") && next.bytes().all(|c| c.is_ascii_digit());
        if (gap == "-" || gap == "_") && (dated || O_SERIES_SUFFIXES.contains(&next.as_str())) {
            return true;
        }
    }
    let from = k.saturating_sub(CONTEXT_WINDOW);
    if last_hit.is_some_and(|h| h >= from) {
        return true;
    }
    runs[from..k]
        .iter()
        .any(|&(s, e)| has_model_word(&line[s..e]))
}

/// Excluded identifiers on one line, as lowercase subjects, in order.
pub fn line_hits(line: &str, state: &mut SweepState) -> Vec<String> {
    let in_block = state.enter_line(line);
    let cleaned = strip_sri_payloads(line);
    let runs = alnum_runs(cleaned.as_bytes());
    let mut hits = Vec::new();
    let mut last_hit = None;
    for (k, &(s, e)) in runs.iter().enumerate() {
        let token = &cleaned[s..e];
        let masked = mask_word_noise(token);
        if !policy::excluded_model_token(&masked) {
            continue;
        }
        if policy::is_o_series(&masked)
            && !o_series_in_context(&cleaned, &runs, k, last_hit, in_block)
        {
            continue;
        }
        hits.push(token.to_ascii_lowercase());
        last_hit = Some(k);
    }
    state.leave_line(line);
    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sri_payloads_are_blanked_only_when_hash_sized() {
        let line = "integrity: sha512-gptHNQghINnc/vTGIk0SOFGFNXw7JVrlRU==";
        assert!(!strip_sri_payloads(line).contains("gpt"));
        // Too short to be a hash: left for the sweep to judge.
        assert_eq!(strip_sri_payloads("sha512-gpt-4o"), "sha512-gpt-4o");
    }

    #[test]
    fn model_block_opens_on_model_key_and_closes_on_dedent() {
        let mut st = SweepState::default();
        assert!(line_hits("reasoning_models:", &mut st).is_empty());
        assert_eq!(line_hits("  - o1", &mut st), vec!["o1"]);
        assert!(line_hits("orders:", &mut st).is_empty());
        assert!(line_hits("  - o1", &mut st).is_empty());
    }

    #[test]
    fn non_ascii_text_near_separators_does_not_panic() {
        // Regression: RustGIS (σ) and Guardrails (closing curly quote)
        // panicked on a byte slice inside a multi-byte character.
        let mut st = SweepState::default();
        for line in ["  \"σ\": {", "“key” = [", "café_model: {", "  - o1 ✓"] {
            let _ = line_hits(line, &mut st);
        }
        assert_eq!(mask_word_noise("o1"), "o1");
    }
}
