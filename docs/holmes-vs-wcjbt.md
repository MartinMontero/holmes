# `docs/holmes-vs-wcjbt.md`

# Holmes vs. WCJBT (and Alfred): The Intent → Knowledge → Build Triad

> **Status:** Canon draft for Martin Montero / AOS. Companion to `holmes-spec-v2.md`. Convention carried from that document: unmarked claims are primary-source-verifiable; **[DIRECTIONAL]** marks design intent or figures not externally verifiable; **[NEEDS-CAVEAT]** marks a claim with a known caveat or unresolved conflict.

---

## 1. Executive summary — the triad thesis

The non-developer builder is not missing *capability* — AI agents supply that. They are missing two different things at once: a **clear, buildable intent**, and the **knowledge and reasoning** to know whether that intent is well-founded. WCJBT, Holmes, and Alfred each own exactly one of three distinct cognitive roles, and the central claim of this document is that these roles are non-overlapping:

- **WCJBT = intent → blueprint ("what to build").** It elicits the builder's intent through the Socratic Intent Engineering method and the PIE flow, then transforms it into a buildable artifact: a project constitution, a spec, an agent prompt, and a policy-clean starter repo drawn from its verified catalog. WCJBT is the **platform and judgment layer** — the catalog, the exclusion policy, and the enforcement engine that sit *above* the agent harness.
- **Holmes = knowledge + evidence + reasoning augmentation ("is this true / what do I need to know / what's the best-supported choice").** Holmes fills the builder's knowledge gaps with facts, specs, designs, case studies, and models, and augments the builder's own critical thinking and deductive reasoning. Its ethos: **"knowledge is power"** and **"the truth will set you free."** Holmes is the research-and-analysis brain defined in `holmes-spec-v2.md`.
- **Alfred = execution ("build it").** The agentic development-and-memory brain — the hands. Alfred takes a hardened blueprint and builds, under a human-in-the-loop discipline.

The triad is symbiotic, not merely co-located: WCJBT's elicited intent becomes Holmes's research brief; Holmes's verified evidence hardens the blueprint before it reaches Alfred; Alfred's build-time questions invoke Holmes; and Holmes's findings flow back into the WCJBT catalog as provenance-bearing labels. **Intent → knowledge → build → learning** is one loop, not three silos.

### The epistemic frame
"Knowledge is power" is most precisely attributed to Francis Bacon, whose *Meditationes Sacrae* (1597) contains "ipsa scientia potestas est" ("knowledge itself is power"); the exact Latin "scientia potentia est" first appears in Thomas Hobbes's *Leviathan* (1668 Latin edition), Hobbes having been Bacon's secretary as a young man. "The truth will set you free" originates in the Gospel of John (8:32 — "Then you will know the truth, and the truth will set you free"). These are used here as design ethos, not ornament: Holmes exists to make the builder's intent **survive contact with reality** — facts checked, assumptions surfaced, alternatives weighed. The lineage is Engelbart's 1962 *Augmenting Human Intellect: A Conceptual Framework*, which defined the goal as "increasing the capability of a man to approach a complex problem situation, to gain comprehension to suit his particular needs, and to derive solutions to problems." Holmes augments the builder's reasoning; it does not replace it.

These three roles also map cleanly onto a recognized disciplinary split. WCJBT does **requirements engineering / requirements elicitation** — "the practice of researching and discovering the requirements of a system from users, customers, and other stakeholders," where the term *elicitation* is deliberately chosen over *gathering* because "good requirements cannot just be collected from the customer." Holmes does **intelligence analysis / decision support** — the discipline formalized by Heuer and Pherson's *Structured Analytic Techniques for Intelligence Analysis*, whose techniques span "hypothesis generation and testing, cause and effect, challenge analysis... and decision support." That these are distinct, separately-literatured disciplines is the academic backbone of the non-redundancy claim: eliciting what someone wants to build is not the same activity as establishing what is true.

---

## 2. The layered model & responsibility matrix (Differentiation)

### 2.1 The layered architecture

WCJBT is not an agent. By its own documentation it is **the layer above the harness**: "We Can Just Build Things is the layer above the harness. It is the structured intent, the verified dependency catalog, the exclusion policy with enforcement teeth, and the PIE-anchored decision flow." A harness — goose, Claude Code, Aider — "executes. It doesn't decide, it doesn't verify, and it doesn't refuse." In WCJBT's own metaphor: "Goose is the drill. This is everything else. The harness is interchangeable; the orchestration layer is the product."

