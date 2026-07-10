import { Keypair } from '@solana/web3.js';

function generateKeypair(): { publicKey: string; isValid: boolean } {
  const keypair = Keypair.generate();
  return {
    publicKey: keypair.publicKey.toBase58(),
    isValid: true,
  };
}
