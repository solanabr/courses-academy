// Predict whether Raydium's CP-Swap pool-creation check will ACCEPT a mint.
//
// The grader calls isRoutable with three positional arguments: the same facts
// you would read off-chain before trying to seed a pool, flattened to scalars:
//   tokenProgram  - 'spl' (classic SPL Token) or 'token2022' (Token Extensions program)
//   extensionList - the Token-2022 extension type names present on the mint,
//                   space-separated in ONE string ('' means no extensions)
//   whitelisted   - true if the mint is in Raydium's static MINT_WHITELIST OR carries
//                   an initialized mint-association account (the documented bypasses)
//
// Return true if the pool WOULD be created, false if the check reverts.
//
// The starter below encodes the naive folklore model: "pools take classic SPL or
// extension-free mints, and reject anything with a Token-2022 extension." That is
// wrong: Raydium allowlists five specific extensions, and it bypasses the check for
// whitelisted mints. Rewrite isRoutable so it matches the source rule.

function isRoutable(tokenProgram: string, extensionList: string, whitelisted: boolean): boolean {
  // NAIVE: any Token-2022 extension is treated as disqualifying.
  return tokenProgram === 'spl' || extensionList === '';
}
