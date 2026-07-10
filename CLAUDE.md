# courses-academy — agent guide

Source of truth for all Superteam Academy content. Projected into Sanity (and then on-chain) by the admin content sync in `solanabr/superteam-academy`. This repo is data, not code: plain YAML, Markdown, and source files.

**When editing a folder, read that folder's `README.md` and its `schema/<type>.schema.json` first** — they hold the fields, the controlled vocabulary, and the worked examples:

| Editing… | Read |
|---|---|
| a course / lesson / slots | [courses/README.md](./courses/README.md) + `schema/course.schema.json`, `schema/lesson.schema.json` |
| an achievement | [achievements/README.md](./achievements/README.md) + `schema/achievement.schema.json` |
| a quest | [quests/README.md](./quests/README.md) + `schema/quest.schema.json` |
| a learning path | [paths/README.md](./paths/README.md) + `schema/path.schema.json` |
| an instructor | [instructors/README.md](./instructors/README.md) + `schema/instructor.schema.json` |

## The rules that actually bite

- **Never hand-edit `courses/*/slots.lock.json`.** It pins on-chain bitmap positions; CI regenerates it and fails on any diff. A wrong slot corrupts real learner progress.
- **Ids are immutable and some are PDA seeds.** Never strip a prefix or rename. `course-*` / `achievement-*` ≤ 32 UTF-8 bytes; the rest ≤ 128.
- **`xpPerLesson × lessonCount ≤ 10000`**, or `finalize_course` reverts forever and nobody can complete the course.
- **A `code` block's `solution` must pass `tests.json`; its `starter` must fail.** CI executes TypeScript blocks; rust and buildable are graded at runtime (fail-closed), not in CI or at sync.
- **Answer keys are public by design.** Grading is by sandboxed execution, not secrecy.
- **`openEnded` never mints XP.** It's a reflection: one learner message, one AI reply.
- **A course's creator is its instructor's wallet.** `course.instructor → instructor.wallet → Course.creator` on-chain. `instructor.wallet` is required and must be on-curve.

## Validate locally

```bash
git clone https://github.com/solanabr/superteam-academy
cd superteam-academy && pnpm install
pnpm --filter @superteam-lms/content-lint exec tsx src/cli.ts /path/to/courses-academy
```

Exit 0 = zero errors. `notice`/`warning` never fail.

## Regenerating from live Sanity

The tree was extracted from the live dataset by `scripts/cs8-extraction/extract.ts` in the app repo — read-only against Sanity and idempotent, with `slots.lock.json` preserved byte-identical.

## Not yet wired end-to-end

`instructor.wallet` flows to `Course.creator`, but the `wallet → platform user` linkage (`profiles.wallet_address`) in the app is still in progress. `teachers.yaml` is informational — nothing enforces it yet.
