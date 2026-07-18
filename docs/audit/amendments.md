# Amendments — Holmes

Format: `A-## | Document | Exact change | Finding(s) resolved | Status (PROPOSED / APPROVED / LANDED)`

Ordered by dependency, then severity. Per Rule 9, every amendment is PROPOSED until the human approves it.

*Amendments are an output of the audit (Phase 7) or of ledgered session findings. Append below.*

**A-01** | `docs/prompts/holmes-master-build-loop-v2.md` L60 | Original restore premise **void** per the F-013 correction 2026-07-17 (no clauses were dropped; the diff baseline was the mis-attributed v1 draft — committed L60 matches the authentic v2 original, sha256 `42ab6208…d457`). Awaiting Martin's pick between: **(a) forward enhancement** — inside L60's parenthetical, append one clause and nothing else: `(a capability change, not a rename)` → `(a capability change, not a rename; \`deepseek-reasoner\` → Flash, not Pro)`; or **(b) WITHDRAWN**. Nothing applied. | F-013 (as corrected) | **APPROVED: pick (a) — Martin, 2026-07-18, in-band ("A-01 ruling: A"). Applied to L60 on the session branch (one clause, nothing else); LANDED on PR merge.**
> Supersedes the original A-01 three-clause restore proposal (premise void; original wording preserved in git history, commit `1b56593`).

**A-02** | `docs/holmes-spec-v2.md` §7 Phase 0 | The line specifying an "Independent **Apache-2.0-compatible** repo" is superseded by D-01 (DECIDED 2026-07-18: AGPL-3.0-or-later, matching Alfred). Spec revision to state the license per D-01 is pending — spec text is canon authored on the pressure-testing surface, so this amendment records the delta rather than editing the spec here. Carried per loop §3.2 ("license-per-D-01 … as explicit amendments pending spec revision"). | F-001 / D-01 | PROPOSED
