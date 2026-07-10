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
  // Validate recipient address
  let recipientPublicKey: PublicKey;
  try {
    recipientPublicKey = new PublicKey(recipientAddress);
  } catch {
    return { success: false, error: 'Invalid recipient address' };
  }

  // Validate amount
  if (amountInSol <= 0) {
    return { success: false, error: 'Amount must be positive' };
  }

  // Check sender balance
  const senderBalance = await connection.getBalance(senderPublicKey);
  const requiredLamports = amountInSol * LAMPORTS_PER_SOL;

  if (senderBalance < requiredLamports) {
    return { success: false, error: 'Insufficient balance' };
  }

  // Build transaction
  const transaction = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: senderPublicKey,
      toPubkey: recipientPublicKey,
      lamports: requiredLamports,
    })
  );

  return { success: true, transaction };
}
