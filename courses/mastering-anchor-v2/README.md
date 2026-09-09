# `mastering-anchor-v2` — maintenance contract

**This file is never published to learners.** It is not a lesson, it is not referenced by
`course.yaml`, and nothing in the compiled bundle reads it. It exists so that whoever picks this
course up next knows what is pinned, what will rot, and who owns the re-verification.

## Why this course needs a contract at all

Every other course in this repo teaches a released stack. This one teaches
**`anchor-lang@2.0.0-rc.1`**, a release candidate that the project labels `rc` in one place and
`alpha` in another. `course.yaml` records that in `subjectVersion`. The consequence is blunt: the
subject can move under the lessons without anyone touching this repo, and a version-stamped course
that nobody re-verifies becomes wrong silently rather than loudly.

Two facts set the shape of everything below:

- **The library pin is immutable, the CLI pin is not.** `anchor-lang = "2.0.0-rc.1"` is a
  crates.io version and the registry forbids republishing it. The CLI is a git install, and a git
  ref can be moved.
- **No GitHub Release was ever cut for the v2 tag**, so `avm install 2.0.0-rc.1` 404s on a
  prebuilt asset that does not exist. Every install block in the course is a `cargo install --git`
  as a result. If a Release ever appears, that is a course-wide edit, not a footnote.

## The pins

All values below are the ones the lessons actually write. The `verified` column is the date the
lessons carry; treat a stale date as an unverified pin.

