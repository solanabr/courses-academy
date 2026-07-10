# Instructors

One YAML file per instructor. An instructor is the teaching persona shown on a course **and** the person who receives its on-chain creator reward.

```yaml
id: instructor-ana-santos           # instructor-<kebab>, permanent
name: Ana Santos
wallet: B7o8NfV81HzjuZFWQTTx3Xdvh77Dqoajwib3kWEnvzJF   # required — your Solana wallet
bio: Solana developer and educator.   # optional
avatar: https://…                   # optional
socialLinks:                        # optional
  twitter: anasantos
  github: anasantos
```

## Your `wallet` is required

A course names its instructor (`course.instructor: instructor-ana-santos`), and the instructor's `wallet` is how they get credited for it — creator reward XP goes to that wallet. It has to be a real Solana wallet address (the one you sign in to the platform with); an invalid one fails the checks. Because rewards are set at the moment a course goes live, get your wallet right before then.

## Rules

- `id` is permanent. Never rename a live instructor.
- Every course needs an instructor — a course without one is rejected.
