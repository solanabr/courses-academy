// Latency stats: summarize a batch of probe samples.
// Reference solution.

function latencyStats(csv: string): { min: number; max: number; mean: number; p95: number } {
  const parsed = csv.split(',').map((s) => Number(s.trim()));
  const samples = [...parsed].sort((a, b) => a - b); // sort a COPY, never the input order

  const n = samples.length;
  const min = samples[0];
  const max = samples[n - 1];
  // Nearest-rank 95th percentile: sort ascending, take index ceil(0.95 * n) - 1.
  const p95 = samples[Math.max(0, Math.ceil(0.95 * n) - 1)];

  // Input is guaranteed non-empty, but noUncheckedIndexedAccess cannot know
  // that: handle the undefined case the flag's way instead of silencing it.
  if (min === undefined || max === undefined || p95 === undefined) {
    throw new Error('latencyStats needs at least one sample');
  }

  const sum = samples.reduce((acc, v) => acc + v, 0);
  const mean = Math.round((sum / n) * 100) / 100;

  return { min, max, mean, p95 };
}
