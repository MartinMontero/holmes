# docs/canon — governing canon, home in the repo

**Precedence (per the BuilderOS spec, as stated in Martin's canon-homecoming
tasking 2026-09-07 — the BuilderOS spec text itself is not on this disk;
label: REPORTED):** **verified primary source > component canon > system
spec.** Within its lane, this repo's own canon governs.

**What lives here:** this directory is the *index*. Holmes's governing
canon already lives in the repo (the homecoming for Holmes happened when
the spec was committed with the "repo copy wins" sync rule). This README
makes that inventory explicit and states the precedence; it does not move
files (moving canon breaks inbound links; pointers beat moves).

## Governing canon (rules, specs, decisions) — all in-repo

| File | Role | Marker vocabulary |
|---|---|---|
| `docs/holmes-spec-v2.md` | Authoritative build reference (v2.1); "the repo copy wins" sync rule | five states (D-15) |
| `docs/triad-canon.md` | Shared north star; **mirror** — canonical home is the WCJBT repo (`docs/canon/triad-canon.md`); on disagreement the WCJBT copy wins | five states (D-15) |
| `docs/epistemic-canon-Holmes.md` | Holmes seat of the epistemic canon; body byte-identical across the three projects — never fork the body | (no claim markers — verified 2026-09-07) |
| `docs/constitution.md` | The twelve standing gates | clean |
| `docs/security.md` | Security posture + honest limits | five states (D-15) |
| `docs/architecture.md` | Derived operating context (re-seeded from the spec) | five states (D-15) |
| `docs/build-roadmap.md` / `docs/roadmap/build-phases.md` | Sequencing and gates | clean |
| `docs/holmes-project-orientation.md` | Operating loop + project mechanics | five states (D-15) |
| `docs/holmes-vs-wcjbt.md` | Differentiation + interface contract | five states (D-15) |
| `docs/acceptance/holmes-denylist-acceptance-criteria.md` | AC-DL-1/2 acceptance criteria (v3) | five states (D-15) |
| `docs/prompts/holmes-master-build-loop-v2.md` | The build loop (process canon) | five states (D-15) |
| `docs/holmes-launch-runbook-v1.md` | Launch runbook | five states (D-15) |
| `docs/holmes-claude-project-instructions.md` | Pressure-testing surface instructions | five states (D-15) |
| `CLAUDE.md` | Standing orders; protected block + spec-derived block | five states (D-15) |
| `docs/audit/decisions.md` | D-ledger — human decisions | append-only |
| `STATE.md` / `LOOP.md` | Live state / loop | live docs |

## Reference material (stays put; not governing)

`docs/research/*` (the Map v2/v3.x), `docs/case-file/*` (historical
provenance record), `docs/audit/*` (findings/amendments/history),
`docs/holmes-spec-v2.1-diff.md` (historical diff record — its old-vocabulary
markers are history, preserved), `docs/upstream/*`, `docs/beta-scope-decision-brief.md`
(input to D-14, decided).

## Out-of-repo stale duplicates (flagged, deletion is Martin's call)

Loose copies in `C:\Users\User\dev\` and `C:\Users\User\dev\holmes\`
(the Kimi-project drop folder): `holmes-spec-v2.md` there is **v2.0 —
STALE** (repo has v2.1, diff: 12+/19- incl. the 2026-07-06 corrections);
`holmes-project-orientation.md` and `wisdom-intuition-knowledge-judgment-v2.md`
there are **content-identical modulo line endings** (EOL-only hash delta,
verified 2026-09-07). The repo copies win. Recommend deleting the loose
copies to kill the two-truths hazard — a deletion needs Martin's GO.

## The vocabulary (D-15, Martin 2026-09-01)

Claim labels in governing canon are the **five system states**:
EXECUTED / VERIFIED-LIVE / CANON / REPORTED / UNVERIFIED (build-workflow
vocabulary per Alfred ADR-0005; loop reports and audits only, never
product UI). The retired `[DIRECTIONAL]`/`[NEEDS-CAVEAT]` markers map:
`[DIRECTIONAL]` → REPORTED; `[NEEDS-CAVEAT]` → the state its sourcing
earns, caveat preserved verbatim. Historical documents keep the old
markers where they recount history.
