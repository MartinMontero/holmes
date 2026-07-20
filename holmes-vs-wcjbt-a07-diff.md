# holmes-vs-wcjbt.md — A-07 canon diff (§6.2) · authored upstream 2026-07-19

**Status:** Upstream canon edit per the sync rule (canon is authored on the pressure-testing surface; the repo lands it via session PR; byte-identical copies re-propagate). Discharges **A-07** (PROPOSED → LANDED on merge). Format mirrors the proven `holmes-spec-v2.1-diff.md` pattern: every FIND must grep-match **exactly once** before editing; mismatch → STOP and report, do not improvise.

**E1 — evidence_pack gains the A-07 fields (exactly per the ledger entry).**

FIND:
```
  risk_flags: [..]
  recommendation: string|null  # options/risks only — never "build X"
```

REPLACE:
```
  risk_flags: [..]
  recommendation: string|null  # options/risks only — never "build X"
  knowability: enum[high_validity, low_validity]
                               # A-07 / Upgrade B: assigned deterministically,
                               # BEFORE any confidence score; never model-
                               # inferred (epistemic canon §3, §5); shares
                               # vocabulary with WCJBT intuition_validity
  limits_of_this_finding:      # A-07: required non-empty at emission (lock 1a)
    what_would_change_the_conclusion: [string]
    what_could_not_be_checked: [string]
    where_the_evidence_runs_out: [string]
```

**E2 — research_brief records the firewalled certainty field. [COMPANION — beyond A-07's letter, within canon §3's mandate; implemented in `holmes-core` since Phase 0; Martin approves or strikes this edit independently of E1.]**

FIND:
```
  scope: string
  catalog_seed: [catalog_ref] # Loop B: search catalog first
```

REPLACE:
```
  scope: string
  catalog_seed: [catalog_ref] # Loop B: search catalog first
  stated_confidence: float|null
                               # builder's self-reported certainty (epistemic
                               # canon §3) — recorded, then FIREWALLED: nothing
                               # in the analytical core reads it (canon §5;
                               # enforced by structural regression test)
```

**Apply procedure (session-side):** session branch → grep-verify each FIND count = 1 → apply → confidence-marker sweep unchanged → amendments.md: A-07 → LANDED (E2 noted with Martin's ruling) → PR → merge on go-ahead.

**Propagation (post-merge, the sync rule's second half):** `holmes-vs-wcjbt.md` is a byte-identical shared canon body across the WCJBT / Holmes / Alfred surfaces. After the repo copy lands: re-propagate the amended file to the other two project-knowledge copies and re-sync the Holmes connector, so no fork of the canon exists anywhere. The root-level duplicate (`holmes-vs-wcjbt.md.md`) observed in project knowledge should be reconciled or removed in the same pass — two spellings of one canon file is drift waiting to happen.

*Cross-references: `docs/audit/amendments.md` A-07; `holmes-core::artifacts` (`Knowability`, `LimitsOfThisFinding`, `ResearchBrief.stated_confidence`); `analysis/emission.rs` (lock-1a enforcement); epistemic canon §3 (Upgrade B), §5 (firewall).*