The WCJBT platform itself is **Astro + Starlight + Cloudflare Workers** (Starlight v0.40.0 per its page metadata), with **Nostr (NIP-07/NIP-46) + AT Protocol OAuth** authentication, an i18n governance system (source_commit freshness, security-gate CI, CODEOWNERS), and supply-chain hardening.

Holmes and Alfred are the **specialized agents that run on this layer**. Per `holmes-spec-v2.md`, both are built on **goose** over **Zed's Agent Client Protocol (ACP)** and tooled over **MCP**:

- **goose** is an open-source AI agent framework. Block released it under **Apache-2.0** in early 2025; per Paperclipped (Feb 12, 2026), "It serves as the reference implementation for MCP, which means if you want to see how MCP is supposed to work in a production agent, Goose is the example." It is now governed by the **Agentic AI Foundation (AAIF)** at the Linux Foundation — formed **December 9, 2025** ("SAN FRANCISCO, Dec. 9, 2025 – The Linux Foundation... today announced the formation of the Agentic AI Foundation (AAIF), and founding contributions of three leading projects... Anthropic's Model Context Protocol (MCP), Block's goose, and OpenAI's AGENTS.md"), whose Platinum members include AWS, Anthropic, Block, Bloomberg, Cloudflare, Google, Microsoft, and OpenAI.
- **ACP** (Agent Client Protocol) was, per Morph's ACP reference, "Created by Zed Industries and released in August 2025, it uses JSON-RPC 2.0 over stdin/stdout to create a universal interface between agents and editors." The editor spawns the agent as a subprocess and exchanges newline-delimited JSON; Google's Gemini CLI was the first external integration. ACP is the editor↔agent layer; MCP is the agent↔tool layer. Per Marc Nuri's reference, "Sessions are bootstrapped via session/new, which can declare the mcpServers the agent should connect to — so ACP and MCP wire up in a single handshake."
- **MCP** (Model Context Protocol) is the tool/data protocol, also an inaugural AAIF project.

So the stack is: **WCJBT platform (catalog + policy + intent flow)** → **goose/ACP substrate** → **Holmes (research agent) and Alfred (build agent), each tooled over MCP**.

### 2.2 Responsibility matrix

| Dimension | **WCJBT** | **Holmes** | **Alfred** |
|---|---|---|---|
| **One-line role** | Intent → blueprint | Knowledge + evidence + reasoning | Execution + memory |
| **Primary user-moment** | "I have a problem; what should I build?" | "Is this true? What do I need to know? Which choice is best-supported?" | "Build it." |
| **Inputs** | Builder's raw problem statement; answers to the Ten Questions | Intent brief / blueprint; open questions; a claim to verify; a decision to support | Hardened blueprint; build plan; recipes/skills |
| **Outputs** | Project constitution, spec, agent prompt, policy-clean starter repo, vetted component shortlist | Case file / evidence pack (findings + provenance + confidence), answers to research briefs, ACH matrices, risk flags | Working code, commits (gated), PKM/memory notes |
| **Core method** | Socratic Intent Engineering + PIE flow + catalog match + 3-layer enforcement | Three engines (abduction + Bayesian likelihood-ratio updating; Socratic + Structured Analytic Techniques; first-principles); six-phase case method | Agentic SDLC inside guardrails; spec-driven implement loop |
| **"Definition of done"** | A buildable, policy-clean blueprint the builder endorses | A claim resolved to a confidence-marked finding with cited provenance, or a decision supported by weighed evidence | Code that meets the spec, committed only with explicit human go-ahead |
| **Decision authority** | Owns the *what to build* decision and policy admission | Owns *what is true / best-supported* (evidence), never authors the blueprint | Owns *how it's implemented*, never makes a sourcing judgment |
| **Substrate** | Astro + Starlight + Cloudflare Workers platform | goose / ACP / MCP agent | goose / ACP / MCP agent |

