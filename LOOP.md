# LOOP.md — Phase 1, build stage (locks 1a–1d; session branch `claude/claude-code-git-bash-path-f30buf`)

**Updated:** 2026-07-19 · Maintained per loop §1. Prior stages (Task 0 documentation loop; Phase 0 build) preserved in git history at `main`.

## Phase 1 scope

Build the analytical core's deterministic legs in `holmes-core::analysis` — hypothesis objects + LR scorer (Engine 1), ACH matrix + KAC (Engine 2), first-principles quarantine (Engine 3), the six-phase case state machine, and the lock-1a emission gate (≥2-independent-source corroboration + A-07 knowability/limits) — with the model-side legs as recipes (`recipes/`) and the `holmes-case` harness. Recipe safety scan (lock 1d) live from the first recipe. **Non-goals:** calibration/knowability *gating* (2.5), the Wall (Phase 2), investigative tools (Phase 3), any Alfred-side change.

## Phase 1 gates (exact commands; exit 0 unless stated)

```
cargo test --release --locked --workspace
cargo run --release --locked -p holmes-guard --bin recipe-scan -- --path recipes
cargo run --release --locked -p holmes-guard --bin recipe-scan -- \
  --path crates/holmes-guard/tests/fixtures/planted_recipe_smuggled.yaml  # MUST exit 1
holmes-case --goose <bin> --provider anthropic --model claude-sonnet-5 \
  --credential-env MY_ANTHROPIC_API_KEY --transcript <path>              # lock 1b, live
```

## Phase 1 evidence (executed 2026-07-19, this container)

Live per-lock status: `STATE.md` Phase 1 inventory. Highlights: workspace suite green in release `--locked` (analysis unit + lock tests + prior 49); lock 1b live case CASE COMPLETE exit 0 (4 hypotheses, 12/12 ACH cells, honest 3-way tie reported, egress 1/1 allowed; transcript in `docs/audit/evidence/`); recipe scan controls both firing; canon §5 firewall structural test in place (caught its own doc comment during the build — reworded per F-025 precedent).

---

# Prior stage record — Phase 0 (2026-07-18, closed 2026-07-19)

