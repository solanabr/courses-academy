// transfer-kit: turn a human-entered stablecoin amount into integer base units.
//
// A mint with `decimals` places (USDC = 6) stores balances as integers: 1 USDC =
// 1_000_000 base units. Parse the decimal string exactly with bigint arithmetic so
// the result is correct for any amount, large or small.
function toBaseUnits(amount: string, decimals: number): bigint {
  const trimmed = amount.trim();
  if (!/^\d+(\.\d+)?$/.test(trimmed)) {
    throw new Error(`invalid amount: ${amount}`);
  }
  const [whole, fraction = ""] = trimmed.split(".");
  if (fraction.length > decimals) {
    throw new Error(`too many decimal places for a ${decimals}-decimal token`);
  }
  const padded = fraction.padEnd(decimals, "0");
  return BigInt(whole) * 10n ** BigInt(decimals) + BigInt(padded || "0");
}
