// Pulse Station, lamports to SOL (m08-l2)
//
// getBalance hands the panel a bigint of LAMPORTS (1 SOL = 1,000,000,000
// lamports). The dashboard shows SOL. Converting through Number silently
// loses precision above 2^53 lamports, so the panel formats with BigInt
// math only, division and remainder, then string work.
//
// Contract:
//   - whole part: lamports / 1_000_000_000n  (BigInt division)
//   - fraction:   lamports % 1_000_000_000n, printed as NINE digits
//     (left-padded with zeros), then trailing zeros trimmed
//   - if the fraction is zero, print the whole part alone: "1", not "1.0"
//   - examples: 1n -> "0.000000001", 1500000000n -> "1.5", 0n -> "0"
//
// The version below has the classic padding bug: it prints the remainder's
// digits raw, so 2500000n renders as "0.2500000" instead of "0.0025",
// off by a factor of a hundred on a balance display. It also never trims
// trailing zeros. Fix both, using string methods only (no Number, no
// parseFloat, the whole point is that BigInt survives where Number lies).
// Keep lamportsToSol the FIRST function in the file, the grader calls it.

function lamportsToSol(lamports: bigint): string {
  const LAMPORTS_PER_SOL = 1000000000n;
  const whole = lamports / LAMPORTS_PER_SOL;
  const frac = lamports % LAMPORTS_PER_SOL;
  if (frac === 0n) {
    return whole.toString();
  }
  // BUG: the fractional digits are not left-padded to 9 places, and
  // trailing zeros are never trimmed.
  return `${whole.toString()}.${frac.toString()}`;
}
