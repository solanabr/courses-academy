/**
 * Drain the record-fair offline queue -- safely.
 *
 * At the Saturday stall you sign sales offline and broadcast them later. Two
 * kinds of transaction end up in the queue:
 *
 *   - 'blockhash' txs are pinned to a recent blockhash and EXPIRE by wall clock.
 *     A blockhash is valid for 150 blocks; at a ~300ms slot time that is only
 *     about 45 seconds (windowSeconds). Past the window the RPC rejects the tx
 *     as an expired blockhash and you must re-sign it.
 *   - 'nonce' txs use a durable nonce and DO NOT expire by wall clock -- that is
 *     the whole point of the fair queue. But a durable nonce is single-use: once
 *     it has advanced (nonceAdvanced === true) the stored value no longer matches
 *     the transaction's recentBlockhash, so a conforming validator rejects those
 *     bytes outright. Rebroadcasting cannot land; it only burns sends and smudges
 *     the ledger the drain depends on. A spent-nonce tx is never resubmitted --
 *     it is reconciled (landed earlier) or escalated (lost), by a human.
 *
 * CALLING CONVENTION: the grader calls this function positionally --
 * drainFairQueue(nowSeconds, windowSeconds, queueJson). The queue arrives as
 * ONE JSON string: a flat array with four slots per entry, in queue order:
 * [id, signedAtSeconds, kind, nonceAdvanced] repeated, where nonceAdvanced is
 * 0 or 1. JSON.parse it and rebuild the items -- that decode is already written
 * for you below; your job starts at the classification loop.
 *
 * Implement the drain: sort the queue into what to submit now, what has expired
 * and needs re-signing, and what is unsafe to touch. Preserve input order in
 * every output list. Pure -- no RPC, no imports -- so it grades deterministically.
 */
function drainFairQueue(
  nowSeconds: number,
  windowSeconds: number,
  queueJson: string
): { submit: string[]; expired: string[]; unsafe: string[] } {
  // Prewritten decode: four flat slots per queue entry, order preserved.
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
    // TODO: this naive pass treats EVERYTHING as a blockhash tx that expires by
    // wall clock. It ignores durable nonces (which do not expire) and never
    // separates a spent nonce out for reconciliation. Fix it: branch on
    // item.kind, and for a nonce tx route nonceAdvanced === true into `unsafe`.
    const age = nowSeconds - item.signedAtSeconds;
    if (age <= windowSeconds) {
      submit.push(item.id);
    } else {
      expired.push(item.id);
    }
  }

  return { submit, expired, unsafe };
}
