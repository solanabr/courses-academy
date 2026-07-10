import { Keypair, PublicKey } from '@solana/web3.js';

function createMockAccount(
  owner: PublicKey,
  lamports: number,
  data: Uint8Array,
  executable: boolean
): {
  publicKey: PublicKey;
  owner: PublicKey;
  lamports: number;
  data: Uint8Array;
  executable: boolean;
  rentEpoch: number;
} {
  // Your code here
}
