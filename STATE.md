# STATE.md — Holmes build state (loop §5)

**Updated:** 2026-07-18 (Session 1 of the Master Build Loop v2) · **Maintainer:** the loop; refreshed at every checkpoint.

## Git state

- Current branch: `claude/holmes-launch-runbook-kwtkta` (session/working branch)
- `origin/main` head: `7ecbc67` (16 commits — verified superset of the 2026-07-13 3-commit observation; no rewrite)
- Ancestry: FF-OK (`git merge-base --is-ancestor origin/main HEAD` true at pre-flight after the D-07 branch restart)
- Landing mechanics: PR-only to `main`, explicit go-ahead (D-08); connector re-sync after every canon merge (F-012)

## Gate Zero (loop §3.0) — CLEARED 2026-07-18

| Call | Decision | Record |
|---|---|---|
| (a) Charter verdict | 2026-07-13 second-pass audit accepted (SHIP WITH FIXES) | D-06 · audit report file itself still ABSENT from `docs/audit/` — staging obligation |
| (b) D-01 license | AGPL-3.0-or-later ratified; `Alfred/LICENSE` quoted from disk; LICENSE swapped byte-identical to Alfred's (`d8a6cc31…96bee`) | D-01, F-001 |
| (c) Git reconciliation | Branch restart + one `--force-with-lease` push approved and executed; nothing lost | D-07 |
| (d) Discipline | Confirmed | D-08 |

## Task 0 status (loop §3)

- **3.0 Gate Zero:** ✅ cleared (above)
- **3.1 Land canon on `main`:** ✅ staged — canon + AC doc (normalized to `docs/acceptance/`, sha256 verified across the move) + Gate-Zero ledger updates on the session branch; **phase PR open, merge awaits go-ahead**
- **3.2 Spec v2.1:** ⛔ SKIPPED-WITH-APPROVAL (Martin, 2026-07-18) — spec still v2; `holmes-spec-v2.1-diff.md` ABSENT on disk; E1–E14 not improvised. **F-014** (open). Unblocks when the human supplies the diff file.
- **3.3 Alias audit:** ✅ executed 2026-07-18 — see table below
- **3.4 Roadmap reconciliation:** ✅ `docs/roadmap/build-phases.md` replaced with the loop-§6 derivation; dropped-unique item flagged as **A-03** (Tauri shell → spec revision pending)

### Task 3.3 — alias audit results (2026-07-18)

Scope: every config, doc, and reference; `deepseek-chat` / `deepseek-reasoner` retire 2026-07-24 15:59 UTC, both routing to V4 Flash (`deepseek-reasoner` → Flash, not Pro — per A-01).

- **Holmes repo: no configs or code exist — zero call sites bind the aliases.** Every hit is documentation: spec L107 (informational note, dated, accurate), loop L60/L145 (the audit instruction + dated facts ledger), runbook §0/§2 (migration guidance), AC doc (provider names only, no aliases), ledgers/provenance (meta-references). **No fixes required in-repo; nothing hard-codes the promo rate as the only rate.**
- Spec pricing phrasing observation (no action): L21 "became permanent" is the spec's strongest pricing claim; the spec itself tempers it at L228 ("vendors change pricing") and the loop's facts ledger carries `[NEEDS-CAVEAT]` on permanence. Budget-both-rates remains the binding rule (loop L60).
- **Sibling repos:** no alias hits in `wecanjustbuildthings.dev`. One Alfred hit: `src/lib/provider-policy.test.ts:88` uses `deepseek-chat` (and L77 `deepseek-r1`) as string fixtures in provider-policy tests — no API call, retirement breaks nothing; **flagged (not modified) as an Alfred-side freshness note** (see cross-repo obligations).
- **Primary re-verification 2026-07-18: BLOCKED** — `api-docs.deepseek.com` returns 403 through this environment's egress. UNVERIFIED-live today; carrying the 2026-07-13 primary verification (loop §8) and 2026-07-16 secondary re-corroboration (runbook §0). Re-verify at next session with open egress, and at Phase RC ("alias audit re-confirmed post-2026-07-24").

## Environment (this container)

- Provider key: ABSENT in-container (D-05 residual — secrets inject at container start; Tier-1 cloud decided; re-verify next container)
- goose: NOT INSTALLED — install approved but blocked by network policy (cross-owner GitHub 403, crates.io blocked). Remedies: new session with `aaif-goose/goose` as an initial source, or vendored binary.
- Siblings on disk: `Alfred` @ `1801bc3`, `wecanjustbuildthings.dev` @ `563220a` (shallow clones, symlinked beside `holmes`)

## Phase 0 lock inventory (loop §6) — all ABSENT

No application code exists (`git ls-files`: docs + governance only; no `crates/`, no `.github/workflows/`). Locks 0a, 0b, 0c, 0d, 0e: **ABSENT** — nothing built, nothing claimed. Build begins next session (needs provider key in-container; goose install).

## Cross-repo obligations — Alfred

| Obligation | Status |
|---|---|
| Artifact-level guard CI test (runs in Alfred's CI) | OPEN — [DIRECTIONAL], never verified; read-only verification pass not yet run |
| OS/artifact-level egress enforcement | OPEN — [DIRECTIONAL] |
| Signed update channel with rollback | OPEN — [DIRECTIONAL] |
| Memory/resurfacing channel | OPEN — [DIRECTIONAL] |
| First-run rendering (tool-approval UX surface) | OPEN — recorded per loop §6 Phase 2.5(iii) |
| `holmes-guard` adoption retiring `provider-lockdown.ts` (strippable TS guard) | OPEN — design obligation from Phase 0 |
| Freshness note: `src/lib/provider-policy.test.ts` L77/L88 use stale/retiring model-id fixtures (`deepseek-r1`, `deepseek-chat`) | NOTED 2026-07-18 — cosmetic; string-policy test, no API call; fix opportunistically Alfred-side |

## Staging obligations — human

1. 2026-07-13 second-pass audit report → `docs/audit/` (D-06 note; provenance doc marks its verdict UNVERIFIED until the file lands)
2. `holmes-spec-v2.1-diff.md` → unblocks Task 3.2 (F-014)
3. Provider key visible in-container (D-05 residual)
4. goose availability (see Environment)

## Resume point

Next session: `git fetch origin` → verify provider key + goose → complete pre-flight items 4/6 → begin **Phase 0 build** (lock 0a first: guard skeleton + AC-DL CI) — or apply `holmes-spec-v2.1-diff.md` first if supplied (Task 3.2).
