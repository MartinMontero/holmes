# Alfred RC re-pin — verification record + standby commit package

Date: 2026-09-07. Labels: EXECUTED / VERIFIED-LIVE / CANON / REPORTED / UNVERIFIED.

## Verdict up front

**The re-pin already landed.** Alfred `src-tauri/Cargo.toml` pins
`holmes-guard` + `holmes-core` to `tag = "v1.0.0-rc.1"` as of commit
`123830df6dcff73fae050a8c18a8826f2342224e` ("deps: pin holmes to
v1.0.0-rc.1 (supersedes 63f877a)"), which is an ancestor of
`origin/main` (`499a9009`). Nothing to commit for THIS re-pin. The
prepared package below is the standby for the NEXT one (RC → final tag).

## Verification evidence (all VERIFIED-LIVE 2026-09-07, this host)

| Check | Result |
|---|---|
| Holmes tag pushed | `git ls-remote --tags` → annotated tag `d9f8a98406af226d22724244715b7634688306ad`, peeled `refs/tags/v1.0.0-rc.1^{}` = commit `9f4fef8281b874d89d67613fb8d1539bdd0b4acf` (PR #18 merge) |
| Alfred dep form | `src-tauri/Cargo.toml` lines 50–51: `tag = "v1.0.0-rc.1"` for both crates — **never a bare branch reference** |
| Lock resolution | `src-tauri/Cargo.lock`: `source = "git+...Holmes.git?tag=v1.0.0-rc.1#9f4fef8281b874d89d67613fb8d1539bdd0b4acf"` — lock commit == peeled tag commit |
| Landed on origin/main | `git merge-base --is-ancestor 123830d origin/main` → true |
| Scope of the commit | 2 files, 4 insertions, 4 deletions (Cargo.toml + Cargo.lock only) |

Re-pin diff (from `git show 123830d`): `rev = "63f877a…"` →
`tag = "v1.0.0-rc.1"` in both dep lines; lock `source` lines updated to
match. Matches LOOP-INTEGRATION.md pin discipline (dep form fixed; one
deliberate lock-regen; re-pin in its own commit). EXECUTED by an earlier
session; I verified, I did not re-do.

## Drift flags (flagged, nothing silently fixed)

1. **Stale comment, Alfred `src-tauri/Cargo.toml` lines 46–49**: still
   says "Pinned to a full commit SHA, never a branch (re-pin to the
   Holmes RC tag, when cut, in its own commit)". The pin is now a
   **tag**, and the re-pin has happened. Comment-only fix, its own
   commit, Martin's call:

   ```
   cd C:\Users\User\dev\Alfred
   git checkout -b chore/holmes-repin-comment origin/main
   # edit lines 46-49 to: "Pinned to the Holmes RC tag v1.0.0-rc.1
   # (commit 123830d), never a branch. Re-pin to the final tag, when cut,
   # in its own commit per LOOP-INTEGRATION.md pin discipline."
   git commit -am "docs: correct holmes pin comment (tag, re-pin landed)"
   git push -u origin chore/holmes-repin-comment
   gh pr create --base main --title "docs: correct holmes pin comment" --body "Comment-only. The dep moved rev->tag in 123830d; the comment still described the old SHA pin and the pending re-pin."
   ```

2. **PR #19 (graph-engineering convention) is NOT in the RC tag**:
   `git merge-base --is-ancestor 4e1b30c v1.0.0-rc.1` → false. The tag
   targets `9f4fef8` (pre-convention). Alfred therefore embeds a Holmes
   without RECIPE.md/gate.ps1. Harmless to the embed (no code change),
   but if the RC tag is meant to include the convention, the tag needs a
   re-cut after PR #19 merges — Martin's decision, never mine.

## Standby package — the NEXT re-pin (RC tag → final tag)

Run only after Martin cuts and pushes the final tag. Substitute the real
tag name for `v1.0.0` below. Never a bare branch reference.

```
cd C:\Users\User\dev\Alfred
git fetch origin --tags
git ls-remote --tags https://github.com/MartinMontero/holmes.git
# confirm: annotated tag exists; record the peeled commit (^{} line)

git checkout -b holmes-repin-v1.0.0 origin/main
# edit src-tauri/Cargo.toml lines 50-51:
#   holmes-guard = { git = "https://github.com/MartinMontero/Holmes.git", tag = "v1.0.0" }
#   holmes-core  = { git = "https://github.com/MartinMontero/Holmes.git", tag = "v1.0.0" }

cargo update -p holmes-guard -p holmes-core   # deliberate lock-regen (pin discipline)
findstr /n "tag=v1.0.0#" src-tauri\Cargo.lock
# confirm BOTH lock source lines show ?tag=v1.0.0#<peeled-commit-SHA>
# and the SHA equals the ls-remote peeled commit

cargo build --locked
cargo test --locked -p <alfred-guard-consumer-tests>   # the artifact-guard suite

git add src-tauri/Cargo.toml src-tauri/Cargo.lock
git commit -m "deps: pin holmes to v1.0.0 (supersedes v1.0.0-rc.1)"
git push -u origin holmes-repin-v1.0.0
gh pr create --base main --title "deps: pin holmes to v1.0.0" --body "Re-pin per LOOP-INTEGRATION pin discipline: tag form, deliberate lock-regen, own commit. Peeled tag commit: <SHA>."
```

Stop before merge (Rule 9). The Alfred-side `artifact-guard` CI job must
be green on the PR head before Martin merges.
