// Predict whether Raydium's CP-Swap pool-creation check will ACCEPT a mint.
//
// Source rule (raydium-cp-swap programs/cp-swap/src/utils/token.rs): the pool
// creation check reverts on ANY Token-2022 extension outside a fixed allowlist of
// exactly five, and it is bypassed for classic SPL mints, for the static
// MINT_WHITELIST, and for mints with an initialized mint-association account.
//
// Calling convention: isRoutable(tokenProgram, extensionList, whitelisted) -
// extensionList is the space-separated extension type names ('' = none).

function isRoutable(tokenProgram: string, extensionList: string, whitelisted: boolean): boolean {
  // Classic SPL mints skip the extension check entirely (the program predates T22).
  if (tokenProgram === 'spl') return true;
  // Static MINT_WHITELIST / mint-association bypass: whitelisted power-extension mints
  // (e.g. regulated PermanentDelegate stablecoins) are permitted despite the extension.
  if (whitelisted) return true;
  // Raydium CP-Swap / CLMM allowlist: exactly these five extensions pass.
  const allowlist = new Set([
    'TransferFeeConfig',
    'MetadataPointer',
    'TokenMetadata',
    'InterestBearingConfig',
    'ScaledUiAmount',
  ]);
  const extensions = extensionList === '' ? [] : extensionList.split(' ');
  // Otherwise EVERY extension present must be on the allowlist: one off-list
  // extension taints the whole mint and reverts pool creation.
  return extensions.every((e) => allowlist.has(e));
}
