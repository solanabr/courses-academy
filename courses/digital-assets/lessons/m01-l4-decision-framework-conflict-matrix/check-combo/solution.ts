// check-combo: the Token-2022 mint extension conflict matrix, derived from source.
//
// The five rules live in `check_for_invalid_mint_extension_combinations`
// (solana-program/token-2022, interface/src/extension/mod.rs). The grader passes
// each mint-level extension the builder wants to enable as its own string
// argument (up to three); return whether the combination is legal, and if not,
// which rule it violates.

type ComboResult = { valid: boolean; reason?: string };

function checkCombo(ext1: string, ext2?: string, ext3?: string): ComboResult {
  const extensions = [ext1, ext2, ext3].filter((e): e is string => typeof e === "string");
  const has = (name: string): boolean => extensions.includes(name);

  // Rule 1: ConfidentialTransferFeeConfig requires BOTH TransferFeeConfig AND ConfidentialTransferMint.
  if (has("ConfidentialTransferFeeConfig") && !(has("TransferFeeConfig") && has("ConfidentialTransferMint"))) {
    return {
      valid: false,
      reason: "ConfidentialTransferFeeConfig requires both TransferFeeConfig and ConfidentialTransferMint",
    };
  }

  // Rule 2: TransferFeeConfig + ConfidentialTransferMint together REQUIRE ConfidentialTransferFeeConfig.
  if (has("TransferFeeConfig") && has("ConfidentialTransferMint") && !has("ConfidentialTransferFeeConfig")) {
    return {
      valid: false,
      reason: "TransferFeeConfig with ConfidentialTransferMint requires ConfidentialTransferFeeConfig",
    };
  }

  // Rule 3: ConfidentialMintBurn requires ConfidentialTransferMint.
  if (has("ConfidentialMintBurn") && !has("ConfidentialTransferMint")) {
    return { valid: false, reason: "ConfidentialMintBurn requires ConfidentialTransferMint" };
  }

  // Rule 4: ScaledUiAmount + InterestBearingConfig mutually exclusive.
  if (has("ScaledUiAmount") && has("InterestBearingConfig")) {
    return { valid: false, reason: "ScaledUiAmount and InterestBearingConfig are mutually exclusive" };
  }

  // Rule 5: NonTransferable + ConfidentialTransferMint invalid UNLESS ConfidentialMintBurn present.
  if (has("NonTransferable") && has("ConfidentialTransferMint") && !has("ConfidentialMintBurn")) {
    return {
      valid: false,
      reason: "NonTransferable with ConfidentialTransferMint requires ConfidentialMintBurn",
    };
  }

  return { valid: true };
}
