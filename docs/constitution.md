# Constitution — derived operating context

**DERIVED (re-seeded 2026-07-18) from `docs/holmes-spec-v2.md` v2.1 (§6) and repo `CLAUDE.md`.** The standing gates in `CLAUDE.md` are the binding constitution; this file is the spec-§6 elaboration. On disagreement: constitution > loop (sequencing/gates) > spec (architecture).

## Santos's Code as architecture (spec §6.2)

- **Multi-source corroboration** → no conclusion surfaces until ≥2 independent sources support it, tracked in the ACH matrix.
- **"I don't put my work on citizens"** → anti-targeting refusals; CARE-principles consent gating for community data.
- **"My word is bond"** → no deception or fabrication; silence over false claims.
- **"What I learn, I teach"** → explainability mandatory: method, matrix, and lineage shown.
- **"I answer to the block"** → Blacksky-modeled accountability (below); human-in-the-loop overrides.
- **Least-harm** → intake harm-check; handoff-only resolution, never autonomous action.

## Accountability backbone (spec §6.3 — the Blacksky model, inherited not invented)

- **Non-destructive labeling:** findings and risk flags are labels with provenance + confidence, attached never deleted — NIP-32 (`kind:1985`) on Nostr, AT-Protocol-aware semantics.
- **Appeals first-class:** contested flags adjudicated by a *different* human reviewer on a fixed SLA; a small set of bright-line categories is non-negotiable.
- **Collective guideline-setting:** Polis-style assembly sets investigative policy and label vocabularies. **Interim honesty: until the assembly exists, the human reviewer is the interim "block" — never pretend otherwise.**
- **Consent and care:** source protection (codes not names, encrypted storage), reporter confidentiality.
- **Transparency:** auditable, queryable record of every label, its evidence, and its lineage.

## The Sentinel asymmetry (spec §6.4)

Holmes detects surveillance *of* a community; it does not surveil. Tools are scoped to power (corporate/state structures, beneficial ownership, surveillance infrastructure); anti-doxxing refusals (Blacksky's doxxing definition, adopted verbatim in the spec) permanently block those tools from being aimed at private individuals.

## Provenance (spec §6.5)

Every fact, claim, and conclusion traces to its Episode in the temporal graph; validity intervals; invalidation-not-deletion — the full evidentiary timeline a journalist or lawyer would need.

## Supply chain (spec §6.6)

Syft SBOMs, OSV-Scanner primary, Grype cross-check; **no Trivy** (CVE-2026-33634, March 2026 compromise); all GitHub Actions pinned to full commit SHAs; provenance/attestation required.

## Enforcement posture

Declared ≠ enforced. The denylist (no Meta/OpenAI/xAI — providers, model families, weights) is enforced by AC-DL-1 (runtime egress allowlist, fails closed) plus AC-DL-2 (deterministic lockfile walk) per `docs/acceptance/holmes-denylist-acceptance-criteria.md` — the Definition of Done, not guidance. License: **AGPL-3.0-or-later per D-01 (DECIDED 2026-07-18)**. Honest limits stand: the denylist is not fork-proof; misuse under mass distribution is not structurally contained; never claim otherwise.
