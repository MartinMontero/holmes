# Build roadmap — derived operating context

**DERIVED (re-seeded 2026-07-18) from `docs/holmes-spec-v2.md` v2.1 (§7), with sequencing/gates deferred to the Master Build Loop v2 §6** (precedence: constitution > loop on sequencing/gates > spec on architecture). To avoid a third divergent copy, the **canonical phase index is `docs/roadmap/build-phases.md`** (loop-§6-derived); this file records how the spec's §7 maps onto it.

## Spec §7 → loop §6 mapping

| Spec §7 | Loop §6 | Delta |
|---|---|---|
| Phase 0 — Scaffold | Phase 0 — Scaffold, guard, ACP, embedding contract, CI | Guard split L1a/L1b/L2 (A-04 pending in spec); "Tauri 2 + SolidJS shell" superseded — embedded in Alfred (A-03); "Apache-2.0-compatible repo" superseded by D-01 AGPL (A-02) |
| Phase 1 — Analytical core | Phase 1 | Smuggling scanner moved here from Phase 4; `knowability`/`limits_of_this_finding` schema amendment recorded in-phase |
| Phase 2 — The Wall | Phase 2 | AC-DL-1 §6 lands here; AC-WP weight provenance added; backend bundling decision (Kuzu abandoned) |
| — | **Phase 2.5 — Safety before surface (HARD GATE)** | Loop addition; spec §7 carry pending (A-05) |
| — | **→ Beta Scope Decision (BLOCKED, human D-##)** | Loop addition |
| Phase 3 — Investigative mode | Phase 3 | Only after 2.5, only per the scope decision |
| Phase 4 — Observability & safety | Phase 4 — Observability & hardening | Scanner moved out (Phase 1); smuggling regression stays green here |
| Phase 5 — Accountability layer | Phase 5 | Assembly hook **stubbed**; interim block = human reviewer, stated honestly |
| — | **Phase RC — Open-beta release candidate** | Loop addition; spec §7 carry pending (A-05) |

Pending spec amendments carried per loop §3.2: **A-02** (license per D-01), **A-03** (embedded-in-Alfred vs shell), **A-04** (L1a/L1b/L2 guard split), **A-05** (Phase 2.5 + Phase RC into §7; spec-v2.2 pipeline per the second-pass audit's A11). See `docs/audit/amendments.md`.

Every lock is met only with executed evidence; per-lock live status lives in `STATE.md`.
