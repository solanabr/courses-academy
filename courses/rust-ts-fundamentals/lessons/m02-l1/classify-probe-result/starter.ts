// Pulse Station, probe classifier (m02-l1)
//
// A probe attempt arrives as a (kind, value) pair:
//   kind 'ok'         → value is the measured latency in ms
//   kind 'timeout'    → value is the timeout budget (ms) that expired
//   kind 'http-error' → value is the HTTP status code the target returned
//
// Classification contract:
//   'ok'         → latency <  400            ⇒ 'up'
//                  400 ≤ latency ≤ 1000      ⇒ 'degraded'
//                  latency > 1000            ⇒ 'down'
//   'timeout'    → always 'down'
//   'http-error' → 429 ⇒ 'degraded' (the target ANSWERED, you are rate-limited)
//                  anything else ⇒ 'down'
//   unknown kind → 'invalid'
//
// This is the untyped v0 logic the lesson just caught lying. Your job:
//   1. Model ProbeResult as a discriminated union (one variant per kind).
//   2. Parse the (kind, value) pair ONCE at the boundary; unknown kinds
//      parse to null and classify as 'invalid'.
//   3. Classify with an exhaustive switch over the union so the compiler
//      proves every variant is handled (assertNever on the default arm).
// Keep classifyProbe the FIRST function in the file, the grader calls it.

function classifyProbe(kind: string, value: number): string {
  // v0: only 'ok' probes were ever considered, the degraded band does not
  // exist, and every other kind is lumped into 'down'. Fix all of it.
  if (kind === 'ok') {
    return value < 1000 ? 'up' : 'down';
  }
  return 'down';
}
