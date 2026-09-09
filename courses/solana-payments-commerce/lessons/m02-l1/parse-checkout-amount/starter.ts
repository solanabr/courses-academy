// transfer-kit: turn whatever the checkout form collected into exact base units.
//
// toBaseUnits (the lab, one rung up) throws, and that is right for it: kit code calls
// it with a string kit produced, so a bad string there is a bug and should stop the
// program. This one sits on the customer boundary, where bad input is ordinary
// traffic - a paste artifact, a currency symbol, a keyboard set to another locale.
// Refusal is a normal outcome there, so it returns a VALUE the checkout can render
// beside the field, never an exception the UI has to catch to stay alive.
//
// The contract:
//   { ok: true,  baseUnits }        exact integer amount for a mint with `decimals` places
//   { ok: false, reason: 'empty' }       nothing but whitespace
//   { ok: false, reason: 'malformed' }   not a plain decimal number: "$12.50", "12,50", "12abc", "."
//   { ok: false, reason: 'negative' }    a well-formed amount with a leading minus
//   { ok: false, reason: 'too-precise' } a digit the mint cannot represent (see below)
//   { ok: false, reason: 'too-large' }   more base units than a u64 holds
//
// Decide the reason in that order, so an all-whitespace field is `empty` rather than
// `malformed`, and "-abc" is `malformed` rather than `negative`.
//
// Two boundaries worth reading twice:
//   - Trailing zeros past the mint's last place are not precision. "1.5000000" on a
//     6-decimal mint is 1500000 base units, exactly representable, so it parses.
//     "1.5000001" is not representable, so it is `too-precise`.
//   - SPL token amounts are u64. At 6 decimals, 18446744073709.551615 is the largest
//     payable amount there is; one base unit more is `too-large`.
//
// Zero parses: "0" is { ok: true, baseUnits: 0n }. Whether a zero-amount charge is
// allowed is the checkout's policy, not the parser's.
//
// TODO: replace the float shortcut below. Read the digits and assemble the result with
// bigint arithmetic - no parseFloat, no Number, no Math anywhere in the parse.

type ParseFailure = 'empty' | 'malformed' | 'negative' | 'too-precise' | 'too-large';

type ParsedAmount =
  | { ok: true; baseUnits: bigint }
  | { ok: false; reason: ParseFailure };

function parseAmount(input: string, decimals: number): ParsedAmount {
  const value = Number(input);
  if (Number.isNaN(value)) {
    return { ok: false, reason: 'malformed' };
  }
  return { ok: true, baseUnits: BigInt(Math.round(value * 10 ** decimals)) };
}
