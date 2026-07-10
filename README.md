# Superteam Academy — Courses

The **source of truth** for every course, lesson, achievement, quest, learning path, and instructor on [Superteam Academy](https://superteam-academy-web.vercel.app).

Content lives here as plain YAML, Markdown, and source files. Teachers contribute by opening a pull request. Once merged, an admin runs the content sync, which projects this repo into Sanity and then on-chain.

```
courses-academy (git)  ──PR + CI──▶  main  ──admin sync──▶  Sanity  ──▶  app + on-chain
```

Nothing is authored in Sanity Studio. Sanity is a rebuildable projection of this repo.

## Layout

```
courses/<slug>/
  course.yaml              # id, difficulty, xpPerLesson, creator, modules
  slots.lock.json          # machine-owned — never hand-edit (see below)
  lessons/<slug>/
    lesson.yaml            # id, title, ordered blocks[]
    intro.md               # a `prose` block
    exercise/
      starter.ts|rs        # must FAIL the tests
      solution.ts|rs       # must PASS the tests
      tests.json
    program.idl.json       # a `program-explorer` block
achievements/<slug>.yaml
quests/<slug>.yaml
paths/<slug>.yaml
instructors/<slug>.yaml
schema/*.json              # JSON Schema, generated — powers editor autocomplete
courses/_template/         # a working example; linted, never published
```

## Lessons are made of blocks

A lesson is metadata plus an **ordered list of typed blocks**. Adding a new kind of activity means adding a block type — it never reshapes a course or a lesson.

| Block | Graded? | Purpose |
|---|---|---|
| `prose` | no | Markdown, from a `.md` file |
| `video` | no | YouTube / Vimeo embed |
| `code` | **yes** | Editor + tests, graded server-side |
| `quiz` | **yes** | Multiple choice, one or many correct |
| `openEnded` | no (required) | A reflection; one AI reply, feedback only. **Never mints XP.** |
| `wallet-funding` | no | Fund a devnet wallet |
| `program-explorer` | no | Call a deployed program from an IDL |
| `deployed-program-card` | no | Show the learner's deployed program |

## `slots.lock.json` — read this before restructuring a course

Each lesson owns a permanent **slot**: its bit position in the on-chain completion bitmap. Slots are decoupled from display order, which is what makes reordering and regrouping lessons safe for learners who are already enrolled.

- **Never hand-edit `slots.lock.json`.** CI regenerates it and fails on a diff.
- Reordering lessons, or moving one between modules, is free — slots don't move.
- Adding a lesson appends a new slot.
- A slot is **never renumbered and never reused**.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md). In short: fork, add or edit content, open a PR. CI validates the whole tree, and **runs every TypeScript reference solution against its own tests**.

Your `course.yaml` must declare you as the creator:

```yaml
creator: { githubId: "61333600" }   # your NUMERIC GitHub user id
```

Find yours with `curl -s https://api.github.com/users/<you> | jq .id`.

> **Not yet wired.** `creator.githubId` is stored and projected into Sanity, but nothing resolves it to a wallet yet, so on-chain `Course.creator` still falls back to the platform authority and creator rewards do not reach the teacher. Tracked in the app repo — see `admin/courses/sync/route.ts`, which still reads the removed `course.author` field.
