/**
 * Decide whether a mint can back a delegation, and whether the token program
 * it was minted under was the right call.
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

  // 1. Not a payment mint at all.
  if (ownerProgram !== "token" && ownerProgram !== "token-2022") {
    return { accepted: false, reason: "INVALID_TOKEN_PROGRAM" };
  }

  // 2. Extensions do not exist on legacy Token. Asking for one here is a
  //    design error, not a runtime one — catch it before you mint.
  if (ownerProgram === "token" && requiredExtension !== "") {
    return {
      accepted: false,
      reason: "design-error-extension-requires-token-2022",
    };
  }

  // 3. The rail's veto. First disqualifying extension wins, in the declared
  //    order, so the verdict is deterministic.
  for (const extension of DISQUALIFYING) {
    if (configured.indexOf(extension) !== -1) {
      return { accepted: false, reason: mintHasCode(extension) };
    }
  }

  // 4. The default, and the right answer most of the time.
  if (ownerProgram === "token") {
    return { accepted: true, reason: "ok-legacy-token" };
  }

  // 5. Accepted, and still the wrong call: nothing justified leaving legacy.
  if (requiredExtension === "") {
    return { accepted: true, reason: "ok-token-2022-not-justified" };
  }

  // 6. Token-2022 for an extension the mint does not actually carry.
  if (configured.indexOf(requiredExtension) === -1) {
    return { accepted: false, reason: "design-error-required-extension-missing" };
  }

  // 7. Justified, configured, and the rail accepts it.
  return { accepted: true, reason: "ok-token-2022-extension-required" };
}

/** "transfer-hook" -> "MINT_HAS_TRANSFER_HOOK" */
function mintHasCode(extension: string): string {
  return "MINT_HAS_" + extension.split("-").join("_").toUpperCase();
}
