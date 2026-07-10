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
  // Encode amount as 8-byte little-endian u64
  const data = new Uint8Array(8);
  const view = new DataView(data.buffer);
  view.setBigUint64(0, BigInt(amount), true); // true = little-endian

  return {
    programId,
    keys: [
      { pubkey: payer, isSigner: true, isWritable: true },
      { pubkey: dataAccount, isSigner: false, isWritable: true },
    ],
    data,
  };
}
