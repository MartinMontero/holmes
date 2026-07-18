# Alfred as Holmes's Security Perimeter — Technical Overview

> **PROVENANCE — read first.** Reconstructed 2026-07-17 from the 2026-07-13 session record (original authored and presented in the claude.ai session "Alfred's security infrastructure vulnerabilities"). Passages the record preserves verbatim are carried verbatim; the remainder is re-authored from the same analysis by the same surface. If the original file is recovered from that chat, diff before committing and prefer the original body; keep the §7 addendum either way — it post-dates the original and changes the evidence picture.

**Status:** Analysis artifact from the claude.ai pressure-testing surface, 2026-07-13 (reconstructed 2026-07-17). Consolidates a concern raised across the 2026-07-06 QA stress-test and the 2026-07-13 kickoff-v3 session, grounded in `holmes-spec-v2.md` (§3.1, §4.6, §6.6, §7) and `holmes-vs-wcjbt.md` (§7). Confidence convention as in the canon: unmarked = primary-source-verified; **[DIRECTIONAL]** = secondary/asserted; **[NEEDS-CAVEAT]** = concept holds, detail unconfirmed.

**Scope honesty, stated up front:** this is not a code audit of Alfred. As of the original writing, the Alfred repository had never been read from this surface — `holmes-vs-wcjbt.md` §7 records it as unverifiable from public primary sources, and every Alfred-internal fact below carries that provenance. What was discovered is structural and epistemic: *where Holmes's security guarantees actually terminate, and the verification status of the infrastructure they terminate in.* No claim is made that Alfred's infrastructure is deficient. The claim is that no evidence exists in this project's record either way — and by the constitution's own standard, "unverified" and "enforced" cannot describe the same control. (§7 updates this picture as of 2026-07-17.)

---

## 1. The finding — perimeter relocation

The settled decision to embed Holmes as library crates (`holmes-core`, `holmes-guard`) inside Alfred — no standalone UI, installer, or updater — is architecturally sound and was made for good reasons (one product surface, one signed artifact, no parallel update channel to secure). Its unavoidable consequence: **Alfred's built, signed artifact becomes the terminal enforcement point for every Holmes constitutional guarantee.** A library cannot enforce what only a process, an installer, or an OS boundary can enforce. The guarantees didn't get weaker; they moved. The new location has not been audited with the rigor the old design required of itself.

## 2. The four relocated obligations

These are load-bearing constitutional promises whose enforcement now lives on Alfred's side of the boundary:

1. **Artifact-level guard test (CI).** The denylist must be proven present and active in the artifact Alfred actually ships — not in Holmes's source tree. A CI test in Alfred's pipeline must exercise the built product and assert excluded vendors are unreachable. Status at original writing: no evidence.
2. **OS/artifact-level egress enforcement.** `holmes-guard`'s in-process egress proxy governs Holmes-spawned work; goose and MCP servers are separate processes a library cannot fence. The out-of-process residual (a hostile binary ignoring proxy env escapes the library boundary) is enforceable only at Alfred's process/OS layer. Status: no evidence.
3. **Signed update channel with rollback.** Every future Holmes guarantee reaches users through Alfred's updater. An unsigned or non-rollbackable channel converts one compromise into fleet-wide compromise. Status: no evidence.
4. **Memory/resurfacing channel.** Loop E and any surfacing of stored findings ride Alfred's path-confined, born-redacted memory discipline (per `epistemic-canon-Holmes.md` §5: reuse the audited channel, never add a new one). Anything that resurfaces stored content can resurface poisoned content. Status: no evidence.

## 3. The safety spine carried [DIRECTIONAL]

The spec instructs Holmes to "reuse Alfred's safety spine." Every component of that spine was, at original writing, asserted from context rather than verified against Alfred's code — all **[DIRECTIONAL]**: Windows Job Object kill-on-close orphan prevention; the recipe safety scanner (invisible/deceptive-Unicode strip + pre-flight preview); deny-by-default tool-permission gating; born-redacted, opt-in, local-only telemetry; path-confined, traversal-proof MCP tools; and Rule 9 as implemented (not as documented).

## 4. Substrate note

goose ships with broad provider support **including denylisted vendors**. Denylist enforcement therefore cannot be assumed from the substrate at any layer — it must be imposed by Holmes/Alfred configuration and compiled enforcement, and proven at the artifact level (obligation 1).

## 5. Precedent

