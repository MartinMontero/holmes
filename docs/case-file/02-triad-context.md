# 02 — Holmes in the Triad: What the Sibling Blueprints Add

Sources: the three sibling NotebookLM decks plus GooseClaw (see `00-provenance.md`). These are conceptual/pitch-grade documents; all claims below are `VERIFIED (blueprint)` — i.e., faithfully extracted — unless marked otherwise. Extraction performed 2026-07-13.

## From "Wecanjustbuildthings.dev: The Master Builder OS"

- Holmes appears as "The Chief Researcher (Holmes)": "Owns the decision of What is true. It supplies evidence and verifies assumptions."
- In the secure-handoff flow, Holmes is invoked "for evidence checking and risk flagging": the Architect authors the blueprint, Holmes supplies verifiable facts, Alfred writes the code.
- Verified findings compound and flow back to the vetted catalog as NIP-32 labels — consistent with the Holmes deck's community-accountability slide.
- The catalog ("Warehouse with a Bouncer") claims "1,300+ license-checked tools" — **UNVERIFIED externally**.
- Contains no Holmes implementation detail and no build sequencing.

## From "Sovereign Builder OS — Leverage Without Surrender"

The richest sibling source for Holmes's operating constraints:

- **Epistemic controls:** mathematical confidence scoring 0-to-1 on every finding; fully cited evidence packs plus risk flags with a "zero hallucination" mandate; **Metacognitive Humility** — Holmes refuses bare high-confidence answers in uncertain domains and states the limits of its findings.
- **The Epistemic Firewall:** a one-way data gate between Verifier and Builder. Alfred, on hitting an unknown, is *forbidden to guess* — it must stop and ask Holmes.
- **Runtime Phase 2, "Hardening the Blueprint," belongs to Holmes:** cross-reference the blueprint against reality, source secure/reliable dependencies, attach cited provenance and "Verified" stamps to project files as frontmatter metadata.
- **Fiduciary boundary (Must Not):** Holmes must never assert unverified facts and is strictly forbidden from writing code.
- **Learning loop:** Alfred's discoveries flow back into "Holmes's memory"; WCJBT's blueprint becomes "Holmes's research brief."
- Names the always-on shared local runtime "**GooseClaw**"; refers only generically to "Holmes's memory" (no Graphiti mention here).

## From "Alfred: Leverage Without Surrender"

- Holmes tagline: "Knowledge + Evidence," answering "What is true?" — "It investigates dependencies, verifies facts, and flags risks."
- **The sourcing-anomaly protocol:** when Alfred hits a sourcing anomaly it "literally drops its tools," pauses the build, writes a research brief, and hands it to Holmes for a verified, safe replacement. Execution "only resumes based exclusively on cited evidence" — citation-gated resumption.
- Holmes's verification stage produces the "Hardened Blueprint": evidence verified, risks mitigated.
- Goose is described as "The Hands" / the execution engine the triad uses — **note the tension** with the Holmes deck, which says Holmes itself is "built on the open-source Goose framework." Whether Goose is Holmes's substrate, Alfred's hands, or a shared runtime is **UNKNOWN** — audit contradiction-hunt item (Phase 2).

## From "GooseClaw: The Fiduciary AI Blueprint"

- **Holmes is entirely absent.** This deck uses a different triad: Alfred = "The Memory," GooseClaw = "The Body & Judgment," Goose = "The Hands." No component maps explicitly to Holmes.
- Relevant anyway for shared OS mechanics Holmes must coexist with: the Rule-9 approval gate (Observe → Draft → Surface → Authorize → Execute), the vendor denylist, the "Glass Box" isolation for untrusted skills, the Supply Chain Gate, path-confined memory boundaries, and per-spend financial authorization.
- **INFERRED:** GooseClaw's naming scheme likely predates or diverges from the triad naming in the other three decks. Reconciling the two architectures is an audit Phase 2 item.

## Cross-document constants (consistent across all decks that mention them)

- **Rule 9 / consent before consequence:** no commit, publish, push, spend, or destructive action without explicit human approval.
- **Vendor exclusion:** zero Meta, OpenAI, or xAI in the stack (the "Bouncer" / "Vendor Line" / "Exclusion Rail").
- **Local-first sovereignty:** keys on disk, nothing syncs unless chosen, telemetry structurally redacted.
- The human is the creator; the triad augments, never replaces.

## What none of the siblings contain — ABSENT (checked 2026-07-13)

No build roadmap, milestones, ship dates, or component build ordering, for Holmes or anything else. Every deck describes the *finished system's runtime behavior* in present-tense aspirational language, with no built-vs-planned markers.
