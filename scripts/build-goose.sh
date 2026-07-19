#!/usr/bin/env bash
# Build goose from pinned source — the substrate binary for Phase 0 lock 0c.
#
# Provenance pin: aaif-goose/goose @ 8e78960e535ab7f34630e7c5921a42f146cbc9f4
# (Apache-2.0; commit verified on disk after fetch, license file checked).
#
# NEVER `cargo install goose-cli`: the crates.io package of that name is an
# unrelated squatter (F-016). Source builds of aaif-goose/goose only.
#
# Trimmed feature set (matches the Session-2 evidence in STATE.md): the
# goose-cli crate is built with --no-default-features --features rustls-tls,
# which compiles out V8/code-mode, telemetry, aws-providers, system-keyring,
# and the updater.
#
# Output: $GOOSE_SRC_DIR/target/release/goose plus a PROVENANCE.txt beside it
# recording origin, commit, toolchain, features, and the binary's sha256.
set -euo pipefail

PIN="8e78960e535ab7f34630e7c5921a42f146cbc9f4"
REPO_URL="https://github.com/aaif-goose/goose"
SRC_DIR="${GOOSE_SRC_DIR:-/home/user/goose-src}"

echo "== goose pinned build =="
echo "source: ${REPO_URL} @ ${PIN}"
echo "srcdir: ${SRC_DIR}"

if [ ! -d "${SRC_DIR}/.git" ]; then
  mkdir -p "${SRC_DIR}"
  git init -q "${SRC_DIR}"
  git -C "${SRC_DIR}" remote add origin "${REPO_URL}"
fi
git -C "${SRC_DIR}" fetch --depth 1 origin "${PIN}"
git -C "${SRC_DIR}" checkout -q --detach "${PIN}"

HEAD_SHA="$(git -C "${SRC_DIR}" rev-parse HEAD)"
if [ "${HEAD_SHA}" != "${PIN}" ]; then
  echo "FATAL: checked-out HEAD ${HEAD_SHA} does not match pin ${PIN}" >&2
  exit 1
fi
echo "pin verified: HEAD = ${HEAD_SHA}"

LICENSE_LINE="$(head -1 "${SRC_DIR}/LICENSE" 2>/dev/null || echo 'LICENSE FILE ABSENT')"
echo "license head: ${LICENSE_LINE}"

cd "${SRC_DIR}"
LOCK_FLAG=""
if [ -f Cargo.lock ]; then LOCK_FLAG="--locked"; fi
echo "building: cargo build --release ${LOCK_FLAG} -p goose-cli --no-default-features --features rustls-tls"
cargo build --release ${LOCK_FLAG} -p goose-cli --no-default-features --features rustls-tls

BIN="${SRC_DIR}/target/release/goose"
if [ ! -x "${BIN}" ]; then
  echo "FATAL: expected binary ${BIN} not found after build" >&2
  exit 1
fi
BIN_SHA="$(sha256sum "${BIN}" | cut -d' ' -f1)"
GOOSE_VERSION="$("${BIN}" --version 2>/dev/null | head -1 || echo 'version probe failed')"

PROV="${SRC_DIR}/target/release/PROVENANCE.txt"
{
  echo "artifact: goose (headless ACP substrate binary)"
  echo "origin:   ${REPO_URL}"
  echo "commit:   ${HEAD_SHA}"
  echo "license:  ${LICENSE_LINE}"
  echo "features: --no-default-features --features rustls-tls (goose-cli)"
  echo "rustc:    $(rustc --version)"
  echo "cargo:    $(cargo --version)"
  echo "version:  ${GOOSE_VERSION}"
  echo "sha256:   ${BIN_SHA}"
  echo "built-by: scripts/build-goose.sh"
} > "${PROV}"

echo "== build complete =="
cat "${PROV}"
