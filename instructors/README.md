# Instructors

One YAML file per instructor. An instructor is the teaching persona shown on a course **and** the person who receives its on-chain creator reward.

```yaml
id: instructor-ana-santos           # instructor-<kebab>, ≤ 128 UTF-8 bytes
name: Ana Santos
wallet: B7o8NfV81HzjuZFWQTTx3Xdvh77Dqoajwib3kWEnvzJF   # REQUIRED — see below
bio: Solana developer and educator.   # optional
avatar: https://…                   # optional
socialLinks:                        # optional
  twitter: anasantos
  github: anasantos
```

## `wallet` is required and load-bearing

A course points at an instructor (`course.instructor: instructor-ana-santos`), and at sync time the instructor's `wallet` becomes that course's **on-chain `Course.creator`** — the Pubkey that receives creator-reward XP. The same wallet is how the platform maps the instructor to a user account (`profiles.wallet_address`).

Because it's the on-chain reward recipient, the wallet is validated strictly:

- **base58**, decodes to exactly **32 bytes**, and lies **on the ed25519 curve** (a PDA / off-curve address is rejected — it couldn't own the reward token account).
- Get yours (or a teacher's) from the wallet they use to sign in to the platform.

There is **no fallback**: a course whose instructor has no wallet, or an invalid wallet, fails the sync loudly rather than paying rewards to the wrong key. `Course.creator` is set once when the course is first deployed on-chain and is immutable after that, so get the wallet right before a course goes live.

> **Not yet wired end-to-end.** The wallet is stored and flows to `Course.creator`, and the resolution is built, but the `wallet → platform user` linkage in the app is still in progress — see the app repo. Don't rely on the platform-identity half yet.

## Rules

- `id` is permanent. Never rename a live instructor.
- An instructor is referenced by courses; a course with no instructor has no on-chain creator and is rejected by the sync.
