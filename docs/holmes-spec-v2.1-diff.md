# holmes-spec-v2 → v2.1 — Edit Set (rev. 2026-07-06)

**Apply to:** `docs/holmes-spec-v2.md` (filename stays stable so references hold; only the in-title version bumps).
**Provenance:** Second-pass QA, July 6, 2026. Every change below is grounded in a primary source checked that day; sources listed at the end. Confidence markers follow the spec convention — several claims are marked *more* conservatively here than in the chat readout, because the spec's unmarked tier requires primary-source verification and some details (exact Qwen launch day, Model Studio pricing) were confirmed only via secondary/distributor listings.
**Method:** each edit is a FIND (exact current text) → REPLACE pair, applied once. E1–E4 are header/changelog; E5–E10 are Correction 1–2 (model roster); E11–E14 are Correction 3 (memory backend). After applying, re-seed the knowledge files per the spec's own sync rule.

---

## E1 — Title (version bump)

FIND:
```
# Holmes — Product & Architecture Specification **v2 (QA-Corrected)**
```
REPLACE:
```
# Holmes — Product & Architecture Specification **v2.1 (QA-Corrected)**
```

## E2 — Status line (add revision date)

FIND:
```
**Verification date:** June 29, 2026 (**rev. 2026-06-29 — all Blacksky `[NEEDS-CAVEAT]` items resolved against primary sources**).
```
REPLACE:
```
**Verification date:** June 29, 2026 (**rev. 2026-06-29 — all Blacksky `[NEEDS-CAVEAT]` items resolved against primary sources**; **rev. 2026-07-06 — v2.1 corrections: Qwen roster, DeepSeek pricing permanence, Graphiti/Kuzu backend — see "What changed from v2"**).
```

## E3 — New changelog section

Insert immediately **after** the "What changed from v1 (QA corrections applied)" section and **before** the "## TL;DR" heading:

```
## What changed from v2 (v2.1, rev. 2026-07-06)

1. **Qwen roster correction** — the proprietary flagship entry "Qwen3-Max-Thinking" did not survive re-verification as a current SKU; replaced with **Qwen3.7-Max** (API-only, closed weights, announced ~May 20–21, 2026). §4.5's Tier-1 line also misfiled this model under "open-weight via API" in v2; now correctly listed as a permitted proprietary API. The open sovereign-tier line (Qwen3.5-27B / Qwen3.6-35B-A3B) is unaffected.
2. **DeepSeek V4 Pro pricing permanence downgraded** — the $0.435 / $0.87 rate is live on DeepSeek's official pricing page (verified 2026-07-06), but "the 75% cut became permanent" is now **[NEEDS-CAVEAT]**: secondary sources conflict (promo-ended-May-31 vs made-permanent) and DeepSeek reserves the right to adjust. Budget both rates. Alias-retirement detail sharpened: `deepseek-chat`/`deepseek-reasoner` retire **2026-07-24 15:59 UTC** and route to **V4 Flash** (not Pro) — a capability change, not a rename.
3. **Graphiti backend note (Kuzu abandonment)** — Graphiti's Kuzu driver is deprecated and Kuzu itself is abandoned (Apple acquired Kùzu Inc.; repo archived Oct 2025), removing the only zero-ops embedded backend for "the wall." Neo4j/FalkorDB remain the maintained options, which makes local-first-for-non-developers a bundled-service packaging problem (Phase 2 decision). Also flagged: Graphiti's documented default LLM/embedding client is **OpenAI** — a denylist configuration task — and its docs warn small-model extraction is failure-prone, bearing directly on the Tier-2 graceful-degradation claim.
```

## E4 — Annotate the superseded v1→v2 changelog item (history preserved, claim not hardened)

FIND:
```
1. **DeepSeek V4 Pro pricing** — the discounted rate is now the **standing** rate (the 75% cut became permanent after May 31, 2026): **$0.435 / 1M input (cache miss), $0.003625 cache-hit, $0.87 / 1M output**. `$1.74 / $3.48` is relabeled the **former reference price**, not "list."
```
REPLACE:
```
1. **DeepSeek V4 Pro pricing** — the discounted rate is now the **standing** rate (the 75% cut became permanent after May 31, 2026): **$0.435 / 1M input (cache miss), $0.003625 cache-hit, $0.87 / 1M output**. `$1.74 / $3.48` is relabeled the **former reference price**, not "list." *[Superseded in v2.1: the rate is verified live, but "became permanent" is downgraded to [NEEDS-CAVEAT] — see "What changed from v2."]*
```

