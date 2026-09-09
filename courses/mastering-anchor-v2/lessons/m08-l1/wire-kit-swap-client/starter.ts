/**
 * Ship the swap's kit client the honest way. There is no official Anchor V2 TS
 * package, so you generate a client with `anchor codama generate -l js` and drive
 * it with @solana/kit. Two things bite people here:
 *
 *   1. THE PIN. kit ships a new major ahead of the ecosystem — kit 8.0.0 was
 *      published while every @solana-program/* package still peers on ^7.0.0. The
 *      durable rule is "pin kit to the major your @solana-program deps declare,"
 *      NOT "pin the latest kit." So the major you pin comes from `splPeer`, not
 *      from `kitLatest`.
 *   2. THE CALL. You never hand-roll the swap instruction — you append the one the
 *      generated client gives you, inside the kit pipe (fee payer = owner, lifetime
 *      = the recent blockhash).
 *
 * Pure — no RPC, no signing, no imports — so it grades deterministically. In the
 * real client these are the `pipe(...)` steps around getSwapInstructionAsync.
 *
 * Fill in subgoals 1–4. Do NOT rebuild the swap instruction by hand — call the
 * generated `swapInstruction` below.
 *
 * @param owner           the swap's caller; pays the fee and signs
 * @param amountIn        tokens in, in base units
 * @param minOut          the slippage floor: fail rather than take less than this
 * @param recentBlockhash the transaction's lifetime
 * @param splPeer         e.g. "^7.0.0" — the range @solana-program/token declares
 * @param kitLatest       e.g. "8.0.0" — npm's `latest` today (a trap)
 */
function planSwapClient(
  owner: string,
  amountIn: bigint,
  minOut: bigint,
  recentBlockhash: string,
  splPeer: string,
  kitLatest: string
): {
  pinnedKitMajor: number;
  feePayer: string;
  lifetime: string;
  instructions: {
    program: string;
    accounts: { address: string; role: string }[];
    data: { amountIn: bigint; minOut: bigint };
  }[];
} {
  // Subgoal 1 — pin kit to the major your @solana-program dep peers on (splPeer),
  // NOT the latest published kit. Parse the leading integer out of the range.
  const pinnedKitMajor = 0;

  // Subgoal 2 — the fee payer is the swap's caller (owner).
  const feePayer = "";

  // Subgoal 3 — the transaction lifetime is the recent blockhash.
  const lifetime = "";

  // Subgoal 4 — append exactly the swap instruction from your generated client.
  const instructions: {
    program: string;
    accounts: { address: string; role: string }[];
    data: { amountIn: bigint; minOut: bigint };
  }[] = [];

  return { pinnedKitMajor, feePayer, lifetime, instructions };
}

// --- your Codama-generated client's builder — treat as given -----------------
// (stands in for getSwapInstructionAsync, which derives the two token-side vault
//  PDAs and lists them with the right roles for a token<->ticket swap).
function swapInstruction(
  owner: string,
  amountIn: bigint,
  minOut: bigint
): {
  program: string;
  accounts: { address: string; role: string }[];
  data: { amountIn: bigint; minOut: bigint };
} {
  const tokenVault = "TokV" + owner.slice(0, 39).padEnd(39, "1");
  const ticketVault = "TktV" + owner.slice(0, 39).padEnd(39, "1");
  return {
    program: "QSwapPr0gram11111111111111111111111111111111",
    accounts: [
      { address: owner, role: "writable-signer" },
      { address: tokenVault, role: "writable" },
      { address: ticketVault, role: "writable" },
      { address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", role: "readonly" },
    ],
    data: { amountIn, minOut },
  };
}
