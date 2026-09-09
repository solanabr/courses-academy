// The billing crank's per-subscription pull guard.
//
// Before the club-billing crank issues a delegate-signed TransferChecked for a
// subscriber, it must decide whether THIS subscription is due to be charged
// right now. Two gotchas the Subscriptions program forces on you live here:
//
//   1. Plans publish their cadence as `periodHours`, but a billing window is
//      measured in SECONDS of chain time. Comparing hours against a Unix
//      timestamp calls everyone due ~3600x too often -- pulls the program
//      refuses, at a base fee per refusal. Convert before you compare.
//   2. Delegation/subscription accounts PERSIST until revoked, so existence
//      proves nothing. The program refuses a lapsed pull on-chain; your job is
//      to mirror its `expiresAtTs` bound (0 means "no expiry") so the refusal
//      happens here for free instead of on-chain at a base fee per tick.
//
// The grader calls decidePull positionally, one scalar per argument, in the
// order declared below: active, expiresAtTs, lastChargedTs, periodHours, now.
// It grades bare declarations, so no `export` keyword here; the lab file you
// save as decide-pull.ts adds `export` to both so 04-pull.ts and the gate
// can import them.
//
// TODO: implement decidePull so it honors active state, expiry, and the
// seconds-based period window. The starter below forgets the ×3600 conversion
// AND never checks expiry: the exact unit-and-clock bugs above.

interface PullDecision {
  shouldPull: boolean;
  reason: string; // "due" when pulling, else why it was held
  nextEligibleTs: number; // earliest Unix second a pull may fire (0 if N/A)
}

function decidePull(
  active: boolean, // false once CancelSubscription has run
  expiresAtTs: number, // Unix seconds; 0 = never expires
  lastChargedTs: number, // Unix seconds of the previous successful pull
  periodHours: number, // plan cadence, in HOURS (as published on the plan)
  now: number, // current chain time, Unix seconds
): PullDecision {
  if (!active) {
    return { shouldPull: false, reason: "canceled", nextEligibleTs: 0 };
  }

  // BUG: periodHours is treated as seconds, and expiry is never checked.
  const periodS = periodHours;
  const nextEligibleTs = lastChargedTs + periodS;

  if (now < nextEligibleTs) {
    return { shouldPull: false, reason: "too-early", nextEligibleTs };
  }

  return { shouldPull: true, reason: "due", nextEligibleTs };
}