## E5 — §3.2 roster table, DeepSeek V4 Pro price cell

FIND:
```
**$0.435 / $0.87** (standing rate; cache-hit **$0.003625**). Former reference price $1.74 / $3.48
```
REPLACE:
```
**$0.435 / $0.87** (current listed rate, verified 2026-07-06; cache-hit **$0.003625**). Reference/regular price $1.74 / $3.48 **[NEEDS-CAVEAT — permanence of the 75% discount unconfirmed: sources conflict (promo-ended vs made-permanent) and DeepSeek reserves the right to adjust; budget both rates]**
```

## E6 — §3.2 roster table, DeepSeek V4 Flash alias note

FIND:
```
Note: `deepseek-chat`/`deepseek-reasoner` aliases retire 2026-07-24
```
REPLACE:
```
Note: `deepseek-chat`/`deepseek-reasoner` aliases retire **2026-07-24 15:59 UTC**, routing to **V4 Flash** non-thinking/thinking respectively — `deepseek-reasoner` maps to **Flash, not Pro**, so migration is a capability change, not a rename; audit every config referencing these aliases before that date
```

## E7 — §3.2 roster table, Qwen proprietary row

FIND:
```
| **Qwen3-Max-Thinking** | Proprietary (API-only) | large | Alibaba Model Studio pricing | Reasoning variant of Qwen3-Max; OpenAI- & Anthropic-compatible APIs; **not** open-weight → Tier-1 only. Source: qwen.ai/blog?id=qwen3-max |
```
REPLACE:
```
| **Qwen3.7-Max** | Proprietary (API-only, closed weights) | 1M [DIRECTIONAL — consistent across secondary/distributor listings] | Alibaba Model Studio; ~$2.50 / $7.50 with cached-input tier [DIRECTIONAL — vendor/secondary figure, re-verify before volume] | Current Qwen proprietary flagship (announced ~May 20–21, 2026 [DIRECTIONAL on exact day]); agent/reasoning-oriented; OpenAI- & Anthropic-compatible endpoints [DIRECTIONAL]; **not** open-weight → Tier-1 only. Supersedes v2's "Qwen3-Max-Thinking" (name unverifiable as a current SKU). Source: qwen.ai/blog?id=qwen3.7 |
```

## E8 — §3.2 stale-claim-corrections paragraph

FIND:
```
**Qwen3-Max-Thinking** is real and **API-only/proprietary** (not open-weight);
```
REPLACE:
```
the current Qwen proprietary flagship is **Qwen3.7-Max** (API-only, closed weights — v2's "Qwen3-Max-Thinking" name did not survive re-verification);
```

## E9 — §4.5 Tier-1 line (also fixes a v2 misclassification)

FIND:
```
permitted open-weight via API (DeepSeek V4, Qwen3-Max-Thinking, Magistral) where appropriate.
```
REPLACE:
```
permitted open-weight via API (DeepSeek V4, Magistral) and permitted proprietary APIs (Qwen3.7-Max) where appropriate.
```

## E10 — §8 Qwen risk bullet

FIND:
```
- **Qwen3-Max-Thinking is API-only/proprietary** — fine as a Tier-1 option, not for the sovereign tier; the open **Qwen3.5-27B / Qwen3.6-35B-A3B** line (plus Magistral/Gemma) carries the local burden.
```
REPLACE:
```
- **Qwen3.7-Max is API-only/proprietary (closed weights)** — fine as a Tier-1 option, never for the sovereign tier; the open **Qwen3.5-27B / Qwen3.6-35B-A3B** line (plus Magistral/Gemma) carries the local burden. Qwen's top-tier line has been closed-weight since late 2025 [DIRECTIONAL]; watch the open Qwen line for sovereign-tier successors instead.
```

## E11 — §3.1 memory-architectures paragraph (backend note)

FIND:
```
it stores in **Neo4j or FalkorDB** and is self-hostable.
```
REPLACE:
```
it stores in **Neo4j or FalkorDB** and is self-hostable. *(Backend note, rev. 2026-07-06: Graphiti also ships a **Kuzu** driver, but it is **deprecated**, and Kuzu itself is **abandoned** — Kùzu Inc. was acquired by Apple and the repo archived in October 2025, leaving only unproven community forks. No zero-ops embedded backend currently survives; see §4.4.)*
```

## E12 — §4.4 "The Wall" engine bullet

