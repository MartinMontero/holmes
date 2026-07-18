# Architecture — derived operating context

**DERIVED (re-seeded 2026-07-18) from `docs/holmes-spec-v2.md` v2.1 (§2–§5).** The spec is authoritative; on any disagreement the spec wins. Markers (`[DIRECTIONAL]`, `[NEEDS-CAVEAT]`) are carried verbatim — never harden them.

## Substrate (spec §4.1)

- **Runtime:** goose (`aaif-goose/goose`, Apache-2.0, Rust, Linux-Foundation/AAIF-governed since 2026-04-07), driven over **Zed's Agent Client Protocol** — JSON-RPC 2.0 over stdio, invoked via the `goose acp` command (no separate crate; distinct from IBM/BeeAI's "Agent Communication Protocol").
- **Tools:** MCP for everything; Holmes ships its own MCP servers (records, OSINT, the-wall memory, link-analysis), path-confined, deny-by-default.
- **Secrets:** goose stores secrets in the platform credential store `[NEEDS-CAVEAT — confirm exact backend per platform in goose docs]`; keys never in files, never logged.
- **Delivery surface:** the spec's §4.1/§7 "Tauri 2 + SolidJS shell" is **superseded**: Holmes ships as `holmes-core` + `holmes-guard` crates **embedded in Alfred** — no standalone UI/installer/updater (loop v2 header; pending spec amendment **A-03**).

## Analytical core (spec §2, §4.2)

Three engines + the six-phase case method, as goose recipes/subagents:
1. **Abduction + likelihood-ratio updating** — hypothesis objects with priors, predicted-present/absent evidence, running log-likelihood; Platt's strong inference.
2. **Socratic/SATs** — ACH matrix (fewest-inconsistencies wins; seek disconfirmation), Key Assumptions Check, devil's-advocate pass.
3. **First principles** — quarantines unverifiable inputs before the rebuild.

Phases: Intake (harm check) → La Lluvia (hypothesis generation) → Collection (documents state of mind) → The Wall (ACH + assumptions + el diablo) → Following the Money → Resolution & Handoff (**no autonomous action** — traceable case file to journalist/lawyer/community).

Orchestration: Orchestrator-Worker case runner + Evaluator-Optimizer critic as Holmes-internal goose roles — no external agent framework adopted.

## Investigative mode (spec §4.3)

Layered on the core, activated explicitly, gated behind the Phase 2.5 safety layer. Public-records spine (OpenCorporates — cite scale figures with dates; SoS registries, PACER, FOIA, campaign finance); OSINT parallel; link analysis via **OpenAleph** (DARC fork, MIT; OCCRP's line unmaintained after Dec 2025) with Graphiti as the temporal case graph.

## The Wall (spec §4.4)

Self-hosted **Graphiti** temporal knowledge graph; **Neo4j or FalkorDB only** (Kuzu driver deprecated; Kuzu abandoned post-Apple-acquisition, Oct 2025 — no zero-ops embedded backend survives; bundling is a Phase 2 packaging decision). **Denylist-clean only when configured:** Graphiti's documented default LLM/embedding client is OpenAI — extraction/embeddings must be pinned to permitted clients and regression-tested (AC-DL-1 §6, lands Phase 2). Bi-temporal provenance; facts invalidated, never deleted. Adaptive RAG router on top.

## Model strategy (spec §4.5, §3.2)

- **Tier 1 (cloud):** Claude, Gemini 3.1 Pro; permitted open-weight via API (DeepSeek V4, Magistral); permitted proprietary API (Qwen3.7-Max — closed weights, never sovereign-tier).
- **Tier 2 (sovereign, local):** non-Meta open weights only — Qwen3.5-27B / Qwen3.6-35B-A3B, Magistral Small, Gemma; via Ollama/Jan/llama.cpp. Private queries default to Tier 2.
- **Alias watch:** `deepseek-chat`/`deepseek-reasoner` retire **2026-07-24 15:59 UTC**, routing to **V4 Flash** non-thinking/thinking — `deepseek-reasoner` → Flash, **not Pro** (capability change, not a rename). DeepSeek V4 Pro discount permanence is `[NEEDS-CAVEAT]` — budget both rates.

## Sibling relationship (spec §5)

Holmes is the *analysis* to Alfred's *memory and hands*: Holmes researches and hands a provenance-complete spec to Alfred; Alfred invokes Holmes over ACP/MCP for investigative questions. Same denylist, same path-confined tools, same born-redacted telemetry, same human-in-the-loop discipline. Holmes's Rule-9 analogue: no conclusion surfaces and no case file is handed off without human review.
