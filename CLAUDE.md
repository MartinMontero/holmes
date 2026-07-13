# CLAUDE.md — Standing orders for any agent working in this repository

This repo is the case file and (eventually) the implementation of **Holmes**, the evidence-and-verification agent of the Non-Dev Builder OS. Until the adversarial audit produces a READY verdict, this repo is documentation-only: do not scaffold application code.

## Standing gates (non-negotiable)

1. **Zero fabrication.** Every external factual claim gets a primary source with a date, or an explicit `UNVERIFIED` tag. Never fill gaps with plausible detail.
2. **Epistemic labels** on all claims in any document you write here:
   - `VERIFIED` — cite the source and the date checked
   - `INFERRED` — show the reasoning
   - `ASSUMED` — flag it
   - `UNKNOWN` — becomes a research item
3. **Evidence or it didn't happen.** Every finding quotes its source (file + section) or states `ABSENT` explicitly. Distinguish "I checked and found nothing" from "I didn't check."
4. **Licensing gate.** Target posture is AGPL-3.0-or-later or GPL-3.0, end to end, including dependency compatibility. *The current LICENSE (Apache-2.0) violates this gate — see D-01; do not relicense without explicit human approval.*
5. **Vendor gate.** No Meta, OpenAI, or xAI anywhere — direct or transitive: SDKs, models, APIs, infra. Google is permitted. Violations are Blockers.
6. **Rule 9 — consent before consequence.** Proposals until approved. Nothing destructive or irreversible (relicensing, force-pushes, deletions, publishing, spending) without explicit human go-ahead.
7. **RPI.** Research → Plan → Implement, in that order, every time.

## Holmes product invariants (from the blueprint — apply once code exists)

- Holmes never authors the blueprint and never writes application code for the builder; it only observes, deduces, and supplies verifiable evidence.
- Holmes takes no autonomous action at handoff: it routes a fully traceable case file to the builder (Phase 6, "Resolution & Handoff — STRICT").
- The Sentinel Asymmetry: Holmes investigates corporate power (registries, court records); anti-doxxing refusals permanently block its tools from being aimed at private citizens. Holmes detects surveillance; it does not surveil.
- Non-destructive truth: facts are never silently deleted. Superseded facts are flagged invalidated and preserved.

## Repo conventions

- Findings: `F-###` in `docs/audit/findings-ledger.md`, format:
  `F-### | Severity | Category | Location | Evidence (quote or ABSENT) | Why it matters | Recommended fix | Confidence (H/M/L)`
- Severities: BLOCKER / MAJOR / MINOR / NIT (definitions in `docs/audit/00-audit-charter.md`).
- Amendments: `A-##` in `docs/audit/amendments.md`, each mapping to the finding(s) it resolves.
- Human decisions: `D-##` in `docs/audit/decisions.md`. Never resolve a D-item yourself.
- IDs are stable for the life of the audit. Do not renumber.

## Tone

Terse. No hyperbole, no praise, no filler. Findings and evidence only. Improve the spec, not morale.
