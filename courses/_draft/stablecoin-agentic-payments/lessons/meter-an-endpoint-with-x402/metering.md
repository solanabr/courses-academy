# 402 Payment Required, For Real

> **Version stamp — re-stamped 2026-09-05** (first checked 2026-07-25). `@x402/core`, `@x402/svm`, `@x402/express`, `@x402/fetch` — **2.23.0 line** (verified installable). `@solana/kit` — per-workspace peer rule (2026-08-23). Everything below was written against 2.19.0 / kit 7.0.0 and is reconciled at authoring. If you are reading this more than a few months later, re-check the pins before you copy anything.

In module 1 you gave your vault app a paid tier: a Subscription Authority, a Recurring Delegation, one collected charge, and a second charge that correctly refused. That model prices a **relationship** — a subscriber, a period, a cap.

This lesson prices a **request**. One route, one price, one payment, no account, no relationship. That is what x402 is for: an HTTP status code that has been sitting unused in RFC 7231 since 1997, wired to a stablecoin transfer.

The flow has three moves and no new infrastructure:

1. A client calls your route with no payment. You answer **402** with a machine-readable challenge describing what you will accept.
2. The client builds a payment matching one of your accepted terms and calls again, carrying the payment in a header.
3. You hand that payload to a **facilitator**, which verifies and settles it on-chain, and then you serve the response.

---

## The package split — read this before you install anything

This is the single worst staleness trap in this catalog, because search engines surface the wrong half and so does one of Solana's own templates.

**Frozen. Do not install. Do not teach.**

`x402` · `x402-express` · `x402-next` · `x402-fetch` · `x402-hono` — all **1.2.0**, published **2026-04-16**. These speak protocol **v1**. solana.com's own "X402 Next.js" scaffold still ships `x402-next`, so "it came from an official template" is not evidence of currency.

**Current. Install these.**

| Package | Version | What you use from it |
| --- | --- | --- |
| `@x402/core` | 2.19.0 | `x402ResourceServer`, `x402HTTPResourceServer`, `x402Client`, `HTTPFacilitatorClient`, `VerifyError`, `SettleError` |
| `@x402/svm` | 2.19.0 | `ExactSvmScheme`, `ExactSvmSchemeV1`, `SOLANA_DEVNET_CAIP2`, `USDC_DEVNET_ADDRESS`, `TOKEN_PROGRAM_ADDRESS`, `TOKEN_2022_PROGRAM_ADDRESS` |
| `@x402/express` | 2.19.0 | `paymentMiddleware` |
| `@x402/fetch` | 2.19.0 | `wrapFetchWithPayment` (that is lesson 6) |

The unscoped packages are still published and still installable. Nothing stops you from typing the wrong one. Read the `@` before you press enter.

`@x402/svm` peer-depends on `@solana/kit >=5.1.0`. That is the **same Kit you already have** from the client you published in course 4 — the metering layer is not a second SDK, it is a scheme plugin that borrows yours.

---

## The challenge envelope

Everything the client needs is in the 402. Here is the worked example you will run in a moment, in full:

```json
{
  "x402Version": 2,
  "error": "payment required",
  "accepts": [
    {
      "scheme": "exact",
      "network": "solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1",
      "asset": "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU",
      "payTo": "<your devnet wallet>",
      "maxAmountRequired": "10000",
      "resource": "https://your-app.vercel.app/api/quote",
      "description": "One FX quote",
      "mimeType": "application/json",
      "maxTimeoutSeconds": 60
    }
  ]
}
```

Field by field, and each of these is a place people get it wrong:

