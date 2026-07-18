# Security — derived operating context

**DERIVED (re-seeded 2026-07-18) from `docs/holmes-spec-v2.md` v2.1 (§3.1, §4.6, §6.6) and Master Build Loop v2 §6 Phase 0.** Markers carried verbatim. **Phase 0 build status:** the guard (`holmes-guard`: L1a/L1b/L2) and the AC-DL-2 gate are implemented and green in release-mode CI; the honest limits below are executed-evidence, not design posture. Sandbox/secrets/telemetry sections remain design posture until their phases build.

## The guard (loop §6 Phase 0 — pending spec carry, A-04)

Three explicit layers, **all policy in compiled Rust** (a UI may *read* the list, never enforce it):
- **L1a — network egress allowlist:** deny-by-default local egress proxy owned by `holmes-guard`; permitted hosts compiled/core-owned; every Holmes-spawned session forced through it.
- **L1b — provider/model-id resolution guard:** permitted ids pass; everything else — including *unknown* ids — rejected at resolution time.
- **L2 — sanitized spawn:** `goose acp` via recorded absolute path; provider env vars stripped; `HTTP(S)_PROXY` → L1a, `NO_PROXY` cleared; resolved model id validated against L1b before and after handshake.
- **L3** (provider-stripped goose distro): deferred — recorded, not built.

Enforcement standard: **AC-DL-1** (runtime, fails closed) + **AC-DL-2** (deterministic lockfile walk) per `docs/acceptance/holmes-denylist-acceptance-criteria.md` (v3); all in-phase criteria green in one CI run is the only compliant state.

**Denylist ≠ CVE scan (AC-DL-2 §6).** AC-DL-2 is *provider exclusion* — no Meta/OpenAI/xAI package in the dependency tree — and is deliberately distinct from the supply-chain **CVE** gate (Syft SBOM + OSV-Scanner + Grype, no Trivy). Both run in CI; neither substitutes for the other. `litellm` should trip **both** — a useful cross-check.

## Sandbox policy (spec §3.1, §4.6)

Any model-generated code runs in a **Firecracker microVM** (E2B OSS self-hosted), **no outbound network by default**. bubblewrap only for trusted, in-house, read-only tasks — justified by the documented Ona incident (`/proc/self/root` denylist bypass + self-disabled sandbox; primary source). The `rm -rf ~/` anecdote is separate and `[DIRECTIONAL]` — do not bundle the two.

## Secrets

**BYOK invariant.** The shipped `holmes-guard` crate reads no vendor credential and requires no vendor, key, or env var of its own — users bring their own keys; the guard governs *where* they may reach (L1a allowlist) and *which* provider a key is admitted for (L2 per-provider credential seam). The smoke-test key is build/CI-only, supplied at run time via `--credential-env` and injected only through the seam. Headless spawns set `GOOSE_DISABLE_KEYRING=1` and the pinned goose build compiles the `system-keyring` feature out. At-rest storage backend for embedders that opt into a keyring: OS credential store (goose platform keyring `[NEEDS-CAVEAT — confirm exact backend per platform in goose docs]`); never in files, never logged.

## Telemetry

Born-redacted, opt-in, local-only: counts, durations, names — never content, prompts, or secrets. Feedback is user-initiated export only; no phone-home.

## Supply chain (spec §6.6)

Syft SBOMs (CycloneDX + SPDX), OSV-Scanner primary, Grype cross-check, **no Trivy** (CVE-2026-33634 — the March 2026 `trivy-action` tag force-push compromise; mutable tags are the lesson), all Actions SHA-pinned, provenance/attestation required.

## Honest limits (stated now, never softened later)

- A user's own stock goose is theirs; **AGPL forks can strip the guard** — governance, not the binary, answers for forks. Never "fork-proof."
- **A hostile tool binary that ignores proxy environment variables escapes the library-level network boundary** — full network-level enforcement in the shipped artifact is an OS/Alfred-layer control, recorded as an Alfred obligation (`STATE.md`).
- **Neutral-name proxying is not detected (AC-DL v3).** The criteria do not catch an excluded model served under a neutral name on a permitted-looking endpoint. Mitigation: keep the L1a allowlist narrow and vet any proxy host before adding it — the allowlist is the control here, not model-name inspection.
- **Namespace completeness is unprovable `[NEEDS-CAVEAT — completeness of any denylist namespace set]`.** AC-DL-2's seed list is a deterministic *floor*, not a proof of absence; it is necessarily incomplete at any moment and grows by ledgered amendment. Pair it with dependency review on every new addition.
- **Weight provenance is a separate criterion (AC-WP, lock 2e)** — verifying downloaded Tier-2 open weights (checksum/signature/attestation, fail closed) lands with the Phase 2/3 model-download path, not here.
- Dual-use risk multiplied by distribution is named plainly and **never claimed structurally contained**.
