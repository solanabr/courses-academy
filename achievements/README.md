# Achievements

One YAML file per achievement. An achievement is a soulbound credential a learner earns by meeting an **award** condition.

```yaml
id: achievement-first-steps         # achievement-<kebab>, ≤ 32 characters, permanent
name: First Steps
description: Complete your first lesson.   # optional
icon: rocket                        # optional icon identifier
glyph: "01"                         # optional 1–2 chars shown on the medal
solTier: false                      # optional; a premium/highlighted tier, default false
category: progress                  # progress | streaks | skills | community | special
xpReward: 50                        # 1–5000
maxSupply: 0                        # optional; 0 = unlimited, default 0
metadataUri: https://…             # optional NFT metadata URI
award: { kind: lessons-completed, gte: 1 }   # REQUIRED — see below
```

## The `award` — how it's earned

`award` is **required**: an achievement with no award kind can never be unlocked. Pick one `kind` from this fixed set:

| `kind` | Extra fields | Unlocks when… |
|---|---|---|
| `lessons-completed` | `gte` (≥1) | the learner has completed ≥ `gte` lessons total |
| `lessons-completed-in-course` | `course`, `gte` (≥1) | ≥ `gte` lessons completed **in** `course` |
| `course-completed` | `course` | that course is finalized |
| `path-completed` | `path` | every course in that learning path is finalized |
| `streak` | `days` (≥1) | a daily streak reaches `days` |
| `user-number` | `lte` (≥1) | the learner's signup number is ≤ `lte` (early-adopter) |
| `community-stat` | `stat`, `gte` (≥1) | a community counter reaches `gte` |
| `manual` | — | admin-granted only (e.g. `bug-hunter`) |

`community-stat`'s `stat` is one of: `totalThreads`, `totalAnswers`, `acceptedAnswers`, `totalCommunityXp`.

```yaml
# a course-completion badge
award: { kind: course-completed, course: course-solana-fundamentals }

# a learning-path badge
award: { kind: path-completed, path: path-solana-core }

# a 7-day streak
award: { kind: streak, days: 7 }
```

### Notes

- `course` and `path` in an award must be real ids — the checks don't currently verify those references, so double-check them by hand.
- `id` is permanent (`achievement-<kebab>`, ≤ 32 characters). Never rename it once live.
- Only the kinds above exist. There's intentionally no "first-try" or "speed" kind, so don't design an achievement that would need one.
