# Second-Pass QA Audit — `holmes-master-build-loop-v1.md`

**Auditor pass date:** 2026-07-13 · **Mode:** read-only (Rule 9) · **Artifact under audit:** `holmes-master-build-loop-v1.md` (authored this session, 2026-07-13; 156 lines)

## 0 — Research inventory & first-pass location

**In context:** project — `holmes-spec-v2.md`, `triad-canon.md`, `holmes-vs-wcjbt_md.md` (double-extension filename), `epistemic-canon-Holmes.md`, `wisdom-intuition-knowledge-judgment-v2.md`, `holmes-project-orientation.md`, `LICENSE` (Apache-2.0). Uploads — kickoff v2.2, kickoff v3, `holmes-denylist-acceptance-criteria.md` (AC doc), `holmes-spec-v2.1-diff.md`, `/loop` protocol. Fetched live 2026-07-13 — repo root `github.com/MartinMontero/holmes`, repo `CLAUDE.md` (direct inspection), `api-docs.deepseek.com/quick_start/pricing`, `github.com/aaif-goose/goose` + LF press release (via search), `github.com/kuzudb/kuzu` archive notice (via search).

**Referenced but not in context (not fetched/readable):** `docs/audit/00-audit-charter.md`, `findings-ledger.md`, `amendments.md`, `decisions.md`, `docs/case-file/*`, `docs/roadmap/build-phases.md` (inner tree per README only); Alfred repo (private — holmes-vs-wcjbt.md:226); `trinity-incarnate-character-bible.md`; `claude-code-epistemic-integration-prompt.md`; Map v3 (canon cites v3 at epistemic-canon:14; project holds v2.0).

**First-pass location.** No separate first-pass report exists; the audit brief points both placeholders at the artifact itself. First-pass = the v1 artifact's embedded caveats (§8 facts ledger, carried AC-doc non-claims) plus the four authoring-turn decisions flagged for veto (loop fusion; D-01 execution; Beta Scope checkpoint; Phase 6 addition). Past-chat searches located two provenance sources, quoted where relied on: the 2026-07-06 QA thread — "A new Phase 2.5 was introduced to move CaMeL-style dual-LLM injection defense, calibration/knowability gating, tool-approval UX, and legal guardrails before the collection surface ships" — and today's v3 thread digest — "V3 incorporated three decisive calls the person made: Holmes shares Alfred's AGPL-3.0-or-later license…" (kind=summary; digest caveat applies).

## 1 — Plan (executed)

Claim classes: repo-state / external facts hardened as "treat as fact" / internal consistency across the 11-doc set / enforceability of locks / process-ledger compliance. Methods: direct fetch for repo + CLAUDE.md; primary-source web verification for goose, DeepSeek, Kuzu; grep-cited cross-doc comparison for everything internal. Stress matrix applied: consistency, failure modes (loop STUCK/BLOCKED paths), security (egress boundary, injection surfaces), supply chain/licensing, load-bearing assumptions, abuse (rubber-stamp go-aheads, poisoned staged canon — bounded by human-staged trust model).

---

## 2 — Verdict

**SHIP WITH FIXES.** The loop architecture, phase ordering, and evidence discipline hold; three blockers — all small, none structural — make the artifact self-contradictory or unsatisfiable as pasted: it violates the repo's own relicense and build-gate standing orders while instructing itself to honor them, and Phase 0's DoD cannot go green as written.

## 3 — Findings

