# Quests

One YAML file per quest. Quests are **global** (not per-course) daily/multi-day challenges that award XP for engagement.

```yaml
id: quest-complete-lesson           # quest-<kebab>, ≤ 128 UTF-8 bytes
name: Complete a Lesson
description: Finish any lesson today.   # optional
type: lesson                        # see the table below
icon: check-circle                  # optional
xpReward: 25                        # 1–5000
targetValue: 1                      # ≥ 1 — how many of `type` to satisfy the quest
resetType: daily                    # daily | multi_day
active: true                        # optional, default true
```

## Quest `type`

| `type` | Progress counts… |
|---|---|
| `lesson` | any lesson completed |
| `lesson_batch` | lessons completed toward a `targetValue` > 1 (e.g. "3 lessons today") |
| `challenge` | lessons containing a `code` block completed |
| `login_streak` | consecutive-day logins (pair with `resetType: multi_day`) |
| `module` | modules completed |

## Rules

- **`targetValue` must be ≥ 1.** A `0` would complete every day and mint free XP; the schema forbids it. (Sanity's own validation doesn't run on programmatic writes, so this schema is the only guard.)
- `resetType` is `daily` or `multi_day`. Note: the current daily-quest RPC computes its period from `type`, not from `resetType`, so `resetType` is carried for the client but has no behavioral effect yet — wiring or dropping it is tracked as a separate app change.
- `active: false` hides a quest without deleting it.