> **Session-3 follow-on (2026-07-19):** lock **0e** — an explicit *non-goal* of this build stage (below) — subsequently had its CVE gate wired in `.github/workflows/supply-chain.yml` (Syft SBOM + OSV-Scanner primary + Grype cross-check; no Trivy). Live status and evidence: `STATE.md` lock 0e.
>
> **Session-4 follow-on (2026-07-19, Phase 0 close):** 0e CVE gate proven in CI (landed via PR #7); lock **0d** built (`crates/holmes-core` embedding contract — §6.2 types + validation, consumer + structural tests, workspace suite wired into the acdl-gate CI job); goose rebuild scripted (`scripts/build-goose.sh`, pin + provenance + sha256; F-016 honored); lock **0c** status lives in `STATE.md` (per-lock evidence there). F-025 ledgered (reword-over-exemption precedent).

## Scope

Build and verify `holmes-guard` (L1a egress allowlist proxy + L1b resolution guard + L2 sanitized spawn) to AC-DL-1 §§1–5, §7 green hermetically in release mode; AC-DL-2 all seven criteria with both controls firing; headless ACP round-trip on a provably permitted model. **Non-goals:** `holmes-core`/embedding contract (0d), SBOM/scanner CI (0e beyond the AC-DL joint gate the 0a/0b locks themselves require), any Alfred-side change, anything touching `main`.

## Journeys

1. Excluded provider requested (env/config/caller) → refused at L1b with a typed denial. ✅
2. Excluded model family on a *permitted* provider (e.g. ollama + llama) → refused. ✅
3. Unknown provider or unknown model id → refused (deny-by-default, not a warning). ✅
4. Retired DeepSeek alias ids → refused; the literals appear in no new code/config (A-01). ✅
5. Tool/MCP egress to a non-permitted host:port → 403 at the L1a boundary; planted server untouched; event recorded (names+counts only). ✅
6. Permitted-path positive: loopback:11434 tunnels and round-trips; the permitted stack resolves. ✅
7. Malformed/unsupported proxy requests (origin-form, bad authority, userinfo, garbage) → fail closed. ✅
8. Poisoned parent env (excluded keys, NO_PROXY=*, provider overrides) → stripped wholesale; child env rebuilt explicitly; HOME/XDG isolated; proxy pinned. ✅
9. BYOK: credential accepted only when it matches the resolved provider; cross-provider and excluded-vendor keys refused; no vendor var required by the shipped crates. ✅
10. Planted excluded dependency graph → AC-DL-2 gate FAILS (negative control); real workspace passes with exemptions listed visibly (positive control). ✅
11. Headless ACP round-trip via L2 against real goose: handshake + session + streamed prompt, resolved id validated pre- and post-handshake, egress evidence attached. ◐ — protocol + guard path executed and verified live; **model-response leg BLOCKED** (no provider credential in-container; model-registry egress 403).

## Gates (exact commands; exit 0 unless stated)

```
cargo test --release --locked -p holmes-guard
cargo run --release --locked -p holmes-guard --bin acdl2-scan -- --root .
cargo run --release --locked -p holmes-guard --bin acdl2-scan -- --root . \
  --lockfile crates/holmes-guard/tests/fixtures/planted.lock   # MUST exit 1, "verdict: FAIL"
grep -rn "deepseek-chat\|deepseek-reasoner" crates .github Cargo.toml Cargo.lock  # MUST find nothing
```

CI: `.github/workflows/acdl-gate.yml` — one job, same run: release-mode hermetic guard tests + AC-DL-1 §6 SCHEDULED marker + both AC-DL-2 controls. Action-free (constitution #7 SHA-pinning satisfied by construction: no third-party actions exist to pin).

## Quality criteria

- Policy only in compiled Rust; zero third-party dependencies in `holmes-guard`.
- Every denial is typed and asserted in tests; no warning-as-pass anywhere.
- Hermetic: every test socket is loopback; no test needs network or a credential.
- Scanner deterministic (same input → same verdict); exemptions visible in every report.
- Born-redacted outputs: proxy events and spawn Debug expose names/counts/keys, never content or credential values.

## Evidence (executed 2026-07-18, this container)

- `cargo test --release --locked -p holmes-guard` → **40 passed, 0 failed** (4 unit + 8 proxy + 8 resolution + 5 spawn + 1 structural + 14 scanner), after adversarial-pass regressions (F-017…F-020) and v3-conformance deltas (F-021…F-024). clippy clean, `cargo fmt --check` clean.
- v3 AC doc normalized: root v3 (sha256 `7124772b…4a1ae4bd`, gate-verified) → `docs/acceptance/`, root removed, v2 to history. Controls per v3 convention: positive (planted lockfile) → FAIL/exit 1; negative (real tree) → CLEAN/exit 0. Dependency-path emitted (§3); multi-ecosystem discovery (§1); documented seed table (§2).
- Positive control → `packages scanned: 13 / files scanned: 20 / verdict: CLEAN`, exit 0, 6 exemptions listed.
- Negative control → 5 violations (async-openai, tiktoken-rs, llama-cpp-2 = namespace; litellm, openrouter = router), `verdict: FAIL`, exit 1.
- A-01 sweep → zero hits in `crates/`, `.github/`, `Cargo.toml`, `Cargo.lock`.
- goose: built from source, origin `aaif-goose/goose` @ `8e78960e535ab7f34630e7c5921a42f146cbc9f4` (2026-07-18, Apache-2.0), v1.43.0, **binary `/home/user/goose-src/target/release/goose`**, trimmed build (`--no-default-features --features rustls-tls`: no V8/code-mode, no telemetry, no aws-providers, no system-keyring, no update).
- 0c partial: `holmes-smoke` vs real `goose acp` — initialize + session/new completed (newline-delimited JSON-RPC 2.0, ACP v1); goose's own configOptions pair (`ollama`/`gemma3:1b`) validated **permitted** post-handshake; 12/12 egress events `localhost:11434 allowed` through L1a; prompt leg timed out with no model server (transcript in session scratchpad). Excluded-provider harness run → exit 3, typed pre-handshake denial.

## Assumptions (judgment calls logged)

- Permitted model families are prefix-scoped from the spec v2.1 roster; exact SKUs are re-verified at Phase RC (loop §6 roster audit). Deny-by-default covers the interim.
- Loopback egress is scoped to 11434 (Ollama default) only; the §5 positive control binds that port.
- `.github/` is scanned by AC-DL-2 (it is config); `docs/` and `*.md` are not (canon legitimately names excluded vendors) — stated in the scan module.
- The kickoff's "AC doc (v3)" citation does not match the repo's v2 re-derivation → built against the repo copy per the sync rule; ledgered as F-015 for the human.
- BYOK constraint (in-band, 2026-07-18): the smoke credential is build/CI-only, read at run time from the operator env via `--credential-env`; shipped crates hardcode no vendor/key/env requirement; the guard seam enforces per-provider key matching.
