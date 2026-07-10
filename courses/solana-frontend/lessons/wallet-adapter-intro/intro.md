# Solana Wallet Adapter Architecture

The Solana Wallet Adapter is the standard library for integrating wallet connections in Solana dApps. It provides a unified interface for multiple wallet providers.

## Why Wallet Adapter?

Without Wallet Adapter, you would need to write custom integration code for each wallet:
- Phantom has its own API
- Solflare has different methods
- Backpack uses a different connection flow

Wallet Adapter **abstracts all of this** into a single, consistent API.

## Core Architecture

```
┌──────────────────────┐
│   Your React App     │
└──────────┬───────────┘
           │
┌──────────▼───────────┐
│  Wallet Adapter      │  ← Hooks: useWallet, useConnection
│  Context Provider    │
└──────────┬───────────┘
           │
    ┌──────┴──────┐
    │             │
┌───▼────┐   ┌───▼────┐   ┌─────────┐
│Phantom │   │Solflare│   │Backpack │  ← Wallet Implementations
└────────┘   └────────┘   └─────────┘
```

## Setting Up the Provider

```typescript
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import { PhantomWalletAdapter, SolflareWalletAdapter } from '@solana/wallet-adapter-wallets';
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import { clusterApiUrl } from '@solana/web3.js';

function App() {
  const network = WalletAdapterNetwork.Devnet;
  const endpoint = clusterApiUrl(network);
  
  const wallets = [
    new PhantomWalletAdapter(),
    new SolflareWalletAdapter({ network }),
  ];

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <YourApp />
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}
```

## Key Hooks

### useWallet()

Access wallet state and methods:

```typescript
import { useWallet } from '@solana/wallet-adapter-react';

function ConnectButton() {
  const { publicKey, connected, connect, disconnect, signMessage } = useWallet();

  if (connected && publicKey) {
    return (
      <div>
        <p>Connected: {publicKey.toBase58().slice(0, 8)}...</p>
        <button onClick={disconnect}>Disconnect</button>
      </div>
    );
  }

  return <button onClick={connect}>Connect Wallet</button>;
}
```

### useConnection()

Access the RPC connection:

```typescript
import { useConnection } from '@solana/wallet-adapter-react';
import { LAMPORTS_PER_SOL } from '@solana/web3.js';

function BalanceDisplay() {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const [balance, setBalance] = useState(0);

  useEffect(() => {
    if (!publicKey) return;

    connection.getBalance(publicKey).then(lamports => {
      setBalance(lamports / LAMPORTS_PER_SOL);
    });
  }, [publicKey, connection]);

  return <p>Balance: {balance.toFixed(2)} SOL</p>;
}
```

## Supported Wallets

The adapter supports all major Solana wallets:
- **Phantom**: Most popular, mobile + extension
- **Solflare**: Advanced features, built-in swap
- **Backpack**: xNFT platform, mobile-first
- **Ledger**: Hardware wallet support
- **Glow**: Focused on validator staking

## Auto-Connect and Persistence

The `autoConnect` prop attempts to reconnect on page load:

```typescript
<WalletProvider wallets={wallets} autoConnect>
  {/* ... */}
</WalletProvider>
```

Wallet Adapter automatically saves the last connected wallet to localStorage and restores it on refresh.

## Next Steps

In the next challenge, you'll implement a wallet connection flow and extract the user's public key.
