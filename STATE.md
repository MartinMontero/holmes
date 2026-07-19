# STATE.md — Holmes build state (loop §5)

**Updated:** 2026-07-19 (Session 4 final — **Phase 0 CLOSED**: 0c completed on the funded account; all locks green or explicitly carried) · prior same-day: Phase 0 close PR #8; Session 3 (0e CVE gate) · 2026-07-18 (Session 2 — Phase 0 build) · **Maintainer:** the loop; refreshed at every checkpoint.

## Git state

- Current branch (Session 4): `claude/claude-code-git-bash-path-f30buf`, restarted from `main` after PR #7 merged (Phase 0 close work). Session-2 build branch landed via PR #6; Session-3 0e work via PR #7.
- `origin/main` head: `b6b19dc` (merge of PR #7 — 0e CVE gate landed); prior `3af25d9` (PR #6), `f8b43a3` (PR #5)
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
| **0a** — AC-DL-1 §§1–5, §7 | **VERIFIED (local, release, hermetic) — v3-conformant** | `cargo test --release --locked -p holmes-guard` → **40 passed / 0 failed** (resolution §2/§5, proxy §2a/§4/§5 with planted proxy-honoring server + planted upstream forward proxy + named-endpoint denial, spawn §3, structural §1, unit; incl. regression + v3-delta tests). §6 SCHEDULED to lock 2b, recorded visibly in CI. **CI-run leg PROVEN 2026-07-19**: acdl-gate run #9 green on `main` head `3af25d9` (first real CI execution), then green on PR #7's fix commit `6595499` (both push- and PR-triggered runs) after correctly catching F-025 on run #10 — the gate demonstrably fires and passes in CI, not just locally |
| **0b** — AC-DL-2 all seven | **VERIFIED (local, release) — v3-conformant** | v3 control convention: **positive** control (planted lockfile) → FAIL / exit 1; **negative** control (real tree, lockfile discovery) → CLEAN / exit 0. Multi-ecosystem lockfile walk (§1), documented seed table with rationale (§2), dependency-path in failure output (§3 — `pulled in via holmes-app -> middleware-lib -> async-openai`). Criteria 1–7 each covered by named tests; c7 = joint workflow, same run as 0a |
| **0c** — ACP round-trip | **COMPLETE (2026-07-19, account funded)** | Funded re-run, same container/command: verdict **ROUND-TRIP COMPLETE**, exit 0 — `anthropic`/`claude-sonnet-5` L1b-permitted pre+post handshake, L2 sanitized spawn, 14 ACP frames, egress **1/1 `api.anthropic.com:443 allowed`** via L1a, streamed model completion `"pong"` (the smoke prompt's exact requested reply; 4 bytes; verdict from the F-026-fixed harness, which cannot count error relays as completions). Born-redacted transcript: `docs/audit/evidence/0c-transcript-2026-07-19-complete.json`. Prior blocked-run evidence retained below, superseded not deleted |
| *0c prior evidence (superseded 2026-07-19)* | *PARTIAL — guard/protocol legs proven, completion credits-blocked* | 2026-07-19, this container: goose rebuilt from the pin via `scripts/build-goose.sh` (`aaif-goose/goose` @ `8e78960e`, v1.43.0, trimmed features, binary sha256 `439a282e…7056`, provenance file beside the binary). `holmes-smoke --provider anthropic --model claude-sonnet-5 --credential-env MY_ANTHROPIC_API_KEY`: pre-handshake L1b permitted → L2 sanitized spawn → real `goose acp` handshake + session/new + prompt (12 frames) → post-handshake configOptions pair (`anthropic`/`claude-sonnet-5`) L1b-**permitted** → egress events **1/1 `api.anthropic.com:443 allowed`** through L1a. The endpoint **authenticated the key** (HTTP 400 billing — "credit balance is too low" — not 401 auth); goose relayed the error as agent text; harness verdict `PROVIDER ERROR RELAYED`, exit 6 (honest verdict added this session — F-026, fixed + re-run confirmed). Negative control: `openrouter` → typed pre-handshake denial, exit 3. Transcript born-redacted (verified: no key material) at `docs/audit/evidence/0c-transcript-2026-07-19.json`. **Remaining to COMPLETE: fund the API account, re-run the same command, expect exit 0 with a model completion. Never faked** |
| **0d** — embedding contract | **COMPLETE (Session 4, 2026-07-19)** | `crates/holmes-core`: §6.2 artifact types as committed canon (`ResearchBrief` in; `EvidencePack`/`CaseFile` out — types and validation only), invariant 5 enforced at construction (empty provenance / out-of-range confidence unrepresentable), invalidation-not-deletion (supersede flags `valid_until`, no removal API), handoff-only terminal state (journalist/lawyer/community/interim human reviewer; no execute/apply API), read-only provider seam re-exporting guard rosters + resolution (`holmes_core::provider`). Zero third-party deps (workspace-internal `holmes-guard` only). Evidence: 9 new tests green in release `--locked` (4 unit + 3 Alfred-shaped consumer `embedding_contract.rs` + 2 structural `no_blueprint_exports.rs` incl. F-020-style fires-check); workspace suite 49/49; clippy clean, fmt clean; CI runs the workspace suite in the acdl-gate job ("Lock 0d" step). Alfred's artifact-level guard-test obligation: already ledgered in the cross-repo table below. Phase 1 schema amendment (`knowability`/`limits_of_this_finding`) deliberately not pre-implemented |
| **0e** — full CI (SBOM/scanners) | **CVE gate PROVEN in CI (2026-07-19)** | `.github/workflows/supply-chain.yml`: Syft SBOM (SPDX+CycloneDX) + OSV-Scanner (primary, exit 1 on any vuln) + Grype (cross-check, `--fail-on high`, exit 2). No Trivy (CVE-2026-33634). Action-free ⇒ SHA-pinning satisfied by construction; scanners pinned (syft `v1.48.0` / osv-scanner `v2.4.0` / grype `v0.116.0`), checksum-verified fail-closed. **Executed CI evidence:** run #1 on `107f0a5` → success in 51s (2026-07-19 07:30:49Z — pinned installs verified against real release checksums, SBOM generated, OSV clean, Grype clean); re-run green ×2 on fix commit `6595499`; landed on `main` via PR #7 (`b6b19dc`), all four checks green at merge. Still open in 0e: provenance/attestation (spec §6.6) — carry; human: mark `acdl-gate` + `supply-chain` required checks on `main` |

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

1. ~~**Fund the API account to finish 0c**~~ — **RESOLVED 2026-07-19**: human funded the account in-band; re-run produced exit 0, verdict ROUND-TRIP COMPLETE, streamed completion `"pong"` (transcript in `docs/audit/evidence/`). D-05 residual closed. Key-storage note stands: never store under `ANTHROPIC_API_KEY` in claude.ai/code env settings (reserved, auto-stripped); `MY_ANTHROPIC_API_KEY` is the working convention.
2. ~~**F-015**: supply AC doc v3~~ — **RESOLVED 2026-07-18**: v3 landed at root, normalized into `docs/acceptance/` (hash-gated), root removed, v2 to history; v3-conformance pass closed F-021…F-024.
3. Mark the `acdl-gate` **and** `supply-chain` workflows **required status checks** on `main` (branch protection is repo-settings, human-owned) — completes AC-DL-2 c7's "required gate" clause and makes the 0e CVE gate binding.
4. Still-unlocated upstream artifacts (F-009/F-011): kickoff v2, `Iterative quality validation process.md`, `claude-code-epistemic-integration-prompt.md`.
5. DeepSeek alias primary re-verification (api-docs.deepseek.com egress) — carry to next open-egress session and Phase RC.

## Resume point

**Phase 0 CLOSED (2026-07-19):** 0a CI-proven · 0b green (49/49 workspace suite) · **0c COMPLETE** (live guarded round-trip with a real model completion) · **0d COMPLETE** (`holmes-core` embedding contract) · 0e CVE gate proven in CI. Carried forward (not Phase-0-blocking): 0e provenance/attestation → RC prep; human repo-settings item: mark `acdl-gate` + `supply-chain` required checks on `main`. **Next: Phase 1 — Analytical core** (loop §6), fresh branch from `main`, on explicit human go-ahead. Every fresh session: `git fetch origin` first; goose is container-ephemeral (`scripts/build-goose.sh`, ~12 min).