- **`x402Version`** — the wire version, `1` or `2`. Both are live in the world right now. You emit `2`.
- **`scheme`** — `"exact"`. The client pays exactly `maxAmountRequired`, no more, no quoting, no conversion.
- **`network`** — a CAIP-2 chain id: the namespace `solana:` plus the first 32 characters of the cluster's genesis hash. Devnet is `solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1`; mainnet is `solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp`. **Import `SOLANA_DEVNET_CAIP2` from `@x402/svm` rather than typing that literal into your server** — a one-character typo produces a challenge no client can match, and the failure looks like "the client just never pays".
- **`asset`** — the mint address, on that network. Devnet USDC is `4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU` (`USDC_DEVNET_ADDRESS`). `network` and `asset` are a **pair**. A mint is an account on one cluster; the mainnet USDC mint does not exist on devnet.
- **`payTo`** — the address that receives the tokens. This is the whole point of the exercise and it is the one value you are going to fill in.
- **`maxAmountRequired`** — **base units, as a decimal string.** 0.01 USDC at 6 decimals is `"10000"`. Not `0.01`. Not the number `10000`. A string, because base-unit amounts are `u64` and JSON numbers stop being exact above 2^53 — and because the moment a float touches a payment field you have signed up for rounding bugs you will find in production, not in tests.
- **`resource`** — the absolute URL being priced. It binds the payment to this route.
- **`maxTimeoutSeconds`** — how long you will hold the door open between issuing the challenge and settling the payment.

`accepts` is an **array** because it is a menu, not a demand. A real server offers several terms — devnet and mainnet, or several assets — and lets the client pick one it can satisfy. In lesson 6 you are on the other side of that menu, doing the picking.

### Where the envelope travels

In the common case it is the JSON response body of the 402. But some servers return an **empty body** and put the whole base64-encoded envelope in a `PAYMENT-REQUIRED` response header — Vercel-hosted ones do this, because of how their edge layer handles bodies on error statuses. A correct server emits both. A correct client reads both. You will build the client-side fallback next lesson; today you emit both so you are never the reason a client fails.

That is why the exercise base64-encodes the envelope as well as returning it: `header` and `body` carry the same bytes.

---

## The facilitator

You do not run an RPC node, you do not hold a hot key for verification, and you do not submit transactions. A **facilitator** exposes three operations:

- **`supported`** — which `(network, asset, scheme)` combinations it can service. Use it to build your `accepts[]` instead of guessing.
- **`verify`** — is this payment payload well-formed, correctly signed, and does it satisfy the terms you advertised? Throws `VerifyError` when not.
- **`settle`** — submit it and confirm it. Throws `SettleError` when not.

Wire one up with `HTTPFacilitatorClient` from `@x402/core` and hand it to `x402ResourceServer`. The illustrative shape, which you run in your own app rather than in the grader:

```ts
import { x402ResourceServer, HTTPFacilitatorClient } from "@x402/core"; // 2.19.0
import { ExactSvmScheme, SOLANA_DEVNET_CAIP2, USDC_DEVNET_ADDRESS } from "@x402/svm"; // 2.19.0
import { paymentMiddleware } from "@x402/express"; // 2.19.0

const facilitator = new HTTPFacilitatorClient({ url: process.env.FACILITATOR_URL! });

const server = x402ResourceServer({
  facilitator,
  schemes: [ExactSvmScheme],
});

app.use(
  paymentMiddleware(server, {
    "GET /api/quote": {
      network: SOLANA_DEVNET_CAIP2,
      asset: USDC_DEVNET_ADDRESS,
      payTo: process.env.MERCHANT_ADDRESS!,
      maxAmountRequired: "10000",
      description: "One FX quote",
    },
  })
);
```

The middleware is doing exactly what your exercise does by hand — building that `accepts[]` entry, returning 402 when no payment arrives, and calling `verify`/`settle` when one does. Write it once by hand so the middleware is never a black box.

**On Kora:** you may run your own facilitator, and Kora is the Solana-native one. It requires cloning the repo and building it locally across several terminals. That is a local-toolchain detour this course does not take. Use a hosted or embedded facilitator on devnet; come back to self-hosting when you have revenue to protect.

---

## The exercise

You are handed a complete, annotated, working challenge builder. Read it top to bottom — it is the reference implementation, and it is short on purpose.

**Change exactly one value: `PAY_TO`.** Put a real base58 devnet address there — the wallet you funded in course 1 is the right one, because in a moment you want to watch USDC land in it. Nothing else in the file needs to move.

The tests check the things that break real integrations: the amount is a decimal string of base units, the network and asset are the devnet pair, the encoded header is genuine base64 of the body, and `payTo` is an address rather than a placeholder.

## Milestone

Take the same envelope shape into your course-4 app, put it in front of one real route, and deploy it. **Keep the URL.** Lesson 6 pays it, lesson 7 has an agent pay it repeatedly out of a capped budget, and lesson 8 asks you for the link.
