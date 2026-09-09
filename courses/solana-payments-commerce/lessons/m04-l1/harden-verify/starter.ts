/**
 * Trust no frontend. A wallet said "paid"; now the SERVER decides whether the
 * order is really fulfillable, from the transaction alone.
 *
 * The grader calls `verifyPayment` positionally: the parsed transaction arrives
 * as ONE JSON string (`txJson`), the order's expected criteria arrive as five
 * flat arguments, and the set of already-fulfilled signatures arrives as ONE
 * JSON string (`seenJson`). JSON.parse both strings first thing (the parse
 * lines are already written for you below), then work with real values.
 * The verdict is a single object:
 *   { ok: true,  reason: 'verified' }               => safe to fulfill
 *   { ok: false, reason: '<why not>' }              => do NOT fulfill
 *
 * The naive version below only de-dupes. That is the trap the whole lesson is
 * about: a frontend "confirmed" plus a signature check will happily fulfill a
 * payment in the WRONG token, of the WRONG mint, for the WRONG amount, against
 * the WRONG order. Harden it, in order, so exactly one reason surfaces per case:
 *   'duplicate'  → this signature was already fulfilled (replay / webhook retry)
 *   'wrong-token-program' → tx.tokenProgram !== expectedTokenProgram
 *   'no-payment' → no transfer landed in recipientAta
 *   'wrong-mint' → the transfer into our ATA was a different mint
 *   'underpaid'  → (postAmount - preAmount) < expectedAmount
 *   'wrong-reference' → tx.memo !== orderRef
 *   'verified'   → all checks pass
 *
 * Pure (no RPC, no imports), so it grades deterministically. In the real
 * verifier these fields come straight off a jsonParsed getTransaction response,
 * which is why `preAmount`/`postAmount` arrive as decimal STRINGS of base units
 * (the RPC's own shape; `uiAmount` is a float and is never the source). Lift
 * them with BigInt() before comparing against `expectedAmount`, which IS a
 * bigint: Number() would round a real settlement above 2**53 base units.
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

  // TODO Check 2 (token program): a frontend can't be trusted about which
  //   program moved the tokens. Compare tx.tokenProgram to expectedTokenProgram.

  // TODO Check 3: locate the transfer that landed in recipientAta.
  //   If none, reason 'no-payment'.

  // TODO Check 4 (mint): that transfer must be expectedMint, not just any token.

  // TODO Check 5 (amount): BigInt(postAmount) - BigInt(preAmount) must be
  //   >= expectedAmount.

  // TODO Check 6 (reference): tx.memo must equal orderRef.

  return { ok: true, reason: "verified" };
}
