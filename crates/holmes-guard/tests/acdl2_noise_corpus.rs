// SPDX-License-Identifier: AGPL-3.0-or-later
//! W0 scanner-noise corpus (BuilderOS lessons strategy, W0 "The Holmes
//! scanner is noisy"). Known false positives and known true positives for
//! the AC-DL-2 gate, run through the same public entry points the
//! `acdl2-scan` binary uses. Hermetic; no network.
//!
//! Sources of the cases (read-only audit, 2026-11-18):
//! `builderos-integration-audit-20261116\machine\d4\_notes.md` and the
//! per-repo scanner output beside it (`machine\d4\*.txt`). Each case names
//! where its shape came from. Cases marked `shape` are written to the
//! pattern of a real hit or of a vendor identity; whether that exact
//! package name exists on a registry is UNVERIFIED and not claimed: they
//! test the matcher's rule, not a registry fact.
//!
//! This file is an AC-DL-2 scan exemption (it names excluded identifiers on
//! purpose; `crates/holmes-guard/tests/` is a compiled exemption).
//!
//! Baseline, EXECUTED 2026-11-25: this file run unchanged against the
//! scanner source at `9109811` (before the W0 fix): false positives fired
//! 28/28, true positives fired 48/48, unreadable inputs failed closed 0/8,
//! known gap (`api.x.ai` alone) not fired. After the fix: 0/28, 48/48,
//! 8/8, gap unchanged.

