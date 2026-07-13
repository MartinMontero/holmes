# Holmes — claude.ai Project Orientation & Index

*Drop this into the Holmes project's knowledge. It's the map: what exists, where it lives, and how the project stays organized.*

---

## 1. Two surfaces, one source of truth

| Surface | Role | Source of truth? |
|---|---|---|
| **Claude Code repo** | Build target. Holds the canonical spec and the operating-context knowledge files in `docs/`. | **Yes** — the repo copy of `holmes-spec-v2.md` is canonical. |
| **This claude.ai project** | Thinking + pressure-testing surface. Where readouts get stress-tested, the spec gets argued and revised, and new phases get planned. | No — drafts here flow **back into the repo**. |

**Sync rule:** revisions are drafted here, then written back into the repo; Claude Code re-seeds the knowledge files from the updated spec. If a draft here and the repo copy ever disagree, **the repo copy wins.** (This rule is also baked into the top of `holmes-spec-v2.md`.)

---

## 2. Set the project up once

1. **Add to project knowledge:** `holmes-spec-v2.md` (the canonical spec) and, optionally, this orientation file. That gives every chat in the project the full architecture without re-pasting.
2. **Paste the custom instructions** from §3 into the project's instructions, so every new chat starts oriented.
3. **Name chats consistently**, e.g. `Holmes · Phase 0 · scaffold`, `Holmes · Phase 1 · analytical core`, `Holmes · spec revision · Blacksky caveats`. Keeps the project list scannable.

---

## 3. Project custom instructions (paste verbatim into project settings)

> **Project: Holmes.** Holmes is the research-and-analysis brain of the Wecanjustbuildthings.dev (WCJBT) non-developer OS, and the sibling of Alfred (the agentic development brain). It is the digital incarnation of the investigator-analyst Santos Reyes. Core thesis: **"the method is the identity"** — Holmes realizes a *method* (abduction + Bayesian likelihood-ratio updating, Structured Analytic Techniques, first-principles reasoning, a six-phase case workflow) plus an ethical *code*, not "an LLM with search tools."
>
> **Source of truth:** `holmes-spec-v2.md` in this project's knowledge is authoritative. Where I'm unsure, defer to it; don't contradict it from memory. The canonical copy lives in the Claude Code repo at `docs/holmes-spec-v2.md`; drafts here flow back to the repo, and the repo copy wins on any disagreement.
>
> **Non-negotiables (the constitution):** (1) Rule 9 — no commit/push without explicit go-ahead. (2) Provider **denylist**, not allowlist — exclude Meta/OpenAI/xAI across the whole tree; Google permitted; open-weights-on-permitted-infra permitted. (3) No fabrication — never invent sources/data; silence over a false claim. (4) Path-confined, deny-by-default tools. (5) Born-redacted, local-only telemetry. (6) Supply-chain hygiene: Syft/OSV-Scanner/Grype, **no Trivy**, SHA-pin Actions. (7) Surveillance-detection-not-surveillance; anti-doxxing. (8) "I answer to the block" — Blacksky-style community accountability + human-in-the-loop.
>
> **How this project works:** this is my pressure-testing surface. Claude Code Desktop builds and emits readouts; I paste them here and you stress-test them against the spec (is the denylist actually enforced, is the ACP round-trip real, are the knowledge files faithful with confidence markers preserved). Be direct and skeptical; flag drift from the spec or the constitution. Preserve `[DIRECTIONAL]` / `[NEEDS-CAVEAT]` markers — don't silently harden caveated claims into facts.

---

## 4. Artifact index (from the originating conversation)

| File / artifact | Purpose | Canonical home | Status |
|---|---|---|---|
| **`holmes-spec-v2.md`** | Authoritative build reference (full architecture + QA corrections + confidence markers) | Repo `docs/` **+** project knowledge | **Current** |
| **`holmes-claude-code-kickoff-phase0-v2.md`** | Phase 0 kickoff prompt for Claude Code Desktop (re-pointed at the spec) | Repo (pasted into Claude Code) | **Current** |
| `holmes-claude-code-kickoff-phase0.md` (v1) | First kickoff prompt (carried bare facts, not the spec) | — | Superseded by v2 |
| "Holmes — Product & Architecture Specification…" (v1 spec) | Original spec | — | Superseded by v2 |
| "QA Report — Holmes…" | Claim-by-claim verification + corrections | Reference / archive | Folded into v2 |
| `trinity-incarnate-character-bible.md` | Santos Reyes source material | Project knowledge (input) | Source input |

Keep only the **Current** rows live in day-to-day work; the rest are archive/provenance.

---

## 5. The build loop (operational sequence)

1. **Pre-flight:** clone Alfred + wecanjustbuildthings.dev as siblings of the empty Holmes folder; ensure goose (`aaif-goose/goose`) is installed; drop `holmes-spec-v2.md` into the folder; decide smoke-test mode (cloud Tier-1 key vs offline Tier-2 via Ollama).
2. **Run Phase 0:** open Claude Code Desktop in the folder → paste `holmes-claude-code-kickoff-phase0-v2.md` → review each step → it stops at the Rule-9 checkpoint and emits a readout.
3. **Pressure-test:** paste that readout into a chat in *this* project → stress-test against the spec → corrections back to Claude Code → iterate to Definition of Done.
4. **Commit + advance:** give explicit go-ahead to commit (Rule 9) → settle the 3–5 Phase 1 locks → start Phase 1 with a fresh scoped prompt.

---

## 6. Open threads to carry forward

- **Blacksky `[NEEDS-CAVEAT]` items — RESOLVED (2026-06-29).** All confirmed against primary sources (docs.blacksky.community moderation + community-guidelines, the appeals form, the ToS) and folded into `holmes-spec-v2.md`: Labeler "cannot delete content"; appeals via web form, "a different moderator," 7-day target; the four non-appealable categories (white supremacy, CSAM, digital blackface, severe threats); Ubuntu "I am because we are"; "protect your peace"; Blacksky's own doxxing definition; the 645-member People's Assembly; NY-law governance. Load the revised spec into project knowledge.
- **Decide the Phase 0 smoke-test mode** (cloud Tier-1 vs fully offline Tier-2).
- **Confirm sibling repos are on disk** so Claude Code can inherit Alfred's conventions rather than reinventing them.
- **Two minor non-Blacksky caveats remain in the spec** (peripheral): the exact Vercel Sandbox GA month, and goose's per-platform secret-store backend. Resolve opportunistically; neither blocks Phase 0.
