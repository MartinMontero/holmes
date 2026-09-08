# Canon homecoming — inventory, split, and diff report

Date: 2026-09-07. Labels per session constitution. Scope: the four
BuilderOS repos on this disk: Holmes (`holmes-clone`), GooseClaw
(`Gooseclaw`), WCJBT (`wecanjustbuildthings.dev`), Alfred (`Alfred`).

## 0. Sourcing honesty

The **Wave-0 inventory document is ABSENT from this disk** (filename and
content searches across `C:\Users\User\dev`, excluding node_modules: no
"Wave-0 inventory", no "canon homecoming" phrase). The Kimi project's
knowledge files are likewise mostly not exported to disk. What IS on disk
and was used: the loose copies in `C:\Users\User\dev\holmes\` (the Kimi
drop folder) and `C:\Users\User\dev\`, plus all four repos. Where the
tasking names a file that is not on disk, it is marked ABSENT — checked,
not assumed.

## 1. Governing vs reference split

### Holmes
- **GOVERNING (in-repo already):** `docs/holmes-spec-v2.md` (v2.1),
  `docs/triad-canon.md`, `docs/epistemic-canon-Holmes.md`,
  `docs/constitution.md`, `docs/security.md`, `docs/architecture.md`,
  `docs/build-roadmap.md`, `docs/roadmap/build-phases.md`,
  `docs/holmes-project-orientation.md`, `docs/holmes-vs-wcjbt.md`,
  `docs/acceptance/holmes-denylist-acceptance-criteria.md`,
  `docs/prompts/holmes-master-build-loop-v2.md`,
  `docs/holmes-launch-runbook-v1.md`,
  `docs/holmes-claude-project-instructions.md`, `CLAUDE.md`,
  `docs/audit/decisions.md` (D-ledger).
- **REFERENCE (stays):** `docs/research/*`, `docs/case-file/*`,
  `docs/audit/*` (ledgers/history), `docs/holmes-spec-v2.1-diff.md`,
  `docs/upstream/*`, `docs/beta-scope-decision-brief.md`.
- **Kimi-side loose copies (dev root / `dev\holmes\`):** see §2 — one
  stale, two identical-modulo-EOL.

### GooseClaw
- **GOVERNING (in-repo):** `specification.md` (brought home 2026-09-07 on
  `canon/specification-homecoming`, ratified per its own header),
  `vendor_gate.py`, `ci.yml`, `CLAUDE.md`, `RECIPE.md` + `gate.ps1`
  (graph convention, PR #1 merged).
- **GOVERNING (Kimi-side only — ABSENT from disk):** the numbered canon
  set `00`–`10`, `19`, `20` (incl. `02_CONSTITUTION.md`,
  `03_DECISION_LOG.md`, `07_THREAT_MODEL.md`, `09_OPEN_DECISIONS.md`),
  `BUILDEROS-CONSTITUTION-GooseClaw.md`, `08_PIN_REGISTER.md` and
  `08_PIN_REGISTER-refreshed-2026-09-03.md`. specification.md cites them
  by tag; they govern but are not in any repo. **Homecoming blocked on
  Martin exporting them from the Kimi project.**
- **REFERENCE:** `docs/audit/*`.

### WCJBT
- **GOVERNING (in-repo):** `CLAUDE.md`, `POLICIES.md`, `SECURITY.md`,
  `DESIGN.md`, `docs/claude-project-instructions.md`, `ROADMAP.md`,
  `docs/OPERATOR-RUNBOOK.md` (role judgment: operating rules — INFERRED
  from filenames/content spot-checks).
- **ABSENT:** `PROJECT_INSTRUCTIONS.md` named in the tasking — not on
  disk anywhere in the repo or outside it (closest in-repo equivalent:
  `docs/claude-project-instructions.md`, tracked). Also ABSENT:
  `docs/canon/triad-canon.md` — see §4.
- **REFERENCE:** `docs/archive/*`, `docs/mobile-audit.md`,
  `wisdom-intuition-knowledge-judgment-v2.md` (repo-root research copy).

### Alfred
- **GOVERNING (in-repo):** `CLAUDE.md`, `SECURITY.md`, `LOOP.md`,
  `LOOP-DESIGN.md`, `LOOP-INTEGRATION.md`, `docs/decisions/*` (ADRs),
  `docs/triad-canon.md` (mirror), `docs/epistemology/*`,
  `builder-os-console-spec.md`, `docs/beta/rollback-checklist.md`.
- **ABSENT (Kimi-side, never recovered):** the `00-PROJECT-INSTRUCTIONS`
  set named in the tasking — not on disk (a same-named file exists only
  in the unrelated `Founders Quest` project). `epistemic-canon-Alfred.md`
  is **unrecovered on every surface checked** (Alfred's own W2 audit,
  `docs/audit/epistemic-canon-landing.md`, open operator item #1) —
  deliberately not re-derived here.
- **REFERENCE:** `docs/audit/*`, `_alfred-inbox/*` (inbox, untracked),
  `docs/epistemology/wisdom-*.md` (the Map).

## 2. Diffs — Kimi-side copies vs repo copies (VERIFIED-LIVE 2026-09-07, sha256)

| Pair | Result |
|---|---|
| `dev\holmes\holmes-spec-v2.md` vs `holmes-clone\docs\holmes-spec-v2.md` | **CONFLICT by staleness**: loose copy is v2.0; repo is v2.1 (12+/19-, the whole 2026-07-06 correction set: Qwen3.7-Max roster fix, DeepSeek permanence downgrade, Kuzu/Graphiti note). Repo copy wins per the spec's own sync rule. Loose copy = stale duplicate. |
| `dev\holmes\holmes-project-orientation.md` vs repo `docs/` copy | **AGREE** — content-identical; hashes differ on line endings only (no textual diff). |
| `dev\holmes\wisdom-…-v2.md` vs repo `docs/research/` copy | **AGREE** — content-identical modulo EOL. (Reference material anyway.) |
| Alfred `docs/triad-canon.md` vs Alfred `_alfred-inbox/triad-canon.md` | **AGREE** — byte-identical (`3D9AF2DC…`, matching Alfred's W2 landing audit). |
| Holmes `docs/triad-canon.md` vs Alfred `docs/triad-canon.md` | **AGREE** — content-identical modulo EOL (`3452B7F6` vs `3D9AF2DC` hashes, zero textual diff). "Holmes-mirror-identical" still holds. |

**Repo-silent areas** (canon asserts, repo silent):
- GooseClaw: the entire numbered canon set + BuilderOS constitution +
  pin register (above) — the spec's `[01]`–`[20]` provenance tags point
  at files no repo holds. Largest homecoming gap.
- WCJBT: the triad canon names the WCJBT repo as its canonical home;
  the repo had no copy at all until this PR set (§4).
- Alfred: the operator-instruction set and the Alfred epistemic variant
  (above).
- Holmes: none — every governing file the tasking names is already
  in-repo; the hazard was stale *out-of-repo* duplicates, flagged above.

## 3. Conflicts for Martin (none resolved silently)

1. **Stale v2.0 spec copy** at `dev\holmes\holmes-spec-v2.md` disagrees
   with repo v2.1 on the Qwen roster, DeepSeek pricing permanence, and
   the Kuzu/Graphiti backend note. Resolution per sync rule: repo wins;
   recommend deleting the loose copies (needs your GO — deletion).
2. **GooseClaw `09_OPEN_DECISIONS.md` staleness** (spec §10 C3): items
   #5/#6 decided by D0901 but the file was never updated — the file is
   Kimi-side; updating it is the ratification PR's job there, or export
   it and we land it here.
3. **Triad-canon home was empty** (WCJBT) while mirrors lived in Holmes
   and Alfred — resolved structurally by §4 (home filled, pointers
   added); mirrors *kept* (non-destructive norm) — if you want the
   Holmes/Alfred mirror files *removed* in favor of pointers only, say
   so; not done unilaterally.
4. `epistemic-canon-Alfred.md` unrecovered (carried from Alfred's W2
   audit) — recover the original or commission a derivation; your call,
   not mine.

## 4. triad-canon.md — home decision

**Home: the WCJBT repo** (`docs/canon/triad-canon.md`, added by this
PR set). Why: (1) the canon's own header designates the WCJBT repo as
canonical home — the file is the authority on its own home; (2) the
triad canon is connective/north-star material and WCJBT is the
connective layer; (3) mirrors already exist in Holmes and Alfred, so no
consumer loses access. Pointers to the WCJBT home added to Holmes's and
Alfred's `CLAUDE.md` (their instruction files). Mirrors retained and
content-verified identical modulo EOL (2026-09-07).
