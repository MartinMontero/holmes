# Holmes — Claude Code Master Build Loop · Phase 0 → Open-Beta RC (v2)

> **What this is.** v2 of the single driving prompt for building Holmes end-to-end in Claude Code — from the repo's current state through Phases 0–5 and Phase RC (open-beta release candidate), under a fused /loop + Rule 9 discipline. Supersedes v1. Applies the 2026-07-13 second-pass audit in full (amendment queue A1–A11) plus the git-state reconciliation the F-012 episode forced. The spec (`docs/holmes-spec-v2.md`, v2.1 after Task 0.2) remains authoritative for architecture; this prompt carries orchestration, locks, and gates. On conflict: spec wins on architecture, this prompt wins on sequencing and gates, the constitution wins on everything.
>
> **What changed from v1 (audit findings in parentheses):**
> 1. **Gate Zero** — one first BLOCKED stop bundling the human calls v1 wrongly executed or omitted: charter-verdict acceptance (F3), D-01 license ratification with on-disk Alfred/LICENSE evidence (F1), git-reconciliation approval (F-012), discipline ratification.
> 2. **Lock 0a fixed** — AC-DL-1 §6 (Graphiti default) scheduled to Phase 2, where the memory layer exists (F2).
> 3. **Guard split L1a/L1b** with an explicit egress-proxy mechanism and an honest residual for out-of-process tools (F4, F7).
> 4. **CLAUDE.md protected-block merge** — re-seeding can never clobber the standing gates (F5); label-system mapping and the ledger row format carried verbatim (F6, F14).
> 5. **Git/branch discipline** — fetch-first; the session branch is the working medium; `main` lands by PR on explicit go-ahead only; no history rewrites; connector re-sync after canon lands (F-012).
> 6. Phase 6 → **Phase RC** (F12); weight-provenance AC added to Phase 2 and the smuggling scanner pulled to Phase 1 (F8, F9); facts-ledger marker hygiene (F10); path-tolerant pre-flight + staging checklist (F11); vendor-doc verification fetches added to the declared egress surface (F15); Apache wording fixed (F13).
>
> **Repo state (verified live 2026-07-13):** `github.com/MartinMontero/holmes` — public, `origin/main` at 3 commits, top level {`docs/`, `CLAUDE.md`, `LICENSE` (Apache-2.0, pending D-01), `README.md`}, no releases. A stale session branch (`claude/…`) may exist whose local view of `main` predates commits 2–3 (ledger F-012). Reconciliation is handled at pre-flight + Gate Zero.
>
> Holmes is a **library-shaped component embedded inside Alfred** — not a standalone app; the beta ships inside Alfred's signed artifact. License is **per D-01** (AGPL-3.0-or-later proposed, ratified at Gate Zero). "The method is the identity."

---

## ▼ PASTE BELOW THIS LINE ▼

You are **Claude Code** in the Holmes repo (`github.com/MartinMontero/holmes`). Mission: carry Holmes to an **open-beta MVP release candidate** — installable by a non-developer inside Alfred, denylist-guard verified in the shipped artifact, safety layer live *before* the investigative surface it protects, local-first proven, honestly documented, every phase gated by executed evidence. Nothing lands on `main`, and nothing irreversible happens anywhere, without explicit human go-ahead.

---

### 1 — Operating discipline (the fused loop)

**Inner loop — /loop protocol.** Within a phase, work autonomously through build→verify cycles on the **working/session branch** — commits and pushes to that branch are the autosave medium, not Rule-9 events. Maintain `LOOP.md` at repo root per phase: **Scope** (one sentence; explicit non-goals), **Journeys** (every end-to-end path, numbered, incl. failure/edge), **Gates** (exact commands that must exit 0), **Quality criteria**, **Assumptions** (judgment calls logged; stop only if resolution needs human input or an irreversible action). Highest-impact unchecked item first; **verify by execution, never inspection**; regression-run what the change touched; record evidence as command → salient output. Never mark a lock met without executed proof. Before declaring a phase done: one full verification pass from a clean state.

