// The paying agent's pre-flight guard.
//
// Before an @x402/svm agent partially signs a payment for a 402 response, it
// must decide whether this call is even payable under its own spendControls:
//   - scheme must be the "exact" SVM scheme (we do not auto-pay "upto" etc.)
//   - network must be a known Solana CAIP-2 id (mainnet or devnet)
//   - the asset must be on the agent's pegged-asset allowlist
//   - memo (the invoice id we later reconcile) must be <= 256 bytes
//   - a fee payer (the facilitator/sponsor) must be named
//   - the USD-converted amount must not exceed the per-payment cap
//
// The grader calls this function positionally, one argument per field:
//   decidePayment(scheme, network, asset, amount, feePayer, memo,
//                 maxUsd, allowedAssetsJson)
// The first six are the payment terms from the 402 (scheme, CAIP-2 network id,
// SPL mint, atomic-unit amount as a decimal string, the sponsor's fee payer,
// and the invoice-id memo). The last two are the agent's own spendControls:
// the per-payment USD cap, and the pegged-asset allowlist serialized as a
// JSON string mapping mint -> decimals (pegged 1:1 to USD). JSON.parse the
// allowlist first thing.
//
// TODO: implement the checks. The starter below pays everything, which is
// exactly the bug that drains an agent's wallet.

interface PaymentDecision {
  willPay: boolean;
  reason: string; // "ok" when paying, else why it was declined
  feePayer: string | null;
  memo: string | null;
}

function decidePayment(
  scheme: string, // "exact" | "upto" | "auth-capture" | "batch-settlement"
  network: string, // CAIP-2 id, e.g. "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp"
  asset: string, // SPL mint address
  amount: string, // atomic units, decimal string
  feePayer: string, // the facilitator/sponsor fee payer
  memo: string | null, // invoice id, later reconciled
  maxUsd: number, // cap per payment
  allowedAssetsJson: string, // JSON string: mint -> decimals (pegged 1:1 to USD)
): PaymentDecision {
  // TODO: JSON.parse(allowedAssetsJson), then replace this pay-everything
  // stub with the real guard.
  return {
    willPay: true,
    reason: "ok",
    feePayer: feePayer ?? null,
    memo: memo ?? null,
  };
}
