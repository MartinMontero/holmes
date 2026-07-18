# Holmes — Provider-Denylist Acceptance Criteria (AC-DL-1, AC-DL-2) · v3 (merged)

> **PROVENANCE.** v3, merged 2026-07-18: the union of the recovered v1 original (authored 2026-07-06, staged into the 2026-07-13 build-loop session) and the v2 re-derivation (2026-07-17, built when v1 was believed lost). v1 supplies the fuller test specification; v2 supplies the post-audit corrections. Reconciled to the 2026-07-13 second-pass audit: **F2** (§6 unsatisfiable in Phase 0 → scheduled to lock 2b), **F4** (§4 egress mechanism + out-of-process residual), **F7** (L1a/L1b guard split), and the kickoff-v3 decisions (Tauri shell deleted; L1b unknown-id rejection). **Numbering follows v1** — the committed Master Build Loop v2 binds to it (lock 0a: §§1–5, §7; lock 2b: §6). Supersedes both parents; v1 is preserved on Martin's machine, v2 in git history.
>
> **Status:** These criteria are the **Definition of Done for the provider denylist — not guidance.** The only denylist-compliant state is **all criteria applicable in-phase green in the same CI run.**

**Design premise (verified 2026-07-06):** goose ships **15+ providers and 70+ MCP extensions compiled/bundled in** — so a config default is not enforcement. Exclusion must be proven at two independent layers: the **runtime egress boundary** (AC-DL-1) and the **dependency tree** (AC-DL-2). Neither alone is sufficient.

**Excluded vendors:** Meta, OpenAI, xAI — by provider id *and* model-family id (Llama, GPT/o-series, Grok) — across the entire model and dependency tree. Permitted: Google, Anthropic, open weights on permitted infrastructure (DeepSeek, Qwen, Magistral/Mistral, Gemma). The list is settled; these tests enforce it, they do not relitigate it.

---

## AC-DL-1 — Runtime egress allowlist (core-owned, deny-by-default, fails closed)

**Claim under test:** at runtime, Holmes cannot send a request to a Meta / OpenAI / xAI inference endpoint — regardless of goose config, environment variables, MCP servers, or a compromised/confused agent — because the compiled Rust guard enforces a deny-by-default **egress allowlist** (permit only named permitted hosts) sitting outside any surface the agent or its config can edit.

**Why an allowlist, not a forbidden-host list:** a forbidden-host list is bypassable by any endpoint not yet on it (new vendor domains, proxies, IP literals, regional subdomains). An allowlist fails closed: anything not explicitly permitted is denied. This is the Ona-incident lesson — a boundary the agent can enumerate and reason around is not a boundary; the allowlist lives in the Rust core, below the agent's reach.

**Acceptance criteria:**

