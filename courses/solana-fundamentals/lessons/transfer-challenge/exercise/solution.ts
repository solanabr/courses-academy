import {
  PublicKey,
  SystemProgram,
  Transaction,
  LAMPORTS_PER_SOL,
} from '@solana/web3.js';

function buildTransferTransaction(
  sender: PublicKey,
  recipient: PublicKey,
  amountInSol: number
): Transaction {
  const transaction = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: sender,
      toPubkey: recipient,
      lamports: amountInSol * LAMPORTS_PER_SOL,
    })
  );

  return transaction;
}
