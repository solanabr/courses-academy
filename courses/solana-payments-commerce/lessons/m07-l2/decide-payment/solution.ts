// The paying agent's pre-flight guard (reference solution).
//
// Called positionally by the grader:
//   decidePayment(scheme, network, asset, amount, feePayer, memo,
//                 maxUsd, allowedAssetsJson)
// allowedAssetsJson is the pegged-asset allowlist serialized as a JSON string
// mapping mint -> decimals (pegged 1:1 to USD).

interface PaymentDecision {
  willPay: boolean;
  reason: string; // "ok" when paying, else why it was declined
  feePayer: string | null;
  memo: string | null;
}

// Known Solana CAIP-2 network ids (mainnet, devnet).
const KNOWN_NETWORKS = new Set<string>([
  "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",
  "solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1",
]);

const MEMO_MAX_BYTES = 256;

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
  const allowedAssets: Record<string, number> = JSON.parse(allowedAssetsJson);

  const decline = (reason: string): PaymentDecision => ({
    willPay: false,
    reason,
    feePayer: null,
    memo: null,
  });

  if (scheme !== "exact") {
    return decline(`unsupported scheme: ${scheme}`);
  }

  if (!KNOWN_NETWORKS.has(network)) {
    return decline(`unknown network: ${network}`);
  }

  const decimals = allowedAssets[asset];
  if (decimals === undefined) {
    return decline(`asset not allowed: ${asset}`);
  }

  if (memo !== null && memo !== undefined) {
    const memoBytes = new TextEncoder().encode(memo).length;
    if (memoBytes > MEMO_MAX_BYTES) {
      return decline(`memo too large: ${memoBytes} bytes`);
    }
  }

  if (!feePayer) {
    return decline("missing fee payer");
  }

  const usd = Number(amount) / 10 ** decimals;
  if (usd > maxUsd) {
    return decline(`exceeds spend cap: $${usd} > $${maxUsd}`);
  }

  return { willPay: true, reason: "ok", feePayer, memo: memo ?? null };
}
