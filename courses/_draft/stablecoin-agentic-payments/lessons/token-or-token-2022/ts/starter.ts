/**
 * Decide whether a mint can back a delegation, and whether the token program
 * it was minted under was the right call.
 *
 * @param ownerProgram      "token" | "token-2022" | anything else
 * @param extensions        pipe-separated extension slugs configured on the
 *                          mint, e.g. "transfer-fee|interest-bearing".
 *                          Empty string means no extensions.
 * @param requiredExtension the one extension your product actually needs.
 *                          Empty string means none.
 */
function classifyPaymentMint(
  ownerProgram: string,
  extensions: string,
  requiredExtension: string
): { accepted: boolean; reason: string } {
  // Checked in this order so a mint carrying several gets a stable verdict.
  const DISQUALIFYING = [
    "confidential-transfer",
    "mint-close-authority",
    "non-transferable",
    "pausable",
    "permanent-delegate",
    "transfer-fee",
  ];
  const configured = extensions === "" ? [] : extensions.split("|");

  // TODO: implement the rule.
  //
  //   1. Neither token program owns it            -> INVALID_TOKEN_PROGRAM
  //   2. Legacy Token but an extension is needed  -> design-error-extension-requires-token-2022
  //   3. First disqualifying extension configured -> MINT_HAS_<UPPER_SNAKE>
  //   4. Legacy Token, nothing needed             -> ok-legacy-token
  //   5. Token-2022, nothing needed               -> ok-token-2022-not-justified
  //   6. Token-2022, needed extension absent      -> design-error-required-extension-missing
  //   7. Otherwise                                -> ok-token-2022-extension-required
  void DISQUALIFYING;
  void configured;
  void ownerProgram;
  void requiredExtension;

  return { accepted: true, reason: "ok-legacy-token" };
}

/** "transfer-hook" -> "MINT_HAS_TRANSFER_HOOK" */
function mintHasCode(extension: string): string {
  return "MINT_HAS_" + extension.split("-").join("_").toUpperCase();
}
