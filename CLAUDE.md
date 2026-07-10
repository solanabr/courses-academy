# courses-academy

Source of truth for all Superteam Academy content. Projected into Sanity (and then on-chain) by the admin content sync in `solanabr/superteam-academy`.

Read `CONTRIBUTING.md` before editing. The rules that actually bite:

- **Never hand-edit `courses/*/slots.lock.json`.** It pins on-chain bitmap positions. CI regenerates it and fails on any diff. A wrong slot corrupts real learner progress.
- **Ids are immutable and are PDA seeds.** Never strip a prefix, never rename. `course-*` and `achievement-*` are capped at 32 UTF-8 bytes.
- **`xpPerLesson × lessonCount ≤ 10000`**, or `finalize_course` reverts forever and nobody can complete the course.
- **A `code` block's `solution` must pass `tests.json`; its `starter` must fail.** CI executes TypeScript blocks. Rust and `buildable` are graded at sync time and only emit a `notice`.
- **Answer keys are public by design.** Grading is by sandboxed execution, not secrecy.
- **`openEnded` never mints XP.** It is a reflection with one AI reply.

## Validate locally

```bash
git clone https://github.com/solanabr/superteam-academy
cd superteam-academy && pnpm install
pnpm --filter @superteam-lms/content-lint exec tsx src/cli.ts /path/to/courses-academy
```

Exit 0 = zero errors. `notice`/`warning` never fail.

## Regenerating from live Sanity

The tree was extracted from the live dataset by `scripts/cs8-extraction/extract.ts` in the app repo. It is read-only against Sanity and idempotent.

## Not yet wired

`creator.githubId` and `teachers.yaml` are stored but unenforced: nothing resolves a githubId to a wallet, so on-chain `Course.creator` still falls back to the platform authority.
