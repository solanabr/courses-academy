import { Connection, PublicKey, LAMPORTS_PER_SOL } from '@solana/web3.js';

interface WalletBalance {
  publicKey: string;
  balanceInSol: number;
}

async function checkMultipleBalances(
  connection: Connection,
  wallets: PublicKey[]
): Promise<WalletBalance[]> {
  // Your code here
}
