# One Approval, Many Budgets

> **Version stamp — re-stamped 2026-09-05** (first checked 2026-07-25). `@solana/subscriptions` **0.5.0 line** (peers kit 7.1.1; verified installable) · `@solana/kit` — per-workspace peer rule (2026-08-23) · `@solana/kit-plugin-rpc` · `@solana/kit-plugin-signer` · `@solana-program/token` (pin the exact version you import, at authoring). Program: `De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44`. Code below was written against 0.4.0 and is reconciled at authoring.
>
> **Use the 0.5.0 line; `0.4.0` is a documented fallback only.** This package is pre-1.0 and moving — `0.1.0` shipped 2026-05-15, `0.4.0` on 2026-07-13, and `0.5.0` since — and its dist-tags have pointed backwards before (`beta` sat on `0.4.0-rc.2`, an *older* build than `latest`, so a dist-tag install could move you backwards). Pin an exact version from the line you verified, and re-read the changelog before you bump.

## Start with the constraint, not the API

Try to build a paid tier by hand and you will reach for `approve` on the user's token account. It works. Then you build the second thing that needs to pull funds, and it does not.

**An SPL token account holds exactly one approved delegate.** Not one per counterparty — one, total. Approve your subscription service and you have consumed it. That user can no longer hold an agent allowance, a merchant agreement, and your monthly charge at the same time on the same mint, because every new `approve` silently replaces the last one. The failure mode is not an error; it is somebody else's charge quietly ceasing to work.

The Subscriptions Delegation Program's answer is one level of indirection. The user approves **one** delegate: a program-controlled **SubscriptionAuthority** PDA, derived per `(user, tokenMint)` pair. That authority then issues as many independently-capped delegations as the user wants, each with its own delegatee, its own cap, its own expiry, and its own revoke.

One approval. Many budgets. That is the whole idea, and every step below is a consequence of it.

## The client

```ts
import { address, createClient } from "@solana/kit";
import { solanaDevnetRpc } from "@solana/kit-plugin-rpc";
import { signer } from "@solana/kit-plugin-signer";
import { subscriptionsProgram } from "@solana/subscriptions";

const client = createClient()
  .use(signer(userSigner))
  .use(solanaDevnetRpc({ rpcUrl: DEVNET_RPC_URL }))
  .use(subscriptionsProgram());
```

The plugin fills the configured signer and payer where it can, derives program PDAs for you, and can send transactions directly. Everything that follows hangs off `client.subscriptions`.

## The six steps

### 1. Derive the user's ATA

```ts
const [userAta] = await findAssociatedTokenPda({ mint, owner: user, tokenProgram });
```

`[address, bump]` tuple — destructure it. `tokenProgram` is a seed (lesson 2), so pass the one that actually owns the mint.

### 2. Derive the SubscriptionAuthority PDA

```ts
const [subscriptionAuthority] = await findSubscriptionAuthorityPda({ tokenMint, user });
```

One per `(user, tokenMint)`. Not per delegatee, not per product.

### 3. Read it before you touch it

```ts
const authority = await fetchMaybeSubscriptionAuthority(rpc, subscriptionAuthority);
if (!authority.exists) {
  // ...step 4
}
```

**Branch on `.exists`.** This is the single most common error against this program: calling `initSubscriptionAuthority` unconditionally, so a returning user's second visit fails at step 4 and the whole flow dies before it reaches the delegation. `fetchMaybe*` is Kit's "this may not be there" fetch — it returns an object with an `exists` discriminant instead of throwing, and ignoring that discriminant is how you turn a normal state into a crash.

The plugin also exposes `client.subscriptions.queries.isSubscriptionAuthorityInitialized(user, tokenMint)` if all you need is the boolean.

### 4. Initialise — only when it is not there

```ts
await client.subscriptions.instructions
  .initSubscriptionAuthority({ tokenMint, userAta, tokenProgram })
  .sendTransaction();
```

`owner` and `payer` are filled from the configured signer, so the visible input is just the mint, the ATA and the token program. This is the transaction where the user's single delegate slot gets consumed — by the authority, once, on purpose.

### 5. Open the Fixed Delegation

```ts
await client.subscriptions.instructions
  .createFixedDelegation({
    tokenMint,
    delegatee: agentAddress,
    nonce: 0n,
    amount: 25_000_000n,       // 25 USDC — BASE UNITS, as a bigint
    expiryTs: BigInt(Math.floor(Date.now() / 1000) + 30 * 24 * 3600),
  })
  .sendTransaction();
```

Four things in that call will bite you:

- **`amount` is base units, as a `bigint`.** `25` is twenty-five *millionths* of a USDC, not twenty-five USDC. There is no decimals conversion anywhere in this API. Do it once, in one function, and never inline it.
- **`nonce` is what lets one delegator hold several delegations to the same delegatee.** It is part of the delegation PDA's seeds. Reuse a nonce with the same delegatee and you get `DELEGATION_ALREADY_EXISTS`; the account is already there.
- **`expiryTs` is a hard stop with no grace period.** One second past it, transfers fail with `DELEGATION_EXPIRED`. Nothing renews, nothing warns, nothing degrades gracefully. If your product needs a renewal, your product builds the renewal.
- **`payer` is optional, and it is how sponsorship works.** Pass a different signer and that account funds the rent instead of the user. The program stores who paid, and the teardown calls take an optional `receiver` to send it back to: on `revokeSubscriptionAuthority` the SDK documents `receiver` as *required when the authority's stored payer differs from `user`, and it must equal that stored payer*. Sponsorship is therefore a two-sided record — who paid on the way in decides who may be paid on the way out. Track it in your own database as well, because the failure surfaces months later.

A fixed delegation is **one capped total that draws down and never refills**. Spend 25 USDC against a 25 USDC cap and it is finished — not "finished for this month". That is precisely the property that makes it a safe budget for something autonomous, which is why **this exact object becomes your agent's spending cap in lesson 7.** Do not think of it as an exercise.

### 6. Pull from it

```ts
await client.subscriptions.instructions
  .transferFixed({
    delegationPda,
    delegator: userAddress,      // an Address — the user does NOT sign here
    delegatorAta: userAta,
    receiverAta: merchantAta,
    tokenMint,
    tokenProgram,
    amount: 5_000_000n,
  })
  .sendTransaction();
```

Look at the types: `delegator` is an `Address`, `delegatee` is a `TransactionSigner`. **The user signs the setup and the revoke; the delegatee signs the transfers.** The user is not present when you charge them — that is the entire point of a delegation, and it is the security model of this module. Lesson 4 makes it a quiz question.

Overrun the cap and you get `AMOUNT_EXCEEDS_LIMIT` — "Transfer amount exceeds delegation limit".

## Your task

Write the planner. Given whether the authority already exists, a human-readable USDC amount, and whether a sponsor is paying, produce:

- `steps` — the SDK operations in the order you would call them, using their exact names
- `amountBaseUnits` — the amount converted to base units as a **`bigint`** (USDC has 6 decimals)
- `payer` — `"user"` or `"sponsor"`

The starter has six numbered subgoals. The first two are filled in. Step 3 is always in the plan — you read the authority whether or not it exists, because that is how you found out.
