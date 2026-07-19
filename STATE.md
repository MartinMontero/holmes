# STATE.md — Holmes build state (loop §5)

**Updated:** 2026-07-19 (Session 3 — lock 0e CVE gate wired; Phase 0 guard re-verified green in a fresh container) · prior: 2026-07-18 (Session 2 — Phase 0 build, locks 0a/0b/0c) · **Maintainer:** the loop; refreshed at every checkpoint.

## Git state

- Current branch (Session 3): `claude/claude-code-git-bash-path-f30buf` (lock 0e CVE-gate work; started clean from `main` head). Session-2 build branch `claude/phase-0-holmes-guard-build-va1er0` landed via PR #6.
- `origin/main` head: `3af25d9` (merge of PR #6 — Phase 0 guard build landed); prior `f8b43a3` (PR #5, Task 0)
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
| **0a** — AC-DL-1 §§1–5, §7 | **VERIFIED (local, release, hermetic) — v3-conformant** | `cargo test --release --locked -p holmes-guard` → **40 passed / 0 failed** (resolution §2/§5, proxy §2a/§4/§5 with planted proxy-honoring server + planted upstream forward proxy + named-endpoint denial, spawn §3, structural §1, unit; incl. regression + v3-delta tests). §6 SCHEDULED to lock 2b, recorded visibly in CI. CI-run leg pending first CI trigger (workflow landed: `.github/workflows/acdl-gate.yml`, action-free) |
| **0b** — AC-DL-2 all seven | **VERIFIED (local, release) — v3-conformant** | v3 control convention: **positive** control (planted lockfile) → FAIL / exit 1; **negative** control (real tree, lockfile discovery) → CLEAN / exit 0. Multi-ecosystem lockfile walk (§1), documented seed table with rationale (§2), dependency-path in failure output (§3 — `pulled in via holmes-app -> middleware-lib -> async-openai`). Criteria 1–7 each covered by named tests; c7 = joint workflow, same run as 0a |
| **0c** — ACP round-trip | **PARTIAL — BLOCKED on model access** | Harness `holmes-smoke` executed against real goose 1.43.0 via L2: initialize + session/new complete; goose-reported pair (`ollama`/`gemma3:1b`) L1b-permitted post-handshake; 12/12 egress events `localhost:11434 allowed` through L1a; excluded-provider run denied exit 3. Model-response leg needs a Tier-1 key in-container **or** Tier-2 model egress (ollama.com 403). Never faked |
| **0d** — embedding contract | ABSENT — not in this session's instructed scope | — |
| **0e** — full CI (SBOM/scanners) | **PARTIAL — CVE gate now wired (Session 3)** | `.github/workflows/supply-chain.yml`: Syft SBOM (SPDX+CycloneDX) + OSV-Scanner (primary, exit 1 on any vuln) + Grype (cross-check, `--fail-on high`, exit 2). No Trivy (CVE-2026-33634). Action-free ⇒ SHA-pinning satisfied by construction. Scanners pinned (syft `v1.48.0` / osv-scanner `v2.4.0` / grype `v0.116.0`) and verified against each release's goreleaser checksums, **fail-closed**. Verified locally: YAML parse (7 steps), version-munging → correct asset names, `sha256sum --ignore-missing` tamper → non-zero. Verified from source: tool versions + asset filenames (2026-07-19 release pages), CLI syntax + exit-code semantics (vendor docs). **First CI run PENDING** — scanner binary download is org-egress-blocked in-container (`githubusercontent.com` 403, same wall as 0c's model leg); first real execution is on GitHub Actions. Provenance/attestation (spec §6.6) still ABSENT — carry |

## Adversarial self-verification (2026-07-18)

7 skeptics attacked each guard property; every reported defect re-reproduced against source before acceptance. 3 claims held (L2 env-strip, BYOK invariant, born-redacted output). 4 confirmed defects **fixed + regression-locked this session**: F-017 (MAJOR, L1a forward-proxy re-dispatch), F-018 (MAJOR, AC-DL-2 router exact-match evasion), F-019 (MINOR, mid-token excluded-family hole), F-020 (MINOR, §1 structural test fidelity). See findings-ledger.

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

1. **Provider access for lock 0c** (D-05 residual): **STILL BLOCKED this session** — `ANTHROPIC_API_KEY` remains unset in-container (keyless probe → 401; no key file). Session 3: a human-supplied Tier-1 key value was verified **valid** (HTTP 200 vs `api.anthropic.com/v1/models`, 2026-07-19), so the credential itself is no longer the blocker — but it is still **not injected** as a container env var, **and** goose is absent from this fresh (ephemeral) container. 0c therefore blocks on two things now: (a) inject the key at container start — note `ANTHROPIC_API_KEY` is a **reserved, auto-stripped** name in claude.ai/code env settings, so inject it via a non-reserved var (e.g. `HOLMES_PROVIDER_KEY`) or the platform secret mechanism; (b) rebuild/vendor goose (`aaif-goose/goose` @ `8e78960e`, trimmed build). Then: `holmes-smoke --goose <goose-bin> --provider <p> --model <m> --credential-env <VAR> --transcript ...`
2. ~~**F-015**: supply AC doc v3~~ — **RESOLVED 2026-07-18**: v3 landed at root, normalized into `docs/acceptance/` (hash-gated), root removed, v2 to history; v3-conformance pass closed F-021…F-024.
3. Mark the `acdl-gate` **and** `supply-chain` workflows **required status checks** on `main` (branch protection is repo-settings, human-owned) — completes AC-DL-2 c7's "required gate" clause and makes the 0e CVE gate binding.
4. Still-unlocated upstream artifacts (F-009/F-011): kickoff v2, `Iterative quality validation process.md`, `claude-code-epistemic-integration-prompt.md`.
5. DeepSeek alias primary re-verification (api-docs.deepseek.com egress) — carry to next open-egress session and Phase RC.

## Resume point

Phase 0: locks 0a/0b green (re-verified in this fresh container, `cargo test --release --locked -p holmes-guard` → 0 failed); 0c harness proven (model leg egress-blocked); **0e CVE gate authored** (`supply-chain.yml`, first CI run pending). Next: (a) trigger CI to exercise `supply-chain.yml`, then mark it a required check alongside `acdl-gate`; (b) complete 0c once a provider key is injected (value verified working, Session 3) **and** goose is rebuilt/vendored in-container; (c) 0d embedding contract; (d) 0e provenance/attestation (spec §6.6). Every fresh session: `git fetch origin` first; goose is container-ephemeral.
