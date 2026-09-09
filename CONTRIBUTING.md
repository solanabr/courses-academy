# Contributing

## Quick start

1. Read [courses/CATALOG.md](./courses/CATALOG.md) — the approved catalog, each course's syllabus, the lesson shapes, and the exact APIs to teach. This document is *what* to write; the rest of this file is *how*.
2. Fork this repo and create a branch.
3. Copy `courses/_template/` to `courses/<your-slug>/` and edit it.
4. Open a pull request. CI runs the full validator.

Your editor will autocomplete and validate as you type — `.vscode/settings.json` maps every file to its JSON Schema in `schema/`. In VSCode / Cursor this needs the **[Red Hat YAML extension](https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml)** (`redhat.vscode-yaml`); the schema files themselves are generated, so don't hand-edit `schema/`.

Each folder has its own `README.md` with the fields, the controlled vocabulary, and a worked example — read the one for what you're editing ([courses](./courses/README.md), [achievements](./achievements/README.md), [quests](./quests/README.md), [paths](./paths/README.md)).

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

**Every course needs a creator wallet.** `course.creator` must be a real Solana address — that's how the author gets credited (and earns creator XP). It becomes `Course.creator` on-chain and **cannot be changed after the course is created**, so use the wallet you actually want paid.

**Capabilities are ordered.** A block that `consumes: [deployed-program]` must appear after a block that produces it, anywhere earlier in the course — **or in a course listed earlier on a learning path this course sits on** (`paths/*.yaml`), which is how a course declares that it consumes the previous course's output. Only a `wallet-funding` block can produce `funded-wallet`; only a `deployable` `code` block can produce `deployed-program`. Producing across the boundary is a path-order fact only: it does not set `prerequisiteCourse`, which is written on-chain and gates enrollment.

**No orphan files.** Every file in a lesson directory must be referenced by a block (`src`, `starter`, `solution`, `tests`, `idl`) or linked from a prose `.md`. This is why translations live in a course-level `l10n/` folder rather than beside the lessons they translate — see [Translations](#translations) below.

**A translation is an overlay, not a copy.** Never duplicate a course to publish it in another language. `course_id` is a PDA seed, so a duplicate forks the course's enrolment, progress bitmap and XP ledger permanently, and the two can never be merged.

**An empty learning path must say why it is empty.** A path with no courses is hidden by the app either way, so it has to declare intent: `draft: true` means courses are on the way (and it must list them — `draft: true` with `courses: []` is an error), `retired: true` means permanently emptied with the id preserved (and its `courses` must stay empty). A path that never shipped to learners is deleted outright rather than retired — there is no id to reserve. See [paths/README.md](./paths/README.md).

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
        - { id: a, label: Data accounts, correct: true, feedback: "State lives in the account, not the instruction that writes it." }
        - { id: b, label: Instructions, correct: false, feedback: "Inputs, not accounts." }
```

Write `feedback` on every option, including the correct ones — otherwise the missing field is the answer key.

With `multiSelect: false` exactly one option may be correct; with `true`, at least one.

## Reflections are never graded

An `openEnded` block asks the learner to write, and the AI replies once with feedback. It awards no XP and gates nothing. Don't use it to test knowledge — use a `quiz` or a `code` block.

## Answer keys are public

`solution` files and all test cases live in this public repo, by design. Grading is by **execution in a sandbox**, not by secrecy. Don't write a lesson whose value depends on the learner not seeing the answer.

## Translations

Every course declares the language it was written in — `sourceLocale: en | pt-BR | es` in `course.yaml` — and gains other languages through an optional `courses/<slug>/l10n/<locale>/` overlay holding a `strings.yaml` and any prose you translated. You don't have to translate everything: coverage is per string and per file, and anything you leave out renders in the course's source language. A translation is its own pull request, per course per language, and it touches nothing outside `l10n/`.

The format, the rules that keep a translation away from ids and answer keys, and a worked example are in [courses/README.md § Translations](./courses/README.md#translations) and `courses/_template/l10n/pt-BR/`. Read them before starting — the overlay is not yet rendered by the app, and the constraints are currently enforced by review rather than by CI.
