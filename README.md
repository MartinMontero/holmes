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

## Project status: Phases 0–2 CLOSED — Phase 2.5 (safety gate) next

The build has run. This repo is a **Rust workspace** (four crates; the language bar is Rust — zero Python surface). Each phase closed as one PR with executed evidence per lock; per-lock inventories live in `STATE.md`, born-redacted transcripts in `docs/audit/evidence/`.

- **Phase 0 — Substrate + guards (PRs #7–#9, closed 2026-07-19).** `holmes-guard`: L1a deny-by-default egress proxy, L1b provider/model resolution, L2 sanitized spawn; AC-DL-1/AC-DL-2 denylist gates proven in CI (`acdl-gate`); action-free CVE gate (Syft + OSV-Scanner + Grype, checksum-pinned — `supply-chain`); pinned goose build with provenance; live guarded ACP round-trip on the smoke model (streamed completion, egress 1/1 allowed); `holmes-core` embedding contract (§6.2 artifact types, invalidation-not-deletion, handoff-only).
- **Phase 1 — Analytical core (PR #10, closed 2026-07-19).** Hypothesis objects with likelihood-ratio scoring (both conditionals required; `Uncalibrated` carried), ACH matrix (complete-or-refuse, ties reported), Key-Assumptions-Check, first-principles quarantine, six-phase case state machine, lock-1a emission gate (≥2 independent source roots + knowability + limits). Live six-phase case on the smoke model: CASE COMPLETE, 64 frames, a 3-way ACH tie honestly flagged. Recipe safety scan (Unicode-smuggling, fails closed) live from the first recipe.
- **Phase 2 — The Wall (PR #11, closed 2026-07-19).** Graphiti **dropped** (D-12/F-027: base-deps include an excluded-vendor SDK and phone-home telemetry); `holmes-wall` is an owned temporal-graph subset on Neo4j Community Edition via `neo4rs` (denylist-clean). Bi-temporal facts, invalidate-not-delete (no delete API exists; Cypher audited), AC-DL-1 §6 landed (default memory layer local-only; CI marker EXECUTING), deterministic ingestion scorer (6/6 grounded live), supervised backend (kill-on-drop, path-confined), weight provenance (SHA-256 fail-closed).
- **Post-phase (PRs #12–#13, 2026-07-19/20).** A-07 canon landed in `docs/holmes-vs-wcjbt.md` (§6.2 schema); lock-1a corroboration heuristic hardened against URL-decoration forgery (F-029/F-031).

Environment-gated legs are carried honestly, never faked (org egress blocks in the build container): the live-Neo4j 2a leg and Tier-2-on-Ollama 2c failure rates run on any open-egress host; 0e provenance/attestation carries to Phase RC.

**Next phase: 2.5 — Safety before surface, a hard gate** (calibration gating for LR scores, knowability gating, adversarial corpus from the carried F-029/F-031 forgery shapes), then the Beta Scope Decision — a D-item reserved for the human. Nothing starts without explicit go-ahead (Rule 9).

## The case file

```
crates/
  holmes-core/     §6.2 artifact types + analytical engines: ACH, LR scorer, KAC,
                   quarantine, six-phase case state machine, lock-1a emission gate
  holmes-guard/    Denylist guards: L1a egress proxy, L1b provider/model resolution,
                   L2 sanitized spawn, AC-DL-2 dependency scanner, recipe-scan
  holmes-wall/     The Wall: bi-temporal facts, invalidate-not-delete, Neo4j backend,
                   supervised process, weight provenance, ingestion scorer
  holmes-smoke/    Live harnesses over goose acp: holmes-smoke (0c), holmes-case (1b),
                   holmes-ingest (2c)
recipes/           First recipes (la-lluvia, el-diablo) — gated by recipe-scan
scripts/
  build-goose.sh   Pinned goose build (commit-pinned, provenance recorded)
.github/workflows/
  acdl-gate.yml    Denylist gates + lock steps (0d, 1d, §6 EXECUTING)
  supply-chain.yml SBOM + CVE gate, action-free, checksum-pinned; D-13 exact-set assert
docs/
  holmes-spec-v2.md                  CANONICAL build spec (+ holmes-spec-v2.1-diff.md)
  prompts/holmes-master-build-loop-v2.md   The operating loop (§6 phase plan)
  holmes-vs-wcjbt.md                 Epistemic canon (§6.2 schema; A-07 landed)
  acceptance/                        AC-DL denylist acceptance criteria (v3)
  case-file/                         Provenance, blueprint KB, triad context
  audit/                             Charter, findings-ledger.md (F-###), amendments.md
                                     (A-##), decisions.md (D-##, human-only),
                                     evidence/ (executed transcripts)
  upstream/                          Drafts for Martin to file upstream (D-12 rider e)
STATE.md / LOOP.md   Live build state (per-lock inventories) / stage doc
```

## Standing gates

All work in this repo is governed by the constitution in [CLAUDE.md](CLAUDE.md): zero fabrication with epistemic labels, evidence-or-it-didn't-happen, the vendor denylist (no Meta, OpenAI, or xAI — direct, transitive, or as model weights; Google permitted), Rule 9 (consent before consequence), RPI, path-confined tools, born-redacted telemetry, supply-chain hygiene (no Trivy, SHA-pinned Actions), and surveillance-detection-not-surveillance.

**License (D-01, decided 2026-07-18):** Holmes is licensed **AGPL-3.0-or-later**, matching Alfred — the LICENSE file carries the GNU AGPL v3 text, and the "or any later version" option applies per the notice convention in its How-to-Apply section. Ratified by the human at Gate Zero with `Alfred/LICENSE` quoted as evidence (`docs/audit/decisions.md` D-01; resolves F-001).

## Next steps

1. **Phase 2.5 — Safety before surface (hard gate):** calibration gating (`CalibrationStatus` stops being carried), knowability gating, and the adversarial corpus of carried provenance-forgery shapes (F-029/F-031). Starts only on explicit go-ahead.
2. **Beta Scope Decision** after 2.5 — a D-item reserved for the human (`docs/audit/decisions.md`).
3. **Carried environment-gated legs** on an open-egress host: live-Neo4j lock-2a leg (`HOLMES_NEO4J_URI`), Tier-2-on-Ollama 2c failure rates; 0e provenance/attestation → Phase RC.
4. **Upstream (the human's steps):** canon propagation per A-07 §Propagation (byte-identical re-copy + connector re-sync); the Graphiti optional-extras PR (`docs/upstream/graphiti-optional-extras-pr-draft.md`, D-12 rider e); pending spec amendments A-02…A-06, A-08 on the pressure-testing surface.
