// Pulse Station, backoff schedule (m02-l3), reference solution
//
// Deterministic exponential backoff with a cap: attempt n (0-based) waits
// min(capMs, baseMs * 2^n). Jitter is deliberately absent here, the graded
// schedule is the base curve; the lesson adds jitter at the call site so
// retries from many probes do not stampede in lockstep.

function backoffSchedule(retries: number, baseMs: number, capMs: number): string {
  const delays: number[] = [];
  for (let attempt = 0; attempt < retries; attempt++) {
    delays.push(Math.min(capMs, baseMs * 2 ** attempt));
  }
  return delays.join(',');
}
