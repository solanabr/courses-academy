/**
 * Overgrowth's fee rail: split harvested marketplace fees, then size the buyback.
 *
 * Conserves the harvest exactly:
 *   burnedFromFees = floor(harvested * burnBps / 10000)
 *   toTreasury     = harvested - burnedFromFees          // sums back to harvested
 *   buyback        = floor(treasurySol / priceLamportsPerToken)  // tokens bought, then burned
 *
 * All integer math on bigints: base units and lamports never carry fractions.
 *
 * @param harvested               withheld fees harvested to the treasury (base units, bigint)
 * @param burnBps                 share of the harvest burned immediately (0..=10000)
 * @param treasurySol             lamports available for the buyback leg (bigint)
 * @param priceLamportsPerToken   buyback price: lamports per SPROUT base unit (bigint)
 */
function routeFees(
  harvested: bigint,
  burnBps: number,
  treasurySol: bigint,
  priceLamportsPerToken: bigint,
): { burnedFromFees: bigint; toTreasury: bigint; buyback: bigint } {
  const burnedFromFees = (harvested * BigInt(burnBps)) / 10000n;
  const toTreasury = harvested - burnedFromFees;
  const buyback = treasurySol / priceLamportsPerToken;
  return { burnedFromFees, toTreasury, buyback };
}
