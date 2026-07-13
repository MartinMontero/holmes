# Decisions Ledger — Holmes

Decisions reserved for the human. Format: `D-## | Decision required | Context / options | Status (OPEN / DECIDED: <answer, date>)`

Agents must never resolve a D-item. A decision is DECIDED only when the human records it here (or says so explicitly and an agent records it verbatim).

**D-01** | **Relicense the repo?** | Current LICENSE is Apache-2.0 (F-001); standing gate requires AGPL-3.0-or-later or GPL-3.0. Options: (a) AGPL-3.0-or-later — strongest copyleft, matches the sovereignty ethos, network-use clause fits a service-shaped agent; (b) GPL-3.0 — copyleft without the network clause; (c) record an explicit constraint override. Decide before any code or outside contribution lands. | OPEN

**D-02** | **Deploy target?** | Blueprint implies self-hosted/local-first but never states the surface (F-002, F-007). Options include: local desktop daemon alongside GooseClaw; self-hosted server; hybrid. Firecracker requires Linux/KVM — this constrains laptop-local designs on macOS/Windows and must inform the choice. | OPEN

**D-03** | **Runtime architecture: what is Goose to Holmes?** | The decks contradict each other (F-004): Holmes "built on Goose" vs. Goose as "The Hands" belonging to Alfred/the triad vs. GooseClaw as a separately-named runtime. Decide the substrate and the component boundaries before the engineering spec. | OPEN

**D-04** | **Audit runner and venue** | The charter assumes a claude.ai project with `docs/case-file/` as KB, phase-gated with GO replies. Confirm venue and who drives (you paste-and-GO, or an agent session runs it end to end with stops at each phase). | OPEN
