# React Patterns for Solana dApps

Building Solana frontends requires handling asynchronous blockchain operations gracefully. Let's explore essential React patterns for production-ready dApps.

## Loading States

Blockchain operations are async and can take seconds. Always show loading feedback:

```typescript
function BalanceDisplay() {
  const { publicKey } = useWallet();
  const { connection } = useConnection();
  const [balance, setBalance] = useState<number | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!publicKey) {
      setBalance(null);
      setLoading(false);
      return;
    }

    setLoading(true);
    connection.getBalance(publicKey)
      .then(lamports => setBalance(lamports / LAMPORTS_PER_SOL))
      .catch(err => console.error(err))
      .finally(() => setLoading(false));
  }, [publicKey, connection]);

  if (loading) return <Spinner />;
  if (!publicKey) return <p>Connect your wallet</p>;
  
  return <p>Balance: {balance?.toFixed(4)} SOL</p>;
}
```

## Error Handling

Network errors, rejected transactions, and insufficient funds are common. Handle them explicitly:

```typescript
function TransferForm() {
  const [error, setError] = useState<string | null>(null);
  const [sending, setSending] = useState(false);

  async function handleTransfer() {
    setError(null);
    setSending(true);

    try {
      const signature = await sendTransaction(transaction, connection);
      await connection.confirmTransaction(signature);
      toast.success(`Transfer successful! ${signature}`);
    } catch (err) {
      if (err instanceof Error) {
        if (err.message.includes('insufficient')) {
          setError('Insufficient SOL balance for this transaction');
        } else if (err.message.includes('rejected')) {
          setError('Transaction rejected by wallet');
        } else {
          setError('Transaction failed. Please try again.');
        }
      }
    } finally {
      setSending(false);
    }
  }

  return (
    <div>
      <button onClick={handleTransfer} disabled={sending}>
        {sending ? 'Sending...' : 'Send SOL'}
      </button>
      {error && <ErrorAlert>{error}</ErrorAlert>}
    </div>
  );
}
```

## Optimistic Updates

Update UI immediately while the transaction confirms in the background:

```typescript
function LikeButton({ postId }: { postId: string }) {
  const [likes, setLikes] = useState(42);
  const [hasLiked, setHasLiked] = useState(false);

  async function handleLike() {
    // Optimistic update
    setLikes(prev => prev + 1);
    setHasLiked(true);

    try {
      // Send transaction
      const signature = await sendLikeTransaction(postId);
      await connection.confirmTransaction(signature);
    } catch (err) {
      // Revert on error
      setLikes(prev => prev - 1);
      setHasLiked(false);
      toast.error('Failed to like post');
    }
  }

  return (
    <button onClick={handleLike} disabled={hasLiked}>
      {hasLiked ? 'Liked!' : 'Like'} ({likes})
    </button>
  );
}
```

## Data Fetching with React Query

`@tanstack/react-query` is perfect for caching blockchain data:

```typescript
import { useQuery } from '@tanstack/react-query';

function useWalletBalance(publicKey: PublicKey | null) {
  const { connection } = useConnection();

  return useQuery({
    queryKey: ['balance', publicKey?.toBase58()],
    queryFn: async () => {
      if (!publicKey) return null;
      const lamports = await connection.getBalance(publicKey);
      return lamports / LAMPORTS_PER_SOL;
    },
    enabled: !!publicKey,
    staleTime: 30_000, // Consider fresh for 30s
    refetchInterval: 60_000, // Auto-refresh every 60s
  });
}

// Usage
function BalanceCard() {
  const { publicKey } = useWallet();
  const { data: balance, isLoading, error } = useWalletBalance(publicKey);

  if (isLoading) return <Skeleton />;
  if (error) return <ErrorState />;
  
  return <p>{balance?.toFixed(4)} SOL</p>;
}
```

## Subscription Pattern for Real-Time Updates

Use WebSocket subscriptions for instant updates:

```typescript
function useAccountSubscription(publicKey: PublicKey | null) {
  const { connection } = useConnection();
  const [accountInfo, setAccountInfo] = useState<AccountInfo<Buffer> | null>(null);

  useEffect(() => {
    if (!publicKey) return;

    const subscriptionId = connection.onAccountChange(
      publicKey,
      (updatedAccountInfo) => {
        setAccountInfo(updatedAccountInfo);
      },
      'confirmed'
    );

    // Cleanup on unmount
    return () => {
      connection.removeAccountChangeListener(subscriptionId);
    };
  }, [publicKey, connection]);

  return accountInfo;
}
```

## Debouncing User Input

Validate addresses without overwhelming the RPC:

```typescript
import { useDebouncedValue } from '@/hooks/useDebouncedValue';

function RecipientInput() {
  const [address, setAddress] = useState('');
  const debouncedAddress = useDebouncedValue(address, 500);
  const [isValid, setIsValid] = useState<boolean | null>(null);

  useEffect(() => {
    if (!debouncedAddress) {
      setIsValid(null);
      return;
    }

    try {
      new PublicKey(debouncedAddress);
      setIsValid(true);
    } catch {
      setIsValid(false);
    }
  }, [debouncedAddress]);

  return (
    <div>
      <input
        value={address}
        onChange={(e) => setAddress(e.target.value)}
        placeholder="Recipient address"
      />
      {isValid === false && <Error>Invalid Solana address</Error>}
      {isValid === true && <Success>Valid address</Success>}
    </div>
  );
}
```

## Best Practices Summary

1. Always show loading states for async operations
2. Handle errors gracefully with user-friendly messages
3. Use optimistic updates for instant UX feedback
4. Cache blockchain data to reduce RPC calls
5. Debounce user input validation
6. Use WebSocket subscriptions for real-time updates
7. Clean up subscriptions on component unmount
