// The issuer's pre-flight check for a confidential SPROUT mint (reference).
//
// Called positionally by the grader:
//   validateConfidentialConfig(extensionList, authorityKey, autoApprove, auditorKey)
// `extensionList` is the mint's extension set as one pipe-separated string.

export interface ConfidentialConfig {
  ok: boolean;
  reason: string;
  config: {
    authority: string;
    autoApproveNewAccounts: boolean;
    auditorElGamalPubkey: string | null;
  } | null;
}

function validateConfidentialConfig(
  extensionList: string,
  authorityKey: string,
  autoApprove: boolean,
  auditorKey: string | null,
): ConfidentialConfig {
  const reject = (reason: string): ConfidentialConfig => ({
    ok: false,
    reason,
    config: null,
  });

  // A named authority is required to configure the extension.
  if (!authorityKey) {
    return reject("missing-authority");
  }

  const extensions = extensionList.split("|");
  const has = (name: string): boolean => extensions.includes(name);

  // Nothing to configure if the confidential extension is not on the mint.
  if (!has("ConfidentialTransferMint")) {
    return reject("confidential-transfer-mint-not-enabled");
  }

  // Combo rule 5: NonTransferable + ConfidentialTransferMint is invalid
  // unless ConfidentialMintBurn is also present.
  if (has("NonTransferable") && !has("ConfidentialMintBurn")) {
    return reject("nontransferable-confidential-requires-mintburn");
  }

  return {
    ok: true,
    reason: "ok",
    config: {
      authority: authorityKey,
      autoApproveNewAccounts: autoApprove,
      auditorElGamalPubkey: auditorKey, // optional: may be null
    },
  };
}