use holmes_guard::scan::{check_package_name, scan_text_for_model_ids, scan_workspace_full};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy)]
enum Input {
    /// A package name as a lockfile parser would hand it to the §2 check.
    Package(&'static str),
    /// One file's text through the §2 (second half) source sweep.
    Source(&'static str),
}

struct Case {
    id: &'static str,
    input: Input,
    origin: &'static str,
}

const fn pkg(id: &'static str, name: &'static str, origin: &'static str) -> Case {
    Case {
        id,
        input: Input::Package(name),
        origin,
    }
}

const fn src(id: &'static str, text: &'static str, origin: &'static str) -> Case {
    Case {
        id,
        input: Input::Source(text),
        origin,
    }
}

/// Known false positives: none of these name Meta, OpenAI or xAI.
const FALSE_POSITIVES: &[Case] = &[
    // -- exact-token `meta` in ordinary package names (audit D4 note 2) --
    pkg("meta/pest_meta", "pest_meta", "ECC, open-ontologies, RuVector Cargo.lock (pest parser generator)"),
    pkg("meta/ptr_meta", "ptr_meta", "RuVector Cargo.lock (rkyv)"),
    pkg("meta/ptr_meta_derive", "ptr_meta_derive", "RuVector Cargo.lock (rkyv)"),
    pkg("meta/import-meta-resolve", "import-meta-resolve", "CompanyOS, kgdiff, orca pnpm-lock.yaml (JS import.meta tooling)"),
    pkg("meta/babel-import-meta", "@babel/plugin-syntax-import-meta", "RuVector npm/package-lock.json (JS import.meta tooling)"),
    pkg("meta/meta-sim-consciousness", "meta-sim-consciousness", "RuVector local crate (examples/exo-ai-2025)"),
    // -- `gpt` inside an unrelated word (word-boundary failure) --
    pkg("gpt/tagptr", "tagptr", "RuVector Cargo.lock (moka -> tagptr; tag + ptr)"),
    src("gpt/tagptr-lock-line", "name = \"tagptr\"\n", "RuVector Cargo.lock source sweep"),
    src("gpt/pendingPtyCount", "    \"pendingPtyCount\": 0,\n", "orca tests/tools/benchmarks/results/*.json:621 (pending + Pty + Count)"),
    // -- `gpt` inside a base64 integrity hash (audit D4 note 3) --
    src(
        "gpt/sri-hash-npm",
        "      \"integrity\": \"sha512-CTLKqLItRCEixEAewD3/j9DB3/o96gpTPD4eJ1v+DGOlxZRZncRQkGYqqnAGCscYd6RNeXfGeiuCphsPtqyIfQ==\",\n",
        "ECC package-lock.json:64",
    ),
    src(
        "gpt/sri-hash-pnpm",
        "    resolution: {integrity: sha512-gptHNQghINnc/vTGIk0SOFGFNXw7JVrlRUtConJRlvaw6DuX0wO5Jeko9sWrMBhh+PsYAZ7oXAiOnf/UKogyiw==}\n",
        "kgdiff pnpm-lock.yaml",
    ),
    // -- the permitted provider inside a camelCase identifier --
    src("llama/OllamaClient", "let client = OllamaClient::new(host);\n", "RuVector *.rs (OllamaClient x12)"),
    // -- Spanish forms of `llamar` ("to call") (audit D4 note 3) --
    src("llama/es-llamada", "  \"missedCall\": \"Llamada perdida\",\n", "shape of CompanyOS apps/mobile/locales/es.json:1052"),
    src("llama/es-llamadas", "  \"recentCalls\": \"Llamadas recientes\",\n", "shape of CompanyOS apps/mobile/locales/es.json:1033"),
    src("llama/es-llamado", "  \"calledBy\": \"Llamado por {name}\",\n", "shape of CompanyOS apps/web/translations/es.json:14834"),
    src("llama/es-llamados", "  \"callsMade\": \"{n} llamados\",\n", "shape of CompanyOS apps/web/translations/es.json:24874"),
    src("llama/es-llamar", "  \"callNow\": \"Llamar ahora\",\n", "shape of CompanyOS apps/mobile/locales/es.json:1206"),
    src("llama/es-llamarlo", "  \"callHim\": \"Puedes llamarlo luego\",\n", "shape of CompanyOS apps/web/translations/es.json:10747"),
    // -- bare o1..o7 that are not OpenAI o-series ids (audit D4 note 3) --
    src(
        "o-series/order-ids",
        "const store = { orders: new Map([['o1', { id: 'o1', amountCents: 5000 }]]) };\n",
        "shape of ECC docker/context-profiles/complex-corpus-v3.json:93 (order ids)",
    ),
    src("o-series/category-O1", "      O1: Violence and Hate.\n", "Guardrails examples/configs/content_safety/prompts.yml:98"),
    src("o-series/category-O2", "      O2: Category two.\n", "shape of Guardrails prompts.yml category codes"),
    src("o-series/category-O3", "      O3: Category three.\n", "shape of Guardrails prompts.yml category codes"),
    src("o-series/category-O4", "      O4: Category four.\n", "shape of Guardrails prompts.yml category codes"),
    src("o-series/category-O5", "      O5: Category five.\n", "shape of Guardrails prompts.yml category codes"),
    src("o-series/category-O6", "      O6: Category six.\n", "shape of Guardrails prompts.yml category codes"),
    src("o-series/category-O7", "      O7: Category seven.\n", "shape of Guardrails prompts.yml category codes"),
    src("o-series/o0-o12-points", "points: { o0: [0, 0], o12: [1, 2] }\n", "shape of RuVector/RustGIS o0, o12 hits"),
    src(
        "o-series/node-id-in-model-fn",
        "fn model_graph_drops_are_noticed() {\n    let g = json!({\"id\": \"o1\", \"kind\": \"output\"});\n}\n",
        "shape of RustGIS crates/engine/src/tests.rs:37013-37018 (graph node ids)",
    ),
];

/// Known true positives: every one names Meta, OpenAI or xAI (or a router
/// reaching them) and must keep firing.
const TRUE_POSITIVES: &[Case] = &[
    // -- OpenAI packages --
    pkg(
        "openai/openai",
        "openai",
        "CompanyOS pnpm, Guardrails requirements",
    ),
    pkg(
        "openai/async-openai",
        "async-openai",
        "planted.lock fixture",
    ),
    pkg(
        "openai/@openai-codex",
        "@openai/codex",
        "CompanyOS .deepsec/pnpm-lock.yaml (audit D4 note 5)",
    ),
    pkg("openai/langchain-openai", "langchain-openai", "Guardrails"),
    pkg(
        "openai/openai-codex-cli-bin",
        "openai-codex-cli-bin",
        "Guardrails requirements",
    ),
    pkg("openai/tiktoken", "tiktoken", "Guardrails"),
    pkg(
        "openai/otel-instrumentation-openai",
        "opentelemetry-instrumentation-openai",
        "Guardrails",
    ),
    // -- Meta / Facebook packages --
    pkg(
        "meta/@meta-scope",
        "@meta/example-sdk",
        "shape: npm scope @meta",
    ),
    pkg(
        "meta/@meta-llama-scope",
        "@meta-llama/example-client",
        "shape: npm scope @meta-llama",
    ),
    pkg("meta/meta", "meta", "shape: package named exactly meta"),
    pkg(
        "meta/meta-llama-tools",
        "meta-llama-tools",
        "acdl2_scanner.rs c2 fixture",
    ),
    pkg("meta/meta-ai-sdk", "meta-ai-sdk", "shape: meta + ai"),
    pkg(
        "meta/facebook-business",
        "facebook-business",
        "shape: facebook namespace",
    ),
    pkg(
        "meta/@facebook-scope",
        "@facebook/example",
        "shape: npm scope @facebook",
    ),
    // -- Llama (Meta's model family) packages --
    pkg(
        "llama/llama-stack",
        "llama-stack",
        "acdl2_scanner.rs c2 fixture",
    ),
    pkg("llama/llama-cpp-2", "llama-cpp-2", "planted.lock fixture"),
    // -- xAI packages --
    pkg("xai/xai-sdk", "xai-sdk", "shape: xai namespace"),
    pkg("xai/grok-client", "grok-client", "shape: grok family"),
    // -- routers reaching excluded vendors --
    pkg("router/litellm", "litellm", "planted.lock fixture"),
    pkg("router/openrouter", "openrouter", "planted.lock fixture"),
    // -- OpenAI imports and calls in source --
    src(
        "openai/js-import",
        "import OpenAI from \"openai\";\n",
        "shape: JS SDK import",
    ),
    src(
        "openai/py-import",
        "from openai import OpenAI\n",
        "shape: Python SDK import",
    ),
    src(
        "openai/rs-use",
        "use async_openai::Client;\n",
        "shape: Rust SDK import",
    ),
    src(
        "openai/env-key",
        "const key = process.env.OPENAI_API_KEY;\n",
        "shape: credential env var",
    ),
    src(
        "openai/gpt-4o",
        "  \"model\": \"gpt-4o\",\n",
        "shape: model id",
    ),
    src(
        "openai/ChatGPT-camel",
        "await signInWithChatGPT();\n",
        "orca signinwithchatgpt",
    ),
    src(
        "openai/ChatGpt-key",
        "  \"resetPersonalChatGptDefault\": \"Reset\",\n",
        "CompanyOS apps/web/translations/en.json",
    ),
    // -- OpenAI o-series ids in model context --
    src(
        "o-series/reasoning-assign",
        "reasoning = \"o3\"\n",
        "acdl2_scanner.rs c3 fixture",
    ),
    src(
        "o-series/json-model",
        "{ \"model\": \"o1\", \"temperature\": 1 }\n",
        "shape: JSON model field",
    ),
    src(
        "o-series/cli-flag",
        "codex exec --model o4-mini \"fix it\"\n",
        "shape: CLI model flag",
    ),
    src(
        "o-series/rust-const",
        "const REASONING_MODEL: &str = \"o3\";\n",
        "shape: Rust const",
    ),
    src(
        "o-series/yaml-block",
        "reasoning_models:\n  - o1\n",
        "shape: YAML list under a model key",
    ),
    src(
        "o-series/json-block",
        "{\n  \"models\": [\n    \"o3\"\n  ]\n}\n",
        "shape: JSON array under a model key",
    ),
    src(
        "o-series/suffix-pro",
        "fallback: o3-pro\n",
        "shape: o-series product suffix",
    ),
    src(
        "o-series/dated",
        "  \"id\": \"o3-2025-04-16\",\n",
        "shape: dated o-series snapshot",
    ),
    // -- Llama model ids in source --
    src(
        "llama/hf-id",
        "model_id = \"meta-llama/Llama-3.1-8B-Instruct\"\n",
        "shape: Hugging Face id",
    ),
    src(
        "llama/ollama-tag",
        "ollama pull llama3.2:3b\n",
        "shape: Ollama tag of a Llama model",
    ),
    src(
        "llama/default-model",
        "default_model: llama-3.3-70b\n",
        "acdl2_scanner.rs c3 fixture",
    ),
    src(
        "llama/codellama",
        "MODEL = \"codellama:7b\"\n",
        "shape: Code Llama",
    ),
    src(
        "llama/LLaMA",
        "title: LLaMA-2 fine-tune\n",
        "shape: paper spelling",
    ),
    src(
        "llama/concatenated",
        "model: qwenllama\n",
        "policy.rs mid-token regression",
    ),
    // -- xAI / Grok endpoints and ids in source --
    src(
        "xai/env-key",
        "let key = std::env::var(\"XAI_API_KEY\");\n",
        "shape: credential env var",
    ),
    src(
        "xai/grok-model",
        "  \"model\": \"grok-3-mini\",\n",
        "shape: model id",
    ),
    src(
        "xai/grok-endpoint",
        "url: https://api.x.ai/v1/chat (model grok-4)\n",
        "shape: xAI endpoint with model",
    ),
    src(
        "xai/grokUsageTab",
        "  \"grokUsageTab\": \"Usage\",\n",
        "orca grokusagetab",
    ),
    // -- the noise rules must not open an evasion route --
    src(
        "evasion/short-sri-prefix",
        "model = \"sha512-gpt-4o\"\n",
        "an SRI-looking prefix too short to be a hash",
    ),
    src(
        "evasion/ollama-runs-llama",
        "ollama run llama3\n",
        "the permitted runner pulling a Llama model",
    ),
    src(
        "evasion/python-list-block",
        "MODELS = [\n    \"o3\",\n]\n",
        "shape: Python list under a model key",
    ),
];

/// Known true-positive gaps: excluded-vendor references today's sweep does
/// not see. Closing one adds a new vendor pattern, which belongs to W1 (one
/// denylist), not to this noise fix. Each must still be missed; when W1
/// closes it, this test fails and the case moves to `TRUE_POSITIVES`.
const KNOWN_TP_GAPS: &[Case] = &[src(
    "xai/bare-host",
    "base_url = \"https://api.x.ai/v1\"\n",
    "shape: xAI API host alone (tokens x, ai: no family token)",
)];

fn fires(case: &Case) -> bool {
    match case.input {
        Input::Package(name) => {
            let mut out = Vec::new();
            check_package_name(name, "corpus", &mut out);
            !out.is_empty()
        }
        Input::Source(text) => !scan_text_for_model_ids(case.id, text).is_empty(),
    }
}

#[test]
fn corpus_false_positives_zero_and_every_true_positive_fires() {
    let fp_fired: Vec<&Case> = FALSE_POSITIVES.iter().filter(|c| fires(c)).collect();
    let tp_missed: Vec<&Case> = TRUE_POSITIVES.iter().filter(|c| !fires(c)).collect();
    println!(
        "CORPUS false positives fired: {}/{}",
        fp_fired.len(),
        FALSE_POSITIVES.len()
    );
    println!(
        "CORPUS true positives fired:  {}/{}",
        TRUE_POSITIVES.len() - tp_missed.len(),
        TRUE_POSITIVES.len()
    );
    for c in &fp_fired {
        println!("  FP fired:  {} ({})", c.id, c.origin);
    }
    for c in &tp_missed {
        println!("  TP missed: {} ({})", c.id, c.origin);
    }
    assert!(
        fp_fired.is_empty(),
        "false positives fired: {:?}",
        fp_fired.iter().map(|c| c.id).collect::<Vec<_>>()
    );
    assert!(
        tp_missed.is_empty(),
        "true positives missed: {:?}",
        tp_missed.iter().map(|c| c.id).collect::<Vec<_>>()
    );
}

#[test]
fn corpus_known_true_positive_gaps_are_still_open() {
    for c in KNOWN_TP_GAPS {
        println!("CORPUS known gap (not fired): {} ({})", c.id, c.origin);
        assert!(
            !fires(c),
            "gap {} now fires: move it to TRUE_POSITIVES and update the W0 notes",
            c.id
        );
    }
}

// ── fail-closed: an input the gate cannot read or parse is a failure ──────

/// One workspace written to disk, scanned as `acdl2-scan --root` would.
struct DiskCase {
    id: &'static str,
    /// (relative path, raw bytes)
    files: &'static [(&'static str, &'static [u8])],
    /// Explicit `--lockfile` paths, relative to the case root.
    lockfiles: &'static [&'static str],
}

/// UTF-16LE with BOM: `{"model": "gpt-4o"}` (Windows editors save this).
const UTF16_BOM_GPT: &[u8] =
    b"\xFF\xFE{\0\"\0m\0o\0d\0e\0l\0\"\0:\0 \0\"\0g\0p\0t\0-\0\x34\0o\0\"\0}\0";
/// The same text as UTF-16LE without a BOM: unreadable as text.
const UTF16_NOBOM_GPT: &[u8] = b"{\0\"\0m\0o\0d\0e\0l\0\"\0:\0 \0\"\0g\0p\0t\0-\0\x34\0o\0\"\0}\0";

const UNREADABLE_INPUTS: &[DiskCase] = &[
    DiskCase {
        id: "explicit-lockfile-missing",
        files: &[("Cargo.toml", b"[package]\nname = \"x\"\n")],
        lockfiles: &["does-not-exist.lock"],
    },
    DiskCase {
        id: "lockfile-not-utf8",
        files: &[("Cargo.lock", b"[[package]]\nname = \"caf\xE9\"\n")],
        lockfiles: &[],
    },
    DiskCase {
        id: "lockfile-package-name-unparsed",
        // TOML literal-string name: valid TOML the line parser cannot read.
        files: &[("Cargo.lock", b"[[package]]\nname = 'tiktoken'\nversion = \"1.0.0\"\n")],
        lockfiles: &[],
    },
    DiskCase {
        id: "lockfile-unrecognized-format",
        files: &[("Cargo.lock", b"this is not a lockfile\n")],
        lockfiles: &[],
    },
    DiskCase {
        id: "npm-lockfile-v1-layout",
        files: &[(
            "package-lock.json",
            b"{\n  \"lockfileVersion\": 1,\n  \"dependencies\": {\n    \"tiktoken\": { \"version\": \"1.0.0\" }\n  }\n}\n",
        )],
        lockfiles: &[],
    },
    DiskCase {
        id: "source-not-utf8",
        files: &[("config.yaml", b"name: caf\xE9\n")],
        lockfiles: &[],
    },
    DiskCase {
        id: "source-utf16-bom",
        files: &[("settings.json", UTF16_BOM_GPT)],
        lockfiles: &[],
    },
    DiskCase {
        id: "source-utf16-no-bom",
        files: &[("settings.json", UTF16_NOBOM_GPT)],
        lockfiles: &[],
    },
];

fn write_case(base: &Path, case: &DiskCase) -> PathBuf {
    let root = base.join(case.id);
    fs::create_dir_all(&root).expect("create case dir");
    for (rel, bytes) in case.files {
        fs::write(root.join(rel), bytes).expect("write case file");
    }
    root
}

#[test]
fn corpus_unreadable_inputs_fail_closed() {
    let base = Path::new(env!("CARGO_TARGET_TMPDIR")).join("acdl2-noise-corpus");
    let mut fail_open = Vec::new();
    for case in UNREADABLE_INPUTS {
        let root = write_case(&base, case);
        let lockfiles: Vec<PathBuf> = case.lockfiles.iter().map(|l| root.join(l)).collect();
        let closed = match scan_workspace_full(&root, &lockfiles) {
            Ok(report) => !report.clean(),
            Err(_) => true,
        };
        if !closed {
            fail_open.push(case.id);
        }
    }
    println!(
        "CORPUS unreadable inputs failed closed: {}/{}",
        UNREADABLE_INPUTS.len() - fail_open.len(),
        UNREADABLE_INPUTS.len()
    );
    for id in &fail_open {
        println!("  passed (fail-open): {id}");
    }
    assert!(
        fail_open.is_empty(),
        "inputs that failed open: {fail_open:?}"
    );
}
