# 00 — Provenance

Every document in this case file derives from the sources below. Per the zero-fabrication gate, anything not traceable to one of these is labeled `INFERRED`, `ASSUMED`, or `UNKNOWN` where it appears.

## Primary source (the artifact under audit)

| Source | Location | Owner | Dated | Verification |
|---|---|---|---|---|
| **"Holmes: The AI Detective.pdf"** — NotebookLM visual blueprint, 17.9 MB | Google Drive, file ID `1bLOaAVOjG58sxcvgjJu-c01-AC-vMfdH` | knoxumedia@gmail.com | created 2026-06-30 | Full text layer read 2026-07-13 via Drive API. OCR is noisy on diagram slides; prose and headings extracted cleanly. Visual diagrams not fully captured — see caveat below. |

## Supporting sources (context on Holmes's role in the OS)

| Source | Location | Dated | Verification |
|---|---|---|---|
| "Wecanjustbuildthings.dev: The Master Builder OS.pdf" | Drive ID `1td6o_Yn5K2EssMrGZP6erm8U7vkVzYma` | 2026-06-30 | Read by extraction agent 2026-07-13 |
| "Sovereign Builder OS.pdf" | Drive ID `1iXGH7sa4pRj1BXLlV7JF6jy32QK1GD9Y` | 2026-06-30 | Read by extraction agent 2026-07-13 |
| "Alfred: Leverage Without Surrender.pdf" | Drive ID `1s52TiQHzLMoKPsgQj4k5OsaQ0bdJ30Mb` | 2026-06-30 | Read by extraction agent 2026-07-13 |
| "GooseClaw: The Fiduciary AI Blueprint" (PDF) | Drive, ID not captured in session records — locate via title search | 2026-06 | Read by extraction agent 2026-07-13. Contains **no** Holmes references; uses a differently-named triad. |

## Process source (the audit instrument)

| Source | Location | Dated | Verification |
|---|---|---|---|
| **"Adversarial QA & Production-Readiness Audit"** (Google Doc, generic master prompt) | Drive doc ID `1iMEtvm004KDeCCENZwx76aYzf-pN1fUhC1_eeY2dKxg` | modified 2026-07-07 | Full text read 2026-07-13. Instantiated for Holmes as `docs/audit/00-audit-charter.md`. |

## Canonical build documents (supplied 2026-07-13, later session — previously local-only)

These existed on the human's local machine but were in neither Drive nor GitHub, which is why the negative results below (accurate for those two surfaces) initially read as "no spec exists." See F-010.

| Source | Repo location | Dated | Verification |
|---|---|---|---|
| **`holmes-spec-v2.md`** — Product & Architecture Specification v2 (QA-corrected), the **authoritative build reference** per its own header | `docs/holmes-spec-v2.md` | verification date 2026-06-29 | Supplied by the human 2026-07-13; read in full; committed verbatim with `[DIRECTIONAL]`/`[NEEDS-CAVEAT]` markers preserved |
| `holmes-project-orientation.md` — claude.ai project orientation & artifact index | `docs/holmes-project-orientation.md` | references rev. 2026-06-29 | Supplied and read in full 2026-07-13 |
| `wisdom-intuition-knowledge-judgment-v2.md` — multidisciplinary epistemology map (v2, QA-integrated), design input for Holmes's analytical core | `docs/research/wisdom-intuition-knowledge-judgment-v2.md` | v2 | Supplied and read in full 2026-07-13 |

Referenced by the orientation doc but **not yet in this repo**: `holmes-claude-code-kickoff-phase0-v2.md` (the current Phase 0 kickoff prompt — see F-009) and `trinity-incarnate-character-bible.md` (Santos Reyes source material).

## The claude.ai Holmes project (third surface, confirmed 2026-07-13)

A claude.ai project ("Holmes — Manage the build of the holmes research aid…") exists as the pressure-testing surface. Its instructions are committed verbatim at `docs/holmes-claude-project-instructions.md` (supplied by the human 2026-07-13, read in full). Its GitHub connector tracks `MartinMontero/holmes` @ **`main`** — see F-012.

