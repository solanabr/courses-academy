import {
  Connection,
  PublicKey,
  SystemProgram,
  Transaction,
  LAMPORTS_PER_SOL,
} from '@solana/web3.js';

interface ValidationResult {
  success: boolean;
  error?: string;
  transaction?: Transaction;
}

async function validateAndBuildTransfer(
  senderPublicKey: PublicKey,
  recipientAddress: string,
  amountInSol: number,
  connection: Connection
): Promise<ValidationResult> {
  // Your code here
}
