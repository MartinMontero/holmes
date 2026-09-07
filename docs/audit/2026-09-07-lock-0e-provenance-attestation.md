# Lock 0e — provenance & attestation record (Phase 0, supply chain)

Date: 2026-09-07. Scope: Phase 0 lock 0e's carried item —
"provenance/attestation (spec §6.6) — carry" (STATE.md); "Not yet
implemented: provenance/attestation (SLSA/cosign)" (docs/security.md).
This file is the record: what attestation exists today, what does not,
and the exact closure proposal. Labels per session constitution.

## Artifact identities (VERIFIED-LIVE 2026-09-07)

| Artifact | Identity |
|---|---|
| RC annotated tag object | `d9f8a98406af226d22724244715b7634688306ad` |
| Tag target (peeled) | commit `9f4fef8281b874d89d67613fb8d1539bdd0b4acf` (PR #18 merge) |
| Tag annotation | "Holmes RC — analytical beta release candidate. 143-test sweep re-run attached in release notes." (from `.tag-message-v1.0.0-rc.1.txt`, repo root) |
| Tag signature | **NONE** — annotated but unsigned (`git tag -v` shows no signature). Flagged, not softened. |
| Dependency tree | `Cargo.lock`, `--locked` everywhere; 157 packages (acdl2-scan, 2026-09-05) |
| Denylist posture | AC-DL-2 `verdict: CLEAN` on the real tree; planted control fires exit 1 (VERIFIED-LIVE 2026-09-05) |

## What attestation exists today

1. **CI SBOM, every run** [CANON + VERIFIED-LIVE]. `supply-chain.yml`
   generates SPDX + CycloneDX SBOMs with syft `v1.48.0` on every
   push/PR; OSV-Scanner `v2.4.0` primary (exit 1 on any known vuln);
   Grype `v0.116.0` cross-check (`--fail-on high`). Scanners are
   version-pinned release binaries verified against each project's own
   goreleaser checksums manifest, failing closed on mismatch. Workflow
   is action-free — the SHA-pinning rule is satisfied by construction.
   Latest green evidence: PR #19 checks, `sbom-and-cve-scan` pass ×2
   (2026-09-05, runs 33960985976 / 33960992468).
2. **Suppression allowlist bounded and asserted** [CANON]. D-13: exactly
   4 ledgered `informational: unmaintained` RUSTSEC IDs via
   `osv-scanner.toml`, `ignoreUntil 2026-10-17`; CI asserts the set
   equals exactly those 4 — a 5th fails the gate. Fail-closed for every
   scored/fixable advisory.
3. **Pinned-toolchain evidence in CI logs** [CANON]: rustc/cargo
   versions echoed each run; sha256sum/curl versions in the CVE gate.
4. **Substrate (goose) provenance convention** [CANON]:
   `scripts/build-goose.sh` pins `aaif-goose/goose` @
   `8e78960e535ab7f34630e7c5921a42f146cbc9f4` (verified on disk after
   fetch) and emits `PROVENANCE.txt` beside the binary: origin, commit,
   license head, features, rustc/cargo, version, binary sha256. The
   2026-07-19 container build recorded sha256 `439a282e…7056` [REPORTED
   — STATE.md 0c; container-ephemeral, not re-produced here].
5. **Historical first-scan evidence** [REPORTED — STATE.md 0e]: CVE
   gate run #1 on `107f0a5`, 2026-07-19 07:30:49Z, 51s, OSV clean,
   Grype clean; re-runs green on `6595499`; landed via PR #7.

## The gap (stated plainly — NOT implemented)

- **No SLSA-style provenance statement and no signature on any release
  artifact.** The tag itself is unsigned; SBOMs are ephemeral per CI
  run (generated, counted into the step summary, discarded) — they are
  not persisted as artifacts or release assets, so "the SBOM of the RC
  tag" is not a fetchable object today.
- Holmes ships **embedded in Alfred** (A-03) — there is no standalone
  Holmes binary/installer to sign. The attestation surface is therefore:
  (a) the git tag, (b) the SBOMs, (c) the goose substrate binary
  (Alfred-side artifact), (d) the `Cargo.lock` tree.

## Closure proposal (PROPOSED — needs a D-item; consistent with repo conventions)

Action-free, matching the existing pin discipline and Alfred's minisign
precedent (Alfred's update channel is already minisign-signed [CANON —
STATE.md cross-repo table]):

1. **Persist the SBOMs.** Add one step to `supply-chain.yml` on tag
   pushes: `sha256sum sbom.spdx.json sbom.cdx.json Cargo.lock >
   sbom.SHA256SUMS` and upload all three + the sums as workflow
   artifacts; on the RC tag, attach them as release assets. (No
   third-party action needed for artifacts? — `actions/upload-artifact`
   IS a third-party action; the action-free convention then prefers:
   commit `sbom.SHA256SUMS` into `docs/audit/evidence/` at tag cut, or
   attach via `gh release upload` run by Martin. Choose at the D-item.)
2. **Sign the sums, not the world.** Martin minisign-signs
   `sbom.SHA256SUMS` locally (`minisign -S -m sbom.SHA256SUMS`), pubkey
   committed to the repo. That yields a verifiable chain: tag commit →
   CI-generated SBOMs → signed sums. Record the pubkey fingerprint in
   this file when done.
3. **Sign future tags** (`git tag -s` or minisign-detached over the tag
   message file, as was used for v1.0.0-rc.1's annotation). Record
   which at the D-item.
4. Update `docs/security.md` "Not yet implemented" line and STATE.md
   lock 0e when (1)–(3) land — with executed evidence, per the loop.

**SLSA provenance via `slsa-github-generator` and keyless cosign were
considered and deferred**: both pull third-party GitHub Actions into a
deliberately action-free workflow; the minisign path above reaches the
same verifiability with zero new CI surface. [INFERRED — trade-off
judgment; Martin rules.]
