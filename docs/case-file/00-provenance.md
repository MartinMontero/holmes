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

## Negative results (checked and found nothing — 2026-07-13)

- **No Holmes-specific build plan, master prompt, kickoff prompt, or milestone roadmap exists in Drive.** Exhaustive search: title and fullText queries for Holmes/Detective/kickoff/build plan/roadmap/master prompt, crossed with Milestone, Graphiti, Evidence Pack, ACP, goose, Phase, acceptance criteria, and recency filters. The string "Holmes" appears only in the four blueprint PDFs above.
- **The GitHub repo (`MartinMontero/holmes`) contained only a LICENSE file** before this case file landed: one commit ("Initial commit", 2026-06-29, GitHub web flow), one branch, no README, no CI, no code.
- **The audit master prompt has never been run against Holmes**: no findings ledger, amendments, verdict, or kickoff prompt for Holmes exists anywhere in Drive.

## Caveats

- The blueprint PDFs are NotebookLM-generated **visual decks**: conceptual/pitch-grade, not engineering specs. Slide diagrams carry meaning the OCR text layer does not fully capture; a human pass over the slides should confirm nothing load-bearing lives only in a diagram.
- The blueprint itself makes external factual claims (e.g., Firecracker microVM startup latency, OpenCorporates entity counts). Those are **the blueprint's claims**, recorded here as such; verifying them against primary sources is audit Phase 1 work.
