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
  l10n/<locale>/         # optional translations — see Translations below
```

Copy [`_template/`](./_template) to start a new course. Everything is validated in CI; `_template/` itself is linted but **never published**.

## `_draft/` — parked courses

`courses/_draft/<slug>/` holds courses that are staged out of the live catalog. Two separate mechanisms make this work, and both matter:

- **Never compiled/published:** the monorepo's content pipeline excludes any path containing a `_draft/` segment from the synced tree (monorepo #973, `lib/content/compile` — the same exclusion that keeps `courses/_template/` out). Parked content cannot reach the bundle, the app, or a `content.lock` bump.
- **Invisible to lint:** the linter discovers content only at fixed depth (`courses/<slug>/`, `paths/<name>.yaml`, …) and skips `_draft/` as a named convention — so parked files are neither validated nor blocking. A consequence worth internalizing: **a green content-lint check says nothing about parked files.** Anything restored out of `_draft/` gets linted for the first time on that PR.

Live `paths/` and `achievements/` files must not reference a drafted course id (dangling references are CI errors — park the referencing file together with its target, or retarget it). The same convention exists as `paths/_draft/` and `achievements/_draft/`. To restore a course, `git mv` it back to `courses/<slug>/` unchanged — its `slots.lock.json` travels with it and must not be regenerated.

## Images and visual assets

Images live in a per-lesson `assets/` folder and are embedded from the lesson's prose markdown with a relative link:

```
courses/<slug>/lessons/<lesson>/
  intro.md               # …contains ![alt text](assets/v01-diagram.png)
  assets/
    v01-diagram.png
```

Every file inside a lesson folder must be referenced from a block or from prose markdown — an unreferenced file is a CI error (orphan check). Write meaningful alt text; it's the accessibility caption.

A course may also keep the *sources* that generated its images (HTML/CSS renders, etc.) in a course-level `visual-src/` folder — e.g. `courses/<slug>/visual-src/<lesson>/v01-diagram.html`. `visual-src/` is a **build input**: the linter ignores it and it is never published, so visuals stay editable and re-renderable from the repo without shipping their sources. The other course-level folder, `l10n/`, is not a build input — see [Translations](#translations).

## Translations

A course is **one course in N languages** — never one course per language. Duplicating a course to translate it would fork its on-chain identity, its enrolment, its progress bitmap and its XP, and two course ids can never be merged afterwards.

> The format below is settled and safe to author against, but **the app does not render overlays yet** — see [Status](#status-authorable-now-not-yet-rendered) before you start.

Every course declares the language it was **written in**, and translations are added as an optional overlay beside it:

```
courses/<slug>/
  course.yaml                      # sourceLocale: pt-BR   ← the language the base tree IS
  lessons/<lesson-slug>/           # the source-language content — never touched by a translation
    lesson.yaml
    intro.md
    assets/v01-diagram.png
  l10n/
    en/                            # OPTIONAL — omit entirely for a single-language course
      strings.yaml                 # every structured string for this course + locale
      assets/banner.webp           # optional localized thumbnail
      lessons/<lesson-slug>/
        intro.md                   # translated prose — same filename as the source
        assets/v01-diagram.png     # optional localized image — same filename
