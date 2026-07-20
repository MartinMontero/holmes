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

## Phase 2.5 — Safety before surface (loop §6; hard gate)

Built before the collection surface it protects. All deterministic and model-free; the model-shaped seams are traits whose signatures *are* the confinement. Implemented in `crates/holmes-core/src/safety/` (`reader`, `approval`, `subjects`) plus the extended emission gate; locked by `crates/holmes-core/tests/safety_locks.rs`.

- **Injection defense — the dual-model quarantine (lock 2.5a).** Untrusted fetched content lives only in `UntrustedContent` (private bytes, born-redacted `Debug`, one name-firewalled raw accessor). The quarantined `ReaderBackend` sees the raw text and the extraction request and **nothing else** — no case handle, no tool broker, no channel out but its return value. Every candidate faces a deterministic validator (rejects — never repairs — empties, oversizes, recipe-scan smuggling classes, control characters, unrequested kinds, floods); survivors become sealed `Extraction` values that carry **no authority** (no API turns extraction text into a grant, a phase transition, or a handoff). The boundary is types + process separation, never prompt text.
- **Calibration + knowability gating (lock 2.5b).** The emission gate now blocks an uncalibrated likelihood from surfacing as a confident finding (`CONFIDENT_FLOOR`), and blocks bare high confidence in a low-`knowability` domain without a prominent uncertainty statement. The analytical core mints only `CalibrationStatus::Uncalibrated` — no fake calibration machinery exists — so the cap **is** the calibration fallback until real outcome data arrives (Phase RC). `downgrade_uncalibrated` is the named, auditable recovery: it supersedes (never deletes) the confident finding with a capped copy and flags the downgrade.
- **Tool-approval protocol (lock 2.5c).** Deny-by-default, per case: no tool fires without a `ToolGrant`, and the only mint is an operator **Approved** decision on a previewed request. Unanswered blocks; denied grants nothing; a decision cannot be flipped. The approval log is born-redacted (case/tool/decision/timestamp — never purpose text, arguments, or content).
- **Legal/defamation guardrails + Sentinel asymmetry (lock 2.5d).** Investigative targeting of a private individual is refused permanently and declines the case terminally (no override API). Holmes adopts **Blacksky's definition of doxxing verbatim** (spec §6.4) and refuses disclosure of any personal, non-public information class without recorded written consent, for any subject. A finding that names a real person carries a stricter emission bar: ≥3 independent source roots, a verbatim quote on every provenance entry, and a prominent uncertainty statement — labeling stays non-destructive (superseded, never deleted) and resolution stays handoff-only.

### Phase 2.5 obligations recorded (not yet discharged)

- **Reader process separation is an Alfred/OS-layer obligation.** The in-process `ReaderBackend` trait confines what the quarantined side *receives*; it cannot stop a hostile backend *implementation* from ignoring its contract. The live leg runs the reader as a separate no-tools model session behind the L2 sanitized spawn — the same out-of-process residual already recorded for the guard boundary below. Until that live leg runs on an open-egress host, hostile-backend-implementation resistance is design posture, not executed evidence.
- **Approval-request rendering is Alfred's surface.** The protocol, the previewable payload shape, and the blocking behavior are Holmes's and are tested headlessly; drawing the approval dialog and capturing the operator's click is Alfred's obligation (spec §4.1 shell).
- **The confident/person floors are ASSUMED numbers.** `CONFIDENT_FLOOR` (0.75), `CALIBRATION_CAP` (0.7), and `PERSON_FINDING_ROOT_FLOOR` (3) are deterministic floors the canon names as rules without values; each is documented at its definition and changeable only by a D-item, never a silent edit.
- **The injection defense does not judge natural language.** A validated extraction may still *say* "ignore your instructions"; the defense is that saying it moves nothing (text has no path to authority), not that hostile phrasing is detected — detecting it would require an LLM in the gate, which canon §5 forbids.

## Sandbox policy (spec §3.1, §4.6)

Any model-generated code runs in a **Firecracker microVM** (E2B OSS self-hosted), **no outbound network by default**. bubblewrap only for trusted, in-house, read-only tasks — justified by the documented Ona incident (`/proc/self/root` denylist bypass + self-disabled sandbox; primary source). The `rm -rf ~/` anecdote is separate and `[DIRECTIONAL]` — do not bundle the two.

## Secrets

**BYOK invariant.** The shipped `holmes-guard` crate reads no vendor credential and requires no vendor, key, or env var of its own — users bring their own keys; the guard governs *where* they may reach (L1a allowlist) and *which* provider a key is admitted for (L2 per-provider credential seam). The smoke-test key is build/CI-only, supplied at run time via `--credential-env` and injected only through the seam. Headless spawns set `GOOSE_DISABLE_KEYRING=1` and the pinned goose build compiles the `system-keyring` feature out. At-rest storage backend for embedders that opt into a keyring: OS credential store (goose platform keyring `[NEEDS-CAVEAT — confirm exact backend per platform in goose docs]`); never in files, never logged.

## Telemetry

Born-redacted, opt-in, local-only: counts, durations, names — never content, prompts, or secrets. Feedback is user-initiated export only; no phone-home.

## Supply chain (spec §6.6)

Syft SBOMs (CycloneDX + SPDX), OSV-Scanner primary, Grype cross-check, **no Trivy** (CVE-2026-33634 — the March 2026 `trivy-action` tag force-push compromise; mutable tags are the lesson), all Actions SHA-pinned, provenance/attestation required.

**Implemented (Phase 0 lock 0e):** `.github/workflows/supply-chain.yml` — action-free (⇒ SHA-pinning satisfied by construction), scanners installed as version-pinned release binaries verified against each release's goreleaser checksums and **failing closed** on mismatch. OSV-Scanner is the strict primary gate (exit 1 on any known vuln in a discovered lockfile); Grype cross-checks the syft SBOM at `--fail-on high` (exit 2). Kept a **separate workflow** from `acdl-gate.yml` so the CVE gate and the provider-denylist gate stay distinct (the §"Denylist != CVE scan" invariant, in CI form). **Not yet implemented:** provenance/attestation (SLSA/cosign) — carried in `STATE.md` lock 0e. First real scan runs on GitHub Actions; in-container execution is blocked by the org egress policy on the release-asset CDN.

## Honest limits (stated now, never softened later)

- A user's own stock goose is theirs; **AGPL forks can strip the guard** — governance, not the binary, answers for forks. Never "fork-proof."
- **A hostile tool binary that ignores proxy environment variables escapes the library-level network boundary** — full network-level enforcement in the shipped artifact is an OS/Alfred-layer control, recorded as an Alfred obligation (`STATE.md`).
- **Neutral-name proxying is not detected (AC-DL v3).** The criteria do not catch an excluded model served under a neutral name on a permitted-looking endpoint. Mitigation: keep the L1a allowlist narrow and vet any proxy host before adding it — the allowlist is the control here, not model-name inspection.
- **Namespace completeness is unprovable `[NEEDS-CAVEAT — completeness of any denylist namespace set]`.** AC-DL-2's seed list is a deterministic *floor*, not a proof of absence; it is necessarily incomplete at any moment and grows by ledgered amendment. Pair it with dependency review on every new addition.
- **Weight provenance is a separate criterion (AC-WP, lock 2e)** — verifying downloaded Tier-2 open weights (checksum/signature/attestation, fail closed) lands with the Phase 2/3 model-download path, not here.
- Dual-use risk multiplied by distribution is named plainly and **never claimed structurally contained**.
