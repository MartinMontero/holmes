# Session audit + plan -- graph-engineering convention install

**Date:** 2026-09-05 (session; file dated 2026-09-04 per Martin's GO instruction). **Session artifacts folded here per GO edit #1 -- root AUDIT.md / PLAN.md dropped.**

---

# AUDIT.md â€” Session audit for the graph-engineering convention install

Date: 2026-09-05. Auditor: goose session. Labels per session constitution:
EXECUTED (I did it) / VERIFIED-LIVE (ran it this session, output in hand) /
CANON (checked-in repo statement) / REPORTED (someone else's evidence) /
UNVERIFIED (not checked or not checkable here).

## 0. Location discrepancy â€” flagged, resolved

- The session working directory `C:\Users\User\dev\holmes` is **not** the
  workspace: 9 loose files (3 edit scripts, 2 spec/orientation docs, 1 tmp
  file), no `Cargo.toml`, not a git repository. VERIFIED-LIVE (`dir`, `git
  status` â†’ "not a git repository").
- The workspace matching the task description (4-crate Rust workspace, CI,
  tag `v1.0.0-rc.1`) is `C:\Users\User\dev\holmes-clone`: git repo, remote
  `https://github.com/MartinMontero/holmes.git`, on `main`, tag
  `v1.0.0-rc.1` present. VERIFIED-LIVE.
- All work below happens in `holmes-clone`. If the intent was a different
  checkout, say so before GO.

## 1. Workspace shape (VERIFIED-LIVE, read from disk)

`Cargo.toml` workspace members: `holmes-core`, `holmes-guard`,
`holmes-smoke`, `holmes-wall`. Resolver 2, edition 2021, AGPL-3.0-or-later.

Dependency edges (from each crate's `Cargo.toml` â€” read, not guessed):

| Crate | Depends on | Third-party deps |
|---|---|---|
| holmes-guard | â€” (std only, by design) | none |
| holmes-core | holmes-guard | none |
| holmes-wall | holmes-guard | neo4rs 0.8.0, tokio 1, sha2 0.10 (dev: tempfile 3) |
| holmes-smoke | holmes-core, holmes-guard, holmes-wall | serde_json 1 |

- holmes-core `lib.rs`: modules `analysis`, `artifacts`, `observability`,
  `safety`; `provider` seam re-exports holmes-guard policy/resolution/spawn.
  Feature `investigative` exists, OFF by default, gates nothing yet (D-14a).
- holmes-guard bins: `acdl2-scan` (AC-DL-2 dependency-tree gate),
  `recipe-scan` (recipe safety scanner). Src: policy.rs, proxy.rs (L1a),
  resolution.rs (L1b), spawn.rs (L2), scan/.
- holmes-wall src: memory.rs, neo4j.rs, graph.rs, cypher.rs, ingest.rs,
  provenance.rs, supervise.rs.
- holmes-smoke: ACP round-trip harness (main.rs, lib.rs), bins
  `holmes-case`, `holmes-ingest`.

## 2. Test counts â€” VERIFIED-LIVE this session (`cargo test --release --locked`)

| Crate | lib | integration | total | Notes |
|---|---|---|---|---|
| holmes-core | 47 | 24 (analysis_locks 4, embedding_contract 3, no_blueprint_exports 2, observability_locks 6, safety_locks 9) | 71 | all pass |
| holmes-guard | 6 | 39 (acdl1_proxy 8, acdl1_resolution 8, acdl1_spawn 5, acdl1_structural 1, acdl2_scanner 14, recipe_scanner 3) | 45 | see Â§4 blocker |
| holmes-wall | 21 | 3 (no_delete_audit 1, wall_locks 2) | 24 | all pass |
| holmes-smoke | 2 | 0 | 2 | all pass |

Workspace total: 142. The task's "holmes-core 47 tests" = lib count only;
the full crate total is 71. RECIPE.md proofs use the full per-crate sums.

## 3. Scanners / existing gates â€” VERIFIED-LIVE this session

- `acdl2-scan --root .` â†’ exit 0, `verdict: CLEAN`, packages scanned: 157,
  files scanned: 67, 8 exemptions applied (all compiled-in, all visible).
- Positive control: `--lockfile crates/holmes-guard/tests/fixtures/planted.lock`
  â†’ exit 1, `verdict: FAIL (5 violations)`. Fires.
- `recipe-scan --path recipes` â†’ exit 0, `verdict: CLEAN`, files scanned: 2
  (`el-diablo.yaml`, `la-lluvia.yaml`).
- Positive control: `--path .../planted_recipe_smuggled.yaml` â†’ exit 1,
  `verdict: FAIL (21 hit(s))`. Fires.
- **Scanner scope (read in `src/scan/mod.rs`):** SCANNED_EXTENSIONS =
  rs, toml, yml, yaml, json, lock, sh, **ps1**, cfg, conf, ini; SKIPPED_DIRS
  = .git, target, docs, node_modules. Consequence: `gate.ps1` **will be
  scanned** and must carry no excluded-vendor identifiers; `.md` files are
  not scanned. Post-install file count must be re-verified (67 â†’ expect 68).

## 4. ENVIRONMENT BLOCKER â€” VERIFIED-LIVE

- `acdl1_proxy`'s positive control binds `127.0.0.1:11434` (the Ollama
  port). On this machine **local `ollama.exe` (PID 19840) is LISTENING on
  127.0.0.1:11434** (netstat). Result today: `cargo test -p holmes-guard`
  fails 1/8 in acdl1_proxy ("cannot bind 127.0.0.1:11434 ... os error
  10048"). Not a code defect â€” an environment precondition.
- Prior evidence: `cargo-test-release-locked-attempt6.log` (2026-09-02,
  REPORTED â€” checked-in-adjacent log, read by me) shows the full workspace
  run green including wall_locks â€” i.e., green when the port is free.
- I did **not** stop Martin's Ollama process (write action on a user
  process; needs explicit GO). gate.ps1 will therefore show an honest
  holmes-guard FAIL on this machine today, plus a preflight diagnostic
  naming the cause. A clean 10/10 local run requires freeing 11434.

## 5. CI â€” VERIFIED-LIVE (files read); required-status REPORTED

- `.github/workflows/acdl-gate.yml`: job `acdl-joint-gate`, ubuntu-latest,
  action-free (SHA-pin rule satisfied by construction). Steps: guard tests
  hermetic release; AC-DL-2 positive + negative controls; workspace tests;
  safety_locks + lib safety::; observability_locks + lib observability::;
  recipe-scan negative + positive controls.
- `.github/workflows/supply-chain.yml`: job `sbom-and-cve-scan`,
  action-free, pinned syft v1.48.0 / osv-scanner v2.4.0 / grype v0.116.0
  (checksum-verified installs); asserts osv-scanner.toml suppressions are
  exactly the ledgered D-13 four; OSV primary (exit 1 on any vuln), Grype
  cross-check `--fail-on high`. No Trivy (CVE-2026-33634).
- "Both required on main": REPORTED (task statement). Branch protection is
  GitHub-side; not verified from this shell. UNVERIFIED here.
- Dead branch `fix/windows-test-portability`: 0 commits ahead of main.
  VERIFIED-LIVE.

## 6. Windows-portability fixtures (GOOSE_PATH pattern) â€” VERIFIED-LIVE

`crates/holmes-guard/tests/acdl1_spawn.rs` lines 13â€“16:
`#[cfg(not(windows))] const GOOSE_PATH: &str = "/opt/holmes/goose";`
`#[cfg(windows)] const GOOSE_PATH: &str = r"C:\holmes\goose.exe";`
Hermetic â€” nothing is spawned; the sanitized command/env map is inspected.
acdl1_spawn passes on Windows today (5/5). This is the pattern new proofs
must follow: platform-branched constants, hermetic assertions.

`scripts/build-goose.sh` is bash-only (POSIX, `set -euo pipefail`) â€” the
one substrate build script; NOT Windows-portable. Flagged; not in scope for
the gate (it builds the external goose binary from a pinned commit, not
this workspace).

## 7. Docs / canon â€” VERIFIED-LIVE

- `docs/holmes-spec-v2.md` exists (244 lines), title v2.1 (QA-Corrected);
  header carries the sync rule: "the repo copy wins".
- CLAUDE.md: standing orders + protected block above a spec-derived
  re-seed marker (`BEGIN/END SPEC-DERIVED OPERATING CONTEXT`). A new
  "Project graph" section must land **above** the BEGIN marker (protected
  block region) â€” additive insert before "## Source of truth".
- `osv-scanner.toml`: exactly 4 IgnoredVulns (RUSTSEC-2024-0436,
  -2025-0012, -2024-0384, -2025-0134), all `informational: unmaintained`,
  all transitive via neo4rs 0.8.0, ignoreUntil 2026-10-17. This is the
  cargo-audit posture: fail-closed for anything scored/fixable (D-13).

## 8. Denylist posture for this task's own files

- New files introduce no dependencies at all (markdown + PowerShell).
- gate.ps1 (.ps1, scanned by acdl2-scan) will name no excluded-vendor
  identifiers; it references the scanners by binary name only.
- Post-install `acdl2-scan` rerun is part of the gate itself â€” the gate
  proves its own files clean.

---

# PLAN.md â€” Session plan: graph-engineering convention install

Subset of RECIPE.md for this session, per the convention being installed.
Subsystems touched, in dependency order, proof lines carried forward.

## Subsystems this session touches

| # | Subsystem | Needs | Proof (carried forward into RECIPE.md) |
|---|-----------|-------|----------------------------------------|
| 1 | RECIPE.md (new) | AUDIT.md findings | File exists at repo root; table rows = 10; ends with DRIFT RULE section |
| 2 | gate.ps1 (new) | RECIPE.md proof lines; cargo; acdl2-scan/recipe-scan bins | `powershell -File gate.ps1` prints "X/Y subsystems PASS" and exits 0 only if all pass; `-SelfTest` plants a failing proof, prints FAIL evidence, restores |
| 3 | CLAUDE.md "Project graph" section (additive) | RECIPE.md, gate.ps1 exist | CLAUDE.md contains "## Project graph" above "## Source of truth"; protected block (above the BEGIN SPEC-DERIVED marker) otherwise untouched; nothing below the marker touched |

## Impact surface (dependency order)

- RECIPE.md and gate.ps1 are scanned-relevant: gate.ps1 is `.ps1` â†’ in
  acdl2-scan's SCANNED_EXTENSIONS. After writing, rerun acdl2-scan:
  verdict must stay CLEAN; `files scanned` moves 67 â†’ 68 (exactly +1).
  That post-install number is what RECIPE.md's proof line records.
- CLAUDE.md is `.md` â†’ not scanned. Additive edit only.
- No existing source file is edited. No dependency added. No workflow
  touched. CI impact: none expected beyond the new scanned file.

## Execution order

1. AUDIT.md (done).
2. This PLAN.md.
3. RECIPE.md â€” 10 subsystem rows from AUDIT Â§1â€“Â§7, parallel tracks
   (holmes-core âˆ¥ holmes-wall after holmes-guard; docs/spec âˆ¥ all code;
   supply-chain posture âˆ¥ crates), DRIFT RULE section.
4. gate.ps1 â€” runs the 10 proofs in dependency order; PASS/FAIL each;
   "X/Y subsystems PASS"; exit 0 iff all pass; `-SelfTest` plants
   holmes-smoke expected-passed := 45 (holmes-guard's count â€” impossible
   for holmes-smoke), watches FAIL fire, restores to 2, reruns clean.
   Preflight: warn (never skip) if 127.0.0.1:11434 is occupied.
5. Re-verify acdl2-scan post-install; set RECIPE.md's files-scanned number
   to the verified post-install value.
6. Run gate.ps1 and gate.ps1 -SelfTest; capture full output for the report.
7. CLAUDE.md insert.
8. Prepare branch `feat/project-graph-convention` + PR body. STOP.
   RULE 9: no commit/push/merge/tag without Martin's explicit GO.

## Known environment caveat (carried into PR body)

Local ollama.exe holds 127.0.0.1:11434 â†’ holmes-guard proof FAILs locally
today (environmental, not code). Gate output will show this honestly.
Clean 10/10 requires the port freed â€” Martin's call (stop Ollama) or accept
CI green + attempt6 log (REPORTED) as the green evidence with the caveat.

## Wave-1 queue note (recorded per Martin's GO 2026-09-04 -- NOT changed in this PR)

acdl1_proxy's positive control should bind an ephemeral port (port 0)
instead of the fixed Ollama port 127.0.0.1:11434, so the suite is immune
to a local Ollama holding the port. Queued for the Holmes Wave-1 items;
no code change in this PR. (When it lands, RECIPE.md's holmes-guard proof
line and gate.ps1's preflight drift -- flag per the DRIFT RULE.)
