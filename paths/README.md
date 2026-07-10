# Learning Paths

One YAML file per path. A path is an ordered shelf of courses shown on the platform (e.g. "Solana Core").

```yaml
id: path-solana-core                # path-<kebab>, ≤ 128 UTF-8 bytes
                                    # NOTE the `path-` prefix — it deliberately
                                    # does not match the type name "learningPath"
slug: solana-core
title: Solana Core
description: Start here…            # optional
tag: Foundation                     # optional short label
order: 1                            # optional display order (lower first), default 0
difficulty: beginner                # beginner | intermediate | advanced
draft: false                        # optional, default false — see below
courses:                            # course ids, in display order
  - course-solana-fundamentals
  - course-building-first-program
```

## The `draft` rule

**A non-draft path must contain at least one course.** An empty shelf renders as a dead link, so a path that isn't ready yet must set `draft: true`:

```yaml
id: path-security
title: Security
difficulty: advanced
draft: true                         # no courses yet — kept out of the catalogue
courses: []
```

`path-completed` achievements target a path by id, so a path referenced by an achievement should be populated (non-draft).

## Rules

- `courses` entries must be real course ids. (CI checks course→module→lesson and path→course references and errors on a missing course.)
- `id` is permanent. Never rename a live path.
