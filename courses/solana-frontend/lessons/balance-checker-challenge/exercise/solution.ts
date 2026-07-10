import { Connection, PublicKey, LAMPORTS_PER_SOL } from '@solana/web3.js';

interface WalletBalance {
  publicKey: string;
  balanceInSol: number;
}

async function checkMultipleBalances(
  connection: Connection,
  wallets: PublicKey[]
): Promise<WalletBalance[]> {
  const balancePromises = wallets.map(async (wallet) => {
    const lamports = await connection.getBalance(wallet);
    const balanceInSol = parseFloat((lamports / LAMPORTS_PER_SOL).toFixed(4));
    
    return {
      publicKey: wallet.toBase58(),
      balanceInSol,
    };
  });

  return Promise.all(balancePromises);
}