FIND:
```
- **Engine:** self-hosted **Graphiti** temporal knowledge graph (Neo4j or FalkorDB backend), provenance-preserving and denylist-clean.
```
REPLACE:
```
- **Engine:** self-hosted **Graphiti** temporal knowledge graph (**Neo4j or FalkorDB** backend — the only maintained options: Graphiti's Kuzu driver is deprecated, and Kuzu is abandoned post-Apple-acquisition, Oct 2025), provenance-preserving, and **denylist-clean only when configured**: Graphiti's documented default LLM/embedding client is **OpenAI**, so Holmes must pin extraction and embeddings to permitted clients (Anthropic / Gemini / Ollama) and regression-test that no OpenAI default survives (see the denylist acceptance criteria, AC-DL-1/2). Graphiti's own docs warn that non-structured-output services "may result in incorrect output schemas and ingestion failures," and that this is "particularly problematic when using smaller models" — direct evidence that Tier-2 (local small-model) ingestion quality must be tested, not assumed (§4.5, §8). **Packaging consequence:** with no embedded backend, local-first for non-developers means bundling FalkorDB (or Neo4j Community) as a Holmes-managed local service — a Phase 2 product/packaging decision.
```

## E13 — §8, new risk bullet (insert immediately after the "OpenAleph maintenance." bullet)

INSERT:
```
- **Graph-backend packaging for non-developers.** Kuzu's abandonment (Apple acquisition, Oct 2025; Graphiti driver deprecated) removed the only zero-ops embedded backend for "the wall." Shipping Neo4j (JVM) or FalkorDB (Redis-module) to non-developers means Holmes must install, run, and supervise a local service invisibly. Decide the bundling strategy (Holmes-managed embedded service vs. optional power-user backend) in Phase 2, and test Graphiti against the chosen backend early.
```

## E14 — §8 pricing-volatility bullet

FIND:
```
- **Pricing volatility.** Re-verify all model prices before committing volume (DeepSeek's discounted rate is now standing, but vendors change pricing; Gemini long-context rates apply above 200K).
```
REPLACE:
```
- **Pricing volatility.** Re-verify all model prices before committing volume. DeepSeek V4 Pro's listed rate is the discounted $0.435/$0.87 (verified 2026-07-06) but its **permanence is [NEEDS-CAVEAT]** — budget against a possible reversion to $1.74/$3.48. Gemini long-context rates apply above 200K.
```

---

## Sources (checked 2026-07-06)

- **DeepSeek pricing + alias retirement:** api-docs.deepseek.com/quick_start/pricing (fetched directly — live rates $0.435/$0.87, cache-hit $0.003625; alias deprecation 2026-07-24 15:59 UTC routing to V4 Flash). Conflict on permanence: costgoat.com/pricing/deepseek-api (promo "until 2026-05-31") vs. felloai.com/deepseek-pricing (cut "made permanent") — hence the [NEEDS-CAVEAT].
- **Qwen3.7-Max:** qwen.ai/blog?id=qwen3.7 ("Qwen3.7: The Agent Frontier" — existence/primary); MarkTechPost 2026-05-21, TechNode 2026-05-21, VentureBeat (proprietary, closed weights, agent framing); OpenRouter and Vercel AI Gateway listings (availability, 1M context, pricing tier) — hence [DIRECTIONAL] on context/pricing/exact day.
- **Kuzu:** The Register 2025-10-14 ("KuzuDB graph database abandoned"); MacRumors 2026-02-11 / BetaKit / MacObserver (Apple acquisition of Kùzu Inc. via EU filing); community forks bighorn (Kineviz) and Ladybug noted, maintenance unproven.
- **Graphiti defaults and driver status:** github.com/getzep/graphiti README (OpenAI default; structured-output warning incl. "particularly problematic when using smaller models"); Graphiti/Zep graph-driver docs (Kuzu driver deprecated; "new projects should use Neo4j or FalkorDB").

## Post-apply

1. Bump complete: title reads v2.1; filename unchanged.
2. Have Claude Code **re-seed** the operating-context knowledge files (`CLAUDE.md`, `docs/architecture.md`, `docs/build-roadmap.md`, `docs/security.md`) from the revised spec, preserving all `[DIRECTIONAL]`/`[NEEDS-CAVEAT]` markers verbatim.
3. Audit any existing configs for `deepseek-chat`/`deepseek-reasoner` now — retirement is 2026-07-24.
