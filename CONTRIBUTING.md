# Contributing

## Quick start

1. Fork this repo and create a branch.
2. Copy `courses/_template/` to `courses/<your-slug>/` and edit it.
3. Open a pull request. CI runs the full validator.

Your editor will autocomplete and validate as you type — `.vscode/settings.json` maps every file to its JSON Schema in `schema/`. In VSCode / Cursor this needs the **[Red Hat YAML extension](https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml)** (`redhat.vscode-yaml`); the schema files themselves are generated, so don't hand-edit `schema/`.

Each folder has its own `README.md` with the fields, the controlled vocabulary, and a worked example — read the one for what you're editing ([courses](./courses/README.md), [achievements](./achievements/README.md), [quests](./quests/README.md), [paths](./paths/README.md), [instructors](./instructors/README.md)).

## Running the validator locally

The linter lives in the app repo, which is public:

```bash
git clone https://github.com/solanabr/superteam-academy
cd superteam-academy && pnpm install
pnpm --filter @superteam-lms/content-lint exec tsx src/cli.ts /path/to/courses-academy
```

Exit code 0 means zero errors. `notice` and `warning` lines never fail the build.

## The rules CI enforces

**Ids are permanent.** `course-…`, `lesson-…`, `achievement-…`, `path-…`, `quest-…`, `instructor-…`, all `kebab-case`. Course and achievement ids are capped at **32 characters**. Renaming an id after a course ships is rejected — it would lose the progress of everyone who's taken it.

**`slots.lock.json` is generated, not edited.** Never change it by hand; the checks regenerate and compare it.

**XP has a ceiling.** `xpPerLesson × lessonCount ≤ 10000` — go over it and the course can't be completed. `xpPerLesson` is 1–100.

**Challenges are executed.** For a TypeScript challenge, the checks run your `solution.ts` against `tests.json` — it must pass every case — and run your `starter.ts`, which **must fail** at least one. A starter that already passes has nothing to solve. Rust challenges are checked when a learner runs them, not on your PR — but their `starter`, `solution`, and `tests` files still have to be present.

**Every course needs an instructor with a wallet.** `course.instructor` must point at an [instructor](./instructors/README.md) file whose `wallet` is a real Solana address — that's how the instructor gets credited (and rewarded) for the course. A course with no instructor, or an invalid wallet, is rejected.

**Capabilities are ordered.** A block that `consumes: [deployed-program]` must appear after a block that produces it, anywhere earlier in the course. Only a `wallet-funding` block can produce `funded-wallet`; only a `deployable` `code` block can produce `deployed-program`.

**No orphan files.** Every file in a lesson directory must be referenced by a block (`src`, `starter`, `solution`, `tests`, `idl`) or linked from a prose `.md`.

**A non-draft learning path needs at least one course.** Mark work-in-progress paths `draft: true`.

## Quizzes

Correctness is keyed to a stable option `id`, never to array position, so reordering options can't silently change the answer.

```yaml
- key: check
  type: quiz
  questions:
    - id: q1
      prompt: Which accounts store state?
      multiSelect: true
      options:
        - { id: a, label: Data accounts, correct: true }
        - { id: b, label: Instructions, correct: false, feedback: "Inputs, not accounts." }
```

With `multiSelect: false` exactly one option may be correct; with `true`, at least one.

## Reflections are never graded

An `openEnded` block asks the learner to write, and the AI replies once with feedback. It awards no XP and gates nothing. Don't use it to test knowledge — use a `quiz` or a `code` block.

## Answer keys are public

`solution` files and all test cases live in this public repo, by design. Grading is by **execution in a sandbox**, not by secrecy. Don't write a lesson whose value depends on the learner not seeing the answer.
