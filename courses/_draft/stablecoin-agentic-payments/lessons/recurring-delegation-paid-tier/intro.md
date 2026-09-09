# Charge Every Month, Cap Every Month

> **Version stamp — re-stamped 2026-09-05** (first checked 2026-07-25). `@solana/subscriptions` **0.5.0 line** (0.4.0 documented fallback only) · `@solana/kit` — per-workspace peer rule (2026-08-23). Program: `De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44`. Code below was written against 0.4.0 and is reconciled at authoring.

The fixed delegation you opened in lesson 3 is a budget: one total, drawn down, gone. A paid tier is a different shape. You want to charge 25 USDC this month, 25 USDC next month, and never more than 25 USDC in any month no matter how many times your billing job runs.

That is a **recurring delegation**, and its three semantics are the whole lesson.

## Opening it

```ts
await client.subscriptions.instructions
  .createRecurringDelegation({
    tokenMint,
    delegatee: merchantAddress,
    nonce: 1n,
    amountPerPeriod: 25_000_000n,        // base units, as a bigint
    periodLengthS: BigInt(30 * 24 * 3600),
    startTs: 0n,                          // start when the transaction lands
    expiryTs: BigInt(Math.floor(Date.now() / 1000) + 365 * 24 * 3600),
  })
  .sendTransaction();
```

Same authority, same `(user, tokenMint)` pair, different **nonce** — which is exactly the "one approval, many budgets" property doing its job. The lesson-3 fixed delegation is still open and untouched.

`startTs: 0n` means "the first period begins when this lands". That convenience has a rule attached: **start-on-landing requires a non-zero `expiryTs`.** Pass both as zero and the program refuses with *"start_ts of 0 (start on landing) requires a non-zero expiry"* — an open-ended pull authorisation with no defined start is not something it will sign off on.

## Collecting

```ts
await client.subscriptions.instructions
  .transferRecurring({
    delegationPda,
    delegator: userAddress,     // Address — the user is not here
    delegatorAta: userAta,
    receiverAta: merchantAta,
    tokenMint,
    tokenProgram,
    amount: 25_000_000n,
  })
  .sendTransaction();
```

`transferRecurring` takes the identical input type as `transferFixed`. Same signing model, and it is worth saying again because it is the security point of this whole module:

> **The user signs the setup and the revoke. The delegatee signs the transfers.**

`delegator` is an `Address`; `delegatee` is a `TransactionSigner`. Your billing job holds the delegatee key. It cannot exceed the cap, it cannot extend the expiry, and it cannot stop the user revoking. Those three facts are what let you run it unattended.

## The three semantics that must stick

### 1. A second pull inside the same period fails

Charge 25 USDC on the 3rd. Charge 1 more USDC on the 4th, same period. The program returns `AMOUNT_EXCEEDS_PERIOD_LIMIT` — *"Transfer amount exceeds period limit"*.

This is a feature, and it is the one your retry logic depends on. A billing job that crashes after the transfer landed but before it wrote its own record will retry. The retry is refused. The cap **is** your idempotency guard, as long as you read the error and treat it as "already collected" rather than "temporary failure, back off and try again in five minutes" for the rest of the month.

Note the neighbouring error you will also meet: `PERIOD_NOT_ELAPSED` — *"Period has not elapsed yet"* — which is what you get when you try to collect for a period that has not started.

### 2. The cap resets on period rollover

Once the clock crosses into the next period, the full `amountPerPeriod` is available again. Nothing is carried, nothing is prorated, and you do not call anything to make it happen. There is no "renew" instruction because there is nothing to renew.

### 3. Skipped periods do not accumulate

This is the one that surprises people, and it is the one the exercise grades hardest.

A merchant who forgets to collect in March and April does not get to collect 75 USDC in May. They get 25 USDC in May. **The unused capacity is gone.** The cap is a per-period ceiling, not an accruing balance.

Read it from the user's side and it is obviously correct: they authorised "up to 25 per month", not "up to 25 per month, banked indefinitely, redeemable in a lump sum by whoever holds the delegatee key". The second thing would be a much larger authorisation wearing the first one's clothes.

If your business needs missed periods to be collectable, that is an invoice, and you build it in your own system. The delegation will not do it for you.

## Revoking

```ts
await client.subscriptions.instructions
  .revokeDelegation({ delegationAccount: delegationPda })
  .sendTransaction();
```

The `authority` — the user — signs, and is filled from the configured signer. This **closes the PDA** and returns its rent. Pass `receiver` when somebody other than the user funded it.

Revocation is permanent for that delegation. There is no pause. Re-establishing the arrangement means a new `createRecurringDelegation` with a new nonce, signed by the user, which means the user is present and consenting again. That is the correct trade.

## The exercise — put the pieces in order

The starter contains the body of the period-accounting function, but scrambled: **three candidate lines for the available amount, of which one is right, and two return statements in the wrong order.** Only one candidate line is active.

Two decoys, both of which are real mistakes people ship:

- **Decoy A — the cap that never resets.** `periodLimit - chargedInCurrentPeriod`, unconditionally. It ignores the rollover entirely, so the delegation works for exactly one period and then quietly refuses everything forever. This is the same class of error as revoking before collecting: an operation that looks correct in the happy path and destroys the arrangement on the second cycle.
- **Decoy B — the accumulating cap.** `periodLimit * (currentPeriod - lastChargedPeriod + 1) - chargedInCurrentPeriod`. This one is worse, because it is *nearly* right: it agrees with the correct answer on every test where no period was skipped. It only diverges when a merchant comes back after missing a few — which is exactly the case where it lets them take money the user never authorised. This is the fixed-delegation model smuggled into a recurring PDA.

The ordering error matters too: if the success return runs before the over-cap guard, the guard is unreachable and every request is approved.

Amounts here are plain integer base units so the arithmetic is legible. In your real client they are `bigint`, converted once, exactly as in lesson 3.

## Milestone — write these down

Before you move to module 2, do this on devnet and keep the output:

1. The **recurring delegation PDA** you created.
2. The **settlement signature** of the charge that succeeded.
3. The **error** you got when you collected a second time inside the same period.

Lesson 8 asks you to publish a deep dive with a reproducible run, and these three artifacts are the run. A screenshot of the second attempt failing with `AMOUNT_EXCEEDS_PERIOD_LIMIT` is worth more than a paragraph claiming it does.

## Optional: where a revenue dashboard would come from

The program emits its state changes as Anchor-compatible **self-CPI inner instructions** through an event-authority PDA — `findEventAuthorityPda`, seed `"event_authority"` — and validates them on the way out with `INVALID_EVENT_AUTHORITY`, `INVALID_EVENT_TAG` and `INVALID_EVENT_DISCRIMINATOR`. An indexer that watches inner instructions for that authority sees every delegation created, every transfer settled and every revoke, per user, in order.

That is how you would build merchant revenue reporting. **Do not build it here.** It is a different course, and this one has an agent to fund.
