# Findings Ledger — Holmes

Format: `F-### | Severity | Category | Location | Evidence (quote or ABSENT) | Why it matters | Recommended fix | Confidence (H/M/L)`

IDs are stable. The audit proper (Phases 0–7) appends here; it may supersede but must not renumber the pre-audit findings below.

## Pre-audit findings (logged 2026-07-13, at case-file creation)

**F-001** | BLOCKER | licensing | `LICENSE` | Repo LICENSE is Apache License Version 2.0 (verified by reading the file 2026-07-13) | Violates the standing licensing gate (AGPL-3.0-or-later or GPL-3.0). Cheap to fix now, expensive after outside contributions arrive — every contributor added under Apache-2.0 complicates relicensing | Human decision D-01: relicense before any code lands | H

**F-002** | MAJOR | spec | `docs/case-file/01-blueprint-kb.md` §"What the blueprint does NOT contain" | ABSENT — blueprint names Goose, Graphiti, and Firecracker but contains no versions, APIs, data models, deployment target, resource budgets, or composition design | The three named technologies have real integration constraints (Graphiti's backend requirements, Firecracker's Linux/KVM requirement) that may contradict the local-first laptop story; nobody has checked | Engineering spec must precede build; audit Phase 3 forces it | H

**F-003** | MAJOR | process | Drive + GitHub (see `docs/case-file/00-provenance.md` §"Negative results") | ABSENT — no build plan, milestones, kickoff prompt, or acceptance criteria exist for Holmes anywhere | Build cannot start without a plan; the blueprint's six phases are runtime workflow, not construction sequence, and conflating them would misorder the build | Run audit through Phase 6, which produces the kickoff prompt; interim proposal in `docs/roadmap/build-phases.md` | H

**F-004** | MAJOR | consistency | `docs/case-file/02-triad-context.md` §"Alfred" | Holmes deck: Holmes is "built on the open-source Goose framework"; Alfred deck: Goose is "The Hands" / the triad's execution engine; GooseClaw deck: Goose is "The Hands" with GooseClaw as "Body & Judgment" and no Holmes at all | The decks disagree about what Goose is and who runs on it — this is the substrate decision, and it's currently contradictory | Audit Phase 2 contradiction hunt; human decision D-03 on the runtime architecture | H

**F-005** | MINOR | claims | `docs/case-file/01-blueprint-kb.md` | Blueprint's external claims are unverified: Firecracker "<1ms startup times" (attributed to the 2019 AWS paper), OpenCorporates "230M+ entities", catalog "1,300+ license-checked tools" | If load-bearing numbers are stale or misquoted, the design inherits the error | Audit Phase 1 verifies each against primary sources with dates | H

**F-006** | MINOR | provenance | earlier working notes (not in this repo) | "ACP" was attached to Holmes in working notes but appears in no source document (checked all four decks 2026-07-13) | Unsourced protocol assumptions leak into architecture as if decided | Logged as UNKNOWN in the KB; audit Phase 2 research item: what protocol connects the triad? | H

**F-007** | MINOR | KB completeness | KB-wide | ABSENT: threat model, test plan, deploy runbook, data model, cost model — the standard expected-but-absent set | These are exactly the documents audit Phase 0 will demand; listing them now saves a round trip | Produce during audit Phases 0–3 | H

**F-008** | NIT | sources | `docs/case-file/00-provenance.md` §"Caveats" | Blueprint PDFs are OCR-noisy NotebookLM decks; diagram content is not fully captured in the text layer | Load-bearing detail could live only in a diagram | Human pass over the slides to confirm; note anything found as KB addenda | M

## Audit findings (Phases 0–7)

*None yet — audit not run. Append below.*
