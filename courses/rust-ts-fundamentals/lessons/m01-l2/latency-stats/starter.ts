// Latency stats: summarize a batch of probe samples.
//
// Your pulse CLI collects latency samples in milliseconds. One sample is noise;
// a summary is a signal. Implement latencyStats so it takes a comma-separated
// string of latency samples (e.g. "120,340,95") and returns an object with:
//
//   min  - the smallest sample
//   max  - the largest sample
//   mean - the arithmetic mean, rounded to 2 decimal places
//          (Math.round(x * 100) / 100)
//   p95  - the 95th percentile by the nearest-rank method:
//          sort ascending, take the element at index Math.ceil(0.95 * n) - 1
//
// Input is guaranteed non-empty and every entry parses as a positive number.
// Note: this file runs self-contained in the grader - no imports, no exports.

function latencyStats(csv: string): { min: number; max: number; mean: number; p95: number } {
  const samples = csv.split(',').map((s) => Number(s.trim()));

  // TODO: compute the real values. Zeros never shipped a status page.
  return { min: 0, max: 0, mean: 0, p95: samples.length };
}
