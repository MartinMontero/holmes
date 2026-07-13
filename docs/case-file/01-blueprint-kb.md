# 01 — The Holmes Blueprint, Distilled

Source: "Holmes: The AI Detective.pdf" (see `00-provenance.md`). Every claim below is labeled. `VERIFIED (blueprint)` means *verified to be what the blueprint says* — not verified to be externally true. External truth-testing is audit Phase 1.

## Identity and role — VERIFIED (blueprint)

- Holmes is "The Detective" of the Non-Dev Builder OS triad: WCJBT (The Architect, "What should we build?"), Holmes (The Detective, "What is true?"), Alfred (The Builder, "How do we execute it?").
- Role: "Intelligence Analysis & Decision Support." The research, evidence, and reasoning brain; fills knowledge gaps and verifies assumptions.
- Ethos: "Traced from Francis Bacon to Thomas Hobbes — to survive contact with reality, facts must be checked and alternatives weighed."
- Core framing: "Standard AI predicts the next word… will confidently build your software on an abandoned, vulnerable database just because you asked." Holmes exists because "an unverified assumption is a fatal vulnerability."

## System invariant — VERIFIED (blueprint, quoted verbatim)

> "Holmes never authors the blueprint. Holmes never writes application code. It only observes, deduces, and supplies verifiable evidence."

Division of labor: Holmes = "The Analyst" (probabilistic investigation; evaluates maintenance health, governance risk, design fit; pauses explicitly to generate chain-of-thought analysis). Alfred = "The Operative" (deterministic execution). Rationale: "When a system fails, engineers know instantly if it was a failure of reasoning (flawed plan) or execution (faulty code)."

## Substrate — VERIFIED (blueprint)

- "Built on the open-source Goose framework (Agentic AI Foundation)."
- **UNKNOWN:** whether Holmes communicates with the rest of the triad via ACP or any specific protocol. "ACP" appears nowhere in the blueprint; it entered earlier working notes without a source. Research item for the audit.

## Reasoning methods — VERIFIED (blueprint)

- **Devil's Advocate:** "actively searching for evidence of why a chosen database will fail" — Holmes attempts to disprove ideas before endorsing them.
- **Analysis of Competing Hypotheses (ACH):** testing different strategies simultaneously, run against an internal scratchpad.
- **Output: Evidence Packs** carrying a mathematical confidence score and a knowability rating (blueprint example: `CONFIDENCE SCORE: 0.95 | KNOWABILITY: HIGH`).

## The Investigative Workflow (six phases) — VERIFIED (blueprint)

1. **Intake — The Harm Check.** Assesses the brief; determines if Holmes can help without creating systemic harm.
2. **La Lluvia — Hypothesis Storm.** Generates a massive storm of hypotheses and defines the exact negative evidence needed to disprove them.
3. **Collection.** "Documents State of Mind" — assumes a document exists for every claim; launches parallel searches through public registries and OSINT.
4. **The Wall.** Maps findings spatially; runs ACH matrices and critique loops to synthesize raw data into a provenanced case file.
5. **Following the Money.** Deep link analysis: detecting layering, mapping corporate structures, identifying shared registered addresses.
6. **Resolution & Handoff — STRICT.** "Holmes takes no autonomous action. It securely routes a fully traceable, completed case file to the builder."

## Memory: "The Wall" as a temporal knowledge graph — VERIFIED (blueprint)

- Motivated by "The Amnesia Problem": standard AI jumbles or forgets facts across multi-day sessions.
- "The Graphiti Solution: Holmes uses a self-hosted temporal graph. Every node is bi-temporal" — facts carry validity windows (e.g., `[VALID: 2025-2026] [SOURCE: OPEN_CORPORATES]`).
- **Non-Destructive Truth:** "Facts are never silently deleted. When new evidence emerges, old facts are flagged as invalidated but preserved, creating a perfect chain of custody."
- **UNKNOWN:** Graphiti version, storage backend, schema, hosting story — the blueprint names the tool only.

## Execution safety: microVM vaults — VERIFIED (blueprint); external claims UNVERIFIED

- Risk framing: "AI writing code to scrape the web is inherently risky. A hijacked script can compromise a developer's machine."
- Defense: "Any investigative script Holmes generates runs natively inside a disconnected microVM vault. If the code goes rogue or the system crashes, the vault simply vanishes."
- Blueprint cites "AWS Firecracker Paper (2019) & Independent Benchmarks" for "<1ms startup times" — **UNVERIFIED externally**; audit Phase 1 item (the Firecracker paper is real but the specific latency claim needs checking against it).

## The Sentinel Asymmetry — VERIFIED (blueprint)

- "Built to investigate power, strictly prohibited from surveilling people."
- Native programmatic access to corporate registries: OpenCorporates ("230M+ entities" — **UNVERIFIED externally**) and PACER/court records.
- "Anti-doxxing refusals permanently block tools from being aimed at private citizens. Holmes detects surveillance; it does not surveil."

## Community accountability — VERIFIED (blueprint)

- "I answer to the block."
- **Cryptographic provenance:** Holmes updates dependency catalogs using decentralized **NIP-32 labels**; every risk flag carries a cryptographically signed confidence score and source URL.
- **Governance:** the "Blacksky model" — Polis community assemblies democratically govern investigative policies, vocabularies, and off-limits topics, in contrast to "a corporation silently deleting a tool from a registry."

## Workspace integration — VERIFIED (blueprint)

- Holmes is "an integrated intelligence layer inside your workspace," not a separate browser tab.
- Findings attach to the top of files (frontmatter-style: confidence score, knowability, provenance URL).
- Builder invokes native commands like **`/verify this claim`** or **`/run an ACH`** in the chat panel.
- "The active note the builder is writing automatically becomes the intake brief for Holmes's investigation."

## Loop E: the human stays the creator — VERIFIED (blueprint)

Intuitive Leap (human) → Rigorous Verification (Holmes) → Verified Reality & Build (Alfred). "Verified knowledge compounds… knowledge repeated with fast feedback hardens into fast instinct." The triad exists to augment the creator, not replace them.

## What the blueprint does NOT contain — ABSENT (checked 2026-07-13)

- No build plan, milestones, component ordering, or ship dates. The six phases above are the *runtime* investigative workflow, not a construction sequence.
- No engineering detail: no versions, APIs, data models, deployment targets, resource budgets, or test strategy.
- No threat model, no cost model, no team/ownership model.
- No mention of ACP, Firecracker orchestration details, or how Goose/Graphiti/microVMs compose into one system.
