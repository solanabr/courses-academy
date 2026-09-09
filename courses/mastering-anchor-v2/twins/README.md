# The m01-l1 twins — maintainer sources

**Never published to learners.** Like the course-root `README.md`, nothing in the
compiled bundle reads this directory. It exists so the next devnet reset is a
script run, not an archaeology project.

m01-l1's lab has learners read the compute-unit meter off two live devnet
programs — the same counter written twice, once per Anchor line — via four
values pinned inline in the lesson. These are their sources:

| crate | line | teaches | body |
|---|---|---|---|
| `counter-v1-twin` | `anchor-lang = "1.1.2"` | `Account<'info, T>`, `has_one` | 32-byte authority + `u64` count |
| `counter-v2-twin` | `anchor-lang = "2.0.0-rc.1"` + course pin set | Pod `Account<T>`, `address = counter.authority` | same 40 bytes, `Address` + `PodU64` |

Same instruction pair (`initialize`, `increment`), same signer + owner +
discriminator checks, per the lesson's step-6 claim — keep that symmetry if you
ever touch the handlers, because the lesson's diagnosis depends on it. Each
crate's `examples/land.rs` builds and signs the init/increment transactions
offline (printed base64 for `sendTransaction`); `redeploy.sh` drives the whole
flow and prints the four pins plus the two expected log lines.

## Current live values (landed + verified 2026-09-02)

- `V1_TWIN_ID` = `8bhX52w9mGGaAFJwsoWLpv3nrZsXzc3ZfE2P622uGt3z` (1714 CU on the reference call)
- `V2_TWIN_ID` = `2fLbW1PG2CeyAgR5krLF9okkqCXRmqy1o3srBh4E26WT` (200 CU on the reference call)
- `V1_TWIN_SIG` = `2BYB5oU12EfJjPTuQjaZCcxwMjWxSBUQ7WjeucW6V35Ge1PRFUda39nV4dU8NnDvt8ZfbN8H4fpAEoYpsEYysR43`
- `V2_TWIN_SIG` = `43beWNMwXpC24VqRf2uVspVDgBgKEMG75brR7LMeLeH3EcuLa9NVibJb9JAGqbe8pUDckPhtTqKRFhuzDtoZpCRs`

## When to re-land

Devnet resets wipe the programs; history pruning eats the signatures first.
Whenever `solana confirm -v <sig> --url devnet` stops resolving, run
`./redeploy.sh` and update every site it lists at the end (all inside
`lessons/m01-l1/` + one visual). Stamp the new verified date.

## Key hygiene

No keypair ever enters this repo. `redeploy.sh` works in a `mktemp -d` copy;
the deployer and counter keys are throwaways you fund, use, and delete. The
`.gitignore` additionally refuses `twins/**/target/` and `twins/**/*.json` in
case someone builds in place.
