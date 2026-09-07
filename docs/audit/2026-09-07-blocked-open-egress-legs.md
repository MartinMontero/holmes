# BLOCKED — open-egress live legs (no simulation, per tasking)

Date: 2026-09-07. Three carried legs need an open-egress host. Per
tasking: **nothing here was simulated or run against a substitute.**
Each entry: the exact host requirement and the exact commands to run
once such a host exists. `$GOOSE` below = the pinned substrate binary
built by `scripts/build-goose.sh` (`aaif-goose/goose` @
`8e78960e535ab7f34630e7c5921a42f146cbc9f4`, trimmed features; emits
`PROVENANCE.txt`). The script is bash — the open-egress host needs bash
(Linux, or Windows + Git Bash per the repo's Git-Bash path session).

## Common host requirement

A host with **unfiltered outbound HTTPS** — specifically reachable:
`github.com` + `static.crates.io`/`index.crates.io` (builds),
`production.cloudfront.docker.com` or `dist.neo4j.org` (leg 2),
`ollama.com` + `registry.ollama.ai` (leg 3), and the permitted provider
endpoint for any Tier-1 leg (`api.anthropic.com:443` through L1a, funded
key via the `MY_`-prefixed env convention — never `ANTHROPIC_API_KEY`,
which is reserved/auto-stripped). The Phase-0 container failed all four
(403s); this Windows host has Ollama but is not the sanctioned run
environment — hence BLOCKED, not run.

---

## BLOCKED entry 1 — Reader process-separation live leg (Phase 2.5a residual)

**What it proves:** the quarantined reader runs as a **separate
no-tools model session** behind the L2 sanitized spawn — so a hostile
backend *implementation* (not just a hostile document) has no in-process
path to the case handle. Until this runs: hostile-backend-implementation
resistance is **design posture** (docs/security.md, stated there
verbatim).

**Harness status: ABSENT.** No env-gated live-leg test or bin exists
for the reader (`crates/holmes-core/src/safety/reader.rs` has no env
gate; `holmes-smoke` has `holmes-case` / `holmes-ingest` only). The
hermetic suite (`safety_locks`, incl. the planted `instruction.html` /
`smuggled.txt` hostile fixtures) is green today — that is the
in-process contract, not the process-separation leg.

**Host requirement:** common host above + a funded permitted key
(Tier-1 leg) or local Ollama (Tier-2 variant).

**Exact commands:**

```
# 0. substrate
GOOSE_SRC_DIR=$HOME/goose-src bash scripts/build-goose.sh
GOOSE=$HOME/goose-src/target/release/goose

# 1. implement the harness (the work item itself; name PROPOSED):
#    crates/holmes-smoke/src/bin/holmes_reader_leg.rs — spawns ONE
#    no-tools goose session via holmes_guard::spawn::sanitized_spawn,
#    passes crates/holmes-core/tests/fixtures/hostile/instruction.html
#    as the untrusted document + an extraction request, seals the
#    returned Extraction, and asserts downstream that (a) no tool grant,
#    phase transition, or handoff was minted from extraction text,
#    (b) the raw-byte canary never reaches the emitted pack,
#    (c) the session's L1a egress log shows only the permitted provider.

# 2. run the leg (Tier-1)
cargo run --release --locked -p holmes-smoke --bin holmes-reader-leg -- \
  --goose "$GOOSE" --provider anthropic --model claude-sonnet-5 \
  --credential-env MY_ANTHROPIC_API_KEY \
  --transcript docs/audit/evidence/2.5a-reader-leg-$(date +%F).json
# expect: exit 0; transcript born-redacted; egress 1/1 api.anthropic.com allowed

# 3. negative control (must refuse, exit 3)
cargo run --release --locked -p holmes-smoke --bin holmes-reader-leg -- \
  --goose "$GOOSE" --provider openrouter --model anything
```

---

## BLOCKED entry 2 — Neo4j Wall live leg (Phase 2 lock 2a)

**What it proves:** invalidation-not-deletion against a **real Neo4j** —
supersede appends (count +1), the superseded record is preserved +
flagged, history stays queryable. Hermetic contract
(`InMemoryWall`, `no_delete_audit`) is green; the live leg is authored
and env-gated (`tests/wall_locks.rs::lock2a_invalidation_not_deletion_against_live_neo4j`
on `HOLMES_NEO4J_URI`/`_USER`/`_PASSWORD`; skips, never fails, when
unset — verified skipping in the 2026-09-05 sweep).

**Host requirement:** common host above + ONE of: Docker (image pull),
or `dist.neo4j.org` download, or an existing Neo4j 5.x instance with
Bolt on 127.0.0.1:7687. (neo4rs 0.8.0, Bolt, localhost-only per
`osv-scanner.toml` rationale.)

**Exact commands:**

```
# 1. Neo4j Community (GPLv3, per D-09/D-12). Pin the minor at run time
#    after checking hub tags; example:
docker run -d --name holmes-neo4j \
  -p 127.0.0.1:7687:7687 -p 127.0.0.1:7474:7474 \
  -e NEO4J_AUTH=neo4j/holmes-lock2a-pw \
  neo4j:5-community
# (or: supervised variant — crates/holmes-wall/src/supervise.rs
#  SupervisedBackend can own the child lifecycle; kill-on-drop proven
#  hermetically. Wire it in the same session if wanted.)

# 2. the live leg
set HOLMES_NEO4J_URI=bolt://127.0.0.1:7687 & set HOLMES_NEO4J_USER=neo4j & set HOLMES_NEO4J_PASSWORD=holmes-lock2a-pw
cargo test --release --locked -p holmes-wall --test wall_locks -- --nocapture
# expect: lock2a RUNS (no SKIP line), 1 passed; lock2b still passes.

# 3. record: paste the output into STATE.md lock 2a (supersede, never
#    delete the env-gated note); commit the STATE.md line only.
```

---

## BLOCKED entry 3 — Tier-2 Ollama ingestion-quality numbers (Phase 2 lock 2c)

**What it proves:** the deterministic `holmes_wall::ingest` scorer's
failure-rate report (grounded / ungrounded-citation /
claim-exceeds-citation / malformed) on the **Tier-2 local model**, not
the Tier-1 smoke model. The scorer is identical for both tiers and the
Tier-1 run is on record (6/6 grounded, transcript committed); the
Tier-2 numbers are the carried gap.

**Host requirement:** common host above + Ollama installed
(`ollama.com` egress) + the model pulled (`registry.ollama.ai` egress)
+ `$GOOSE` built. The run itself is loopback-only (L1a permits
127.0.0.1:11434; no cloud endpoint — AC-DL-1 §6).

**Exact commands:**

```
# 1. model — the memory layer's default Tier-2 id (memory.rs):
ollama pull qwen3.6-flash
# (Tier-2 roster alternates per spec v2.1: qwen3.5-27b /
#  qwen3.6-35b-a3b / magistral-small / gemma — a roster run is the
#  same command repeated per model; model choice is Martin's.)

# 2. the leg
cargo run --release --locked -p holmes-smoke --bin holmes-ingest -- \
  --goose "$GOOSE" --provider ollama --model qwen3.6-flash \
  --transcript docs/audit/evidence/2c-ingest-transcript-tier2-$(date +%F).json
# expect: exit 0; born-redacted transcript; L1a egress log shows ONLY
# 127.0.0.1:11434 (zero cloud endpoints — assert this in the session notes)

# 3. record: the grounded/ungrounded/exceeds/malformed counts go into
#    STATE.md lock 2c next to the Tier-1 6/6; commit transcript + line.
```

---

**Standing note for all three:** when any leg lands, RECIPE.md's
relevant proof line drifts (a SKIP becomes a run; a count may move) —
flag in that PR's body per the DRIFT RULE, never silently edit.
