# Reading a Challenge and Paying It

> **Version stamp — re-stamped 2026-09-05** (first checked 2026-07-25). `@x402/core`, `@x402/svm`, `@x402/fetch` — **2.23.0 line**. `@solana/kit` — per-workspace peer rule (2026-08-23). The frozen v1 line (`x402`, `x402-fetch`, 1.2.0, 2026-04-16) is still on npm and still installable — do not. Code below was written against 2.19.0 and is reconciled at authoring.

Last lesson you were the server. Now you are the caller, and the caller has the harder job: the server publishes one shape and knows what it published. The client has to handle whatever it is handed.

Two things make that hard in practice, and both of them are in today's exercise:

1. **The envelope is not always where the docs say it is.** Some servers return an empty body and put the whole base64 envelope in a response header.
2. **Two protocol versions are live at the same time.** They use different header names and different envelope fields. Every tutorial that picks one and hardcodes it will rot; this lesson teaches you to read the version off the wire.

---

## The loop

```
fetch(url)                       → 402
locate the envelope              → body, or the PAYMENT-REQUIRED header
decode it                        → JSON (base64-decode first if it came from the header)
read envelope.x402Version        → decides the retry header name
select one accepts[] entry       → filter on scheme + network + asset
sign a payment for those terms
fetch(url, { headers: { <name>: <payload> } })  → 200
```

In production `wrapFetchWithPayment` from `@x402/fetch` does all of that:

```ts
import { wrapFetchWithPayment } from "@x402/fetch"; // 2.19.0
import { x402Client } from "@x402/core"; // 2.19.0
import { ExactSvmScheme, ExactSvmSchemeV1 } from "@x402/svm"; // 2.19.0

const client = x402Client({
  signer: agentSigner,             // your @solana/kit@7 signer
  schemes: [ExactSvmScheme, ExactSvmSchemeV1],
});

const paidFetch = wrapFetchWithPayment(fetch, client);
const res = await paidFetch("https://api.example.dev/quote");
```

Note the `schemes` array. You register **both** — and this is the point of the lesson.

---

## The version seam

`@x402/svm@2.19.0` exports `ExactSvmScheme` **and** `ExactSvmSchemeV1`. That is not legacy cruft kept around for types. It is there because **both wire versions are live right now**:

| | v1 | v2 |
| --- | --- | --- |
| Emitted by | `x402`, `x402-express`, `x402-next`, `x402-fetch`, `x402-hono` @ 1.2.0 | `@x402/*` @ 2.19.0 |
| Envelope field | `"x402Version": 1` | `"x402Version": 2` |
| Payment request header | `X-PAYMENT` | `PAYMENT` |
| Settlement response header | `X-PAYMENT-RESPONSE` | `PAYMENT-RESPONSE` |

A client that registers only the v2 scheme fails silently against every server still on the frozen package line — and since solana.com's own Next.js x402 template ships `x402-next`, there are more of those than you expect.

**Never assume the version. Read `x402Version` out of the envelope you were handed, then build the retry to match.** That single habit is the difference between a client that works for a year and one that works until the first server you did not test against.

---

## The menu

`accepts[]` is a list of terms the server will take. It is ordered by the server's preference, which has nothing to do with your capability. In today's fixtures the **first** entry is always an EVM entry — that is deliberate, and it is the shape of the real bug:

```ts
const terms = envelope.accepts[0];   // ← you are now trying to pay on Base
```

Filter instead, on all three of the fields that have to line up:

```ts
const terms = envelope.accepts.find(
  (entry) =>
    entry.scheme === "exact" &&
    entry.network === SOLANA_DEVNET_CAIP2 &&
    entry.asset === USDC_DEVNET_ADDRESS
);
```

`network` and `asset` are a pair. The `v2-header` fixture offers Solana **mainnet** USDC and Solana **devnet** USDC as separate entries, with the same scheme and the same price. Matching only on network, or only on asset, picks the wrong one.

And "no entry matches" is a normal outcome, not an error. Servers that only accept EVM money exist. Report it and move on — your caller needs to know it was not paid, not receive an exception.

---

## The exercise

`handle402(captureId)` receives one of four captured HTTP responses and returns a report saying what it found and what it would pay.

Inside the function there are **eight commented-out blocks**. Six of them are the ones you need, in the wrong order. **Two are wrong and must be deleted.** The block numbers are labels, not positions.

Here is what the two wrong blocks get wrong. This is the whole reason they are in the file:

- **One retries before decoding.** It calls `JSON.parse(res.body)` and takes `accepts[0]`, hardcoding version 2 and hardcoding "the envelope is in the body". It produces a plausible-looking result on exactly one of the four fixtures and is wrong on the rest — which is precisely how this bug survives review. Decoding is not a formality you can skip; it is where you learn which protocol you are speaking.
- **One selects on the wrong chain namespace.** It filters for `eip155:` entries. Those are real, valid, payable terms — by somebody with an EVM wallet. The lesson is that "found a match" is not the same as "found a match I can satisfy".

Ordering is not guesswork: every block after the first uses a local that an earlier block declares. Follow the dependency chain — `res` → `headerEnvelope`/`fromHeader` → `raw`/`envelope` → `paymentHeader`/`envelopeFrom` → `selected`.

The base64 decoder is given to you at the bottom of the file. It is not part of the puzzle; `atob()` and `Buffer` simply do not exist in the grader.

## What this unlocks

You now have both halves: a route that charges, and a client that pays. Next lesson the caller stops being you and becomes a loop — and the interesting question stops being "can it pay?" and becomes "what stops it?"