**CVE-2026-33634** (the March 2026 `trivy-action` compromise: 76 of 77 tags force-pushed, weaponized binary across five distribution channels, CI credential stealer) is the canonical demonstration that the update/distribution channel is the highest-leverage attack surface. It is the reason obligation 3 is a ship-blocker, not a nicety, and the reason all Actions are SHA-pinned.

## 6. Closure conditions

Each item closes by moving from [DIRECTIONAL] to VERIFIED with file/line (or CI-run) evidence recorded in `STATE.md`, or becomes a ledgered Alfred work item:

1. Read-only verification pass over the Alfred sibling repo (pre-flight already requires it on disk — this is an afternoon, not a project): confirm LICENSE, locate and read the guard, updater, telemetry, tool-gating, and memory implementations.
2. Artifact-level guard test added to Alfred CI (obligation 1) — ledgered as a cross-repo obligation until green.
3. Egress residual documented honestly in `docs/security.md`; OS/artifact-level enforcement recorded as Alfred's obligation; D-## opened if the residual is unacceptable.
4. Updater: signing + rollback verified against the implementation, not the README (obligation 3).
5. Memory channel: Loop E rides the audited path only; memory-poisoning controls verified before any resurfacing feature ships (obligation 4).
6. `holmes-guard` adoption plan: retire Alfred's app-side TypeScript guard (ships as editable text; strippable) in favor of the compiled Rust guard.

RC does not exit while any of the four obligations remains [DIRECTIONAL].

---

## 7. ADDENDUM — 2026-07-17 evidence update (live `MartinMontero/Alfred` view)

Direct observation of the public Alfred repo (GitHub UI, 2026-07-17) materially updates §§2–3 — from *"no evidence either way"* to *"corroborated [DIRECTIONAL], verification now cheap and timely."* README and commit messages are declarations, not verification — declared ≠ enforced applies to Alfred's own README — but the following is now on record:

- **License (D-01 evidence — RESOLVED).** GitHub license detection: AGPL-3.0. LICENSE commit: "feat: relicense AGPL-3.0, re-identify Onyx→Alfred." README Provenance & License: Alfred is **AGPL-3.0-or-later** as its only license; the upstream `derekross/onyx` MIT notice is preserved verbatim in `THIRD-PARTY-NOTICES/` as **attribution for that code, not a license over Alfred**, with provenance in `ATTRIBUTION.md`, `UPSTREAM.md`, `NOTICE`. This is the correct legal structure and discharges the onyx-MIT concern that reopened D-01. The "match Alfred" rationale for Holmes's AGPL-3.0-or-later holds.
- **Obligation 3 (updater) — in active completion.** `src-tauri`: "release public key wired (ceremony complete)" and workflows: "full tauri-plugin-updater wiring — ceremony-bl…" — both within hours of this addendum. Signing evidence is now fresh; **rollback capability remains unverified**. Verify while the ceremony is current.
- **Obligation 4 (memory) — partial.** `mcp/`: "memory-poisoning review gate, enforced at th…" (≈4 days prior); README Phase 5 lists "memory-poisoning + privacy controls" as *still ahead*. Treat as in-flight; do not build Loop E surfacing against it yet.
- **Obligations 1–2 (guard/egress) — concern intact.** README claims a Phase 1 "app-side provider denylist" and Phase 4 "excluded vendors unreachable through Alfred"; language split **TypeScript 80.9% / Rust 5.2%** is consistent with the guard living app-side in editable TS. No artifact-level CI guard test visible. This remains the widest gap and the core case for `holmes-guard` adoption.
- **Spine context.** README: Phases 0–4 complete, Phase 5 (observability & agent safety) in progress; `SECURITY.md` carries a STRIDE threat model + security policy; deny-by-default tool gating, recipe safety scanner, and born-redacted telemetry are claimed shipped in Phase 5-to-date.

**Consequence for sequencing:** the §6.1 read-only pass should run this week — the updater evidence is perishable, Alfred's Phase 5 is mid-flight (verification can influence it rather than post-audit it), and Holmes RC remains blocked on obligations 1–4 regardless of Holmes-side progress.

---

*Cross-references: `holmes-spec-v2.md` §3.1 (Ona incident), §4.6 (safety spine), §6.6 (CVE-2026-33634), §7 (phase locks); `holmes-vs-wcjbt.md` §6.4 (invariants), §7 (Alfred sourcing [NEEDS-CAVEAT]); `epistemic-canon-Holmes.md` §5 (Loop E plumbing invariant); kickoff v3 — `STATE.md` cross-repo obligations ledger, Phase 0/6 locks; D-01 (license ratification — evidence satisfied per §7 addendum, 2026-07-17).*