Project knowledge inventory — corrected 2026-07-13 (later session) after the claude.ai project itself enumerated its knowledge folder: it holds exactly **six files**, all now supplied and reconciled. The four other artifacts visible in the earlier screenshot (`holmes-denylist-acceptance-criteria.md`, `holmes-spec-v2.1-diff.md`, `Iterative quality validation process.md`, `holmes-claude-code-kickoff-phase0-v2(2).md`) are **chat attachments** in the project's "Optimized Holmes Project build prompt" conversation, *not* project knowledge — claude.ai confirmed it cannot return them verbatim (retrieval yields excerpts only), so the originals must come from where they were drafted.

| In project knowledge | In this repo? |
|---|---|
| `holmes-spec-v2.md` | ✅ `docs/holmes-spec-v2.md` — re-supplied 2026-07-13, **diffed byte-identical** to repo copy (no surface drift) |
| `holmes-project-orientation.md` | ✅ `docs/` — re-supplied, **diffed byte-identical** |
| `wisdom-intuition-knowledge-judgment` map | ✅ **v3.1 is now current**: `docs/research/wisdom-intuition-knowledge-judgment-v3.1.docx` (source of record, supplied 2026-07-13) + `.md` conversion (method noted in the file header). v2.0 **preserved** at `docs/research/` as superseded — non-destructive truth. The epistemic canon's "v3" reference now resolves: v3.0's output files were lost; v3.1 (rebuilt 2026-07-13 **using this repo's committed v2 as the verbatim recovery base**) supersedes it, with all three v3 upgrades re-verified in its Appendix B. NOTE: the canon's shared body cites "v3" — re-pointing it to "v3.1" is an edit to the *shared* canon, which per its own maintenance rule must be made centrally (WCJBT repo) and re-propagated to all three copies, not forked here. |
| `holmes-vs-wcjbt.md` — boundary/interface contract, named source-of-truth doc | ✅ `docs/holmes-vs-wcjbt.md` — supplied and read in full 2026-07-13 |
| `triad-canon.md` — shared triad canon (canonical home: WCJBT repo; this is the mandated Holmes-repo mirror) | ✅ `docs/triad-canon.md` — supplied and read in full 2026-07-13 |
| `epistemic-canon-Holmes.md` — Holmes copy of the shared epistemic canon (Upgrade B: metacognitive-humility layer; `knowability`; confidence→routing firewall) | ✅ `docs/epistemic-canon-Holmes.md` — supplied and read in full 2026-07-13 |

Still missing from the repo (chat attachments, not project knowledge — see F-009/F-011): `holmes-claude-code-kickoff-phase0-v2.md`, `holmes-denylist-acceptance-criteria.md`, `holmes-spec-v2.1-diff.md`, `Iterative quality validation process.md`. Also referenced but never sighted: `claude-code-epistemic-integration-prompt.md` (named in the epistemic canon as its build-instructions companion) and `trinity-incarnate-character-bible.md`.

## Negative results (checked and found nothing — 2026-07-13, scope: Drive and GitHub only)

- **No Holmes-specific build plan, master prompt, kickoff prompt, or milestone roadmap exists in Drive.** Exhaustive search: title and fullText queries for Holmes/Detective/kickoff/build plan/roadmap/master prompt, crossed with Milestone, Graphiti, Evidence Pack, ACP, goose, Phase, acceptance criteria, and recency filters. The string "Holmes" appears only in the four blueprint PDFs above. *(Still true of Drive; the artifacts turned out to live on the local machine — see above.)*
- **The GitHub repo (`MartinMontero/holmes`) contained only a LICENSE file** before this case file landed: one commit ("Initial commit", 2026-06-29, GitHub web flow), one branch, no README, no CI, no code.
- **The generic audit master prompt has never been run against Holmes as a fresh adversarial audit.** However, a **v1→v2 QA pass did happen** before this case file existed: its corrections are folded into `docs/holmes-spec-v2.md` ("What changed from v1") and the wisdom doc's Corrections Log. The earlier claim that "no QA has touched Holmes" was wrong and is corrected here.

## Caveats

- The blueprint PDFs are NotebookLM-generated **visual decks**: conceptual/pitch-grade, not engineering specs. Slide diagrams carry meaning the OCR text layer does not fully capture; a human pass over the slides should confirm nothing load-bearing lives only in a diagram.
- The blueprint itself makes external factual claims (e.g., Firecracker microVM startup latency, OpenCorporates entity counts). Those are **the blueprint's claims**, recorded here as such; verifying them against primary sources is audit Phase 1 work.
