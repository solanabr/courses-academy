// Derive the SOL a pump.fun-style bonding curve must take in to GRADUATE.
//
// Constant-product invariant on virtual reserves:
//     k = virtualSolReserves * virtualTokenReserves
// Graduation drains the real token reserve, so the virtual token reserve falls by
// exactly realTokenReserves. The invariant fixes the final virtual SOL reserve:
//     finalVirtualToken = virtualTokenReserves - realTokenReserves
//     finalVirtualSol   = k / finalVirtualToken
// SOL added to the curve = finalVirtualSol - virtualSolReserves.
//
// Called positionally: graduationSol(virtualSolReserves, virtualTokenReserves,
// realTokenReserves): SOL, then both token reserves in millions of tokens.
//
// Pump's published constants (30 SOL / 1073M / 793.1M) give:
//     30 * 1073 / (1073 - 793.1) - 30 = 115.005... - 30 ~= 85.005 SOL.

function graduationSol(
  virtualSolReserves: number,   // SOL
  virtualTokenReserves: number, // millions of tokens
  realTokenReserves: number     // millions of tokens sellable off the curve
): number {
  const k = virtualSolReserves * virtualTokenReserves;
  const finalVirtualToken = virtualTokenReserves - realTokenReserves;
  const finalVirtualSol = k / finalVirtualToken;
  return round3(finalVirtualSol - virtualSolReserves);
}

function round3(x: number): number {
  return Math.round(x * 1000) / 1000;
}