**Outer gates — Rule 9 + BLOCKED.** Rule-9 territory (explicit human go-ahead, always): anything touching `main` (PR merge, direct push, tag, release); any history rewrite anywhere; destructive or irreversible actions — including relicensing, per CLAUDE.md Gate 6; spend decisions; credential needs; network egress beyond the declared surface. **Declared egress surface:** package registries, the configured smoke-test model endpoint, and primary-source vendor-documentation fetches for verification. Stop conditions verbatim: **DONE** (every LOOP.md item ✅ with evidence; UNVERIFIED only where out-of-environment, with reason), **BLOCKED** (needs human input — Gate Zero and the Beta Scope Decision are BLOCKED by design), **STUCK** (same failure survives 3 materially different attempts; report attempts, hypothesis, next step). Never stop for effort or length; checkpoint full state to `LOOP.md`/`STATE.md` so a fresh session resumes exactly — and every fresh session begins with `git fetch origin`.

**Ledger discipline (in-repo, binding).** Honor `CLAUDE.md`'s standing gates. Route discoveries into the existing ledgers — findings to `docs/audit/findings-ledger.md`, amendments to `docs/audit/amendments.md`, human calls to `docs/audit/decisions.md`. Row format verbatim: `F-### | Severity | Category | Location | Evidence (quote or ABSENT) | Why it matters | Recommended fix | Confidence (H/M/L)`. Severities BLOCKER / MAJOR / MINOR / NIT. IDs are stable — never renumber. **Never resolve a D-item yourself.**

**Label mapping (two systems, one rule).** Ledger entries use CLAUDE.md labels: `VERIFIED` (cite source + date) / `INFERRED` (show the chain) / `ASSUMED` (flag it) / `UNKNOWN` (becomes a research item). Spec-derived docs preserve the canon markers verbatim: unmarked = primary-verified; `[DIRECTIONAL]` ≈ INFERRED/ASSUMED; `[NEEDS-CAVEAT]` = concept holds, exact detail unconfirmed. Never silently harden either kind.

**Epistemic discipline.** Defer to the spec; verify live facts against installed tooling and primary sources, never memory; if a fact, file, command, or capability can't be verified, stop and say so. Silence over a false claim.

---

### 2 — Pre-flight (read-only evidence gathering; any FAIL = STOP and report)

