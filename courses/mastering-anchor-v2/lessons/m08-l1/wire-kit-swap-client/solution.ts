/**
 * Ship the swap's kit client the honest way. There is no official Anchor V2 TS
 * package, so you generate a client with `anchor codama generate -l js` and drive
 * it with @solana/kit. The pin comes from your @solana-program dep's peer range,
 * not from npm's `latest` kit; the swap call comes from the generated builder.
 *
 * Pure — no RPC, no signing, no imports — so it grades deterministically.
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
  // NOT the latest published kit. "^7.0.0" -> 7.
  const pinnedKitMajor = parseInt(splPeer.replace(/[^0-9.]/g, ""), 10);

  // Subgoal 2 — the fee payer is the swap's caller (owner).
  const feePayer = owner;

  // Subgoal 3 — the transaction lifetime is the recent blockhash.
  const lifetime = recentBlockhash;

  // Subgoal 4 — append exactly the swap instruction from your generated client.
  const instructions = [swapInstruction(owner, amountIn, minOut)];

  return { pinnedKitMajor, feePayer, lifetime, instructions };
}

// --- your Codama-generated client's builder — treat as given -----------------
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
