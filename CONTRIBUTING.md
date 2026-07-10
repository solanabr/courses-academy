# Contributing

## Quick start

1. Fork this repo and create a branch.
2. Copy `courses/_template/` to `courses/<your-slug>/` and edit it.
3. Open a pull request. CI runs the full validator.

Your editor will autocomplete and validate as you type — `.vscode/settings.json` maps every file to its JSON Schema in `schema/`.

## Running the validator locally

The linter lives in the app repo, which is public:

```bash
git clone https://github.com/solanabr/superteam-academy
cd superteam-academy && pnpm install
pnpm --filter @superteam-lms/content-lint exec tsx src/cli.ts /path/to/courses-academy
```

Exit code 0 means zero errors. `notice` and `warning` lines never fail the build.

## The rules CI enforces

**Ids are permanent.** `course-…`, `lesson-…`, `achievement-…`, `path-…`, `quest-…`, `instructor-…`, all `kebab-case`. Course and achievement ids become on-chain PDA seeds and are capped at **32 bytes**. Changing an id after it ships is rejected — it would orphan every learner's progress.

**`slots.lock.json` is machine-owned.** Never edit it by hand. CI regenerates and compares.

**XP has a hard ceiling.** `xpPerLesson × lessonCount ≤ 10000`. Above it, `finalize_course` reverts forever: no learner could ever complete the course, no credential, no creator reward. `xpPerLesson` is 1–100.

**Code blocks are executed.** For a TypeScript `standard` block, CI runs your `solution.ts` against `tests.json` — it must pass every case — and runs your `starter.ts`, which **must fail** at least one. A starter that already passes is a bug. Rust and `buildable` blocks are graded at sync time instead, and CI reports them as a `notice`.

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
