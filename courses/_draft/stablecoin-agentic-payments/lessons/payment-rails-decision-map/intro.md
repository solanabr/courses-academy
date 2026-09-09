# Which Rail, and Is It Legal Where You Are

> **Version stamp — re-stamped 2026-09-05** (first checked 2026-07-25). `@solana/kit` — pin the major your workspace's `@solana-program/*`/client deps peer against (per-workspace rule, 2026-08-23) · `@solana/subscriptions` **0.5.0 line** (peers kit 7.1.1; 0.4.0 is a documented fallback only) · `@x402/core`, `@x402/svm`, `@x402/express`, `@x402/fetch` **2.23.0 line** · `@solana/pay` **1.0.26** (reference-only — never installed here). Subscriptions Delegation Program: `De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44`. This course is TypeScript only — no Rust, no local toolchain. Prose, tables and code below were written against the 2026-07-25 pins (kit 7.0.0, subscriptions 0.4.0, x402 2.19.0, pay 1.0.23) and are reconciled at authoring.

If you came through Course 4 you finished it with a deployed app and a published client. Either way, the app this course starts from earns nothing.

Nothing in it charges anybody. There is no subscription, no metered route, no allowance, no invoice. This module fixes exactly that, and the first decision is not a line of code — it is which payment rail your requirement actually needs, because picking the wrong one costs you a rewrite three lessons later.

## Starting here, with no Course 4 behind you

This course is a deliberate entry point. If you write TypeScript, have never touched Rust and want the payments material on its own, start here — nothing below needs a program you wrote, and every exercise in this course runs against fixtures rather than against your own deployment.

What normally arrives from Course 4 is a pair of artifacts: a **deployed app** and a **published client** for it. Here is what stands in for each:

