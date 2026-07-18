# CLAUDE.md — Standing orders for any agent working in this repository

This repo is the case file and (eventually) the implementation of **Holmes**, the research-and-analysis brain of the WCJBT Non-Dev Builder OS and sibling of Alfred. Core thesis: **"the method is the identity."**

## Source of truth

**`docs/holmes-spec-v2.md` is the authoritative build reference.** Where anything in this file, the case-file docs, or your memory disagrees with the spec, the spec wins. Revisions are drafted on the claude.ai pressure-testing surface and written back here; **the repo copy wins** on any disagreement (sync rule in the spec's header). Preserve `[DIRECTIONAL]` and `[NEEDS-CAVEAT]` markers — never silently harden caveated claims into facts.

This repo is documentation-only until Phase 0 runs from its kickoff prompt (`holmes-claude-code-kickoff-phase0-v2.md`, not yet committed — F-009) with explicit human go-ahead. Do not scaffold application code before that.

## Standing gates (non-negotiable — the constitution, per the orientation doc + spec)

1. **Zero fabrication.** Every external factual claim gets a primary source with a date, or an explicit `UNVERIFIED` tag. Never fill gaps with plausible detail.
2. **Epistemic labels** on all claims in any document you write here:
   - `VERIFIED` — cite the source and the date checked
   - `INFERRED` — show the reasoning
   - `ASSUMED` — flag it
   - `UNKNOWN` — becomes a research item
3. **Evidence or it didn't happen.** Every finding quotes its source (file + section) or states `ABSENT` explicitly. Distinguish "I checked and found nothing" from "I didn't check."
4. **Licensing — AGPL-3.0-or-later, per D-01 (DECIDED 2026-07-18).** Ratified by the human at Gate Zero with `Alfred/LICENSE` quoted from disk (evidence in `docs/audit/decisions.md` D-01); `LICENSE` carries the AGPL v3 text byte-identical to Alfred's. Author no license identifiers other than AGPL-3.0-or-later (Apache-2.0 *dependencies* unaffected). The spec's §7 "Apache-2.0-compatible repo" line is superseded on this point — carried as a pending spec amendment (A-02), never silently propagated.
5. **Vendor gate — a denylist, not an allowlist.** No Meta, OpenAI, or xAI anywhere — direct or transitive: SDKs, models, APIs, infra, **model weights** (no Llama even locally). Google is permitted; open-weights-on-permitted-infra permitted. Violations are Blockers. Once code exists, the denylist is a runtime guard *and* a regression test.
6. **Rule 9 — consent before consequence.** Proposals until approved. No commit/push without explicit go-ahead when building; nothing destructive or irreversible (relicensing, force-pushes, deletions, publishing, spending) without explicit human go-ahead, ever.
7. **RPI.** Research → Plan → Implement, in that order, every time.
8. **Path-confined, deny-by-default tools.** Read-only tools may run free; every write/shell action asks first.
9. **Born-redacted, local-only telemetry.** Counts, durations, names — never content, prompts, or secrets.
10. **Supply-chain hygiene.** Syft SBOMs, OSV-Scanner, Grype; **no Trivy** (CVE-2026-33634); pin GitHub Actions to full commit SHAs.
11. **Surveillance-detection-not-surveillance.** Anti-doxxing refusals are permanent; tools scoped to power, never private citizens.
12. **"I answer to the block."** Blacksky-style community accountability + human-in-the-loop; until the assembly exists, the human reviewer is the interim block.

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

---

<!-- BEGIN SPEC-DERIVED OPERATING CONTEXT — re-seeded from docs/holmes-spec-v2.md v2.1 on 2026-07-18. Everything ABOVE this marker is the protected block: it is never modified by a re-seed (loop §6, lock 0e). Re-seeds replace only what is between these markers. -->

## Spec-derived operating context (v2.1)

- **Substrate:** goose (`aaif-goose/goose`, Apache-2.0, Rust) over Zed's ACP (`goose acp`, JSON-RPC 2.0/stdio); all tools MCP, path-confined, deny-by-default. Secrets in the OS credential store `[NEEDS-CAVEAT — confirm goose backend per platform]`.
- **Delivery:** Holmes ships as `holmes-core` + `holmes-guard` crates **embedded in Alfred** — no standalone UI/installer/updater (spec §4.1/§7 shell superseded; pending amendment A-03).
- **Models:** Tier-1 cloud — Claude, Gemini 3.1 Pro, DeepSeek V4, Magistral, Qwen3.7-Max (proprietary, never sovereign). Tier-2 sovereign — non-Meta open weights (Qwen3.5-27B / Qwen3.6-35B-A3B, Magistral Small, Gemma). `deepseek-chat`/`deepseek-reasoner` retire 2026-07-24 15:59 UTC → V4 Flash (`deepseek-reasoner` → Flash, not Pro). DeepSeek Pro discount permanence `[NEEDS-CAVEAT]` — budget both rates.
- **The Wall:** Graphiti temporal graph, Neo4j/FalkorDB only (Kuzu abandoned); Graphiti's default LLM client is OpenAI — pin permitted clients, regression-tested (AC-DL-1 §6, Phase 2); invalidation-not-deletion.
- **Derived files:** `docs/architecture.md`, `docs/constitution.md`, `docs/build-roadmap.md`, `docs/security.md` — re-seeded together with this block; canon markers preserved verbatim everywhere.

<!-- END SPEC-DERIVED OPERATING CONTEXT -->
