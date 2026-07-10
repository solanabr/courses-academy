import { PublicKey } from '@solana/web3.js';

function buildSIWSMessage(
  domain: string,
  publicKey: PublicKey,
  nonce: string,
  statement: string
): string {
  const issuedAt = new Date().toISOString();
  
  return `${domain} wants you to sign in with your Solana account:
${publicKey.toBase58()}

${statement}

URI: https://${domain}
Version: 1
Nonce: ${nonce}
Issued At: ${issuedAt}`;
}
