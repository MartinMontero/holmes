# Amendments — Holmes

Format: `A-## | Document | Exact change | Finding(s) resolved | Status (PROPOSED / APPROVED / LANDED)`

Ordered by dependency, then severity. Per Rule 9, every amendment is PROPOSED until the human approves it.

*Amendments are an output of the audit (Phase 7) or of ledgered session findings. Append below.*

**A-01** | `docs/prompts/holmes-master-build-loop-v2.md` L60 | Original restore premise **void** per the F-013 correction 2026-07-17 (no clauses were dropped; the diff baseline was the mis-attributed v1 draft — committed L60 matches the authentic v2 original, sha256 `42ab6208…d457`). Awaiting Martin's pick between: **(a) forward enhancement** — inside L60's parenthetical, append one clause and nothing else: `(a capability change, not a rename)` → `(a capability change, not a rename; \`deepseek-reasoner\` → Flash, not Pro)`; or **(b) WITHDRAWN**. Nothing applied. | F-013 (as corrected) | PROPOSED — awaiting pick (a)/(b)
> Supersedes the original A-01 three-clause restore proposal (premise void; original wording preserved in git history, commit `1b56593`).
