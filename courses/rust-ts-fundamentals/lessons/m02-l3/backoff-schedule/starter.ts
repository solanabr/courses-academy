// Pulse Station, backoff schedule (m02-l3)
//
// Compute the retry-delay schedule the fleet uses when a target answers 429.
// Contract (deterministic, jitter is added at the call site, NOT here):
//   attempt n (0-based) waits  min(capMs, baseMs * 2^n)  milliseconds,
//   for n = 0 .. retries-1.
// Return the delays joined with commas, e.g. "500,1000,2000,4000,5000".
// retries = 0 returns the empty string.
//
// The naive version below is the kind of schedule that earns a fleet a
// rate-limit ban: it starts doubling immediately (first wait is 2x base, not base)
// and it never applies the cap, so late attempts wait absurdly long.
// Fix both. Keep backoffSchedule the FIRST function, the grader calls it.

function backoffSchedule(retries: number, baseMs: number, capMs: number): string {
  const delays: number[] = [];
  for (let attempt = 0; attempt < retries; attempt++) {
    // BUG 1: exponent is off by one, attempt 0 should wait baseMs.
    // BUG 2: capMs is never applied.
    delays.push(baseMs * 2 ** (attempt + 1));
  }
  return delays.join(',');
}
