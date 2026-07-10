import { PublicKey } from '@solana/web3.js';

interface AccountMeta {
  pubkey: PublicKey;
  isSigner: boolean;
  isWritable: boolean;
}

interface Instruction {
  programId: PublicKey;
  keys: AccountMeta[];
  data: Uint8Array;
}

function buildCustomInstruction(
  programId: PublicKey,
  payer: PublicKey,
  dataAccount: PublicKey,
  amount: number
): Instruction {
  // Your code here
}
