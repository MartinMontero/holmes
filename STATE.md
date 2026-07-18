# STATE.md — Holmes build state (loop §5)

**Updated:** 2026-07-18 (Session 2 — Phase 0 build, locks 0a/0b/0c) · **Maintainer:** the loop; refreshed at every checkpoint.

## Git state

- Current branch: `claude/phase-0-holmes-guard-build-va1er0` (session/working branch, started from `origin/main` `f8b43a3` — clean, FF-OK)
- `origin/main` head: `f8b43a3` (merge of PR #5 — Task 0 session landed with explicit go-ahead)
- Ancestry: FF-OK
- Landing mechanics: PR-only to `main`, explicit go-ahead (D-08); connector re-sync after every canon merge (F-012)

## Gate Zero (loop §3.0) — CLEARED 2026-07-18

| Call | Decision | Record |
|---|---|---|
| (a) Charter verdict | 2026-07-13 second-pass audit accepted (SHIP WITH FIXES) | D-06 · report on disk `docs/audit/` |
| (b) D-01 license | AGPL-3.0-or-later ratified; LICENSE byte-identical to Alfred's | D-01, F-001 |
| (c) Git reconciliation | Branch restart + one approved `--force-with-lease`; nothing lost | D-07 |
| (d) Discipline | Confirmed | D-08 |

## Task 0 status (loop §3) — COMPLETE

3.0 ✅ · 3.1 ✅ (landed via PR #5) · 3.2 ✅ (spec v2.1, E1–E14, derived files re-seeded; F-014 resolved) · 3.3 ✅ (alias audit; primary re-verification still egress-blocked — carry) · 3.4 ✅ (roadmap reconciled; A-03). Standing spec amendments: A-02…A-05 (+ A-05 note: `GOOSE_DISABLE_KEYRING` now primary-cited from source @ `8e78960e`, `crates/goose/src/config/base.rs`).

## Phase 0 lock inventory (loop §6)

| Lock | Status | Evidence (executed 2026-07-18, this container) |
|---|---|---|
| **0a** — AC-DL-1 §§1–5, §7 | **VERIFIED (local, release, hermetic)** | `cargo test --release --locked -p holmes-guard` → 28 passed / 0 failed (resolution §2/§5, proxy §4/§5 with planted proxy-honoring server, spawn §3, structural §1, unit). §6 SCHEDULED to lock 2b, recorded visibly in CI. CI-run leg pending first push (workflow landed: `.github/workflows/acdl-gate.yml`, action-free) |
| **0b** — AC-DL-2 all seven | **VERIFIED (local, release)** | Positive control: 13 packages / 20 files / CLEAN / exit 0, exemptions listed. Negative control: planted lockfile → 5 violations / FAIL / exit 1. Criteria 1–7 each covered by a named test (`tests/acdl2_scanner.rs`); c7 = joint workflow, same run as 0a |
| **0c** — ACP round-trip | **PARTIAL — BLOCKED on model access** | Harness `holmes-smoke` executed against real goose 1.43.0 via L2: initialize + session/new complete; goose-reported pair (`ollama`/`gemma3:1b`) L1b-permitted post-handshake; 12/12 egress events `localhost:11434 allowed` through L1a; excluded-provider run denied exit 3. Model-response leg needs a Tier-1 key in-container **or** Tier-2 model egress (ollama.com 403). Never faked |
| **0d** — embedding contract | ABSENT — not in this session's instructed scope | — |
| **0e** — full CI (SBOM/scanners) | PARTIAL — AC-DL joint gate landed (action-free ⇒ SHA-pinning trivially satisfied); Syft/OSV/Grype not yet wired | — |

## Environment (this container)

- Provider key: **ABSENT** (`ANTHROPIC_API_KEY` unset; only `ANTHROPIC_BASE_URL=https://api.anthropic.com`, keyless probe → 401; no Google key; D-05 residual)
- goose: **INSTALLED from source** — origin `aaif-goose/goose` @ `8e78960e535ab7f34630e7c5921a42f146cbc9f4` (Apache-2.0, verified on disk), v1.43.0, binary **`/home/user/goose-src/target/release/goose`** (trimmed: `--no-default-features --features rustls-tls` — no V8/code-mode, telemetry, aws-providers, system-keyring, updater). Container-ephemeral: rebuild per session or vendor.
- **Do not `cargo install goose-cli`** — the crates.io name is a squatter (F-016).
- Ollama: install blocked (ollama.com + GitHub release egress 403) — Tier-2 local smoke unavailable here.
- Siblings: `Alfred` / `wecanjustbuildthings.dev` **ABSENT from this container** (fresh clone, no symlinks). D-01 evidence already ledgered; no Alfred-touching work in scope.

## Cross-repo obligations — Alfred

| Obligation | Status |
|---|---|
| Artifact-level guard CI test (runs in Alfred's CI) | OPEN — [DIRECTIONAL] |
| OS/artifact-level egress enforcement (L1a residual: hostile binary ignoring proxy env) | OPEN — restated in `holmes-guard` docs + `docs/security.md` |
| Signed update channel with rollback | OPEN — [DIRECTIONAL] |
| Memory/resurfacing channel | OPEN — [DIRECTIONAL] |
| First-run rendering (tool-approval UX surface) | OPEN |
| `holmes-guard` adoption retiring `provider-lockdown.ts` | OPEN — adoption surface now real: `policy::PROVIDER_SELECTING_ENV_VARS`, `spawn::sanitized_spawn`, readable policy tables |
| Freshness note: `src/lib/provider-policy.test.ts` stale model-id fixtures | NOTED 2026-07-18 — cosmetic |

## Staging obligations — human

1. **Provider access for lock 0c** (D-05 residual): inject `ANTHROPIC_API_KEY` (or `GOOGLE_API_KEY`) at container start, **or** open egress for Ollama + a permitted Gemma/Qwen weight. Then: `holmes-smoke --goose <abs> --provider <p> --model <m> --credential-env KEY --transcript ...`
2. **F-015**: supply AC doc v3 or confirm the v2 re-derivation is current.
3. Mark the `acdl-gate` workflow a **required status check** on `main` (branch protection is repo-settings, human-owned) — completes AC-DL-2 c7's "required gate" clause.
4. Still-unlocated upstream artifacts (F-009/F-011): kickoff v2, `Iterative quality validation process.md`, `claude-code-epistemic-integration-prompt.md`.
5. DeepSeek alias primary re-verification (api-docs.deepseek.com egress) — carry to next open-egress session and Phase RC.

## Resume point

Phase 0 checkpoint reached on the session branch (locks 0a/0b green locally; 0c harness proven, model leg blocked). Next: human reviews checkpoint readout → phase PR to `main` on explicit go-ahead → complete 0c when provider access exists → 0d/0e (embedding contract; SBOM/scanner CI). Every fresh session: `git fetch origin` first.
