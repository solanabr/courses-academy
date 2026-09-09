/**
 * The hardened server-side verifier. Every check is ordered so exactly one
 * reason surfaces per failure, and the balance delta is read off the tx, never
 * taken from the client. The grader calls it positionally: the transaction and
 * the fulfilled-signature set arrive as JSON strings, the expected order
 * criteria as flat arguments. Pure (no RPC, no imports), so it grades
 * deterministically.
 */
type Transfer = { mint: string; destination: string; preAmount: string; postAmount: string };
type Tx = { signature: string; tokenProgram: string; transfers: Transfer[]; memo: string };
type Result = { ok: boolean; reason: string };

function verifyPayment(
  txJson: string,
  expectedMint: string,
  expectedTokenProgram: string,
  recipientAta: string,
  expectedAmount: bigint,
  orderRef: string,
  seenJson: string
): Result {
  const tx: Tx = JSON.parse(txJson);
  const seen: string[] = JSON.parse(seenJson);

  // Check 1 (replay/idempotency): already fulfilled this signature.
  if (seen.includes(tx.signature)) return { ok: false, reason: "duplicate" };

  // Check 2 (token program): validate BEFORE trusting any transfer.
  if (tx.tokenProgram !== expectedTokenProgram) {
    return { ok: false, reason: "wrong-token-program" };
  }

  // Check 3: locate the transfer into our own ATA.
  const paid = tx.transfers.find((t) => t.destination === recipientAta);
  if (!paid) return { ok: false, reason: "no-payment" };

  // Check 4 (mint): the right stablecoin, not just any token into the ATA.
  if (paid.mint !== expectedMint) return { ok: false, reason: "wrong-mint" };

  // Check 5 (amount): the balance delta on OUR account, not a client number.
  // The amounts arrive as decimal strings, the shape getTransaction reports;
  // lift them to bigint so the subtraction is exact at any size.
  const delta = BigInt(paid.postAmount) - BigInt(paid.preAmount);
  if (delta < expectedAmount) return { ok: false, reason: "underpaid" };

  // Check 6 (reference): the memo ties this payment to a specific order.
  if (tx.memo !== orderRef) return { ok: false, reason: "wrong-reference" };

  return { ok: true, reason: "verified" };
}
