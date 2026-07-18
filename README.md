# Holmes — The Detective of the Non-Dev Builder OS

> *"Building sovereign software with AI that knows the exact difference between a hallucinated guess and a verified fact."* — Holmes blueprint, June 2026

Holmes is the research, evidence, and reasoning brain of the Non-Dev Builder OS triad:

| Role | Component | Question it owns |
|---|---|---|
| The Architect | WCJBT (wecanjustbuildthings.dev) | *What should we build?* |
| **The Detective** | **Holmes (this repo)** | ***What is true?*** |
| The Builder | Alfred | *How do we execute it?* |

Holmes investigates dependencies, verifies claims, and flags risks. It produces **Evidence Packs** — fully cited findings with mathematical confidence scores and knowability ratings. It operates under one system invariant, quoted from the blueprint:

> Holmes never authors the blueprint. Holmes never writes application code. It only observes, deduces, and supplies verifiable evidence.

## Project status: SPEC'D, PRE-PHASE-0 — no code yet

This repository contains **no code yet**, but it now holds the canonical documents:

- **`docs/holmes-spec-v2.md`** — the authoritative build reference (QA-corrected v2, verification date 2026-06-29): goose/ACP substrate, the three analytical engines + six-phase case method, Graphiti "Wall," two-tier model strategy, the Blacksky-derived accountability layer, and the **Phase 0–5 build roadmap**. The repo copy is the single source of truth.
- **`docs/holmes-project-orientation.md`** — the project map and operating loop (Claude Code builds and emits readouts; the claude.ai project pressure-tests them against the spec).
- The **case file** (below) — provenance, the blueprint knowledge base, and the audit ledgers.

The build starts when Phase 0 runs from its kickoff prompt (`holmes-claude-code-kickoff-phase0-v2.md` — not yet committed, see F-009) with explicit human go-ahead, per Rule 9.

## The case file

```
docs/
  holmes-spec-v2.md                  CANONICAL build spec (source of truth)
  holmes-project-orientation.md      Project map, custom instructions, build loop
  research/
    wisdom-intuition-knowledge-judgment-v2.md   Epistemology map (analytical-core design input)
  case-file/
    00-provenance.md     Where every source document lives, and when it was verified
    01-blueprint-kb.md   The Holmes blueprint deck, distilled, with epistemic labels per claim
    02-triad-context.md  What the sibling blueprint decks say about Holmes's role and constraints
  audit/
    00-audit-charter.md  The Adversarial QA & Production-Readiness Audit, instantiated for Holmes
    findings-ledger.md   F-### findings (seeded; several resolved by the spec landing)
    amendments.md        A-## amendments (empty until the audit runs)
    decisions.md         D-## decisions reserved for the human (D-01/D-05…D-08 decided; D-02–D-04 open)
  roadmap/
    build-phases.md      SUPERSEDED pointer → the real roadmap is spec §7 (Phases 0–5)
```

## Standing gates

All work in this repo is governed by the constitution in [CLAUDE.md](CLAUDE.md): zero fabrication with epistemic labels, evidence-or-it-didn't-happen, the vendor denylist (no Meta, OpenAI, or xAI — direct, transitive, or as model weights; Google permitted), Rule 9 (consent before consequence), RPI, path-confined tools, born-redacted telemetry, supply-chain hygiene (no Trivy, SHA-pinned Actions), and surveillance-detection-not-surveillance.

**License (D-01, decided 2026-07-18):** Holmes is licensed **AGPL-3.0-or-later**, matching Alfred — the LICENSE file carries the GNU AGPL v3 text, and the "or any later version" option applies per the notice convention in its How-to-Apply section. Ratified by the human at Gate Zero with `Alfred/LICENSE` quoted as evidence (`docs/audit/decisions.md` D-01; resolves F-001).

## Next steps

1. ~~Decide D-01 (license) and D-05 (smoke-test mode)~~ — both decided 2026-07-18 at Gate Zero (`docs/audit/decisions.md`).
2. Stage the still-missing upstream artifacts: the 2026-07-13 second-pass audit report (→ `docs/audit/`), `holmes-spec-v2.1-diff.md` (F-011/F-014), and `holmes-claude-code-kickoff-phase0-v2.md` (F-009) if it remains relevant alongside the master build loop.
3. Optionally run the fresh adversarial audit (`docs/audit/00-audit-charter.md`) — D-04, still open.
4. Continue the **Master Build Loop v2** (`docs/prompts/holmes-master-build-loop-v2.md`): Phase 0 build begins at the next session with a provider key in the environment; every phase stops at its Rule-9 checkpoint and emits a readout for pressure-testing.
