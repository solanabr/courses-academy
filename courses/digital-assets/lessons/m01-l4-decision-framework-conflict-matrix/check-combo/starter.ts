// check-combo: derive the Token-2022 mint extension conflict matrix from source.
//
// The five rules live in `check_for_invalid_mint_extension_combinations`
// (solana-program/token-2022, interface/src/extension/mod.rs). The grader passes
// each mint-level extension the builder wants to enable as its own string
// argument (up to three); return whether the combination is legal, and if not,
// why.
//
// The five rules:
//   1. ConfidentialTransferFeeConfig requires BOTH TransferFeeConfig AND ConfidentialTransferMint.
//   2. TransferFeeConfig + ConfidentialTransferMint together REQUIRE ConfidentialTransferFeeConfig.
//   3. ConfidentialMintBurn requires ConfidentialTransferMint.
//   4. ScaledUiAmount + InterestBearingConfig are mutually exclusive.
//   5. NonTransferable + ConfidentialTransferMint is invalid UNLESS ConfidentialMintBurn is also present.
//
// TODO: only rule 4 is implemented. Add rules 1, 2, 3, and 5 so an illegal
// combination is rejected the way the source function rejects it.

type ComboResult = { valid: boolean; reason?: string };

function checkCombo(ext1: string, ext2?: string, ext3?: string): ComboResult {
  const extensions = [ext1, ext2, ext3].filter((e): e is string => typeof e === "string");
  const has = (name: string): boolean => extensions.includes(name);

  // Rule 4: ScaledUiAmount + InterestBearingConfig mutually exclusive.
  if (has("ScaledUiAmount") && has("InterestBearingConfig")) {
    return { valid: false, reason: "ScaledUiAmount and InterestBearingConfig are mutually exclusive" };
  }

  return { valid: true };
}
