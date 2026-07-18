# Amendments — Holmes

Format: `A-## | Document | Exact change | Finding(s) resolved | Status (PROPOSED / APPROVED / LANDED)`

Ordered by dependency, then severity. Per Rule 9, every amendment is PROPOSED until the human approves it.

*Amendments are an output of the audit (Phase 7) or of ledgered session findings. Append below.*

**A-01** | `docs/prompts/holmes-master-build-loop-v2.md` L60 | Original restore premise **void** per the F-013 correction 2026-07-17 (no clauses were dropped; the diff baseline was the mis-attributed v1 draft — committed L60 matches the authentic v2 original, sha256 `42ab6208…d457`). Awaiting Martin's pick between: **(a) forward enhancement** — inside L60's parenthetical, append one clause and nothing else: `(a capability change, not a rename)` → `(a capability change, not a rename; \`deepseek-reasoner\` → Flash, not Pro)`; or **(b) WITHDRAWN**. Nothing applied. | F-013 (as corrected) | **APPROVED: pick (a) — Martin, 2026-07-18, in-band ("A-01 ruling: A"). Applied to L60 on the session branch (one clause, nothing else); LANDED on PR merge.**
> Supersedes the original A-01 three-clause restore proposal (premise void; original wording preserved in git history, commit `1b56593`).

**A-02** | `docs/holmes-spec-v2.md` §7 Phase 0 | The line specifying an "Independent **Apache-2.0-compatible** repo" is superseded by D-01 (DECIDED 2026-07-18: AGPL-3.0-or-later, matching Alfred). Spec revision to state the license per D-01 is pending — spec text is canon authored on the pressure-testing surface, so this amendment records the delta rather than editing the spec here. Carried per loop §3.2 ("license-per-D-01 … as explicit amendments pending spec revision"). | F-001 / D-01 | PROPOSED

**A-03** | `docs/holmes-spec-v2.md` §7 Phase 0 (and §4.1 shell references) | The "Tauri 2 + SolidJS shell" element of spec Phase 0 has no home in the loop's §6 phase plan: Holmes is a library-shaped component **embedded in Alfred** (settled architecture per the loop header and launch runbook §0) — the shell/surface belongs to Alfred. Flagged during Task 3.4 roadmap reconciliation (2026-07-18) per the loop's anything-dropped-becomes-an-A-## rule; spec revision pending (same batch as the standing embedded-in-Alfred amendment the loop §3.2 names). | — (Task 3.4 rule; no F-item) | PROPOSED
> 2026-07-18 (post-3.2): survives the v2.1 diff — E1–E14 do not touch §7's shell line.

**A-04** | `docs/holmes-spec-v2.md` §4.6 / §7 Phase 0 | Carry the loop §6 guard split into the spec: **L1a** deny-by-default egress proxy (core-owned), **L1b** provider/model-id resolution guard (unknown ids rejected), **L2** sanitized spawn (env stripped, proxy injected, id validated pre/post handshake), L3 deferred-recorded. Spec currently states the denylist as "runtime guard + regression test" without the layer architecture. Standing amendment per loop §3.2 ("L1+L2 guard"). | — (loop §3.2 carry) | PROPOSED

**A-05** | `docs/holmes-spec-v2.md` §7 (roadmap) | Spec-v2.2 pipeline per the second-pass audit's **A11** (report on disk 2026-07-18): add **Phase 2.5 — Safety before surface (hard gate)** and **Phase RC — Open-beta release candidate** to §7; record the **Beta Scope Decision** checkpoint; carry the **D-01 outcome** (AGPL-3.0-or-later — supersedes §7's "Apache-2.0-compatible" line, folding in A-02); resolve the §4.1 secrets `[NEEDS-CAVEAT]` when the goose docs lines are cited. | F-014 context; audit A11 | PROPOSED