### 2.3 Where each responsibility begins and ends
- **WCJBT begins** when a builder has a problem and **ends** when a blueprint exists. It decides *what to build* and *what is admissible*; it does not research open empirical questions or write the code.
- **Holmes begins** when there is a question, claim, or decision that needs evidence and **ends** when that question is resolved with cited provenance and a confidence mark. It never authors the blueprint and never builds.
- **Alfred begins** when there is a hardened blueprint and **ends** at committed, spec-meeting code. It never makes a sourcing judgment — when it hits one, it asks Holmes.

---

## 3. Overlap audit — "looks redundant → actually distinct"

Redundancy is a real risk because all three touch "planning," "specs," and "recommendations." Each apparent collision is resolved below by naming the decision owner, the evidence provider, and the hand-off artifact.

| # | Apparent overlap | Looks redundant because… | Actually distinct because… | Decision owner / provider / hand-off artifact |
|---|---|---|---|---|
| 1 | **WCJBT catalog vetting** vs. **Holmes "should I use dependency X?"** | Both assess whether a dependency is trustworthy. | WCJBT vetting is a **deterministic, policy-bound gate**: ownership (Meta/OpenAI/xAI exclusion), license-at-commit, CVE scan — a binary admit/reject on fixed rules. Holmes does **open-ended empirical investigation**: maintenance health, governance risk, design fit, alternatives, the *why*. WCJBT answers "does it pass policy?"; Holmes answers "is it actually a good idea, and what's the evidence?" | Owner: WCJBT owns admission. Provider: Holmes supplies evidence *into* the catalog. Artifact: evidence pack → catalog metadata/label. |
| 2 | **WCJBT blueprint/spec** vs. **Holmes case-file/spec output** | Both emit structured Markdown "specs." | WCJBT's spec is a **build contract** (the *what*: constitution, requirements, component list). Holmes's case file is an **evidence artifact** (the *is-it-true*: findings, sources, confidence, competing hypotheses). One tells the agent what to make; the other tells the builder what's known. | Owner: WCJBT authors the blueprint. Provider: Holmes's case file hardens it. Artifact: case file / evidence pack. |
| 3 | **WCJBT intent-elicitation/planning** vs. **Alfred planning** | Both produce a "plan." | WCJBT plans *what to build and why* (requirements engineering / intent). Alfred plans *how to implement* (task breakdown, file-level steps, the spec-driven implement loop). The boundary is the blueprint: above it is WCJBT; below it is Alfred. | Owner: WCJBT owns intent/requirements; Alfred owns implementation plan. Artifact: blueprint → build plan. |
| 4 | **Holmes research/analysis** vs. **WCJBT catalog curation & enforcement engine** | Both "evaluate tools." | The enforcement engine is **machine-deterministic and offline** — "it matches against a checked-in policy file, so the same input always produces the same result." Holmes is **probabilistic and investigative** — abduction, Bayesian updating, judgment under uncertainty. A parser never weighs a hypothesis; Holmes never hard-fails a build. | Owner: engine enforces; Holmes investigates. Artifact: Holmes findings → policy/label updates (human-reviewed). |
| 5 | **Holmes "what to build" suggestions** vs. **WCJBT intent** | Holmes could drift into proposing the product. | Holmes must **stay in its lane**: it supplies evidence and surfaces options/risks; it never decides the product direction. The builder + WCJBT own intent. Holmes can say "Option A has stronger evidence than Option B"; it cannot say "build A." | Owner: builder/WCJBT. Provider: Holmes. Invariant: Holmes never authors the blueprint (see §6.4). |
| 6 | **Alfred memory (PKM)** vs. **Holmes knowledge graph ("the wall")** | Both store knowledge. | Alfred's PKM is **build/project memory** (notes, code context, decisions made). Holmes's wall is a **self-hosted Graphiti temporal knowledge graph** holding *evidence with full provenance and validity windows* — bi-temporal facts that are invalidated, not deleted, each traceable to a source episode. One remembers the project; the other remembers the evidence and how it changed. | Owner: Alfred owns project memory; Holmes owns the evidence graph. Artifact: provenance-bearing findings. |

**Net:** Every collision resolves to the same boundary rule — **WCJBT decides what to build, Holmes establishes what's true, Alfred implements** — distinguished further by *determinism vs. judgment* (engine vs. Holmes) and *contract vs. evidence* (blueprint vs. case file).

---

## 4. Symbiosis design — the feedback loops

