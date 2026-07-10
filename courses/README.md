# Courses

Each course is a **folder** under `courses/`. The folder name is the course slug; the `id` inside `course.yaml` is its permanent identity.

```
courses/<slug>/
  course.yaml            # metadata + module → lesson structure
  slots.lock.json        # machine-owned — never hand-edit
  lessons/<slug>/
    lesson.yaml          # title + ordered blocks[]
    intro.md             # a `prose` block's markdown
    exercise/
      starter.ts|rs      # what the learner starts with — must FAIL the tests
      solution.ts|rs     # the reference answer — must PASS the tests
      tests.json         # graded cases
    program.idl.json     # a `program-explorer` block's IDL
```

Copy [`_template/`](./_template) to start a new course. Everything is validated in CI; `_template/` itself is linted but **never published**.

## `course.yaml`

```yaml
id: course-solana-fundamentals      # course-<kebab>, ≤ 32 characters, permanent
slug: solana-fundamentals
title: Solana Fundamentals
description: A beginner course covering the fundamentals…
difficulty: beginner                # beginner | intermediate | advanced
duration: 10                        # hours (display only)
xpPerLesson: 10                     # 1–100; XP for completing each lesson
xpReward: 600                       # XP shown on the course card; ≤ 5000
creatorRewardXp: 500                # optional; XP to the instructor per completion; ≤ 5000, default 0
minCompletionsForReward: 10         # optional; completions before creator reward pays; default 0
trackId: 1                          # optional grouping; default 0
trackLevel: 1                       # optional position in track; default 0
tags: [solana, blockchain, beginner]
instructor: instructor-ana-santos   # the course's instructor (see instructors/README.md)
prerequisiteCourse: course-…        # optional; must be another real course id, not itself
modules:
  - key: getting-started            # kebab, unique within the course
    title: Getting Started
    description: …
    lessons:                        # lesson ids, in display order
      - lesson-intro-solana
      - lesson-setup-environment
```

### Rules the checks enforce

- **`id` is permanent.** `course-<kebab>`, ≤ 32 characters. Never rename it once the course is live — learners' progress is tied to it.
- **A lesson may appear in only one module**, and module `key`s are unique within a course.
- **`xpPerLesson × total lessons ≤ 10000`** — a platform cap. Go over it and the course can't be completed. `xpPerLesson` is 1–100.
- **At most 256 lessons per course.**
- `prerequisiteCourse`, if set, must be a real course and cannot be the course itself.

## `slots.lock.json` — do not touch

```json
{ "version": 1, "slots": { "lesson-intro-solana": 0, "lesson-setup-environment": 1 }, "retired": [], "next": 2 }
```

Each lesson owns a permanent **slot** that tracks learner progress, kept separate from display order. That separation is what lets you reorder lessons, move one between modules, or insert one — all without disturbing learners who've already started.

- **Never hand-edit it.** It's generated automatically, and the checks fail on any manual change.
- A slot is **never renumbered and never reused**. Removing a lesson retires its slot.
- You don't create or maintain this file — the tooling does.

## Lessons are ordered blocks

See [the block-type table in the root README](../README.md#lessons-are-made-of-blocks). A lesson is `id`, `slug`, `title`, and a `blocks[]` array. Adding a new kind of activity is a new block type — it never reshapes a course or a lesson.

For a `code` block, the reference `solution` must pass `tests.json` and the `starter` must **fail** at least one case. TypeScript challenges are checked automatically when you open a PR; Rust challenges are checked when a learner runs them.
