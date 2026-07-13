# Build Phases — SUPERSEDED

**Status: SUPERSEDED (2026-07-13).** The canonical phased roadmap is **`docs/holmes-spec-v2.md` §7** — Phases 0–5, each with a "next-build lock":

- **Phase 0 — Scaffold.** Repo + knowledge files seeded from the spec; Tauri 2 + SolidJS shell; goose over ACP; denylist as runtime guard **and** regression test; supply-chain CI (Syft/OSV-Scanner/Grype, SHA-pinned actions, no Trivy). *Lock: denylist + ACP round-trip verified end-to-end.*
- **Phase 1 — Analytical core.** Three engines + six-phase state machine as goose recipes/subagents. *Lock: ACH matrix + multi-source corroboration gate operational.*
- **Phase 2 — The Wall.** Self-hosted Graphiti; Adaptive RAG router; full provenance lineage. *Lock: every fact traceable + invalidation-not-deletion verified.*
- **Phase 3 — Investigative mode.** MCP servers for records/OSINT/link-analysis; OpenAleph; Firecracker sandbox. *Lock: tools gated behind the safety layer; surveillance-detection asymmetry enforced.*
- **Phase 4 — Observability & safety.** Born-redacted telemetry; recipe safety scanner; deny-by-default permissions. *Lock: Skillsmith-style security audit passed.*
- **Phase 5 — Accountability layer.** Blacksky-derived labeling (NIP-32), appeals, Polis assembly hook, transparency log, CARE consent gating. *Lock: "I answer to the block" demonstrably live.*

The Phase 0 entry point is `holmes-claude-code-kickoff-phase0-v2.md` — **not yet in this repo** (F-009).

The pre-audit draft that previously occupied this file (Phases A–F, INFERRED from the blueprint decks) was written before the spec surfaced and is preserved in git history (commit `762751a`) for provenance. It got the shape roughly right but is not authoritative; do not build from it.
