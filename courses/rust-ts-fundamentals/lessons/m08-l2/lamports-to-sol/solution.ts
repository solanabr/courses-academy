// Pulse Station, lamports to SOL (m08-l2), reference solution

function lamportsToSol(lamports: bigint): string {
  const LAMPORTS_PER_SOL = 1000000000n;
  const whole = lamports / LAMPORTS_PER_SOL;
  const frac = lamports % LAMPORTS_PER_SOL;
  if (frac === 0n) {
    return whole.toString();
  }
  const fracDigits = frac.toString().padStart(9, '0').replace(/0+$/, '');
  return `${whole.toString()}.${fracDigits}`;
}
