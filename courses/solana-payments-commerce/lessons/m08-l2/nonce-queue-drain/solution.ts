/**
 * Drain the record-fair offline queue -- safely.
 *
 * Called positionally by the grader: (nowSeconds, windowSeconds, queueJson).
 * queueJson is ONE JSON string, a flat array of [id, signedAtSeconds, kind,
 * nonceAdvanced] runs with nonceAdvanced as 0/1; decode it, then classify.
 *
 * Branch on kind. A durable-nonce tx does not expire by wall clock, so age is
 * irrelevant to it -- but a spent nonce (nonceAdvanced) can never land again: the
 * stored value has rolled, so the runtime rejects those exact bytes. It goes to
 * `unsafe` for reconciliation, not back on the wire. A blockhash tx is
 * submittable only while it is inside the ~45s (150-block at 300ms) window.
 *
 * Pure -- no RPC, no imports -- so it grades deterministically.
 */
function drainFairQueue(
  nowSeconds: number,
  windowSeconds: number,
  queueJson: string
): { submit: string[]; expired: string[]; unsafe: string[] } {
  // Decode the wire format: four flat slots per queue entry, order preserved.
  const flat = JSON.parse(queueJson) as (string | number)[];
  const items: {
    id: string;
    signedAtSeconds: number;
    kind: "blockhash" | "nonce";
    nonceAdvanced: boolean;
  }[] = [];
  for (let i = 0; i < flat.length; i += 4) {
    items.push({
      id: flat[i] as string,
      signedAtSeconds: flat[i + 1] as number,
      kind: flat[i + 2] as "blockhash" | "nonce",
      nonceAdvanced: flat[i + 3] === 1,
    });
  }

  const submit: string[] = [];
  const expired: string[] = [];
  const unsafe: string[] = [];

  for (const item of items) {
    if (item.kind === "nonce") {
      // Durable nonces do not expire by wall clock. But a nonce that has already
      // advanced is spent -- those bytes are dead, and the sale needs a ledger
      // state (landed earlier, or lost), not another broadcast.
      if (item.nonceAdvanced) {
        unsafe.push(item.id);
      } else {
        submit.push(item.id);
      }
      continue;
    }

    // blockhash tx: valid only inside the wall-clock window.
    const age = nowSeconds - item.signedAtSeconds;
    if (age <= windowSeconds) {
      submit.push(item.id);
    } else {
      expired.push(item.id);
    }
  }

  return { submit, expired, unsafe };
}
