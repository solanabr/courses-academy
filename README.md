# Superteam Academy — Courses

The **source of truth** for every course, lesson, achievement, quest, learning path, and instructor on [Superteam Academy](https://superteam-academy-web.vercel.app).

Content lives here as plain YAML, Markdown, and source files. Teachers contribute by opening a pull request. Once merged, an admin runs the content sync, which projects this repo into Sanity and then on-chain.

```
courses-academy (git)
      │  teacher opens a PR
      ▼
   CI validates the whole tree  ──►  merge to main
      │
      ▼
   admin runs the content sync
      │  createOrReplace, idempotent
      ▼
   Sanity (a rebuildable projection)  ──►  the app (GROQ)  +  on-chain (course/achievement PDAs)
```

**Nothing is authored in Sanity Studio.** Sanity is a cache of this repo and can be rebuilt from it at any time.

## Where things live

| Folder | What's in it | Docs |
|---|---|---|
| `courses/<slug>/` | a course: `course.yaml`, `slots.lock.json`, `lessons/` | [courses/README.md](./courses/README.md) |
| `achievements/*.yaml` | earnable credentials + their unlock conditions | [achievements/README.md](./achievements/README.md) |
| `quests/*.yaml` | global daily / multi-day XP challenges | [quests/README.md](./quests/README.md) |
| `paths/*.yaml` | ordered shelves of courses | [paths/README.md](./paths/README.md) |
| `instructors/*.yaml` | teaching personas + the wallet that receives creator XP | [instructors/README.md](./instructors/README.md) |
| `courses/_template/` | a complete, valid example to copy — linted, never published | [courses/_template/README.md](./courses/_template/README.md) |
| `schema/*.json` | generated JSON Schema — powers editor autocomplete + CI | — |
| `teachers.yaml` | maintainer-controlled teacher registry (informational for now) | — |

## Lessons are made of blocks

A lesson is metadata plus an **ordered list of typed blocks**. Adding a new kind of activity means adding a block type — it never reshapes a course or a lesson.

| Block | Graded? | Purpose |
|---|---|---|
| `prose` | no | Markdown, from a `.md` file |
| `video` | no | YouTube / Vimeo embed |
| `code` | **yes** | Editor + tests, graded by execution |
| `quiz` | **yes** | Multiple choice, one or many correct |
| `openEnded` | no (required) | A reflection: one learner message, one AI reply, feedback only. **Never mints XP.** |
| `wallet-funding` | no | Fund a devnet wallet |
| `program-explorer` | no | Call a deployed program from an IDL |
| `deployed-program-card` | no | Show the learner's deployed program |

## The golden rules

These are the ones that bite. CI enforces all of them.

- **Ids are permanent and some are on-chain PDA seeds.** `course-*` and `achievement-*` are capped at 32 UTF-8 bytes; the rest at 128. Never rename an id that's live — it orphans learner progress.
- **`slots.lock.json` is machine-owned.** It pins on-chain bitmap positions. Never hand-edit it; CI regenerates and fails on a diff. This is what makes reordering, regrouping, and inserting lessons safe for enrolled learners.
- **`xpPerLesson × lessonCount ≤ 10000`.** Above it, `finalize_course` reverts forever and nobody can complete the course.
- **A `code` block's `solution` must pass its `tests`, and its `starter` must fail.** CI executes TypeScript blocks; rust and buildable are graded at runtime (fail-closed).
- **Answer keys are public by design.** `solution` files and test cases live in this public repo. Grading is by sandboxed execution, not secrecy — don't write a lesson whose value depends on hiding the answer.
- **A course's creator is its instructor.** `course.instructor → instructor.wallet` becomes the on-chain `Course.creator` (the creator-XP recipient). See [instructors/README.md](./instructors/README.md).

## Add a course in five steps

1. `cp -r courses/_template courses/<your-slug>`
2. Edit `course.yaml` — set `id`, `slug`, `title`, `instructor`, and the `modules → lessons` structure.
3. Write your lessons under `lessons/<slug>/` (blocks, markdown, starter/solution/tests).
4. Make sure an [instructor](./instructors/README.md) file exists with your on-curve wallet, and `course.instructor` points at it.
5. Validate locally (below), then open a PR. Don't touch `slots.lock.json`.

## Validate locally

Your editor validates as you type if you have the Red Hat **YAML** extension (`.vscode/settings.json` wires each file to its schema). To run the full linter:

```bash
git clone https://github.com/solanabr/superteam-academy
cd superteam-academy && pnpm install
pnpm --filter @superteam-lms/content-lint exec tsx src/cli.ts /path/to/courses-academy
```

Exit 0 means zero errors; `notice`/`warning` never fail the build.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for the full rule set.