The triad is an organism because verified knowledge, intent, and execution circulate. Each loop below names the **hand-off artifact** and the **direction of the arrow**.

### Loop A — Holmes → WCJBT (findings improve the catalog)
Holmes investigates a dependency and finds, say, a governance risk or a stealth relicense. **Arrow:** Holmes → WCJBT catalog. **Artifact:** an *evidence pack* that updates catalog metadata. On the Nostr substrate this rides as **NIP-32 labels** (kind `1985`), which support "distributed moderation, collection management, license assignment, and content classification." Per NIP-32, a label tag MAY carry a JSON-encoded 4th element where "quality may have a value of 0 to 1... confidence may have a value of 0 to 1. This indicates the certainty which the author has about their rating," and "support is an array of URLs and/or Nostr ids with information to justify the labeling." This is the non-destructive labeling backbone derived from the Blacksky community-accountability model — a risky dependency Holmes flags becomes a labeled, provenance-bearing entry rather than a silent deletion.

### Loop B — WCJBT → Holmes (catalog grounds/seeds research)
Before searching the open web, Holmes **searches the catalog first**, cites it, and treats it as a trusted corpus. WCJBT's own quickstart guidance makes this explicit for agents: "Don't ask the agent 'which library should I use for Nostr?' — ask the catalog. Every entry was license-verified at commit and screened against the exclusion policy." **Arrow:** WCJBT catalog → Holmes intake. **Artifact:** catalog entries as seed evidence (with their existing license/verification provenance). This grounds Holmes's research in an already-vetted body and prevents it from re-deriving what the platform already verified deterministically.

### Loop C — WCJBT → Holmes → Alfred (intent becomes brief, evidence hardens blueprint)
WCJBT's elicited intent/blueprint becomes **Holmes's research brief**. Holmes returns evidence that **hardens the blueprint** — confirming choices, flagging risks, supplying missing facts/specs/case studies — before it goes to Alfred to build. **Arrows:** WCJBT → Holmes (research brief) → WCJBT/Alfred (hardened blueprint). **Artifact:** research brief in, evidence pack out, hardened blueprint forward.

### Loop D — Alfred → Holmes (build-time questions invoke Holmes)
When Alfred hits a question it cannot resolve by sourcing judgment ("which crypto library is actually maintained?", "is this API deprecated?"), it **does not guess** — it invokes Holmes. **Arrow:** Alfred → Holmes (question) → Alfred (cited answer). **Artifact:** a scoped research brief and a confidence-marked answer.

### Loop E — the full intent → knowledge → build → learning loop
1. Builder + **WCJBT** elicit intent → blueprint.
2. **Holmes** fills knowledge gaps, weighs options, hardens the blueprint (evidence pack).
3. **Alfred** builds (gated commits) → project memory.
4. Build-time learnings + Holmes's verified findings flow back into the **WCJBT catalog** (labels/provenance) and into **Holmes's wall** (temporal facts with validity windows), improving the next cycle.

```
        ┌─────────────────────────────────────────────────────────┐
        │                     WCJBT (platform)                     │
        │   intent-elicitation · catalog · policy · enforcement    │
        └─────────────┬───────────────────────────▲───────────────┘
        blueprint /   │ research brief             │ evidence pack /
        seed corpus   │ (Loops C, B)               │ NIP-32 labels (Loop A)
                      ▼                            │
                ┌──────────┐   question (Loop D)   │
                │  HOLMES  │◄──────────────┐       │
                │ evidence │               │       │
                │ + reason │──────────────┐│       │
                └────┬─────┘  cited answer ││       │
       hardened      │                    ▼│       │
       blueprint     ▼                ┌────┴────┐   │
                  ───────────────────►│ ALFRED  │───┘ learnings
                                      │  build  │
                                      └─────────┘
```

---

## 5. The in-workspace integration map — Holmes inside the builder's environment

Holmes is **not a separate destination**. It is woven into the WCJBT/Alfred editing environment — a Tauri 2 + SolidJS local-first workspace **[DIRECTIONAL]** (see §7 caveat on Alfred sourcing; the parent `derekross/onyx` confirms a "Tauri 2.0 — Rust-based desktop framework · SolidJS — Reactive UI framework · CodeMirror 6 — Text editor" stack, an "Integrated AI Assistant," and a "Skills System"). The mapping below shows, feature by feature, what Holmes contributes and how it stays distinct from WCJBT's intent/blueprint role and Alfred's build role.

