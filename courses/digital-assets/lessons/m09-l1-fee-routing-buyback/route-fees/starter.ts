/**
 * Overgrowth's fee rail: split harvested marketplace fees, then size the buyback.
 *
 * The economy works like this:
 *   1. A Token-2022 transfer fee is WITHHELD on recipient SPROUT accounts, then
 *      harvested to the treasury. Call the harvested amount `harvested` (base units).
 *   2. A `burnBps` share of that harvest is burned immediately; the rest stays in
 *      the treasury for operations.
 *   3. Separately, the treasury holds `treasurySol` lamports and buys SPROUT back on
 *      its buyback counterparty at `priceLamportsPerToken`, then burns what it bought.
 *
 * The rail must CONSERVE the harvest: burnedFromFees + toTreasury === harvested.
 * Nothing is created or destroyed by the split itself.
 *
 * This starter breaks conservation (toTreasury ignores the burn) and mis-sizes the
 * buyback (it multiplies instead of dividing by price). Fix both.
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
  const toTreasury = harvested; // BUG: forgets to subtract the burned share
  const buyback = treasurySol * priceLamportsPerToken; // BUG: should divide by price
  return { burnedFromFees, toTreasury, buyback };
}