1. **Fetch first.** `git fetch origin`. Record `git log --oneline origin/main`. Expected: a **superset** of the 3-commit state observed 2026-07-13 (initial LICENSE → README/CLAUDE.md/docs; more commits are fine, e.g. the F-012 landing). If `origin/main` is *shorter* or rewritten relative to that observation → **STOP**: possible history rewrite; file a Gate-6-class F-### and escalate. Never build on a rewritten `main` unexplained.
2. **Branch state.** Name the current branch. `git merge-base --is-ancestor origin/main HEAD && echo FF-OK || echo DIVERGED`. If DIVERGED: produce `git diff --name-status $(git merge-base origin/main HEAD)..HEAD`, run a dry-run merge on a throwaway ref, report conflicts, abort — this evidence goes to Gate Zero. Never plan a resolution that discards `origin/main`'s CLAUDE.md standing gates or seeded ledger entries (F-001, D-01).
3. **Sibling repos** — `Alfred/` and `wecanjustbuildthings.dev/` readable. **Read `Alfred/LICENSE` now and quote it verbatim** — D-01 evidence. Alfred is ground truth for shared conventions; conflicts flagged (A-##/F-###), never silently resolved.
4. **goose is the AAIF build** — origin `aaif-goose/goose`; `goose acp --help` exists; resolve and record the **absolute binary path**; all spawning uses it. Config may still namespace under `…\Block\goose\` — record as found.
5. **Spec + canon + AC doc staged** — `holmes-spec-v2.md`; `triad-canon.md`; `holmes-vs-wcjbt.md`; `epistemic-canon-Holmes.md`; `holmes-denylist-acceptance-criteria.md`. Accepted at repo root **or** `docs/`; normalize into `docs/` (AC doc → `docs/acceptance/`); normalize the `holmes-vs-wcjbt.md.md` double extension if present. Any absent → **STOP** and hand the human this staging checklist — these files are persisted from upstream and **never authored here**.
6. **Smoke-test provider** — cloud Tier-1 (Anthropic or Google key) **or** offline Tier-2 (Ollama with a non-Meta weight: Qwen3.5-27B / Qwen3.6-35B-A3B / Magistral Small / Gemma), confirmed responding. None = STOP.

---

### 3 — Task 0 (idempotent; §3.0 precedes all writes)

- **3.0 — GATE ZERO (BLOCKED — the run's first stop; no file writes before it clears).** Present with evidence, then wait:
  **(a) Charter verdict.** CLAUDE.md forbids scaffolding "until the adversarial audit produces a READY verdict." Propose recording the **2026-07-13 second-pass audit** (verdict: SHIP WITH FIXES; fixes applied in this v2) as that verdict in `decisions.md`. Confirm, or direct otherwise.
  **(b) D-01 — license.** Evidence: the quoted `Alfred/LICENSE` from disk; provenance: the 2026-07-13 v3-session decision (AGPL-3.0-or-later, triad unity — digest-sourced, so confirm in-band). CLAUDE.md Gate 4: no relicense without explicit human approval. On explicit "AGPL confirmed": swap `LICENSE` to AGPL-3.0-or-later, record D-01 with the human as decider, update README and F-001. If `Alfred/LICENSE` is **not** AGPL-3.0, the "matching Alfred" premise fails — present options; decide nothing.
  **(c) Git reconciliation** (if pre-flight found divergence or a stale prior session branch): the PR-based landing plan with the dry-run conflict report. Approve or direct. Correct **F-012** in the ledger with the fetch evidence — the true finding is *stale connector sync + stale session clone*, not *empty main*.
  **(d) Discipline ratification** (one line): working-branch writes autonomous under /loop; `main` and all irreversibles Rule-9-gated.
  Record every answer as D-## entries.
- **3.1 — Land the canon on `main`.** Ensure spec + canon + AC doc + Gate-Zero ledger updates are on the working branch; open a **PR to `main`**; merge only on explicit go-ahead. After any PR that lands canon: **remind the human to re-sync the claude.ai project's GitHub connector** (stale-sync fingerprint: project knowledge holding LICENSE only). Going forward, canon changes merge to `main` promptly; the connector never points at a scratch branch.
- **3.2 — Spec v2.1.** If the spec title still reads **v2**, apply `holmes-spec-v2.1-diff.md` (E1–E14) exactly, then re-seed the derived knowledge files (`CLAUDE.md` operating context per the protected-block rule in §6/0e, `docs/architecture.md`, `docs/constitution.md`, `docs/build-roadmap.md`, `docs/security.md`), markers preserved verbatim. If already v2.1, spot-verify E5–E14 and move on. Carry the standing amendments — license-per-D-01, L1+L2 guard, embedded-in-Alfred, Phase 2.5, Phase RC — as explicit amendments pending spec revision.
- **3.3 — Time-critical alias audit.** Audit every config, doc, and reference for `deepseek-chat` / `deepseek-reasoner`: deprecation **2026-07-24 15:59 UTC**, both routing to **V4 Flash** (a capability change, not a rename; `deepseek-reasoner` → Flash, not Pro) — verified live 2026-07-13 against the DeepSeek pricing page. Budget logic must tolerate both rates ($0.435/$0.87 current listed; $1.74/$3.48 reference); the discounted rate is never hard-coded.
- **3.4 — Reconcile the roadmap draft.** Replace `docs/roadmap/build-phases.md` (marked PROPOSED) with the §6 phase plan; anything the draft has that this plan lacks becomes a flagged A-## rather than silently dropped.

---

### 4 — Constitution (enforce, don't declare)

1. **Rule 9** — no landing on `main`, no push/tag/release, no destructive/irreversible/outward action without explicit human go-ahead.
2. **Provider denylist** — Meta / OpenAI / xAI excluded across the entire model and dependency tree (provider ids *and* model families: Llama, GPT/o-series, Grok). Google permitted, Anthropic permitted, open-weights-on-permitted-infra permitted. Settled — enforce, don't relitigate. Enforcement = **AC-DL-1** (runtime egress allowlist, core-owned, fails closed) **plus** **AC-DL-2** (deterministic lockfile-walk exclusion); all criteria applicable in-phase green in the same CI run is the only compliant state. Full criteria: `docs/acceptance/holmes-denylist-acceptance-criteria.md` — they are the Definition of Done, not guidance.
3. **No fabrication.**
4. **Local-first, sovereign, non-developer-usable.** Every guarantee is a product promise in what ships inside Alfred's signed artifact.
5. **Surveillance-detection-not-surveillance; anti-doxxing.**
6. **"I answer to the block"** — human-in-the-loop now; the interim block is the human reviewer until the assembly exists, stated honestly wherever governance is described.
7. **Supply-chain hygiene** — Syft SBOMs (CycloneDX + SPDX), OSV-Scanner primary, Grype cross-check, **no Trivy** (CVE-2026-33634), every Action SHA-pinned.
8. **Open standards** — goose / ACP / MCP; Nostr + AT Protocol conventions (flows are later phases).
9. **License — per D-01 as ratified at Gate Zero** (AGPL-3.0-or-later proposed). Until ratified: author no new license identifiers anywhere.

**Triad boundaries as CI** (`holmes-vs-wcjbt.md` §6.4): Holmes exports no blueprint/spec/constitution type; every emitted finding carries provenance + confidence (+ `knowability` and limits per the epistemic canon, Phase 1); no LLM call inside any deterministic gate.

---

### 5 — State detection & resume

Before building, inventory the repo against every §6 lock and write `STATE.md`: per lock, **VERIFIED** (evidence re-executed now, never trusted from history), **PARTIAL**, or **ABSENT**. `STATE.md` also records: current branch, `origin/main` head, ancestry status, and a standing **Cross-repo obligations — Alfred** section (artifact-level guard test, OS/artifact-level egress enforcement, updater/rollback, first-run rendering). Resume at the earliest unmet lock; every resumed session re-runs `git fetch origin` first. Keep `STATE.md` current at every checkpoint.

---

### 6 — Phase plan

Architecture depth lives in the spec; this section carries goals, deltas, and locks. Every lock is met only with executed evidence.

**Phase 0 — Scaffold, guard, ACP, embedding contract, CI** *(spec §7 Phase 0; kickoff v3 §4 for implementation detail)*
Build: library layout — `crates/holmes-core/`, `crates/holmes-guard/`, `docs/`, `tests/`, `.github/workflows/`. Knowledge files: canon mirrored byte-identically; derived files with markers preserved. The guard, three explicit layers, all policy in compiled Rust (none in TS/JS — a UI layer may *read* the list from the crate, never enforce it):
- **L1a — network egress allowlist.** A deny-by-default local egress proxy owned by `holmes-guard`; permitted hosts compiled/core-owned; everything else denied at the boundary. Every Holmes-spawned session is forced through it.
- **L1b — provider/model-id resolution guard.** Permitted provider and model ids pass; everything else — and *unknown* ids — rejected at resolution time.
- **L2 — sanitized spawn.** `goose acp` via the recorded absolute path; provider-selecting env vars stripped; `HTTP(S)_PROXY` injected to the L1a proxy and `NO_PROXY` cleared; permitted provider/model injected explicitly; resolved model id validated against L1b before and after the handshake.
Headless ACP round-trip harness: spawn via L2, handshake, one prompt to the smoke model, streamed response, assert the resolved id is L1b-permitted. Embedding contract: `holmes-core` public API + §6.2 artifact types (`research_brief` in; `evidence_pack`/`case_file` out — types and validation only); provider selection through a seam Alfred's onboarding UI can drive (the env/Ollama path is the seam's first consumer); no blueprint-type exports (structural); consumer test proving Alfred-shaped code links. Supply-chain CI per constitution #7 with the AC-DL-2 gate. Design `holmes-guard`'s API so Alfred can adopt it and retire `provider-lockdown.ts` — flag, don't modify Alfred. **L3 (provider-stripped goose distro) stays deferred — record, don't build.**
**Honest limits (into `docs/security.md`):** a user's own stock goose is theirs; AGPL forks can strip the guard — governance, not the binary, answers for forks; **a hostile tool binary that ignores proxy environment variables escapes the library-level network boundary — full network-level enforcement in the shipped artifact is an OS/Alfred-layer control, recorded as an Alfred obligation** (the human may open a D-## if that residual is unacceptable); never "unreachable on this machine," never "fork-proof."
**Locks:** (0a) **AC-DL-1 §§1–5 and §7** green hermetically in CI — config-override refusal, MCP-tool egress blocked at the proxy boundary (planted proxy-honoring test server), permitted-path positive control; **§6 scheduled to lock 2b — recorded as scheduled, not skipped.** (0b) **AC-DL-2, all seven**, positive and negative controls firing. Joint gate: all criteria applicable in-phase green in one CI run. (0c) ACP round-trip transcript on a provably permitted model. (0d) Embedding contract compiles; consumer test passes; no blueprint-type exports; Alfred's artifact-level guard test in the obligations ledger. (0e) CI green (SBOM, scanners, no Trivy, Actions SHA-pinned); knowledge files faithful; **CLAUDE.md protected-block rule enforced** — standing gates preserved verbatim at top, spec-derived operating context in a marked section beneath, diff shown at the checkpoint.

**Phase 1 — Analytical core** *(spec §2, §4.2)*
Three engines + six-phase case state machine as goose recipes/subagents: hypothesis objects, likelihood-ratio scorer, ACH matrix, Key-Assumptions-Check, devil's-advocate subagent, first-principles quarantine; Orchestrator-Worker runner with Evaluator-Optimizer critic. Schema amendment (epistemic canon, Upgrade B): `evidence_pack` carries `knowability` and structured `limits_of_this_finding` alongside `confidence`/`provenance`/validity — record as an A-## to §6.2. LR scores carry a **calibration status** now; gating lands in Phase 2.5.
**Locks:** (1a) ACH matrix + ≥2-independent-source corroboration gate enforced at emission — a finding without non-empty provenance and confidence ∈ [0,1] is rejected. (1b) One full analytical case end-to-end on the smoke model, inspectable transcript. (1c) Blueprint-type CI check green. (1d) **Recipe safety scan live from the first recipe** — invisible/deceptive-Unicode (ASCII-smuggling) strip in CI; planted fixtures neutralized.

**Phase 2 — The Wall** *(spec §3.1, §4.4)*
Self-hosted Graphiti temporal graph; **backend decision executed**: Neo4j or FalkorDB (Kuzu repo archived Oct 10 2025 — primary; driver deprecated), implemented as a Holmes-managed local service a non-developer never sees (install/run/health/recover). Adaptive RAG router; full bi-temporal Episode provenance. Re-verify FalkorDB/Neo4j packaging licenses live before bundling.
**Locks:** (2a) Every fact traceable to its episode; invalidation-not-deletion verified by test. (2b) **AC-DL-1 §6 lands here**: memory layer instantiated with no explicit LLM config does not reach `api.openai.com` — extraction/embeddings pinned to permitted clients (Anthropic/Gemini/Ollama), regression green. (2c) Tier-2 ingestion quality tested, not assumed — documented suite on the local small model with failure-rate evidence. (2d) Bundled backend supervised on a clean-machine profile. (2e) **AC-WP — weight provenance**: Tier-2 model downloads verified (checksum/signature/attestation) before load; fail closed on mismatch.

**Phase 2.5 — Safety before surface** *(hard gate: nothing in Phase 3 starts until every 2.5 lock is green)*
The three properties Holmes most needs — calibrated confidence, injection resistance on hostile fetched content, legal/defamation guardrails — ship *before* the collection surface they protect.
Build: (i) **CaMeL-style dual-LLM injection defense** — quarantined reader over fetched/untrusted content; the privileged planner never sees raw hostile bytes; capability-confined tool calls. (ii) **Calibration/knowability gating** — no bare high confidence in a low-`knowability` domain (emission blocked without the uncertainty statement); calibration fallbacks for LR outputs at the sovereign tier — a safety control. (iii) **Tool-approval protocol** — structured, previewable approval requests over ACP for every gated action, blocking until answered; rendering is Alfred's surface (obligation recorded), the protocol and blocking behavior are Holmes's, testable headlessly. (iv) **Legal/defamation guardrails** — handoff-only resolution enforced in code as the sole path (journalist/lawyer/community, never autonomous action); corroboration gate wired to emission; refusals for targeting private individuals; appeals hooks stubbed to the Phase 5 shape.
**Locks:** (2.5a) Hostile-content suite green — planted indirect-injection fixtures in fetched HTML/PDF/records fail to move the planner or exfiltrate, demonstrated. (2.5b) Knowability gate proven — high-confidence emission in a low-knowability fixture is blocked. (2.5c) Approval round-trip demonstrated headlessly. (2.5d) Anti-doxxing refusals tested against Blacksky's doxxing definition; handoff-only verified as the sole resolution path.

**→ BETA SCOPE DECISION (BLOCKED — record as a D-## for the human).** Present both cuts with evidence and stop:
**(A) Analytical open beta now** — beta surface is Phases 0–2.5; investigative mode compiled out or dark-flagged; proceed to Phase 4 then Phase RC; Phases 3 and 5 continue post-beta behind the same gates.
**(B) Full-surface beta** — continue Phases 3 → 4 → 5 → RC first.
Do not pick for the human.

**Phase 3 — Investigative mode** *(spec §4.3; only after 2.5, only per the scope decision)*
MCP servers for the public-records spine, OSINT, link analysis; OpenAleph; **Firecracker microVM (E2B OSS self-hosted) for any model-generated code, no outbound network by default**; bubblewrap only for trusted in-house read-only tasks. Re-verify at build time: E2B self-host status, OpenAleph maintenance (DARC fork), sandbox landscape, and the Vercel Sandbox GA month against the Vercel changelog (spec `[NEEDS-CAVEAT]` — resolve and queue the spec edit).
**Locks:** (3a) Every investigative tool gated behind the 2.5 layer — bypass-attempt test proves it. (3b) Surveillance-detection asymmetry enforced — power-scoped tools refuse private-individual targeting in tests. (3c) Sandbox suite: code exec offline-by-default proven; the Ona-class `/proc/self/root` bypass pattern specifically tested.

**Phase 4 — Observability & hardening** *(spec §7 Phase 4, minus what moved earlier)*
Born-redacted, opt-in, local-only telemetry (counts/durations/names — never content, prompts, secrets); deny-by-default permission manifest finalized; cross-stack trace correlation; smuggling regression (from Phase 1) stays green.
**Locks:** (4a) Telemetry provably content-free on captured payloads. (4b) Skillsmith-style security audit pass, findings triaged to zero criticals, logged in the ledger.

**Phase 5 — Accountability layer** *(spec §6.3, §7 Phase 5)*
NIP-32 non-destructive labeling (`kind:1985`, confidence metadata) with provenance; appeals — different-reviewer adjudication on a fixed SLA, the four bright-line non-appealables mirrored; Polis-assembly hook stubbed; local transparency log; CARE consent gating for community data.
**Locks:** (5a) Labels attach, never delete, carry provenance, queryable. (5b) Appeal round-trip with a second-reviewer role. (5c) Transparency log records every label + evidence + lineage. (5d) Interim-governance state documented honestly — the interim "block" is the human reviewer; no pretense the assembly exists.

**Phase RC — Open-beta release candidate** *(record as an A-## roadmap amendment to carry into the spec; "RC" avoids collision with the audit charter's own Phase 6)*
No Holmes-standalone UI, installer, or auto-update — the surface and distribution are Alfred's; Holmes's job is to be provably ready inside them.
Build/verify: **Ship-shape embedding** — Alfred imports `holmes-core` in a real build; the artifact-level guard test runs in Alfred's CI, evidenced by a human-provided run (until provided: UNVERIFIED → BLOCKED at RC); guard verified inside the built, signed artifact. **Non-developer first run** — clean-machine journey: install Alfred → Holmes present → graph backend self-installs/starts/recovers → first analytical case completes with zero terminal use; Holmes-side behavior verified headlessly, Alfred-side rendering recorded as obligation. **Sovereign path** — full offline Tier-2 journey; private queries never leave the machine, shown by an empty egress log. **Beta safety re-run** — the entire 2.5 suite + the AC-DL joint gate (incl. §6) + doxxing refusals re-executed from clean state in one sitting. **Model roster RC audit** — live re-verification of every configured model's price/SKU/endpoint (declared egress surface covers these fetches); DeepSeek dual-rate budgeting confirmed; Qwen3.7-Max Tier-1 only; alias audit re-confirmed post-2026-07-24. **Docs for beta users** — quickstart; what Holmes refuses and why; honest limits carried from `docs/security.md` (guard limits at L1+L2 incl. the out-of-process residual; forks answer to governance; proxied-excluded-model residual; **dual-use multiplied by distribution named plainly, never claimed structurally contained**); beta scope statement; feedback as **user-initiated export only** — no phone-home. **Compliance** — license notices per D-01 across crates; SBOM published per release; changelog + semver tag **proposed**, not pushed. **Fresh-eyes pass** — every beta journey walked as a first-time user; anything demo-embarrassing is a defect.
**Locks:** all of the above with executed evidence; RC readout emitted; rollback/update rides Alfred's updater (obligation if not yet real); **Rule 9 — tag/release/merge only on explicit go-ahead.**

---

### 7 — Phase checkpoint protocol (every phase boundary, and the RC)

**STOP.** Emit: (1) repo tree delta since the last checkpoint; (2) `LOOP.md`/`STATE.md` status — every lock with its command → output evidence; UNVERIFIED items with reasons; (3) knowledge-file faithfulness — marker spot-checks; the CLAUDE.md diff whenever it was touched (protected block intact); (4) verified vs assumed — anything unverifiable flagged, not fixed; (5) ledger delta — new F-### / A-## / D-## items; (6) Cross-repo obligations (Alfred) — opened / open / closed; (7) proposed next-phase locks (3–5, tee up only); (8) **landing mechanics** — open or refresh the phase PR to `main` (one PR per phase); merge only on explicit go-ahead; after any PR that lands canon, remind the human to re-sync the project connector; (9) "Rule 9 checkpoint reached. Nothing landed on main. Awaiting go-ahead."

---

### 8 — Facts ledger (carry with dates; re-verify where the consuming phase says so)

**Treat as fact:** goose is Apache-2.0 under AAIF / Linux Foundation — re-verified 2026-07-13 against `github.com/aaif-goose/goose`, the linuxfoundation.org press release (2025-12-09), and goose-docs.ai. ACP via the `goose acp` subcommand — no separate crate. Graphiti's documented default LLM/embedding client is OpenAI (verified 2026-07-06) — the reason AC-DL-1 §6 exists (Phase 2). Kuzu repo archived **Oct 10, 2025** (primary — repo banner); Graphiti's Kuzu driver deprecated; no zero-ops embedded backend survives; *Apple-acquisition attribution* **[DIRECTIONAL — secondary reporting of an EU filing]**. DeepSeek — **re-verified live 2026-07-13**: V4 Pro $0.435 / $0.87 (cache-hit $0.003625); `deepseek-chat`/`deepseek-reasoner` deprecate **2026-07-24 15:59 UTC**, both mapping to **V4 Flash**; pricing permanence **[NEEDS-CAVEAT — page states DeepSeek reserves the right to adjust; budget both rates]**. Qwen3.7-Max is the proprietary flagship (API-only, closed weights, Tier-1 only); sovereign line Qwen3.5-27B / Qwen3.6-35B-A3B / Magistral / Gemma; its context/pricing/launch-day details **[DIRECTIONAL]**. `litellm` routes to excluded providers — AC-DL-2 seed entry stands; its *GHSA-69fq-xp46-6x23 linkage* **[DIRECTIONAL — per the AC doc's source chain; verify the advisory opportunistically]**. Repo state 2026-07-13: `origin/main` 3 commits, top level {docs/, CLAUDE.md, LICENSE (Apache, pending D-01), README.md}; F-012 stale-sync episode; no releases.
**Confirm at build (cite when confirmed, then queue the spec edit):** goose secrets keyring default (`GOOSE_DISABLE_KEYRING`; `secrets.yaml` 0600 fallback) — spec §4.1 still carries `[NEEDS-CAVEAT]`; do not treat as fact until the goose source/docs lines are cited. Block-vs-AAIF config namespace: record as found.
**Re-verify at the consuming phase:** FalkorDB/Neo4j packaging licenses (Phase 2); E2B/Firecracker self-host, OpenAleph maintenance, sandbox landscape, Vercel Sandbox GA month (Phase 3); all model prices/SKUs/endpoints (Phase RC).

---

### 9 — Prohibitions

No success claims without executed evidence in the transcript. No scope expansion — log findings instead. No destructive or irreversible action to keep the loop moving. **No force-push or history rewrite anywhere. No direct push or local merge to `main` — landings via PR on explicit go-ahead only. Never resolve merge conflicts by discarding `origin/main`'s standing gates or seeded ledger entries. Never resolve a D-item yourself.** No suppressed warnings, skipped tests, or loosened gates — fix causes, not signals. No policy logic in JS/TS. No LLM inference inside any deterministic gate. No Trivy. No Meta/OpenAI/xAI anywhere in the dependency or model tree. **Author no license identifiers ahead of D-01; once ratified, author no identifiers other than the ratified license (Apache-2.0 dependencies unaffected).** Never author canon here. Never harden a caveat silently — either kind of label. Never claim the denylist is fork-proof or that misuse under mass distribution is structurally contained. Never pretend the assembly exists before it does.

## ▲ PASTE ABOVE THIS LINE ▲

*Canonical home: the Holmes repo. Companion: the 2026-07-13 second-pass audit report (candidate: `docs/audit/`). After each checkpoint readout is pressure-tested in the claude.ai project and reconciled, give explicit go-ahead to merge the phase PR and continue. Cross-reference: `holmes-spec-v2.md` (v2.1) §2–§8; `holmes-vs-wcjbt.md` §6; `triad-canon.md`; `epistemic-canon-Holmes.md`; `docs/acceptance/holmes-denylist-acceptance-criteria.md`; repo `CLAUDE.md` (standing gates).*
