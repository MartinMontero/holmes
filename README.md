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

## Project status: CASE FILE OPEN — nothing built yet

This repository currently contains **no code**. What it contains is the case file: the verified knowledge base distilled from the design blueprints, the adversarial audit charter that gates the build, and the ledgers that every finding, amendment, and human decision will be recorded in.

The build does not start until the audit (see below) produces a verdict and a kickoff prompt. That is deliberate — it is Holmes's own methodology applied to Holmes.

## The case file

```
docs/
  case-file/
    00-provenance.md     Where every source document lives, and when it was verified
    01-blueprint-kb.md   The Holmes blueprint, distilled, with epistemic labels per claim
    02-triad-context.md  What the sibling blueprints say about Holmes's role and constraints
  audit/
    00-audit-charter.md  The Adversarial QA & Production-Readiness Audit, instantiated for Holmes
    findings-ledger.md   F-### findings (seeded with pre-audit findings)
    amendments.md        A-## amendments (empty until the audit runs)
    decisions.md         D-## decisions reserved for the human (seeded)
  roadmap/
    build-phases.md      PROPOSED build phasing — a pre-audit draft, replaced by audit Phase 6 output
```

## Standing gates

All work in this repo is governed by the gates in [CLAUDE.md](CLAUDE.md): zero fabrication with epistemic labels, evidence-or-it-didn't-happen, the AGPL/GPL licensing gate, the vendor gate (no Meta, OpenAI, or xAI — direct or transitive), Rule 9 (consent before consequence), and RPI (Research → Plan → Implement).

⚠️ **Known gate violation at repo creation:** the current LICENSE is Apache-2.0, which conflicts with the licensing gate. See finding F-001 and decision D-01 — relicensing is a human call and has not been made yet.

## Next step

Run the audit: open a claude.ai project with `docs/case-file/` as the knowledge base, paste `docs/audit/00-audit-charter.md` as the first message, and reply `GO` phase by phase. Phase 7 produces the go/no-go verdict; Phase 6 produces the kickoff prompt that starts the actual build.