```

### The six rules

1. **`l10n/<locale>/` mirrors the course folder.** Only files that *differ* appear in it. Anything absent falls back to the source language, per item.
2. **Filenames match their source exactly** — never renamed, never suffixed. That is what lets a translated `intro.md` keep writing `![](assets/v01-diagram.png)` unchanged, and it handles lessons with more than one prose file for free.
3. **Three filenames are forbidden anywhere under `l10n/`:** `course.yaml`, `lesson.yaml`, `slots.lock.json`. The bundle compiler matches those names by suffix at *any* depth, so it would ship your overlay as a duplicate course or lesson. Content-lint catches the two `.yaml` names and fails the build; **it does not catch `slots.lock.json`**, because that scan only walks `.yaml` — a stray lock file reaches the bundle unflagged. Structured strings go in `strings.yaml`.
4. **An overlay carries display strings and images only.** Never ids, slugs, `skills`, block `key`s or order, quiz `correct` flags, XP/creator/track fields, code `starter`/`solution`, `parsons` `lines`/`correctOrder`, or test `input`/`expectedOutput`. Those reach the on-chain surface or the grader, and a translation must be incapable of touching them.
5. **Fallback is to the source language, not to English.** There is no privileged English layer — for a `sourceLocale: pt-BR` course, untranslated strings and images render in Portuguese.
6. **Available languages are derived, never authored** — `sourceLocale` plus whatever `l10n/*` folders exist. There is no list to keep in sync.

### Single-language courses are the normal case

`l10n/` is optional, so all three shapes fall out of the same mechanism with no flags:

| Shape | On disk | Available in |
|---|---|---|
| Portuguese only | `sourceLocale: pt-BR`, no `l10n/` | `pt-BR` |
| Portuguese + English | `sourceLocale: pt-BR` + `l10n/en/` | `pt-BR`, `en` |
| English + two translations | `sourceLocale: en` + `l10n/pt-BR/` + `l10n/es/` | `en`, `pt-BR`, `es` |

Adding a language later is `mkdir l10n/<locale>/`. Dropping one is `git rm -r l10n/<locale>/`, and the course keeps working in its source language immediately. A single-language course pays nothing for machinery it does not use.

### `l10n/<locale>/strings.yaml`

One file per course per language, keyed wherever possible by the stable identifiers — module key, lesson slug, block key, question id, option id, test case id — so reordering questions or options in the source can never mis-bind a translation. Prose is *not* in this file; a translated `.md` sitting at the mirrored path is the index.

```yaml
locale: en                           # must match the folder name (see below)

course:
  title: Solana & Superteam in Two Minutes
  description: The two-minute pill…
  modules:
    pilula:                            # module key from course.yaml
      title: The Pill
      description: Why the money is moving…

lessons:
  solana-em-2-minutos:                 # lesson slug = its folder name
    title: Solana in 2 minutes
    blocks:
      check:                           # block key from lesson.yaml
        questions:
          q1:
            prompt: Solana is as fast as what?
            explanation: Instant payments are the anchor…
            options:
              b:
                label: Confirms in under half a second
              c:
                label: Like a bank queue
                feedback: The point is precisely NOT being a queue.
      challenge:                       # a `code` block
        hints:                         # keyed by index; sparse is fine
          0: A PDA has no key; the program signs for it.
          2: Check the seed order before the bump.
        tutorNotes:                    # same shape — this is what the AI Partner reads
          0: Learners usually forget the discriminator.
        tests:                         # keyed by test case id
          t1:
            description: Compiles against anchor-lang 1.1.2
            failureMessage: VaultState is missing fields.
      order-the-lines:                 # a `parsons` block
        prompt: Put the lines in the order that compiles.
        explanation: The semicolon is what changes the meaning.
        # `lines` and `correctOrder` are absent by design — the lines are code,
        # and `correctOrder` is the answer key. Neither is ever overlaid.
      watch:
        url: "https://…"               # optional dubbed/subtitled variant
```

The only field you declare that could be derived is `locale`, and it earns its place: macOS filesystems are case-insensitive, so `l10n/pt-br/` and `l10n/pt-BR/` are the same folder on your machine but different ones in git. Stating it lets that mismatch be caught. The source language is *not* repeated here — it lives in `course.yaml`, where a second copy could only drift.

`hints` and `tutorNotes` are the only translatable fields the source stores as a bare list with no ids, so their overlay keys are **array indices**. Keying them rather than re-listing them keeps rule 1 exact — translate hints 0 and 2 and leave 1 in the source language — instead of forcing all-or-nothing on the block.

A worked example — deliberately partial, covering 2 of the template's 5 lessons — lives at [`_template/l10n/pt-BR/`](./_template/l10n/pt-BR).

### Translating the text but not the images is fine

Most translators will not re-render artwork, and they are not expected to. Fallback is **per file**, so a lesson with three of its ten images localized shows three localized and seven source — never all-or-nothing, and never blocked on a designer.

```
courses/<slug>/l10n/pt-BR/
  strings.yaml
  lessons/an-anchor-vault/
    intro.md          # translated prose
                      # no assets/ — every image falls back to the source
```

Three things follow, and they are the point:

- **An untranslated image costs nothing.** Nothing is duplicated unless someone actually re-rendered it.
- **Mixed-language pages are legal.** An English diagram inside Portuguese prose ships. Your translated `intro.md` still carries translated alt text, so the accessibility caption is localized even when the image is not.
- **A localized image keeps the source filename, extension included**, and inherits the 1 MiB per-asset cap. If you can re-render, put the source in `l10n/<locale>/visual-src/` the same way the base course does.

### What the app will do with this

- It derives the language list from `sourceLocale` + your `l10n/` folders, and lists the course under exactly those.
- It merges **per item** — string by string, file by file, image by image — falling back to the source language for anything you did not translate.
- It reads **only the source tree** when grading. Quiz `correct` flags and test `expectedOutput` never come from a translation.
- A course reached at a language it does not have still renders, in its source language, with a notice — links never break.

### Status: authorable now, not yet rendered

**The app side is not built yet.** `l10n/` folders are inert today — the bundle compiler does not read them, so a translation you merge is staged, not shipped. Writing one now is safe and purely additive; it starts rendering when the app lands, with no change to what you authored.

Two consequences while that is true:

- **`strings.yaml` draws a content-lint `warning`** — *"unclassified content file … it is NOT linted by any gate"*, suggesting you park it under `_draft/`. **Ignore it, and do not move the file.** Warnings never fail the build. It disappears once the linter's classifier learns the path.
- **Nothing enforces the rules above except review** and the editor schemas in [`schema/`](../schema). Rule 4 in particular is a convention until the app ships a gate for it, so a translation PR needs a human to check it.

## `course.yaml`

```yaml
id: course-solana-fundamentals      # course-<kebab>, ≤ 32 characters, permanent
slug: solana-fundamentals
sourceLocale: en                    # the language this course is WRITTEN in; en | pt-BR | es
title: Solana Fundamentals
description: A beginner course covering the fundamentals…
difficulty: beginner                # beginner | intermediate | advanced
duration: 10                        # hours (display only)
xpPerLesson: 10                     # 1–100; XP for completing each lesson
xpReward: 600                       # XP shown on the course card; ≤ 5000
creatorRewardXp: 500                # optional; XP to the creator per completion; ≤ 5000, default 0
minCompletionsForReward: 10         # optional; completions before creator reward pays; default 0
trackId: 1                          # optional grouping; default 0
trackLevel: 1                       # optional position in track; default 0
tags: [solana, blockchain, beginner]
creator: B7o8NfV81HzjuZFWQTTx3Xdvh77Dqoajwib3kWEnvzJF   # the course creator's Solana wallet (Course.creator on-chain)
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
- **`sourceLocale` is set once, at creation.** It states which language the course was authored in; changing it later would silently relabel every untranslated string. Translate into a new language by adding `l10n/<locale>/` — never by editing `sourceLocale`.

## `slots.lock.json` — do not touch

```json
{ "version": 1, "slots": { "lesson-intro-solana": 0, "lesson-setup-environment": 1 }, "retired": [], "next": 2 }
```

Each lesson owns a permanent **slot** that tracks learner progress, kept separate from display order. That separation is what lets you reorder lessons, move one between modules, or insert one — all without disturbing learners who've already started.

- **Never hand-edit it.** It's generated automatically, and the checks fail on any manual change.
- A slot is **never renumbered and never reused**. Removing a lesson retires its slot.
- You don't create or maintain this file — the tooling does.

## Lessons are ordered blocks

See [the block-type table in the root README](../README.md#a-lesson-is-a-list-of-blocks). A lesson is `id`, `slug`, `title`, and a `blocks[]` array. Adding a new kind of activity is a new block type — it never reshapes a course or a lesson.

A lesson may also carry an optional top-level `versionStamp`:

```yaml
versionStamp:
  checkedAt: "2026-09-05"        # the date the pins were verified, YYYY-MM-DD
  packages:                      # package/tool name → version string
    "@solana/kit": "7.1.1"
    "@solana/subscriptions": "0.5.0"
```

It records what the lesson's code was verified against. [CATALOG §6](./CATALOG.md#6-technical-currency-ruling) requires a stamp on every **code-bearing** lesson; the content-lint gate that fails an unstamped code lesson is a pending app-repo work item, so until it lands, review enforces the requirement.

For a `code` block, the reference `solution` must pass `tests.json` and the `starter` must **fail** at least one case. TypeScript challenges are checked automatically when you open a PR; Rust challenges are checked when a learner runs them.