| Workspace feature (shortcut) | What it already does | What **Holmes** contributes | Boundary preserved |
|---|---|---|---|
| **Integrated AI Assistant** (`Ctrl+\``) | Context-aware chatbot for writing, editing, research | Holmes's three engines and six-phase case method become **invokable modes**: "verify this," "run an ACH," "key-assumptions check," "first-principles this." | Assistant ≠ blueprint author. Holmes answers/evidences; WCJBT still owns intent; Alfred still owns code. |
| **Skills System** | Extensible capabilities (doc creation, research, more) | Holmes ships as **skills**: each engine and case-phase is a scoped, invokable skill (e.g., `holmes.collection`, `holmes.the-wall`, `holmes.ach`). Mirrors WCJBT's existing "agent skills" pattern that "encode real engineering discipline into repeatable workflows." | Skills are scoped workflows with stop-and-ask triggers; a Holmes skill emits evidence, never commits code. |
| **File Context** | AI reads/understands the current note | Holmes uses the current note as **intake** — the claim/brief/decision under investigation comes from what the builder is writing. | Read for evidence, not to author the plan. |
| **Note graph: Backlinks (`Ctrl+Shift+B`), Outline (`Ctrl+Shift+O`), wikilinks, Link Unlinked Mentions** | Cross-references and structure | The note graph **is Holmes's working memory — "the wall."** Backlinks/wikilinks/outline become the visible projection of the Graphiti evidence graph; "Link Unlinked Mentions" converts raw mentions into `[[wikilinks]]` that bind evidence nodes together. | The wall holds *evidence*; project memory (Alfred) and intent docs (WCJBT) remain distinct note classes. |
| **Properties Panel** (YAML frontmatter, `Ctrl+Shift+P`) | Structured note metadata | Holmes attaches **evidence/citation provenance** as frontmatter: source URLs, retrieval date, confidence (0–1), validity window, hypothesis status. The note-local mirror of NIP-32 label metadata. | Provenance lives in frontmatter; the blueprint's requirements live elsewhere. |
| **Daily Notes (`Ctrl+D`), Templates (`Ctrl+T`), Slash Commands (`/`)** | Capture, scaffolding, insert headings/lists/callouts/tables | Holmes templates scaffold a **case file** (Intake → La Lluvia → Collection → The Wall → Following the Money → Resolution & Handoff); slash commands insert evidence callouts and ACH tables. | Templates produce case files (evidence), not constitutions/specs (WCJBT) or task lists (Alfred). |

**Why this is the decisive plane:** differentiation and symbiosis both have to hold *in the same editor*. They hold because the three write **different note classes** with different "definitions of done": WCJBT writes intent/blueprint docs, Holmes writes provenance-bearing case files, Alfred writes code and project memory. The note graph lets them reference each other (symbiosis) without merging roles (differentiation).

---

## 6. The boundary / interface contract (implementable)

This section is written so Claude Code can act on it.

### 6.1 Role surfaces — accepts / emits / must-NOT

**WCJBT**
- **Accepts:** raw problem statements; Ten-Questions answers; a pasted dependency/stack to check; evidence packs from Holmes (to update catalog).
- **Emits:** project constitution, spec, agent prompt, policy-clean starter repo, vetted component shortlist; deterministic enforcement verdicts (admit/reject + reason); NIP-32-style catalog labels.
- **MUST NOT:** assert an unverified empirical claim as true (it states policy facts and catalog provenance, not research conclusions); write application code; author an implementation/task plan.

**Holmes**
- **Accepts:** a research brief (from WCJBT intent or an Alfred build-time question); a claim to verify; a decision to support; a note as File Context.
- **Emits:** case file / evidence pack — findings, full provenance, confidence marks, ACH matrices, key-assumptions checks, risk flags; cited answers; temporal facts written to the wall.
- **MUST NOT:** author the blueprint or decide the product direction; commit or build code; assert a finding without provenance and a confidence mark; act as surveillance (it is surveillance-*detection*, not surveillance).

