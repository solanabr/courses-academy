/**
 * Token-2022 TransferFee: compute the fee WITHHELD on a single transfer.
 *
 * The real Token-2022 `calculate_fee` does three things this starter gets wrong:
 *   1. It rounds the fee UP (ceiling), not down.
 *   2. It caps the fee at `maximumFee`.
 *   3. It returns 0 when basisPoints is 0 or amount is 0.
 *
 * Fix the function so it matches the on-chain behavior for every test case.
 *
 * The grader calls transferFee(amount, basisPoints, maximumFee) directly -
 * keep transferFee declared plainly at top level (no export, no imports).
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
  // TODO: ceiling division, cap at maximumFee, and the zero cases.
  return (amount * BigInt(basisPoints)) / 10000n;
}
