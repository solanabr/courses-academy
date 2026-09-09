// The issuer's pre-flight check for a confidential SPROUT mint.
//
// Before you build the raw `initializeConfidentialTransferMint` instruction,
// you validate the requested setup against the Token-2022 combination rules so
// the mint init cannot fail on-chain:
//
//   - a confidential-transfer authority pubkey must be named (checked first)
//     -> reason: "missing-authority"
//   - the mint must actually carry the ConfidentialTransferMint extension, or
//     there is nothing confidential to configure
//     -> reason: "confidential-transfer-mint-not-enabled"
//   - combo rule 5: NonTransferable + ConfidentialTransferMint is INVALID
//     unless ConfidentialMintBurn is also present on the mint
//     -> reason: "nontransferable-confidential-requires-mintburn"
//
// The grader calls the function positionally with four scalars:
//   validateConfidentialConfig(extensionList, authorityKey, autoApprove, auditorKey)
// `extensionList` is the mint's full extension set as ONE pipe-separated
// string, e.g. 'NonTransferable|ConfidentialTransferMint': split it on '|'
// first. The auditor ElGamal pubkey is OPTIONAL (a single global key on the
// mint): it may be null. When set, every confidential transfer must also
// encrypt its amount under it. `autoApprove` maps to auto_approve_new_accounts.
//
// TODO: implement the checks. The starter below approves everything, which
// would ship a mint whose init reverts (or a NonTransferable/confidential mint
// that can never be created).

export interface ConfidentialConfig {
  ok: boolean;
  reason: string; // "ok" when valid, else why it was rejected
  config: {
    authority: string;
    autoApproveNewAccounts: boolean;
    auditorElGamalPubkey: string | null;
  } | null;
}

function validateConfidentialConfig(
  extensionList: string, // pipe-separated extension names on the mint
  authorityKey: string, // confidential-transfer authority pubkey ("" if unset)
  autoApprove: boolean, // auto_approve_new_accounts
  auditorKey: string | null, // auditor_elgamal_pubkey, or null (optional)
): ConfidentialConfig {
  const extensions = extensionList.split("|");
  // TODO: enforce the authority check and combination rules before building.
  return {
    ok: true,
    reason: "ok",
    config: {
      authority: authorityKey,
      autoApproveNewAccounts: autoApprove,
      auditorElGamalPubkey: auditorKey,
    },
  };
}
