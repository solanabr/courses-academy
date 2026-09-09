// Derive how much SOL a pump.fun-style bonding curve must take in before it
// GRADUATES: the moment every real token has been sold off the curve and the
// pool migrates to an AMM.
//
// The curve is a constant product on VIRTUAL reserves:
//     k = virtualSolReserves * virtualTokenReserves   (held constant)
// Buyers pay SOL into the virtual SOL reserve and pull tokens out of the virtual
// token reserve. Graduation happens when the REAL token reserve is fully drained,
// i.e. the virtual token reserve has fallen by exactly realTokenReserves.
//
// The grader calls graduationSol with three positional numbers, in this order:
//     graduationSol(virtualSolReserves, virtualTokenReserves, realTokenReserves)
// Units (kept human-scale on purpose): SOL for the SOL reserve, MILLIONS of tokens
// for both token reserves. Return the SOL that must be ADDED to the curve to
// graduate (final virtual SOL reserve minus the initial virtual SOL reserve).
//
// With pump's first-party constants (30 SOL / 1073M virtual / 793.1M real) the
// answer is ~85 SOL: but it is NOT the flat token price times the supply. The
// starter below makes exactly that mistake: it prices every real token at the
// curve's OPENING price and sums them. That ignores the curve steepening as the
// token reserve shrinks, so it under-counts badly. Rewrite graduationSol so it
// derives the threshold from the constant-product invariant.

function graduationSol(
  virtualSolReserves: number,   // SOL
  virtualTokenReserves: number, // millions of tokens
  realTokenReserves: number     // millions of tokens sellable off the curve
): number {
  // NAIVE: price every real token at the opening spot price and add it up.
  const openingPrice = virtualSolReserves / virtualTokenReserves;
  return round3(openingPrice * realTokenReserves);
}

function round3(x: number): number {
  return Math.round(x * 1000) / 1000;
}
