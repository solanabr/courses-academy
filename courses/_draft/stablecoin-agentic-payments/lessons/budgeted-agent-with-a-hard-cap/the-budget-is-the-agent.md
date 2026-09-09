# An Agent With a Wallet That Can Say No

> **Version stamp — re-stamped 2026-09-05** (first checked 2026-07-25). `@solana/subscriptions` **0.5.0 line** (0.4.0 documented fallback only; pre-1.0 and moving). `@solana/kit` — per-workspace peer rule (2026-08-23). `@x402/core`, `@x402/svm`, `@x402/fetch` **2.23.0 line**. Code below was written against the 2026-07-25 pins and is reconciled at authoring.

Here is the scope fence, up front, so you know what this lesson is not.

**No agent framework.** No SendAI Solana Agent Kit, no ElizaOS, no LangChain, no Vercel AI SDK. The "agent" in this lesson is a `for` loop. Every one of those frameworks will be a different set of names in eighteen months, and none of them solves the problem that actually matters here.

The problem that matters is this: **an autonomous process needs money, and giving a process money is giving it the ability to lose all of it.** The standard answer is to fund a hot keypair and hope. That is not a budget. That is a blast radius.

You already built the alternative. In lesson 3 you opened a **Fixed Delegation** — a program-enforced allowance with a capped total, drawn down by `transferFixed`. That is the agent's wallet. Not a keypair holding funds: an authorisation, on the user's account, that the program refuses to exceed.

The difference is where the limit lives. A limit in your code is a limit an attacker edits out. A limit in a program-owned account is a limit that survives your code being wrong, your key being stolen, and your loop running a hundred times more than you meant it to. That is why Allowances are documented as the budget primitive for agents, and it is the whole reason this course ends here rather than with a model integration.

One layer is not the whole answer, though. **Solana Payments & Commerce** (`solana-payments-commerce`) teaches the other half of this same question: a **client-side spend control** (`spendControls`, with its $1 default and its silent-decline footgun) enforced in the paying code before a transaction is even built. These are complementary layers, not competing answers — the client-side cap is the cheap, adjustable first guard; the on-chain Fixed Delegation is the one that survives your code being wrong. Use both, and know which one refused you when a payment stops.

---

## The loop

```
budget = the Fixed Delegation from lesson 3   (remaining_amount, base units, u64)

for each task step, up to maxCalls:
    if remaining < price:  HALT — budget exhausted
    call the metered endpoint from lesson 5, through the wrapped fetch from lesson 6
    settle the 402 by drawing `price` from the delegation with transferFixed
    if the program refuses:  HALT — refused, record the code
    remaining -= price
report: calls, spent, remaining, signatures, why it stopped
```

Three exits. Only one of them is success. **The exit conditions are the learning objective, not error handling bolted on at the end** — which is why today's exercise grades the decision logic and not the happy path.

### Exit 1 — budget exhausted (you decide)

Before every call, compare `remaining` against the price. If you cannot afford it, stop. Do not build the transaction and let the chain tell you.

A transaction that reaches execution and fails still pays a fee. Sending one you already know will be refused buys you nothing but a worse report: you learn "the program refused" when you could have known "the budget ran out" — different facts, different alerts, different fixes.

### Exit 2 — refused (the program decides)

A refusal from the delegation program is **terminal**. Do not retry it.

Backoff exists for transient failures — congestion, an expired blockhash, a flaky RPC. A policy refusal is not transient. The allowance is state on-chain; waiting does not change it. Retrying asks the same program the same question and gets the same answer, one fee poorer, three times over.

Record the code the program returned, **verbatim**, and stop. Do not pattern-match on the string to decide whether to continue — your loop halts on any refusal, and the code is diagnostic output for a human, not a control-flow input. The one code worth recognising on sight is `AMOUNT_EXCEEDS_PERIOD_LIMIT` from lesson 4, and even that one is a halt: it means a recurring cap has not rolled over yet.

### Exit 3 — completed

The task finished inside its budget. `halted: false`. This is the boring one, and it should be.

---

## Base units are u64

`remaining_amount` and every price in this system is a `u64` count of base units. `Number.MAX_SAFE_INTEGER` is 2^53 − 1. A u64 goes to 2^64 − 1.

Convert once with `BigInt(...)`, do the arithmetic in `BigInt`, return decimal strings. One of the scenarios in the exercise spends a u64-sized allowance to exactly zero — it comes out right in BigInt and comes out wrong, silently, in `Number`. Silently is the part that should worry you: there is no exception, just a balance that is off by a bit you will find in production.

---

## What the exercise grades

`runAgentLoop(scenarioId)` gets an allowance, a price, a call budget and a list of settlement responses, and returns a report. The network round trips are captured; what you write is the spend / halt / report logic.

Two of the scenarios are traps, and both are traps that real loops fall into:

- **`s-exhausted`** supplies five successful settlements for a task the allowance can only fund twice. A loop that settles first and checks the budget afterwards reports three calls and a negative balance.
- **`s-refused`** puts a *successful* settlement immediately after the refusal. A loop that logs the refusal and continues will find it, count it, and report a spend that never happened.

Neither trap produces an exception. Both produce a confident, wrong report. That is what makes them worth grading.

---

## Where this goes next

`@x402/mcp` (2.19.0) puts the same payment loop behind the Model Context Protocol, so a tool an assistant calls can charge for itself and the assistant pays out of exactly this kind of capped allowance. Same primitive, one layer up. Worth an afternoon after this course.

**A note on the bounty market, since you will be tempted.** Agent bounties on Superteam Earn are the worst odds on the platform — roughly $53 per competitor, with a median around 91 submissions. Payments listings and technical deep dives pay far better per unit of effort. What you learned here is the **budget primitive**, which is a component of things people actually pay for. Lesson 8 points you at the listings where that is true.

## Milestone

You need three things from this module for lesson 8, so collect them now:

1. **The URL of your metered endpoint** — the live one from lesson 5, running in your course-4 app.
2. **The settlement signatures** your agent produced — at least one, from a real devnet draw against your Fixed Delegation.
3. **The delegation address**, and the `remaining_amount` before and after.

That set is the proof. Anyone can describe an agent with a budget; you can show the transactions where one hit its cap and stopped.