| # | Sev | Component | Evidence | Verdict | Fix |
|---|---|---|---|---|---|
| F1 | BLOCKER | Task 0.1 (license / D-01) | v1:43 instructs "replace `LICENSE` with AGPL-3.0-or-later… record the resolution in decisions.md." Repo CLAUDE.md Gate 4 (fetched 2026-07-13, verbatim): "do not relicense without explicit human approval"; Gate 6 lists relicensing as irreversible; conventions: "Never resolve a D-item yourself." v1:25 simultaneously binds the run to those gates. Decision provenance exists — v3 thread digest, 2026-07-13: "three decisive calls the person made… AGPL-3.0-or-later" [INFERRED — model-written summary] — but approval is not in-band to the repo. Premise unchecked: "matching Alfred" rests on Alfred's LICENSE being AGPL; holmes-vs-wcjbt.md:226 flags this for re-verification (parent `onyx` is MIT) [VERIFIED in-context]. spec:214 still reads "Apache-2.0-compatible repo." | FALSE as an executable step; internal contradiction | Task 0.1 → evidence-gather (read + quote `Alfred/LICENSE` from the on-disk sibling), present D-01 as the run's first BLOCKED item; swap only on explicit in-band go-ahead; make v1:60 (§4.9) and v1:151 ("No Apache-2.0 emissions") conditional on D-01. |
| F2 | BLOCKER | Lock 0a (AC-DL-1 §6) | v1:78 requires "AC-DL-1, all seven criteria… including… the Graphiti-OpenAI-default regression" in Phase 0. AC doc:24: §6 "instantiates Holmes's memory layer" — the memory layer first exists in Phase 2 (spec §4.4; v1 Phase 2). Lock 2b re-states the same test. [VERIFIED — direct text] | Phase 0 DoD unsatisfiable → loop STUCK by construction | 0a = AC-DL-1 §§1–5, 7; §6 scheduled to lock 2b, recorded as scheduled-not-skipped; joint-gate wording: "all criteria applicable in-phase." |
| F3 | BLOCKER | Task 0 vs audit charter | CLAUDE.md header (verbatim): "Until the adversarial audit produces a READY verdict, this repo is documentation-only: do not scaffold application code." README: build starts only after the audit's verdict + kickoff. v1:3 declares itself that kickoff but never requires the verdict. [VERIFIED — direct fetch] | Pasting v1 today has Claude Code scaffolding in violation of standing orders v1:25 tells it to honor | Task 0 gains a verdict gate: confirm a READY verdict exists (this report may serve, if the human accepts it in-band) or the human explicitly supersedes the charter; record in `decisions.md`; absent → BLOCKED. |
| F4 | MAJOR | AC-DL-1 §4 enforceability (MCP egress) | AC doc:22 demands tool-initiated calls be "blocked by the egress allowlist… network-level"; AC doc §1 says "Rust/Tauri core." v3 deleted the Tauri shell — Holmes is a library inside Alfred; goose and MCP servers are separate processes. A library crate cannot impose a network-level boundary on other processes cross-platform without OS/parent-app control. [INFERRED — architecture chain] | Promise carried without a mechanism → Phase 0 STUCK or a dishonest `docs/security.md` | Specify: Holmes-spawned sessions forced through a deny-by-default local egress proxy (env sanitized; `HTTP(S)_PROXY` injected, provider vars stripped); AC-DL-1 §4's planted-test-server check passes honestly against it; document the residual (a hostile binary ignoring proxy env escapes the library boundary); record artifact/OS-level enforcement as an Alfred obligation; open a D-## if the residual is unacceptable. |
| F5 | MAJOR | CLAUDE.md re-seed collision | v1:44 re-seeds "`CLAUDE.md` operating context… from the revised spec" while v1:25 depends on CLAUDE.md's standing gates. Unqualified re-seed can clobber the gates. [VERIFIED — direct text] | Self-defeating instruction | Protected-block merge: standing gates preserved verbatim at top; spec-derived operating context in a marked section beneath; diff shown at checkpoint. |
| F6 | MAJOR | Epistemic label systems | CLAUDE.md Gate 2 defines VERIFIED/INFERRED/ASSUMED/UNKNOWN; canon uses unmarked/[DIRECTIONAL]/[NEEDS-CAVEAT]; v1 mandates both, no mapping. [VERIFIED] | Ledger drift guaranteed over a multi-phase run | One mapping paragraph in §1: ledger rows use CLAUDE.md labels; spec-derived docs preserve canon markers; equivalences stated (VERIFIED≈unmarked; INFERRED/ASSUMED≈[DIRECTIONAL]; UNKNOWN→research item). |
| F7 | MINOR | L1 conflation | v1 Phase 0 fuses host allowlist and model-id resolution into one clause ("permit named permitted hosts; everything else — and unknown ids — rejected at resolution time"). Two mechanisms. | Implementable ambiguity | Split: L1a network egress allowlist (AC-DL-1); L1b provider/model-id resolution guard (v3 §4.3). Both Rust-core. |
| F8 | MINOR | Weight provenance omitted | AC doc:60 flags Tier-2 model-download provenance as "a separate acceptance criterion for the Phase 2/3 model-download path — flagged here, not solved here." v1 carries it nowhere. | Dropped caveat | Add AC-WP stub (checksum/signature verification of pulled weights) to Phase 2 locks. |
| F9 | MINOR | Recipe scanner timing | Recipes (agent-facing parse surface) ship from Phase 1; the Unicode/ASCII-smuggling scanner arrives Phase 4 (v1, per spec §7). | Phases 1–3 smuggling window | Pull the strip/scan (or a pre-commit equivalent) into Phase 1 CI. |
| F10 | MINOR | Marker hygiene, facts ledger | v1:144 hardens three items: (a) "Kuzu: archived Oct 2025 **post-Apple-acquisition**" — archive VERIFIED primary ("This repository was archived by the owner on Oct 10, 2025," github.com/kuzudb/kuzu); the Apple clause is absent from the primary notice ("Kuzu is working on something new") and rests on secondary reporting per the v2.1 diff's own sources. (b) goose-secrets "per goose source" — spec:132 still [NEEDS-CAVEAT]; no citation carried (v3:97). (c) litellm↔GHSA-69fq-xp46-6x23 linkage — carried from AC doc:35, advisory unchecked. | Silently hardened caveats | (a) mark Apple clause [DIRECTIONAL] or drop it (archival alone drives Phase 2); (b) cite goose source lines + queue the spec edit, or downgrade to confirm-at-build; (c) mark [DIRECTIONAL] pending advisory fetch — the AC-DL-2 rationale (litellm routes to excluded providers) stands regardless. |
| F11 | MINOR | Pre-flight path rigidity | v1:36 requires the spec at `docs/holmes-spec-v2.md`; orientation §5.1 and v2.2:37 stage it at folder root ("ready to be moved into `docs/`"). Project filename `holmes-vs-wcjbt.md.md` needs normalizing on mirror. | False-STOP risk on first run | Accept root or `docs/`, normalize into `docs/`; add an explicit human staging checklist (spec, 3 canon files, AC doc). |
| F12 | MINOR | Phase-namespace collision | Build "Phase 6" (v1:112) vs charter "Phase 6 produces the kickoff prompt" (README). | Ledger ambiguity | Rename build Phase 6 → **Phase RC** (or prefix B0–B6 in ledger entries). |
| F13 | NIT | Prohibition wording | v1:151 "No Apache-2.0 emissions in this repo" — Apache *dependencies* (goose, most crates) are required and clean into AGPL. | Ambiguous | "No Apache-2.0 license identifiers on files authored in this repo" (conditional on D-01 per F1). |
| F14 | NIT | Findings-row format | v1 routes F-### entries but omits CLAUDE.md's fixed row format, severity definitions (charter), and no-renumber rule. | Convention drift | Reference the format string and rules verbatim in §1. |
| F15 | NIT | RC-phase egress friction | Phase 6/RC requires live model-price verification; v1 §1's declared egress surface excludes vendor-doc fetches → per-fetch approval churn. | Friction | Add "primary-source vendor-documentation fetches for verification" to the declared surface. |
| F16 | NIT | Upstream stale reference (logged, not a v1 defect) | epistemic-canon:14 cites the Map at "v3, fully sourced"; project holds v2.0 (wisdom doc:3). | Canon maintenance item | Fix in canon maintenance; log F-### in the repo ledger. |