1. **Guard location & immutability.** Egress filtering is implemented in the **compiled Rust core** (`holmes-guard`, L1a) — not in goose config, not in TypeScript/JavaScript, not in a knowledge file, not in a prompt. *(v1's "Rust/Tauri core" is amended: the Tauri shell is deleted; the enforcement point is the library crate plus its egress proxy.)* The permitted-host list is a core-owned artifact; **there is no runtime code path by which the agent, an MCP tool, or an env var mutates it.** If a UI layer needs the list, it reads it from the crate, never enforces it.
2. **Default-deny proven — egress and resolution.** (a) A test issues an outbound request to each of a representative excluded set — at minimum `api.openai.com`, a Meta Llama API host, and `api.x.ai` — and asserts each is **blocked at the egress layer**: a guard-level denial, not a downstream 401/404, not merely "unconfigured." (b) At the resolution layer (**L1b**), permitted provider/model ids pass; excluded ids **and unknown ids** are rejected at resolution time, before any client is instantiated. Fails closed on both paths.
3. **Config-override attempt fails.** With `GOOSE_PROVIDER`/`GOOSE_MODEL` (and equivalent env/config) explicitly set to an excluded provider, launching an inference call still results in denial. Spawning is env-sanitized (provider-selecting variables stripped, permitted provider/model injected explicitly); the resolved model id is validated against §2(b) **before and after** the ACP handshake. Setting config never creates a reachable path.
4. **MCP/tool egress blocked at the proxy boundary.** Holmes-spawned sessions (goose, MCP tools) are forced through the deny-by-default local egress proxy (`HTTP(S)_PROXY` injected, provider vars stripped). A **deliberately-planted, proxy-honoring test MCP server** that calls an excluded endpoint directly — bypassing goose's provider abstraction — is **blocked at the network level**. **Residual, stated honestly (F4):** a hostile binary that ignores proxy environment variables escapes the library boundary; artifact/OS-level enforcement is **Alfred's obligation**, recorded in the cross-repo obligations ledger — it cannot be closed from this repo.
5. **Permitted paths still work.** The same suite asserts a permitted endpoint (Anthropic or Google endpoint, or a local Ollama socket) is **reachable** — proving the allowlist permits the sovereign and permitted-frontier paths, that the denylist has not silently become an allowlist blocking everything, and that the tests aren't passing by breaking all networking.
6. **Graphiti default neutralized — SCHEDULED to Phase 2 (lock 2b), recorded, never waived.** Graphiti's documented default LLM/embedding client is OpenAI (verified 2026-07-06) — the reason this criterion exists. When the memory layer lands: instantiate it with **no explicit LLM config** and assert it does **not** reach `api.openai.com` — either because extraction/embeddings are pinned to permitted clients (Anthropic / Gemini / Ollama) or because egress denies it. A known upstream default becomes a caught regression. Unsatisfiable before Phase 2 (audit blocker **F2**) — hence scheduled, visibly, in CI output.
7. **CI-gating, hermetic, release-mode.** §§1–5 and §7 run in CI on every PR and block merge; guard tests run in **release mode**. Execution is offline/hermetic — excluded hosts are asserted-blocked against a local resolver/mock, never by transiting the internet, so the tests never depend on excluded services being up. While §6 is scheduled, CI records it as **SCHEDULED** so the deferral cannot silently become an omission.

**Definition of done for AC-DL-1:** §§1–5 and §7 hold in Phase 0 (lock 0a); §6 holds at Phase 2 (lock 2b). The allowlist is core-owned and agent-unreachable.

---

## AC-DL-2 — Dependency-tree exclusion (deterministic lockfile walk)

**Claim under test:** no first-party or transitive dependency in any Holmes lockfile is published by, or is a client SDK for, an excluded vendor — so an excluded vendor cannot enter the tree as ordinary software even when no model endpoint is configured. Package presence is a distinct risk surface from endpoint reachability; it is deterministic to check — WCJBT's Layer-2 lockfile-walk pattern (`holmes-vs-wcjbt.md` §6.4), inherited, not reinvented.

**Acceptance criteria:**

1. **All lockfiles walked.** Every dependency lockfile across all ecosystems present — at minimum Rust (`Cargo.lock`), Node (`package-lock.json` / `pnpm-lock.yaml` / `yarn.lock`), and any Python surface (`uv.lock` / `poetry.lock` / `requirements*.txt`) introduced by Graphiti or tooling. **The resolved lockfile (transitive closure) is the unit of enforcement** — walking manifests alone is insufficient.
2. **Deterministic namespace + identifier denylist.** Matching runs against a checked-in, version-controlled list — deterministic string/namespace matching, **no LLM inference in the gate** (same input → same verdict; the epistemic firewall between judgment and deterministic enforcement). **Seed entries:** `openai`, `tiktoken`, `llama-stack`, `llama-*` (Meta), `xai*`/`grok*`, and `litellm` (routes to excluded providers; flagged in the March 2026 TeamPCP wave — GHSA-69fq-xp46-6x23 linkage **[DIRECTIONAL — per v1's source chain; verify the advisory opportunistically]**). Additionally scan manifests, configs, and code constants for excluded **model-family identifiers** (llama*, gpt-*/o-series, grok*). Document each entry's rationale in-file; the list grows by ledgered amendment.
3. **Fail-closed on match.** Any hit **fails the build** with a message naming the offending package, the lockfile, and **the dependency path that pulled it in** — a transitive introduction is traceable to its parent. No warn-and-continue.
4. **Positive control.** A fixture deliberately adding an excluded package (e.g., a pinned `openai` in a test manifest) **is caught** — proving the gate fires and hasn't silently no-op'd. Runs in isolation; never pollutes the real tree. *(Convention, fixed here for CI naming: positive = the gate fires on bad input.)*
5. **Negative control.** A permitted-but-adjacent package (the Anthropic SDK, a Google client, `ollama`) is asserted **not** to trip the gate — the matcher isn't over-broad and doesn't block permitted infrastructure.
6. **Denylist ≠ ambient CVE scan.** AC-DL-2 is provider exclusion — distinct from the supply-chain CVE gate (Syft SBOM + OSV-Scanner + Grype, **no Trivy**). Both run in CI; neither substitutes for the other. If `litellm` appears it should trip **both** — a useful cross-check.
7. **CI-gating, SHA-pinned.** Runs on every PR and blocks merge; implementing Actions are SHA-pinned (constitution rule #7); participates in the joint same-run rule below.

**Definition of done for AC-DL-2:** all seven hold; positive and negative controls both pass; the list is checked in with documented rationale; the gate is deterministic and SHA-pinned.

---

## Joint gate (the ship condition)

A Holmes build is denylist-compliant **only when AC-DL-1 (criteria applicable in-phase) and AC-DL-2 both pass in the same CI run.** Denylist enforced in code and proven by regression test — not declared in prose. Rule 9 applies: explicit human go-ahead before this or any phase is committed.

## What these criteria deliberately do NOT claim

- **Neutral-name proxying.** They do not detect an excluded model served under a neutral name on a permitted-looking endpoint. Residual risk: note in `docs/security.md`; mitigate by keeping the allowlist narrow and vetting any proxy host before adding it.
- **Weight provenance (→ AC-WP, lock 2e).** Verifying that downloaded Tier-2 open weights are what they claim is a separate criterion for the Phase 2/3 model-download path: checksum / signature / attestation **before load, failing closed on mismatch**. Flagged here by design; specified and satisfied at lock 2e.
- **Namespace completeness.** AC-DL-2's list is necessarily incomplete at any moment — a deterministic floor, not a proof of absence. Pair with dependency review on every new addition. **[NEEDS-CAVEAT — completeness of any denylist namespace set]**

---

*Sources (v1, checked 2026-07-06): goose bundled surface — github.com/aaif-goose/goose README + third-party guide (15+ providers, 70+ MCP extensions); Graphiti OpenAI default — github.com/getzep/graphiti README; litellm in the TeamPCP wave — GHSA-69fq-xp46-6x23 / CVE-2026-33634 advisory chain. Cross-references: `holmes-master-build-loop-v2.md` (pre-flight §2.5, Gate Zero, locks 0a/0b/2b/2e, constitution #2); 2026-07-13 second-pass audit (F2, F4, F7, F8); kickoff v3 §4.3; `holmes-spec-v2.md` §4.5/§6; `holmes-vs-wcjbt.md` §6.4; `alfred-security-perimeter-overview.md` (obligations 1–2). Staging: repo root or `docs/`; the loop normalizes to `docs/acceptance/holmes-denylist-acceptance-criteria.md`.*
