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
  // Your code here
}
