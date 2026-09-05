# RECIPE.md — the persistent dependency graph

Read this before any work in this repo. It maps every subsystem, what it
needs, and the **proof** that it works. A proof is a checkable criterion
ending in a number, a filename, or a named output — never a status message.
`gate.ps1` (repo root) runs every proof line below, in dependency order,
and exits 0 only if all pass.

Counts below were verified live on 2026-09-05 against
`cargo test --release --locked` per crate (see AUDIT.md §2). Test-count
proofs are exact on purpose: a legitimate change that adds tests makes the
proof **drift**, and the DRIFT RULE below governs that — flag it in the PR
body, never silently edit the number.

Platform note (born portable): proofs run under Windows PowerShell via
`gate.ps1`. The holmes-guard proof has one environment precondition —
loopback port `127.0.0.1:11434` must be free (the acdl1_proxy positive
control binds it; a local Ollama holds it). The GOOSE_PATH fixture pattern
(`#[cfg(windows)]` / `#[cfg(not(windows))]` constants, hermetic — nothing
spawned) is the convention any new platform-sensitive proof must follow.

## The graph

| Subsystem | Needs | Proof |
|---|---|---|
| holmes-guard (L1a egress proxy, L1b resolution, L2 sanitized spawn, AC-DL-2 scanner, recipe scanner) | Rust toolchain (cargo); loopback 127.0.0.1:11434 free | `cargo test --release --locked -p holmes-guard` exits 0 and summed `N passed` across all suites = **45** (6 lib + 39 integration: acdl1_proxy 8, acdl1_resolution 8, acdl1_spawn 5, acdl1_structural 1, acdl2_scanner 14, recipe_scanner 3) |
| holmes-core (embedding contract, ACH analysis locks, safety/emission gate, observability locks) | holmes-guard | `cargo test --release --locked -p holmes-core` exits 0 and summed `N passed` = **71** (47 lib + 24 integration: analysis_locks 4, embedding_contract 3, no_blueprint_exports 2, observability_locks 6, safety_locks 9) |
| holmes-wall (memory, Neo4j, supervise, provenance) | holmes-guard; neo4rs 0.8.0 pinned in `Cargo.lock` | `cargo test --release --locked -p holmes-wall` exits 0 and summed `N passed` = **24** (21 lib + no_delete_audit 1 + wall_locks 2) |
| holmes-smoke (Phase 0c ACP round-trip harness) | holmes-core + holmes-guard + holmes-wall | `cargo test --release --locked -p holmes-smoke` exits 0 and summed `N passed` = **2** |
| AC-DL-2 dependency-denylist gate | holmes-guard (`acdl2-scan` bin); `Cargo.lock` | `cargo run --release --locked -p holmes-guard --bin acdl2-scan -- --root .` exits 0 with `verdict: CLEAN`, `packages scanned: 157`, `files scanned: 68`; the planted control `--lockfile crates/holmes-guard/tests/fixtures/planted.lock` exits non-zero with `verdict: FAIL` |
| Recipe safety scan (Lock 1d) | holmes-guard (`recipe-scan` bin); `recipes/` | `cargo run --release --locked -p holmes-guard --bin recipe-scan -- --path recipes` exits 0 with `verdict: CLEAN` and `files scanned: 2`; the planted control `--path crates/holmes-guard/tests/fixtures/planted_recipe_smuggled.yaml` exits non-zero with `verdict: FAIL` |
| CI joint gate | all rows above | `.github/workflows/acdl-gate.yml` contains job `acdl-joint-gate` and `.github/workflows/supply-chain.yml` contains job `sbom-and-cve-scan` (2 filenames, 2 job names). Live required-status on `main` is GitHub-side — not locally provable; treat as REPORTED unless checked on the PR page |
| Supply-chain posture (cargo audit posture, D-13) | `osv-scanner.toml` | the `RUSTSEC-####-####` ID set in `osv-scanner.toml` equals exactly **4** IDs: RUSTSEC-2024-0384, RUSTSEC-2024-0436, RUSTSEC-2025-0012, RUSTSEC-2025-0134 (mirrors the CI rider-b assertion; the live OSV/Grype scan itself runs in `sbom-and-cve-scan`) |
| Windows-portability fixtures (GOOSE_PATH pattern) | holmes-guard tests | `crates/holmes-guard/tests/acdl1_spawn.rs` contains **both** `#[cfg(windows)]` with `C:\holmes\goose.exe` and `#[cfg(not(windows))]` with `/opt/holmes/goose` — platform-branched constants, hermetic assertions |
| Spec canon (`docs/holmes-spec-v2.md`) | — | `docs/holmes-spec-v2.md` exists and contains the sync-rule marker `repo copy wins` (bolded in source, `docs/holmes-spec-v2.md` line 7) |

## Parallel tracks

After **holmes-guard** (the only zero-dependency code subsystem),
**holmes-core** and **holmes-wall** share zero dependencies — each needs
only holmes-guard — and can proceed in parallel. Independent of all code:
**spec canon** (docs-only) and **supply-chain posture** (osv-scanner.toml
only) share nothing with the crates or with each other. **holmes-smoke**
is the merge point: it needs core + guard + wall and can start only after
all three. The two scanner gates (AC-DL-2, recipe) need only holmes-guard
plus their inputs (`Cargo.lock`, `recipes/`) and run parallel to core/wall.

## DRIFT RULE

Every proof line is re-verified by any PR that touches its subsystem. If a
proof drifts — a test count moves, a scanned-file count moves, a fixture
path changes, a port precondition changes — the PR body **flags the drift
explicitly** (old value → new value → why). A drifted proof is never
silently edited, in this file or in `gate.ps1`. An unexplained drift is a
blocked PR, not a documentation update.
