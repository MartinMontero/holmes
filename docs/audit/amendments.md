# Amendments — Holmes

Format: `A-## | Document | Exact change | Finding(s) resolved | Status (PROPOSED / APPROVED / LANDED)`

Ordered by dependency, then severity. Per Rule 9, every amendment is PROPOSED until the human approves it.

*Amendments are an output of the audit (Phase 7) or of ledgered session findings. Append below.*

**A-01** | `docs/prompts/holmes-master-build-loop-v2.md` L60 | Replace the Task 3.3 line with: `- **3.3 — Time-critical alias audit.** Audit every config, doc, and reference for \`deepseek-chat\` / \`deepseek-reasoner\`: the aliases retire **2026-07-24 15:59 UTC**, both routing to **V4 Flash** (\`deepseek-reasoner\` → Flash, **not Pro** — a capability change, not a rename) — verified live 2026-07-13 against the DeepSeek pricing page. **Fix or flag before anything else ships.** Budget/config logic must tolerate both DeepSeek rates ($0.435/$0.87 current listed; $1.74/$3.48 reference); the promo rate is never hard-coded.` — restores the draft's dropped clauses while retaining the landing's provenance annotation and markdown. Alternative if the human prefers: restore the drafted sentence verbatim (drops the annotation). | F-013 | PROPOSED