| Pin | Value | Channel | Verified | Taught in |
|---|---|---|---|---|
| `anchor-cli` (install-channel lesson) | `2.0.0-rc.1` | git `otter-sec/anchor`, `--branch anchor-next` | 2026-08-22 | m01-l2 |
| `anchor-cli` (every lesson after) | `2.0.0-rc.1` | git `--tag v2.0.0-rc.1` (= commit `e4878b6d`) | 2026-08-22 | m02-l1 onward |
| `anchor-cli` (verify Dockerfile) | `2.0.0-rc.1` | git `--rev e4878b6d` | 2026-08-22 | m08-l2 |
| `anchor-cli` (fuzz CLI) | not version-pinned — tracks `master` HEAD, V1 line past `1.1.2` | git `otter-sec/anchor`, `--branch master`, `--root ~/.anchor-master` | 2026-08-22 | m07-l3 |
| `anchor-lang` (library) | `2.0.0-rc.1` | crates.io, published 2026-08-12 (immutable) | 2026-08-22 | m02-l1 onward |
| `wincode` | `0.5`, `features = ["derive"]` | crates.io | 2026-08-22 | m02-l1 onward |
| `solana-address` (standalone crates) | `=2.6.0` | crates.io | 2026-08-22 | m01-l2's greeter, m04-l2's `borrow_probe`, m10-l2's port crate |
| Rust (MSRV) | `1.89.0` | rustup; the scaffold writes `rust-toolchain.toml` | 2026-08-22 | m01-l2 |
| macOS build workaround | `CARGO_PROFILE_RELEASE_LTO=off` | cargo release-profile env var | 2026-08-22 | m01-l2 |
| Solana CLI | `3.1.10` | agave-install — **LOCAL-CI / DOCKER PIN ONLY** | 2026-08-22 | m01-l2, m08-l2 |
| `solana-verify` | `0.5.1` | crates.io | 2026-08-22 | m08-l2 |
| Docker | `27.x` or newer | daemon must be running or the build fails fast | 2026-08-22 | m08-l2 |
| `mollusk-svm` | `0.15.1` (published 2026-08-29) | crates.io | 2026-09-01 | m06-l1, m06-l2, m09-l3 |
| `mollusk-svm-programs-token` | `0.15.1` | crates.io | 2026-09-04 | m06-l1, m06-l2, m09-l3 |
| `solana-sdk` (Mollusk's dev-dep) | `4` | crates.io | 2026-09-01 | m06-l1, m06-l2, m09-l3, m09-l1 |
| `solana-short-vec` / `solana-signature` | `>=3.2.2, <3.3` / `>=3.4.1, <3.5` | crates.io — dev-deps, **only in the Mollusk crates** | 2026-09-01 | m06-l1, m06-l2, m09-l3 |
| `solana-address` (**every arcade-workspace member**) | `>=2.6.1, <2.7` | crates.io | 2026-09-01 | m02-l1, m03-l1, m04-l3, m05-l1, m06-l1, m06-l2, m09-l3 |
| `crucible-fuzz-cli` / `crucible-fuzzer` | `0.2.1` | pinned by Anchor **`master`**'s CLI, not by the RC | 2026-08-22 | m07-l3 |
| `surfpool` | `>= 1.1.2` | `run.surfpool.run` install script | 2026-08-22 | m09-l3 |
| `pinocchio` / `pinocchio-system` / `pinocchio-pubkey` | `0.9` / `0.4` / `0.3` | crates.io — **move as a set** | 2026-08-22 | m09-l1 |
| `@solana/kit` | `^7` | npm — match the major `@solana-program/*` peers on | 2026-08-22 | m08-l1 |
| `@solana-program/system` / `@solana-program/token` | `0.13.0` / `0.15.0` | npm | 2026-08-22 | m08-l1 |

### Pins that are load-bearing in a non-obvious way

- **`wincode 0.5` and the `solana-address` ceiling travel together.** `anchor-lang@2.0.0-rc.1`
  pins `wincode 0.5`; `solana-address 2.7.0` moved to `wincode 0.6`. Float either one and cargo
  puts two `wincode` majors in the graph, which is issue #4937 — a trait-bound error in
  `#[account(borsh)]` in a feature nobody touched. This is the single most likely way a learner's
  build breaks. What the pin has to express is the `< 2.7` ceiling; whether it is written as an
  equality or a range is the next bullet's problem.
- **`solana-address` is a WORKSPACE-scoped pin, not a per-crate one.** Cargo resolves one
  `solana-address` for every member of a workspace at once. The arcade workspace ends up holding
  all five programs — `cabinet-counter`, `quarter-vault`, `quarter-prize`/`prize-escrow`,
  `token-ticket-swap`, `floor-registry` — and m06-l1 puts Mollusk in one of them, whose SVM stack
  requires `^2.6.1`. So **every member** carries `solana-address = ">=2.6.1, <2.7"`, from m02-l1
  onward. Leave any single member on `=2.6.0` and the whole workspace fails to resolve with
  `error: failed to select a version for solana-address … all possible versions conflict` —
  reproduced against the real five-member membership, 2026-09-01. The three crates that never
  join that workspace (m01-l2's `greeter`, m04-l2's `borrow_probe`, m10-l2's port copy) keep the
  exact `=2.6.0`: nothing in their graphs needs 2.6.1, and the tighter pin is the course's
  default discipline.
- **The two Mollusk dev rows retire together with the ceiling, and only in the Mollusk crates.**
  `solana-short-vec >=3.2.2, <3.3` and `solana-signature >=3.4.1, <3.5` exist for one reason:
  both crates moved to `wincode 0.6` in versions that still satisfy `solana-message 4.4.0`, so
  an unpinned Mollusk graph resolves and then fails to *build*. Unlike `solana-address` these are
  dev-dependencies, so they shape the workspace lock without every sibling having to declare
  them — only `token-ticket-swap` and `floor-registry` carry them. Drop all three rows on the day
  Anchor V2 moves to `wincode 0.6`, never one at a time.
- **The tag, not the branch, from m02-l1 onward.** The `anchor-next` tip has already moved to
  `wincode 0.6` and demands `solana-address 2.7.0`. Track the branch and the `< 2.7` ceiling —
  in either the exact or the range form — refuses the resolve before anything compiles. m01-l2 installs off the branch **on purpose**, because the
  install channel is that lesson's subject; every later lesson pins the tag.
- **Solana `3.1.10` is a build pin, never a currency claim.** Current stable Agave was `v4.2.1` at
  authoring. Two lessons defuse this explicitly; do not "fix" the number to look newer.
- **`anchor fuzz` lives on `master`, not on the RC.** The `anchor-next` CLI ships
  `anchor test --profile`, `anchor debugger`, and `anchor coverage`, and has no `fuzz` subcommand.
  m07-l3 depends on that split being true; re-check it whenever either branch moves.
- **The course consumes programs by IDL — `declare_program!` + a workspace-root `idls/`
  directory — never by `features = ["cpi"]` path deps.** Two rc.1 defects force this, both
  reproduced by execution: a program that references the `cpi` modules of **two**
  `no-entrypoint` rungs dies at SBF link on `duplicate symbol: __anchor_dispatch` (the
  four-rung capstone cannot exist on that road), and a workspace-root `cargo build-sbf`
  feature-unifies `no-entrypoint` onto a consumed rung and silently emits an
  entrypoint-less `.so` that the loader rejects. Zero `cpi` dep rows exist in the course;
  if one ever returns, both hazards return with it. The `idls/*.json` files are compile
  inputs (`include_bytes!`-tracked), so they must be re-harvested after any interface
  change — m04-l3, m05-l1, and m09-l3 each say so in place.
- **The pinocchio trio in m09-l1 is deliberately two minor lines behind.** `pinocchio 0.11` renamed
  `AccountInfo` to `AccountView` and `Pubkey` to `Address`. The lab pins `0.9` because it is the
  last line where `pinocchio-pubkey` still resolves and the pre-rename names map cleanly onto v1
  instincts. Do not "upgrade" it without rewriting the lesson.

## Re-verify cadence

| Trigger | Scope | Action |
|---|---|---|
| Every 60 days, unconditionally | Whole table | Re-run the checks below; stamp a new `verified` date even when nothing moved. A fresh date on an unchanged value is the record that a human looked. |
| A new Anchor V2 RC or the stable release | Whole course | Full re-authoring pass, not a version bump. `rc.2` may move constraint spellings, the `CpiHandle` grammar, or the feature-flag set — all of which are taught, quizzed, and graded here. |
| A GitHub Release appears for a v2 tag | m01-l2 | The avm-404 story is the lesson's spine. If `avm install` starts working, m01-l2 needs rewriting, not editing. |
| `@solana-program/*` peers move off kit `^7` | m08-l1 | Re-pin per workspace, per the rule the lesson teaches. Never repo-wide. |
| Anchor `master` merges `fuzz` into the RC line | m07-l3 | The branch-split fact the lesson turns on stops being true. |
| A published V2 audit or a committed stable date | m10-l4 | The conclusion routes on exactly those two signals. Both are currently absent; if either lands, the decision tree changes. |
| Devnet reset, or the twin signatures prune | m01-l1 | The lesson pins two live devnet programs + reference signatures inline (verified 2026-09-02). Run [`twins/redeploy.sh`](./twins/README.md) and re-pin the four values it prints. |

Minimum check, and it is fast:

```bash
anchor --version                                   # expect 2.0.0-rc.1
cargo search anchor-lang                           # expect the pinned values still resolve
cargo search wincode                               # cargo search takes one crate per call —
cargo search solana-address                        # multiple args join into one query string
cargo search solana-verify                         # and return nothing useful
cargo search mollusk-svm
npm view @solana-program/token@0.15.0 peerDependencies
git ls-remote https://github.com/otter-sec/anchor.git 'refs/tags/v2.0.0-rc.1*'
# read the `^{}` (peeled) line, not the bare tag line — an annotated tag's bare
# line returns the tag-object SHA (2f77733f...), not the commit; expect e4878b6d
# Probe the API, not the HTML page: github.com now renders /releases/tag/<tag>
# with a 200 even when no Release object exists (observed 2026-09-05). The API
# 404 below is the signal that still means "no Release, avm's asset fetch fails".
curl -s -o /dev/null -w '%{http_code}\n' \
  https://api.github.com/repos/otter-sec/anchor/releases/tags/v2.0.0-rc.1   # expect 404
```

When a value moves, update it in **every** lesson that writes it — the grep targets are
`2.0.0-rc.1`, `wincode`, `solana-address`, `3.1.10`, and `e4878b6d` — and then update this table.
The lessons are the source of truth for what a learner types; this file is the index over them.

## Maintenance ownership

This repo has no teacher registry and no `githubId` indirection. Per the owner ruling of
2026-07-28, **the creator wallet in `course.yaml` is the teacher pointer**, and it is immutable
after the course is created on-chain:

- **Owner (of record):** `course.creator` = `3WECquwCtcKVRYNWBPFWE28ag3b1CDKchLZPXxifAJzQ`
  → resolves to a platform account through `profiles.wallet_address`.
- **Day-to-day maintainer contact:** `TBD (owner to fill)` — no personal name or handle exists
  anywhere in this repo's model, and inventing one here would be a fabrication. Fill this line with
  the handle of whoever actually runs the 60-day check.
- **Escalation:** anything that changes `id`, `slug`, `creator`, `trackId`, or `trackLevel` is not
  a maintenance action. Those are immutable or PDA-derived; changing them after on-chain creation
  costs a close-and-recreate. Route those to the repo owner.

`xpPerLesson × lessonCount = 20 × 30 = 600`, which is what `xpReward` records. If a lesson is ever
added or removed, `xpReward` moves with it, and the product must stay at or under 10000 or
`finalize_course` reverts permanently.

See [`../README.md`](../README.md) for the course format and the rules that bite.
