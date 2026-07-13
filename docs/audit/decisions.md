# Decisions Ledger — Holmes

Decisions reserved for the human. Format: `D-## | Decision required | Context / options | Status (OPEN / DECIDED: <answer, date>)`

Agents must never resolve a D-item. A decision is DECIDED only when the human records it here (or says so explicitly and an agent records it verbatim).

**D-01** | **Which license governs Holmes?** | Current LICENSE is Apache-2.0 (F-001). The *generic* audit template's gate requires AGPL-3.0-or-later or GPL-3.0 — but `docs/holmes-spec-v2.md` §7 Phase 0 specifies an "Independent **Apache-2.0-compatible** repo" (goose itself is Apache-2.0), and the constitution in the orientation doc lists no copyleft gate. Options: (a) keep Apache-2.0 as the spec implies — the existing LICENSE stands; (b) AGPL-3.0-or-later — copyleft posture, still compatible with Apache-2.0 *dependencies*; (c) GPL-3.0. Note "Apache-2.0-compatible" is ambiguous between (a) and (b); only you can say which was meant. Decide before any code or outside contribution lands. | OPEN — new evidence 2026-07-13

**D-02** | **Deploy target?** | Blueprint implies self-hosted/local-first but never states the surface (F-002, F-007). Options include: local desktop daemon alongside GooseClaw; self-hosted server; hybrid. Firecracker requires Linux/KVM — this constrains laptop-local designs on macOS/Windows and must inform the choice. | OPEN — **largely answered by spec v2 §4.1** (Tauri 2 + SolidJS desktop, goose over ACP, self-hosted Graphiti). Residual question, per spec §8: sandbox choice per workload/OS (Firecracker vs V8-isolate path) — decide at Phase 3.

**D-03** | **Runtime architecture: what is Goose to Holmes?** | The decks contradict each other (F-004): Holmes "built on Goose" vs. Goose as "The Hands" belonging to Alfred/the triad vs. GooseClaw as a separately-named runtime. Decide the substrate and the component boundaries before the engineering spec. | OPEN — **spec v2 §4.1/§5 answers this** (Holmes and Alfred are siblings on goose over Zed's ACP; decks superseded). Recommend closing as DECIDED-BY-SPEC; needs your confirmation, not mine.

**D-04** | **Audit runner and venue** | The charter assumes a claude.ai project with `docs/case-file/` as KB, phase-gated with GO replies. Confirm venue and who drives (you paste-and-GO, or an agent session runs it end to end with stops at each phase). Context added 2026-07-13: a v1→v2 QA pass already happened (folded into spec v2), and the orientation doc defines the operating loop (Claude Code builds + emits readouts → claude.ai project pressure-tests). The fresh adversarial audit may be redundant with that loop, or valuable as a pre-Phase-0 gate — your call. | OPEN

**D-05** | **Phase 0 smoke-test mode** | From `docs/holmes-project-orientation.md` §6 open threads: cloud Tier-1 key vs. fully offline Tier-2 via Ollama for the Phase 0 smoke test. Blocks running the Phase 0 kickoff. | OPEN (carried over from the originating conversation)
