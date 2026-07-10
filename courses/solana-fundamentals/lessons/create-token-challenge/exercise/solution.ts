import { Connection, Keypair } from '@solana/web3.js';
import { createMint } from '@solana/spl-token';

async function createTokenMint(
  connection: Connection,
  payer: Keypair
): Promise<string> {
  const mint = await createMint(
    connection,
    payer,
    payer.publicKey,
    payer.publicKey,
    9
  );

  return mint.toBase58();
}
