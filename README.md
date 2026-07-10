# Superteam Academy — Courses

This repository holds every course on [Superteam Academy](https://superteam-academy-web.vercel.app) — the lessons, challenges, quizzes, quests, and achievements. It's all plain text: YAML, Markdown, and code files.

**Anyone can contribute.** Add a course, add a lesson, fix a typo, improve a challenge — open a pull request. Automated checks validate your changes, a maintainer reviews them, and once merged your content goes live on the platform.

New here? Jump to **[Add a course](#add-a-course-in-five-steps)** or read the full **[contributor guide](./CONTRIBUTING.md)**.

## How content is organized

| Folder | What's in it | Guide |
|---|---|---|
| `courses/<slug>/` | a course: its details, modules, and lessons | [courses/README.md](./courses/README.md) |
| `achievements/*.yaml` | badges learners earn, and how they're unlocked | [achievements/README.md](./achievements/README.md) |
| `quests/*.yaml` | daily / streak challenges that award XP | [quests/README.md](./quests/README.md) |
| `paths/*.yaml` | ordered collections of courses | [paths/README.md](./paths/README.md) |
| `instructors/*.yaml` | the people who teach the courses | [instructors/README.md](./instructors/README.md) |
| `courses/_template/` | a complete example to copy when starting a course | [courses/_template/README.md](./courses/_template/README.md) |

## A lesson is a list of blocks

Every lesson is a title plus an ordered list of **blocks**. You mix and match them to build the lesson:

| Block | What it is |
|---|---|
| `prose` | Markdown text |
| `video` | a YouTube / Vimeo embed |
| `code` | a coding challenge — editor, starter code, and tests |
| `quiz` | multiple choice, one or many correct answers |
| `openEnded` | a reflection prompt; the learner writes, the AI replies once |
| `wallet-funding` | a button to fund a devnet wallet |
| `program-explorer` | an interface to call a deployed program |
| `deployed-program-card` | shows the learner's deployed program |

## A few rules worth knowing up front

The automated checks enforce these — they're mostly about not breaking things for learners who've already started a course.

- **Once a course is live, don't rename its `id` (or a lesson's).** Learners' progress is tied to those ids, so renaming one loses their history. Add and reorder freely; just don't rename.
- **Don't hand-edit `slots.lock.json`.** It's generated automatically. Editing it by hand can corrupt learner progress; the checks will stop you.
- **A challenge's reference solution must pass its own tests, and its starter must fail them** — a starter that already passes has nothing to solve.
- **Answers are public.** Solution files and tests live right here in the open. Challenges are graded by running the learner's code, not by hiding the answer — so don't write a lesson that only works if the answer is secret.
- **Every course needs an instructor.** Point `course.instructor` at an [instructor](./instructors/README.md) file. Their wallet is how they receive credit (and rewards) for the course.

Each folder's README has the exact fields and a worked example.

## Add a course in five steps

1. Copy the template: `cp -r courses/_template courses/<your-slug>`
2. Edit `course.yaml` — the title, difficulty, instructor, and the modules-and-lessons outline.
3. Write your lessons under `lessons/<slug>/` (text, challenges, quizzes).
4. Make sure there's an [instructor](./instructors/README.md) file for you, and `course.instructor` points at it.
5. Check your work locally (below), then open a pull request.

## Check your work

If you use VSCode or Cursor with the [Red Hat YAML extension](https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml), your files are validated **as you type** — you'll see errors before you even commit.

To run the full check yourself:

```bash
git clone https://github.com/solanabr/superteam-academy
cd superteam-academy && pnpm install
pnpm --filter @superteam-lms/content-lint exec tsx src/cli.ts /path/to/courses-academy
```

Exit code `0` means you're good. (You don't have to run it — the same check runs automatically on your pull request.)

Full details, including quizzes and reflections, are in **[CONTRIBUTING.md](./CONTRIBUTING.md)**.
