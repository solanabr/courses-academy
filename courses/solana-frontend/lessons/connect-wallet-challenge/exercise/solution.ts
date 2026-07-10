import { Keypair, PublicKey } from '@solana/web3.js';

function simulateWalletConnection(): {
  publicKey: string;
  connected: boolean;
  isValid: boolean;
} {
  const keypair = Keypair.generate();
  const publicKeyStr = keypair.publicKey.toBase58();
  
  return {
    publicKey: publicKeyStr,
    connected: true,
    isValid: PublicKey.isOnCurve(keypair.publicKey.toBytes()),
  };
}
