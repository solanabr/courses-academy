// transfer-kit: turn a human-entered stablecoin amount into integer base units.
//
// A mint with `decimals` places (USDC = 6) stores balances as integers: 1 USDC =
// 1_000_000 base units. The checkout takes a string the customer typed ("12.50")
// and must hand TransferChecked an exact bigint, never a fraction of a cent off.
//
// The float shortcut below looks fine on small amounts and breaks silently once
// the base-unit value passes 2**53 (JavaScript's safe-integer ceiling): large
// transfers send the wrong number of cents, with no error.
//
// TODO: rewrite toBaseUnits so it parses the decimal string EXACTLY, with no
// floating-point math. Reject an amount with more fractional digits than the mint
// has decimals (that is an over-precise amount the mint cannot represent).
function toBaseUnits(amount: string, decimals: number): bigint {
  return BigInt(Math.round(parseFloat(amount) * 10 ** decimals));
}
