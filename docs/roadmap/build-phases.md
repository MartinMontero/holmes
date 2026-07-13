# Build Phases — PROPOSED (pre-audit draft)

**Status: PROPOSAL.** No build-phases plan exists in any source document (F-003). This draft exists so the audit has a strategy to attack (Phase 6 audits build strategy); the audit's rewritten kickoff prompt supersedes this file. Do not treat as approved. Label: INFERRED throughout — derived from the blueprint's architecture, not stated in it.

Every phase boundary is a Rule 9 stop point: human review before the next phase starts.

## Phase A — Case file & audit *(current)*

- ✅ Case file landed: verified blueprint KB, provenance, triad context, audit charter, ledgers (this commit).
- ☐ Resolve D-01 (license) — gates everything.
- ☐ Run the Adversarial QA audit, Phases 0–7, against `docs/case-file/`.
- ☐ Land approved amendments; record D-decisions.
- **Exit criteria:** audit verdict is READY or READY WITH AMENDMENTS, and the Phase 6 kickoff prompt is committed.

## Phase B — Engineering spec

- Resolve the substrate contradiction (D-03): Goose extension vs. standalone agent vs. GooseClaw module.
- Specify: Graphiti deployment + bi-temporal schema for "The Wall"; Evidence Pack format (confidence score, knowability, provenance frontmatter); sandbox strategy for investigative scripts (Firecracker vs. alternatives given D-02's OS constraints); the six-phase investigative workflow as a state machine; NIP-32 label emission; the Sentinel Asymmetry guardrails as enforced policy, not prose.
- All dependencies checked against the licensing and vendor gates *concretely* (SBOM draft).
- **Exit criteria:** a competent stranger could build from the spec without asking questions.

## Phase C — Walking skeleton

- One end-to-end thread, smallest possible: intake a claim → run a bounded investigation → write facts to a self-hosted Graphiti instance → emit one cited Evidence Pack with a confidence score.
- CI from day one: lint, typecheck, tests, license scan, vendor-exclusion scan (the audit Phase 3 gate spec).
- **Exit criteria:** `/verify this claim` works once, honestly, with real provenance.

## Phase D — The investigative workflow, complete

- Implement all six runtime phases: Harm Check intake, La Lluvia hypothesis storm, parallel Collection (registries + OSINT), The Wall (ACH matrices, critique loops, bi-temporal invalidation), Following the Money (link analysis), strict Resolution & Handoff.
- Devil's Advocate and ACH as first-class, testable reasoning modes.
- **Exit criteria:** a full case file produced from a real brief, every fact carrying source + validity window.

## Phase E — Hardening & the Sentinel Asymmetry

- Sandboxed execution for generated investigative scripts (per D-02 decision).
- Anti-doxxing refusals enforced structurally (policy engine + tests that prove tools cannot be aimed at private citizens).
- Non-destructive truth verified: invalidation never deletes; chain of custody testable.
- NIP-32 signed labels with confidence scores and source URLs.
- **Exit criteria:** red-team pass against the Phase 5 audit personas.

## Phase F — Triad integration & governance

- Handoff interfaces: WCJBT blueprint → Holmes research brief; Holmes case file → Alfred (Epistemic Firewall, one-way); Alfred sourcing-anomaly briefs → Holmes.
- Workspace integration: findings as file frontmatter, `/verify` and `/run an ACH` commands, active-note-as-intake-brief.
- Community governance hooks (Polis assemblies / Blacksky model) — scope per audit outcome; likely last because it depends on a community existing.
- **Exit criteria:** defined by the audit's kickoff prompt, not this draft.