External facts re-verified this pass (all VERIFIED, primary): goose Apache-2.0 under AAIF/Linux Foundation — github.com/aaif-goose/goose ("part of the Agentic AI Foundation (AAIF) at the Linux Foundation"), linuxfoundation.org press release 2025-12-09, goose-docs.ai. DeepSeek — pricing page live 2026-07-13: V4 Pro $0.435/$0.87, cache-hit $0.003625; footnote (1): "`deepseek-chat` and `deepseek-reasoner` will be deprecated on 2026/07/24 15:59 UTC… correspond to the non-thinking mode and thinking mode of `deepseek-v4-flash`, respectively" — v1's Task 0.3 claim exact. Repo state — public, main, 3 commits, top-level {docs, CLAUDE.md, LICENSE (Apache-2.0), README.md}, no releases.

## 4 — Caveat disposition

| Caveat (verbatim from first-pass) | Disposition |
|---|---|
| "permanence **[NEEDS-CAVEAT — sources conflict; budget both rates]**" (v1:144) | **STILL OPEN — reconfirmed.** Live page 2026-07-13 shows no permanence statement and "DeepSeek reserves the right to adjust." Closes only with a vendor statement; budget both rates stands. |
| "`deepseek-chat`/`deepseek-reasoner` retire **2026-07-24 15:59 UTC** → V4 Flash" (v1:144) | **RESOLVED.** Verified live against the primary pricing page (footnote 1). 11 days out — Task 0.3 urgency confirmed. |
| "Kuzu: archived Oct 2025 post-Apple-acquisition" (v1:144) | **Split.** Archival **RESOLVED** primary (repo banner, Oct 10 2025). Apple clause **STILL OPEN → F10a** ([DIRECTIONAL], secondary only). |
| goose "Apache-2.0, AAIF/Linux Foundation — verified 2026-07-13" (v3:11, carried v1:144) | **RESOLVED.** Independently re-verified this pass against three primaries. |
| goose secrets "OS keyring… per goose source" (v3:97 / v1:144) vs spec:132 [NEEDS-CAVEAT] | **STILL OPEN.** No citation carried; spec never edited. Closes: cite goose source lines + spec edit, or confirm at build (F10b). |
| "Graphiti's documented default LLM/embedding client is OpenAI (verified 2026-07-06)" (v1:144, per AC doc) | **RESOLVED as of 2026-07-06** (AC doc primary check; getzep/graphiti issue #1132 corroborates repo activity). AC-DL-1 §6 exists to catch drift — now correctly scheduled to Phase 2 per F2. |
| "Block-vs-AAIF config namespace: record as found" (v1:145) | **STILL OPEN by design** — build-time observation. |
| "Vercel Sandbox GA timing remains [NEEDS-CAVEAT] in the spec" (v1 Phase 3) | **STILL OPEN.** Prior QA reportedly verified 2026-01-30 GA [past-chat, uncited here]; closes with one Vercel-changelog fetch at Phase 3 + spec edit. |
| Phase-scheduled re-verify list: FalkorDB/Neo4j licenses; E2B; OpenAleph; RC model prices (v1:145) | **STILL OPEN by design** — correctly scheduled. |
| AC-doc non-claim: proxied excluded model on a permitted host | **STILL OPEN** — documented residual; mitigation = narrow allowlist + vet proxy hosts (carry into `docs/security.md`). |
| AC-doc non-claim: "model-weight provenance… flagged here, not solved here" (acdoc:60) | **ESCALATED → F8.** v1 dropped it from every phase; add AC-WP to Phase 2. |
| Authoring decision (a): /loop-vs-Rule-9 fusion (working-tree writes autonomous) | **STILL OPEN for ratification; compatible.** CLAUDE.md Gate 6 gates destructive/irreversible + outward actions, not working-tree writes. One in-band "confirmed" closes it. |
| Authoring decision (b): D-01 relicense executed in Task 0.1 | **ESCALATED → F1 (BLOCKER).** CLAUDE.md Gate 4 forbids exactly this without explicit in-band approval; premise (Alfred's actual LICENSE) also unchecked. |
| Authoring decision (c): Beta Scope Decision as BLOCKED checkpoint | **RESOLVED.** Matches the repo's D-## pattern and /loop's BLOCKED semantics. |
| Authoring decision (d): Phase 6 (open-beta RC) addition | **STILL OPEN as spec amendment** (A11); rename per F12. |
| Qwen3.7-Max context/pricing/exact-day [DIRECTIONAL] (v2.1 diff E7) | **STILL OPEN.** v1 did not harden the sub-claims; closes at Alibaba Model Studio primary before volume (RC audit). |

## 5 — Unverifiable (and what closes each)

1. `docs/audit/*` and `docs/case-file/*` contents — charter severity definitions, exact F-001/D-01 text, seeded decisions. Inner tree is README-attested only. Closes: read on disk at pre-flight (or fetch blobs). Bounded: v1 pre-flight STOPs on absence.
2. Alfred repo contents — LICENSE, `acp-client.ts`, conventions (private; wcjbt:226). Closes: on-disk sibling read at pre-flight — mandatory under F1.
3. Whether the v3-thread "the person made" the AGPL call is an explicit ratification or a digest collapse (kind=summary). Closes: one line from the human at D-01.
4. goose secrets keyring source lines. Closes: goose source/docs citation at build.
5. litellm ↔ GHSA-69fq-xp46-6x23 linkage. Closes: advisory fetch.
6. Vercel Sandbox GA month. Closes: Vercel changelog fetch (Phase 3).
7. Qwen3.7-Max 1M context / ~$2.50-$7.50 / launch day. Closes: Model Studio primary.

## 6 — Amendment queue (dependency-ordered; hand to Claude Code as v1 → v1.1)

1. **A1 (F3)** — Task 0 verdict gate: require a READY audit verdict (this report, if accepted in-band) or explicit human supersession of the charter; record in `decisions.md`. Blocks everything.
2. **A2 (F1)** — Task 0.1 rewrite: read + quote `Alfred/LICENSE`; present D-01 as the first BLOCKED item; swap only on explicit go-ahead; conditionalize v1:60 and v1:151 on D-01.
3. **A3 (F2)** — Lock 0a → AC-DL-1 §§1–5,7; §6 scheduled to lock 2b; joint-gate wording "all criteria applicable in-phase."
4. **A4 (F4, F7)** — L1a/L1b split; L2 = enforced local egress proxy with sanitized env; honest-limit language in `docs/security.md`; Alfred/OS-level enforcement logged as cross-repo obligation; optional D-## on the residual.
5. **A5 (F5)** — CLAUDE.md protected-block merge rule for all re-seeding; diff at checkpoint.
6. **A6 (F6, F14)** — Label-mapping paragraph in §1; carry CLAUDE.md's F-### row format, charter severities, no-renumber rule.
7. **A7 (F11)** — Pre-flight path tolerance (root or `docs/`), filename normalization, explicit human staging checklist.
8. **A8 (F8, F9)** — Add AC-WP (weight-provenance) stub to Phase 2 locks; pull the Unicode/ASCII-smuggling strip into Phase 1 CI.
9. **A9 (F10)** — Facts-ledger hygiene: Kuzu-Apple → [DIRECTIONAL]; goose-secrets → cite-or-confirm-at-build; litellm/GHSA → [DIRECTIONAL]; append "DeepSeek page re-verified 2026-07-13."
10. **A10 (F12, F13, F15)** — Rename build Phase 6 → Phase RC everywhere; "no Apache-2.0 identifiers on authored files" wording; add vendor-doc verification fetches to the declared egress surface.
11. **A11 (spec pipeline)** — Queue spec v2.2 edits to carry back once the run starts: Phase 2.5 + Phase RC into §7; §4.1 secrets caveat resolution when cited; D-01 outcome; Vercel GA when fetched; log F16 (Map v3-vs-v2.0) as a canon-maintenance F-###.

*End of second-pass audit. Rule 9: nothing edited; all fixes above are proposals.*