- **The deployed app** — the frozen devnet reference vault, program `D7ZFoWvEG5NBnkJy6iC98rhwj2qhgq8xhSD42cdTRAQd`, vault PDA `FY86s1fAwUiFQTjVFYprsiV6fwNH7e955MSUBo73FP4j`. Its upgrade authority is revoked, so the address and the byte layout will not move under you. It is the thing you are putting a price on when a lesson says "your app".
- **The published client** — there is no npm package to install, and you do not need one. The IDL is committed at [`onchain-academy/reference-vault/vault_program.idl.json`](https://github.com/solanabr/superteam-academy/blob/main/onchain-academy/reference-vault/vault_program.idl.json), which is everything a generated client is built from; and the exercises in this course exercise the *payment* side — subscription authorities, delegations, x402 challenges — never the vault's own instructions. If you want the typed client anyway, generating it from that IDL is Course 4's lesson 1.

The rails, the caps, the 402 handshake and the compliance boundary are all the same either way. The one thing the fallback does not do is earn the credential: that still requires **your own** artifact, the app and client you built, not a reference someone else deployed.

There are three rails worth knowing on Solana today, and one legal boundary that decides whether a requirement is even buildable from Brazil.

## Rail 1 — the Subscriptions Delegation Program

Program id `De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44`. Audited, shared (you do not deploy it — you call it), live on mainnet since 2026-06-02. It exists because of a limitation in the SPL token model that you will hit the moment you try to build a paid tier by hand:

**A token account can have exactly one approved delegate.**

Approve your subscription contract as the delegate for the user's USDC account and you have burned the only slot. That user can now never hold an allowance for an agent, or a merchant agreement, or a second subscription, on the same mint. The Subscriptions program's answer is a program-controlled **Subscription Authority** PDA per `(user, mint)` pair: the user approves the authority once, and the authority then hands out as many independently-capped delegations as the user wants. One approval, many budgets. That is lesson 3.

On top of that authority the program offers three models:

| Model                    | Cap behaviour                                              | Use it for                                            |
| ------------------------ | ---------------------------------------------------------- | ----------------------------------------------------- |
| **Fixed delegation**     | One capped **total**. Draws down and does not refill.      | An allowance. An agent's budget. A prepaid balance.   |
| **Recurring delegation** | Cap **resets every period**. Unused periods do not stack.  | Payroll, contractor payouts, a monthly paid tier.     |
| **Subscription plan**    | Merchant publishes terms; approved pullers collect.        | A published price list many customers subscribe to.   |

You build a fixed delegation in lesson 3 and a recurring delegation in lesson 4. The fixed delegation from lesson 3 is the same object that becomes your agent's spending cap in lesson 7 — it is not a throwaway exercise.

## Rail 2 — Solana Pay

A human, a QR code, one payment, done. Point-of-sale and checkout. It carries no cap, no schedule and no delegation, because there is nothing to authorise ahead of time: the payer is standing there approving the transfer.

**You will not install it in this course, and that is a deliberate decision, not an omission.**

`@solana/kit@7.0.0` shipped on 2026-06-30. Most of the payments ecosystem has not caught up yet, and the peer ranges are worth reading rather than guessing. Off the published manifests on 2026-07-25:

| Package | Peer range on `@solana/kit` | Admits 7.0.0? |
| --- | --- | --- |
| `@x402/svm@2.19.0` | `>=5.1.0` | yes |
| `@solana/subscriptions@0.4.0` | `^6.4.0` | no |
| `@solana/pay@1.0.23` | `^6.9.0` | no |

So the honest position is not "Solana Pay is broken and the others are fine". Two of the three rails declare a kit-6 peer, and pinning `@solana/kit@7.0.0` means carrying a **documented peer-dependency override** for the Subscriptions half. We take that override because Subscriptions is load-bearing for this entire module — you cannot build a paid tier without it. We do not take a second one for a rail we only need as a *decision*, especially when `@solana/pay` also peers `@solana-program/token ^0.12.0` against `@x402/svm`'s `^0.9.0` — disjoint ranges, and peers are yours to satisfy.

Name the seam. Pin the versions. Write down which override you took and why. That habit is worth more than the rail.

Solana Pay is on your decision map as an **answer**; it is not on your `package.json`. And when the answer *is* Solana Pay, the build-it course exists in-house: **Solana Payments & Commerce** (`solana-payments-commerce`) ships the checkout and POS end to end. This lesson decides; that course builds.

Read the current reference at `solana.com/docs/payments`. Do **not** use `solana.com/developers/payments` — as of 2026-07 it still quotes USDC statistics dated 1/31/22 and a Shopify integration that has moved on.

## Rail 3 — x402

An HTTP status code turned into a payment protocol. The client calls your route, the server answers `402 Payment Required` with a machine-readable challenge naming the network, the asset and the address to pay, the client pays and retries with a payment header, the server settles and serves the response. Stateless, per request, no account relationship at all.

That property is exactly what makes it the agent rail: an autonomous caller with no login and no card can discover a price and satisfy it inside one request cycle. Module 2 is x402 end to end.

## The legal boundary — BCB Resolution 561

> **Maintenance note (2026-09-05).** The canonical, dated Brazil-compliance fact-set (Res BCB 521 art. 76-A/76-B, Res 561, Lei 14.478 art. 3º, PL 4.308 status) lives in **Solana Payments & Commerce** m06-l2. This lesson teaches the boundary as a *method* and consumes that shared fact-set — it must never fork its own copy. One review ritual updates both courses in one PR whenever the BCB or the bill moves.

If you are building from Brazil, this is the constraint no English-language Solana course is going to tell you about.

**Banco Central do Brasil Resolution 561** was published 2026-04-30 and takes effect **2026-10-01**. Its practical rule for our purposes:

> A regulated eFX provider may **not** take reais, convert them to USDT / USDC / BTC, and settle the resulting obligation abroad on-chain.

That closes a specific pattern — using stablecoins as the cross-border settlement leg of a foreign-exchange operation — for regulated operators. It does **not** close crypto in Brazil. Individuals may still buy, sell, hold and transfer through authorised VASPs.

What stays open, and what this whole course is scoped to:

- **Domestic merchant acceptance** — a Brazilian business charging a Brazilian customer.
- **Contractor payouts.**
- **Subscriptions** — the paid tier you build in this module.
- **Agent budgets** — the allowance you cap in module 2.

Every example in this course sits inside that box. If a requirement's settlement leg is a regulated cross-border FX conversion, the correct engineering answer is **not a rail** — it is "this is out of scope for the entity as regulated," and you route it to compliance before you route it to a program id.

**Which is why the compliance check runs first.** A decision function that reaches "recurring delegation" and only afterwards asks "wait, is this allowed?" has already told a product manager to build the wrong thing. Compliance is a gate, not a fallback branch.

## The worked example

Below is the full decision map, annotated. Read it, then make exactly one change.

The function takes a requirement described by three facts and returns a rail name:

| Parameter                 | Values                                                          |
| ------------------------- | --------------------------------------------------------------- |
| `cadence`                 | `one-off` · `per-request` · `periodic` · `capped-total`          |
| `settlement`              | `domestic-br` · `cross-border-fx` · `n-a`                        |
| `merchantPublishesTerms`  | `true` when the merchant publishes a plan customers subscribe to |

and the rail names it can return:

- `blocked-by-bcb-561` — the compliance stop
- `solana-pay` — human, one-off, standing there
- `x402` — priced per request
- `subscription-plan` — published terms, approved pullers collect
- `recurring-delegation` — cap resets per period
- `fixed-delegation` — one capped total, drawn down
- `unsupported` — a cadence the map does not model; fail loudly rather than guess

The starter is complete and correct except for one thing: **the compliance guard is at the bottom instead of the top.** Every requirement with `settlement === "cross-border-fx"` and a cadence the map understands will be answered with a rail before the guard is ever reached. Move the guard so it runs before any rail is chosen. That is the whole edit.