**Alfred**
- **Accepts:** a hardened blueprint; a build plan; recipes/skills; cited answers from Holmes.
- **Emits:** code, gated commits, project/PKM memory notes, build-time questions.
- **MUST NOT:** make a sourcing/evidentiary judgment itself (it asks Holmes); admit a dependency that violates the WCJBT denylist; commit or push without explicit human go-ahead (Rule 9 **[DIRECTIONAL]**, per `holmes-spec-v2.md`/Alfred context).

### 6.2 Hand-off artifacts (shapes)

```yaml
# intent_brief  (producer: WCJBT  ·  consumer: Holmes)
intent_brief:
  problem: string            # the community problem, in plain language
  for_whom: string
  ten_questions: map         # Socratic Intent Engineering answers
  constraints: [string]      # e.g. local-first, denylist, jurisdiction
  open_questions: [string]   # what WCJBT could not resolve → Holmes brief

# blueprint  (producer: WCJBT  ·  consumer: Alfred, hardened via Holmes)
blueprint:
  constitution: string       # non-negotiable project principles
  spec: string               # the "what", not the "how"
  components: [catalog_ref]   # vetted, policy-clean shortlist
  agent_prompt: string
  policy_state: {denylist_clean: bool, licenses_checked_at_commit: bool}

# research_brief  (producer: WCJBT or Alfred  ·  consumer: Holmes)
research_brief:
  question: string           # claim to verify / decision to support
  origin: enum[intent, build_time]
  scope: string
  catalog_seed: [catalog_ref] # Loop B: search catalog first

# evidence_pack / case_file  (producer: Holmes  ·  consumer: WCJBT, Alfred, builder)
evidence_pack:
  question: string
  findings:
    - claim: string
      confidence: float        # 0..1
      provenance: [url]        # named sources, verbatim quotes
      valid_from: date
      valid_until: date|null   # bi-temporal; superseded not deleted
  competing_hypotheses: [..]   # ACH matrix
  key_assumptions: [..]
  risk_flags: [..]
  recommendation: string|null  # options/risks only — never "build X"

# build_plan  (producer: Alfred  ·  consumer: human + Alfred)
build_plan:
  tasks: [{step, files, depends_on, parallelizable}]
  commit_gate: human_go_ahead   # Rule 9
```

### 6.3 Interaction protocols over ACP/MCP
- Holmes and Alfred each run as **ACP agents** (goose over ACP; JSON-RPC 2.0 over stdio) inside the workspace; tools are exposed via **MCP**, declared at `session/new`.
- **WCJBT → Holmes:** the platform hands an `intent_brief`/`research_brief` (Markdown artifact) into a Holmes ACP session; Holmes returns an `evidence_pack`.
- **Alfred → Holmes (Loop D):** Alfred, on hitting a sourcing question, opens (or messages) a Holmes session with a `research_brief`; receives a cited answer; resumes. Alfred never resolves the question itself.
- **Holmes → WCJBT (Loop A):** Holmes emits NIP-32 labels (kind 1985, with `confidence`/`support`) that update catalog metadata, gated by human review before any policy effect.
- **Human-in-the-loop (Rule 9):** Alfred's `commit_gate` requires explicit human go-ahead before any commit/push. The **denylist** (Meta/OpenAI/xAI excluded; Google permitted; open-weights-on-permitted-infra permitted) is enforced by WCJBT's engine and respected by both agents' model selection (Holmes's two-tier model strategy — Tier-1 frontier: Claude, Gemini 3.1 Pro, permitted open-weight via API; Tier-2 sovereign: non-Meta open weights — per `holmes-spec-v2.md`).

### 6.4 Non-redundancy invariants (tests Claude Code can implement)
1. **Holmes never authors the blueprint.** Test: no Holmes output artifact has type `blueprint`/`constitution`/`spec`; Holmes emits only `evidence_pack`/`case_file`. CI fails if a Holmes skill writes to the blueprint path.
2. **WCJBT never asserts an unverified fact as true.** Test: catalog claims carry either a deterministic policy verdict (license/ownership/CVE) or a Holmes-sourced label with `provenance` + `confidence`; no free-text empirical assertion without one of these.
3. **Alfred never makes a sourcing judgment.** Test: Alfred, on encountering an unresolved evidentiary question, must emit a `research_brief` to Holmes rather than a decision; CI/lint flags any Alfred "I'll assume…" on a sourcing matter.
4. **Determinism vs. judgment separation.** Test: the enforcement engine has no probabilistic/LLM call; Holmes findings never hard-fail a build directly — they update labels/blueprint, and only the deterministic engine gates commits.
5. **Every Holmes finding is provenance-bearing.** Test: each `findings[]` entry has non-empty `provenance` and a `confidence` in [0,1]; else reject.
6. **Rule 9 holds.** Test: no commit/push path exists that bypasses `commit_gate: human_go_ahead`. **[DIRECTIONAL]** pending Alfred source confirmation.

