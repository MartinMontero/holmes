# Beta Scope Decision — brief for Martin (D-14)

**Status:** PROPOSED — the agent produces this; **Martin rules.** Do not read a recommendation into it; both cuts are argued from evidence and the trade is a values call, not a technical one.
**Date:** 2026-07-20 · **Gate:** loop §6, the checkpoint immediately after Phase 2.5 (Safety before surface).

---

## The decision

The loop reaches a fork after Phase 2.5:

- **(A) Analytical open beta now.** Ship Phases 0–2.5 as the beta surface. Investigative mode (Phase 3) is compiled out or dark-flagged. Proceed to Phase 4 (integration/UX) then Phase RC (release candidate). Phases 3 and 5 continue *after* beta, behind the same gates.
- **(B) Full-surface beta.** Build Phases 3 → 4 → 5 → RC first; beta only once the investigative and accountability surfaces exist.

Pick one. The loop says do not pick for the human — so this brief lays out what each buys and costs, grounded in what Phase 2.5 actually produced.

## What is actually built and green (the evidence the decision rests on)

- **Phase 0** — guarded substrate: L1a egress allowlist, L1b provider/model resolution, L2 sanitized spawn; AC-DL-1/AC-DL-2 denylist gates and the action-free CVE gate, both binding CI checks; live guarded ACP round-trip on the smoke model. (PRs #7–#9.)
- **Phase 1** — analytical core: ACH, likelihood-ratio scorer, KAC, first-principles quarantine, six-phase case machine, lock-1a emission gate; one full live case end-to-end. (PR #10.)
- **Phase 2** — The Wall: owned bi-temporal temporal-graph subset on Neo4j via `neo4rs` (Graphiti dropped, D-12); invalidate-not-delete; AC-DL-1 §6 landed; supervised backend; weight provenance. (PR #11.)
- **Phase 2.5** — the safety layer (this session): injection-defense quarantine (2.5a), calibration + knowability gating at emission (2.5b), tool-approval protocol (2.5c), legal/defamation guardrails + anti-doxxing refusals (2.5d). 130 workspace tests green; fmt/clippy clean; denylist + recipe gates green.
- **Adversarial verification** — 14 defense claims independently attacked, 17 skeptic agents, 3 real gaps found and **fixed with locking regressions before this checkpoint** (F-034 cross-crate raw-bytes reach → `pub(crate)` + workspace firewall; F-035 invisible-Unicode coverage → extended smuggling vocabulary; F-036 forgeable consent → sealed `ConsentRecord`); 12 defenses held.

The load-bearing fact for this decision: **the safety layer Phase 3 would need already exists and has survived an adversarial pass.** Option A is not "ship without the safety work" — it is "ship *with* it, and withhold only the collection tools."

## The case for (A) — analytical open beta now

- **Real users, real feedback, lower blast radius.** The analytical surface (brief in → evidence pack out, handoff-only) takes *operator-supplied* material and reasons over it. It fetches nothing on its own, targets no one, and cannot act. The dual-use exposure that dominates Holmes's risk profile ("The Promethean Backfire": detection tools turned to surveillance) lives almost entirely in Phase 3's collection tools — which A withholds.
- **The calibration and knowability gates get exercised where it matters.** Uncalibrated confidence and low-knowability domains are exactly what an analytical beta will surface at volume; the 2.5b gate (and the downgrade path) get real pressure before the investigative surface multiplies the stakes.
- **Sovereignty story is already true for A.** Local-first review (D-10), handoff-only resolution, born-redacted telemetry, no phone-home — all hold for the analytical surface today.
- **Sequencing matches the loop's own thesis.** "Safety before surface" is satisfied for the analytical surface *now*; shipping it does not front-run any unbuilt guard.

Costs of A: the headline investigative capability (public records, OSINT, link analysis) is absent, so early adopters see the reasoning brain without the collection muscle; Phase 3/5 still must clear their gates later; two beta waves instead of one.

## The case for (B) — full-surface beta first

- **One coherent product.** Investigators get the whole method — collection through accountability — rather than a reasoning core they must feed by hand. The "method is the identity" thesis lands more completely.
- **The Phase 3/5 gates get built under real design pressure, not retrofitted.** Surveillance-detection asymmetry (3b), sandbox escape resistance (3c), and the Blacksky-derived accountability layer (Phase 5) are exactly the surfaces where post-hoc bolting is dangerous; building them before any external exposure keeps the hard gates ahead of users.
- **No half-capability first impression.** Avoids a beta that teaches users Holmes "can't collect."

Costs of B: materially longer to any external feedback; the highest-dual-use surface (Phase 3 tools) reaches users in the *same* release as everything else, concentrating risk; the sandbox and OSINT work (Firecracker/E2B self-host, OpenAleph, the `/proc/self/root`-class bypass tests) is substantial and still unbuilt; more unknowns between here and a shippable artifact.

## What does NOT change either way

- Every later phase stays behind its Rule-9 gate and its lock suite; neither cut relaxes a gate.
- The RC "beta safety re-run" (the whole 2.5 suite + AC-DL joint gate + doxxing refusals, from clean state) is required before any release candidate under both A and B.
- Merge of *this* PR still waits on Martin and the upstream source review (connector re-sync); the scope decision does not gate the Phase 2.5 code landing.

## The trade, in one line

**A** optimizes for *early, lower-risk feedback and a shorter path to a real release*, accepting a less complete first impression and a second beta wave. **B** optimizes for *a single complete product and building the highest-risk gates before any exposure*, accepting a materially longer path and concentrating the dual-use surface into one release.

This is a values-and-timing call about how Holmes meets its first users and when its highest-dual-use surface ships. It is yours.
