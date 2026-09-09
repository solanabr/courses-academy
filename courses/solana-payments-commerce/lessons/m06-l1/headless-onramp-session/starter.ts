// Headless Coinbase Onramp: build the server-side session request AND the
// client onramp URL for Wavelength's SOL-less buyer.
//
// The headless flow has TWO halves:
//   1. Your SERVER calls the session-token endpoint with the destination
//      address + the chains/assets it may receive. Coinbase returns a
//      one-time `sessionToken` that BINDS those params server-side.
//   2. Your CLIENT opens the onramp URL carrying only that `sessionToken`.
//      The wallet address must NOT appear in the client URL: binding it to
//      the token is the whole point (a tampered client URL cannot redirect
//      funds to a different address).
//
// TODO: this starter leaks the address into the URL and defaults to the
// wrong network. Fix both halves so the tests pass.

interface OnrampInit {
  requestBody: {
    addresses: { address: string; blockchains: string[] }[];
    assets: string[];
  };
  onrampUrl: string;
}

function initHeadlessOnramp(
  destinationAddress: string,
  sessionToken: string,
  fiatAmount: number
): OnrampInit {
  // TODO: the destination should be received on Solana, not Ethereum.
  const requestBody = {
    addresses: [{ address: destinationAddress, blockchains: ['ethereum'] }],
    assets: ['USDC'],
  };

  // TODO: never put the raw wallet address in the client URL; carry the
  // sessionToken instead, and default the network/asset to Solana USDC.
  // (Query is built by hand: the grading sandbox is a bare JS realm with
  // no URLSearchParams.)
  const params: [string, string][] = [
    ['address', destinationAddress],
    ['defaultNetwork', 'ethereum'],
    ['presetFiatAmount', String(fiatAmount)],
  ];
  const query = params
    .map(([k, v]) => `${k}=${encodeURIComponent(v)}`)
    .join('&');
  const onrampUrl = `https://pay.coinbase.com/buy/select-asset?${query}`;

  return { requestBody, onrampUrl };
}
