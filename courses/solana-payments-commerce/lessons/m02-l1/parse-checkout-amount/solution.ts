// transfer-kit: turn whatever the checkout form collected into exact base units.
//
// The customer boundary refuses by returning a reason, not by throwing: bad input here
// is ordinary traffic, and the checkout needs something it can render beside the field.

type ParseFailure = 'empty' | 'malformed' | 'negative' | 'too-precise' | 'too-large';

type ParsedAmount =
  | { ok: true; baseUnits: bigint }
  | { ok: false; reason: ParseFailure };

/** SPL token amounts are u64: this is the largest transferable base-unit count. */
const U64_MAX = 2n ** 64n - 1n;

function parseAmount(input: string, decimals: number): ParsedAmount {
  const trimmed = input.trim();
  if (trimmed === '') {
    return { ok: false, reason: 'empty' };
  }

  const match = /^(-?)(\d+)(?:\.(\d+))?$/.exec(trimmed);
  if (!match) {
    return { ok: false, reason: 'malformed' };
  }

  const [, sign, whole, fraction = ''] = match;
  if (sign === '-') {
    return { ok: false, reason: 'negative' };
  }

  // Digits past the mint's last place are only an error when one of them is not a
  // zero: "1.5000000" is 1.5, while "1.5000001" is an amount the mint cannot hold.
  const kept = fraction.slice(0, decimals);
  if (/[1-9]/.test(fraction.slice(decimals))) {
    return { ok: false, reason: 'too-precise' };
  }

  const baseUnits =
    BigInt(whole) * 10n ** BigInt(decimals) +
    BigInt(kept.padEnd(decimals, '0') || '0');
  if (baseUnits > U64_MAX) {
    return { ok: false, reason: 'too-large' };
  }

  return { ok: true, baseUnits };
}
