# Holmes — Provider-Denylist Acceptance Criteria (AC-DL) · v2 (re-derivation)

> **PROVENANCE — read first.** The 2026-07-13 original file was not recovered (authored in-session, never persisted to project knowledge). This is a **re-derivation, rebuilt 2026-07-17 on the pressure-testing surface** — the surface the loop names as this file's author ("persisted from upstream and never authored here"). It is built **to the Master Build Loop v2's binding references**, which are the design authority: pre-flight §2.5 (staging), lock 0a (AC-DL-1 §§1–5, §7 hermetic; §6 scheduled), lock 0b (AC-DL-2, all seven), lock 2b (§6 lands), lock 2e (AC-WP), constitution #2 (joint same-run rule), and audit amendments F4 (egress proxy + residual) and F8 (weight provenance). Sections pinned by the record are marked **[CARRIED]**; sections reconstructed to satisfy the loop's named checks are **[RE-DERIVED]** and require Martin's review before staging. If the original surfaces, reconcile — the binding requirement is that the loop's references resolve.
>
> **Status:** These criteria are the **Definition of Done for the provider denylist — not guidance.** The only denylist-compliant state is: **all criteria applicable in-phase green in the same CI run.**

**Scope.** Excluded across the entire model and dependency tree: **Meta, OpenAI, xAI** — by provider id *and* model-family id (Llama, GPT/o-series, Grok). Permitted: Google, Anthropic, and open-weight models on permitted infrastructure (DeepSeek, Qwen, Magistral/Mistral, Gemma). The list is settled; these criteria exist to make it *enforced*, not declared.

---

## AC-DL-1 — Runtime egress allowlist (core-owned, deny-by-default, fails closed)

**§1 — Ownership. [CARRIED, as amended by F4]** All policy logic lives in the **compiled Rust core** (`holmes-guard`). No policy logic in TypeScript/JavaScript anywhere — if a UI layer needs the list, it reads it from the crate, never enforces it. *(Original wording "Rust/Tauri core" is amended: the v3 embedding decision deleted the Tauri shell; the enforcement point is the library crate plus its egress proxy.)* Bypassing the guard means rebuilding from source.

**§2 — Resolution guard (L1b). [RE-DERIVED from kickoff v3 §4.3 + loop lock 0a]** Provider/model-id resolution is deny-by-default: permitted ids pass; excluded ids **and unknown ids** are rejected at resolution time, before any client is instantiated. Fails closed — absence from the permitted set is rejection, not a warning.

**§3 — Config/env-override refusal (L2). [CARRIED — lock 0a "config-override refusal"]** goose config precedence is env → config.yaml → defaults, and stock goose ships every provider compiled in. Holmes therefore never inherits the parent environment blindly: when spawning `goose acp` (absolute path), provider-selecting variables are stripped, the permitted provider/model is injected explicitly, and the resolved model id is validated against §2 **before and after** the handshake. Test: an env var or config file demanding an excluded provider is refused, with the refusal asserted.

**§4 — Tool/MCP egress blocked at the proxy boundary. [CARRIED — original §4 as amended by F4]** Holmes-spawned sessions (goose, MCP tools) are forced through a **deny-by-default local egress proxy**: environment sanitized, `HTTP(S)_PROXY` injected, provider variables stripped. Verification uses a **planted proxy-honoring test server** standing in for an excluded endpoint: tool-initiated calls to it must be blocked at the proxy boundary. **Residual, stated honestly:** a hostile binary that ignores proxy environment variables escapes the library boundary — artifact/OS-level enforcement is **Alfred's obligation**, recorded in the cross-repo obligations ledger; it cannot be closed from this repo.

**§5 — Permitted-path positive control. [CARRIED — lock 0a]** Anthropic, Google, DeepSeek, Qwen, Magistral, and Gemma resolve and pass. This proves the denylist has not silently become an allowlist that also blocks the permitted set.

**§6 — Memory-layer default-config test. [CARRIED — lock 2b + facts ledger; SCHEDULED to Phase 2, recorded, not skipped]** Graphiti's documented default LLM/embedding client is OpenAI (verified 2026-07-06) — the reason this criterion exists. When the memory layer lands (Phase 2): instantiate it with **no explicit LLM config** and assert no traffic reaches `api.openai.com`; extraction and embeddings pinned to permitted clients (Anthropic / Gemini / Ollama); regression test green thereafter. Unsatisfiable before the memory layer exists (audit blocker F2) — hence scheduled to lock 2b, never waived.

**§7 — Hermetic CI execution + joint gate. [CARRIED — locks 0a/0b, constitution #2]** §§1–5 and §7 run hermetically in CI, guard tests in **release mode**. Joint rule: **all criteria applicable in-phase green in the same CI run** is the only compliant state — no criterion satisfied "in a different run," none waived. While §6 is scheduled, CI records it as SCHEDULED, visibly, so the deferral cannot silently become an omission.

---

## AC-DL-2 — Deterministic dependency-tree exclusion (lockfile walk)

Seven criteria **[count CARRIED — lock 0b "all seven"; itemization RE-DERIVED except where marked]**. The gate is machine-deterministic: same input → same verdict; **no LLM call inside this gate** (triad boundary, `holmes-vs-wcjbt.md` §6.4).

1. **Deterministic full-graph walk.** The scanner walks the complete lockfile/dependency graph — direct and transitive — deterministically. No inference, no network judgment calls.
2. **Excluded-vendor package namespaces.** Packages published by or under Meta/OpenAI/xAI namespaces are rejected wherever they appear in the tree.
3. **Excluded model-family identifiers.** Manifests, configs, and code constants are scanned for excluded model-family ids (llama*, gpt-*/o-series, grok*); matches fail the gate.
4. **Router/gateway seed list. [seed CARRIED — facts ledger]** Packages whose function is routing to providers — reaching excluded vendors through an intermediary is still reaching them. Seed entry: **`litellm`** (routes to excluded providers). *Its GHSA-69fq-xp46-6x23 linkage is **[DIRECTIONAL — per the original's source chain; verify the advisory opportunistically]**.* The seed list is maintained in-repo and grows by ledgered amendment.
5. **Negative control.** A deliberately planted excluded dependency on a throwaway branch **fails** the gate — proving the scanner actually fires.
6. **Positive control.** The permitted stack passes untouched — proving the exclusion has not become an allowlist.
7. **CI wiring.** Runs as a required gate on every PR plus on schedule, and participates in the §7 joint same-run rule with AC-DL-1.

---

## AC-WP — Weight provenance (flagged here, not solved here) **[CARRIED — original ~L60 via F8; lands lock 2e]**

Tier-2 model downloads (the Phase 2/3 sovereign path) require a separate acceptance criterion: pulled weights verified by **checksum / signature / attestation before load, failing closed on mismatch**. Flagged in this document by design; specified and satisfied at Phase 2 lock 2e.

---

*Cross-references: `holmes-master-build-loop-v2.md` — pre-flight §2.5, Gate Zero, locks 0a/0b/2b/2e, constitution #2, facts ledger; kickoff v3 §4.3 (L1/L2 guard design, honest limits); 2026-07-13 second-pass audit — F2 (blocker: §6 unsatisfiable in Phase 0), F4 (MCP-egress mechanism + residual as Alfred obligation), F7 (L1a/L1b split), F8 (weight provenance); `alfred-security-perimeter-overview.md` (obligations 1–2); `holmes-vs-wcjbt.md` §6.4 (no LLM in deterministic gates). Staging: place at repo root or `docs/`; the loop normalizes to `docs/acceptance/holmes-denylist-acceptance-criteria.md`.*
