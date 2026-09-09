/**
 * Token-2022 TransferFee: compute the fee WITHHELD on a single transfer.
 *
 * Mirrors the on-chain `calculate_fee`:
 *   fee = 0                                   if basisPoints == 0 or amount == 0
 *   fee = min( ceil(amount * bps / 10000), maximumFee )   otherwise
 *
 * Ceiling division is done as (numerator + 9999) / 10000 on bigints.
 *
 * @param amount        pre-fee transfer amount, in base units (bigint)
 * @param basisPoints   transfer_fee_basis_points (0..=10000)
 * @param maximumFee    hard cap on the withheld fee, in base units (bigint)
 * @returns             the fee withheld on the recipient account (bigint)
 */
function transferFee(
  amount: bigint,
  basisPoints: number,
  maximumFee: bigint,
): bigint {
  if (basisPoints === 0 || amount === 0n) return 0n;
  const numerator = amount * BigInt(basisPoints);
  const raw = (numerator + 9999n) / 10000n; // ceiling division
  return raw < maximumFee ? raw : maximumFee;
}