---

## 7. Open questions / risks

- **Alfred sourcing is unverifiable from public primary sources. [NEEDS-CAVEAT]** The repository `github.com/MartinMontero/Alfred` is not publicly accessible/indexed (not present among the owner's visible "popular repositories," and the full repo listing is robots-disallowed to automated fetch). The Alfred-specific facts in this document — Tauri 2 + SolidJS, ~11 path-confined MCP tools, goose-via-ACP, the provider denylist, and "Rule 9" — are carried from `holmes-spec-v2.md`/the supplied context and are marked **[DIRECTIONAL]**. Independently, the cited parent project **`derekross/onyx` is MIT-licensed, not AGPL-3.0** (its README's License section reads simply "MIT"), though it does confirm the **Tauri 2.0 + SolidJS + CodeMirror 6 + nostr-tools** stack plus an "Integrated AI Assistant" and "Skills System." The "relicensed from onyx under AGPL-3.0" claim should be re-verified against the actual Alfred LICENSE file before publication — note that an MIT parent *can* legally be extended under AGPL-3.0, so the relicense is plausible, but the premise that onyx is AGPL is incorrect.
- **WCJBT enforcement-engine specifics. [NEEDS-CAVEAT]** The public docs describe a **three-layer TypeScript enforcement engine** (`enforcement/cli.ts`) that runs in CI on every PR and weekly; Layer 2 walks the lockfile graph across **13 formats** (npm, pnpm, yarn classic & Berry, Cargo, uv, Poetry, pip-compile, Go modules, Bundler, Hex, pub, Gradle); Layer 1 reads manifests across 8+ ecosystems; the engine has a **40+-test** suite. The figures "~2,727 lines" and "~18 parsers" are **[DIRECTIONAL]** — not directly verifiable from the published pages. Supply-chain hygiene is confirmed: **Syft** SBOMs (CycloneDX + SPDX), **OSV-Scanner** (primary gate) + **Grype** (independent cross-check), and a deliberate exclusion of **Trivy** — "its popular action was compromised in a March 2026 supply-chain attack (CVE-2026-33634), force-pushed to credential-stealing malware."
- **Catalog size. [DIRECTIONAL]** The site states "1,300+ verified tools"; the precise "~1,355 YAML entries" is not directly confirmed on the public catalog page.
- **Redundancy drift risk (ongoing).** The most likely future collision is **Holmes drifting into product recommendations** (violating invariant #1/#5) or **WCJBT's recommendation surface asserting research conclusions** (violating invariant #2). Monitor by auditing artifact types emitted per agent and asserting the §6.4 invariants in CI.
- **Label-trust risk.** NIP-32 labels carrying Holmes findings into the catalog must be human-reviewed before policy effect, or a single mis-weighted `confidence` could degrade catalog trust. Keep the deterministic engine as the only commit gate.
- **Provider-denylist consistency.** Holmes's Tier-1 model use must stay consistent with the WCJBT denylist; a model-selection regression would breach the shared constraint. Cross-check at model-config CI. (Note: goose itself ships with broad provider support including excluded vendors, so the denylist must be enforced at the WCJBT/agent configuration layer, not assumed from the substrate.)

---

*Cross-reference: `holmes-spec-v2.md` (canonical Holmes definition — three engines [abduction + Bayesian likelihood-ratio updating; Socratic engine + Structured Analytic Techniques — ACH, Key Assumptions Check, devil's advocate; first-principles], six-phase case method [Intake → La Lluvia → Collection → The Wall → Following the Money → Resolution & Handoff], self-hosted Graphiti temporal-knowledge-graph wall with full provenance, two-tier model strategy, the digital incarnation of Santos Reyes, the "method is the identity" thesis, the Blacksky-derived community-accountability + non-destructive labeling backbone, and surveillance-detection-not-surveillance).*