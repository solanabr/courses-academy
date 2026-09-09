// The billing crank's per-subscription pull guard (reference solution).

// Bare declarations here: the grader executes the widget copy without module
// syntax; the lab's decide-pull.ts carries `export` on both declarations.
interface PullDecision {
  shouldPull: boolean;
  reason: string; // "due" when pulling, else why it was held
  nextEligibleTs: number; // earliest Unix second a pull may fire (0 if N/A)
}

const SECONDS_PER_HOUR = 3600;

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

  // Expiry wins over the period window: a lapsed sub is never charged, even if
  // its window is technically open. 0 means "no expiry".
  if (expiresAtTs !== 0 && now >= expiresAtTs) {
    return { shouldPull: false, reason: "expired", nextEligibleTs: 0 };
  }

  // Convert the plan's HOURS cadence into the SECONDS the timestamps use.
  const periodS = periodHours * SECONDS_PER_HOUR;
  const nextEligibleTs = lastChargedTs + periodS;

  if (now < nextEligibleTs) {
    return { shouldPull: false, reason: "too-early", nextEligibleTs };
  }

  return { shouldPull: true, reason: "due", nextEligibleTs };
}
