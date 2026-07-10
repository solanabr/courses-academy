# Transaction Notifications

Real-time notifications keep users informed about transaction status without requiring them to poll or refresh. Let's build a robust notification system.

## Toast Notifications

Toasts are perfect for transaction feedback. Use libraries like `react-hot-toast` or `sonner`:

```typescript
import toast from 'react-hot-toast';
import { ExternalLink } from 'lucide-react';

function useTransactionToast() {
  const cluster = 'mainnet-beta';

  function toastSignature(signature: string) {
    const explorerUrl = `https://explorer.solana.com/tx/${signature}?cluster=${cluster}`;
    
    toast.success(
      <div className="flex items-center gap-2">
        <span>Transaction confirmed!</span>
        <a
          href={explorerUrl}
          target="_blank"
          rel="noopener noreferrer"
          className="text-purple-600 hover:underline flex items-center gap-1"
        >
          View <ExternalLink className="w-3 h-3" />
        </a>
      </div>,
      { duration: 8000 }
    );
  }

  function toastError(message: string) {
    toast.error(message, { duration: 5000 });
  }

  return { toastSignature, toastError };
}

// Usage
async function handleTransfer() {
  const { toastSignature, toastError } = useTransactionToast();

  try {
    toast.loading('Sending transaction...');
    const signature = await sendTransaction(tx, connection);
    
    toast.loading('Confirming...');
    await connection.confirmTransaction(signature);
    
    toast.dismiss();
    toastSignature(signature);
  } catch (err) {
    toast.dismiss();
    toastError('Transaction failed. Please try again.');
  }
}
```

## WebSocket Subscriptions for Real-Time Updates

Monitor accounts for changes without polling:

```typescript
function useAccountChangeNotifications(publicKey: PublicKey | null) {
  const { connection } = useConnection();
  const previousBalance = useRef<number | null>(null);

  useEffect(() => {
    if (!publicKey) return;

    const subscriptionId = connection.onAccountChange(
      publicKey,
      (accountInfo) => {
        const newBalance = accountInfo.lamports / LAMPORTS_PER_SOL;

        if (previousBalance.current !== null) {
          const change = newBalance - previousBalance.current;

          if (change > 0) {
            toast.success(`Received ${change.toFixed(4)} SOL`);
          } else if (change < 0) {
            toast.info(`Sent ${Math.abs(change).toFixed(4)} SOL`);
          }
        }

        previousBalance.current = newBalance;
      },
      'confirmed'
    );

    return () => {
      connection.removeAccountChangeListener(subscriptionId);
    };
  }, [publicKey, connection]);
}
```

## Transaction Status Polling

For critical operations, poll transaction status:

```typescript
async function waitForConfirmation(
  connection: Connection,
  signature: string,
  commitment: Commitment = 'confirmed',
  timeout = 60000
): Promise<boolean> {
  const start = Date.now();

  while (Date.now() - start < timeout) {
    const status = await connection.getSignatureStatus(signature);

    if (status.value?.confirmationStatus === commitment) {
      return true;
    }

    if (status.value?.err) {
      throw new Error('Transaction failed');
    }

    // Wait 1s before next check
    await new Promise(resolve => setTimeout(resolve, 1000));
  }

  throw new Error('Transaction confirmation timeout');
}

// Usage with notifications
async function sendWithNotifications(tx: Transaction) {
  const toastId = toast.loading('Sending transaction...');

  try {
    const signature = await sendTransaction(tx, connection);
    
    toast.loading('Confirming on-chain...', { id: toastId });
    
    await waitForConfirmation(connection, signature);
    
    toast.success(
      <span>
        Transaction confirmed!
        <a href={`https://explorer.solana.com/tx/${signature}`}>View</a>
      </span>,
      { id: toastId }
    );
  } catch (err) {
    toast.error('Transaction failed', { id: toastId });
  }
}
```

## Notification Permission (Browser Notifications)

For background notifications when tab isn't focused:

```typescript
function useNotificationPermission() {
  const [permission, setPermission] = useState<NotificationPermission>(
    typeof Notification !== 'undefined' ? Notification.permission : 'default'
  );

  async function requestPermission() {
    if (typeof Notification === 'undefined') return;
    
    const result = await Notification.requestPermission();
    setPermission(result);
  }

  function notify(title: string, body: string) {
    if (permission !== 'granted') return;
    
    new Notification(title, {
      body,
      icon: '/solana-logo.png',
      badge: '/notification-badge.png',
    });
  }

  return { permission, requestPermission, notify };
}

// Usage
function TransactionMonitor() {
  const { notify } = useNotificationPermission();
  const { publicKey } = useWallet();
  const { connection } = useConnection();

  useEffect(() => {
    if (!publicKey) return;

    const sub = connection.onAccountChange(
      publicKey,
      () => {
        if (!document.hasFocus()) {
          notify('Solana Transaction', 'Your wallet balance changed');
        }
      },
      'confirmed'
    );

    return () => connection.removeAccountChangeListener(sub);
  }, [publicKey, connection, notify]);

  return null;
}
```

## Notification Sound

Add audio feedback for important events:

```typescript
function useTransactionSound() {
  const successSound = useRef<HTMLAudioElement | null>(null);
  const errorSound = useRef<HTMLAudioElement | null>(null);

  useEffect(() => {
    successSound.current = new Audio('/sounds/success.mp3');
    errorSound.current = new Audio('/sounds/error.mp3');
  }, []);

  function playSuccess() {
    successSound.current?.play();
  }

  function playError() {
    errorSound.current?.play();
  }

  return { playSuccess, playError };
}
```

## Notification Queue

Manage multiple notifications without overwhelming the UI:

```typescript
function NotificationQueue() {
  const [queue, setQueue] = useState<Notification[]>([]);
  const maxVisible = 3;

  function addNotification(notification: Notification) {
    setQueue(prev => [...prev, notification]);

    setTimeout(() => {
      setQueue(prev => prev.filter(n => n.id !== notification.id));
    }, notification.duration || 5000);
  }

  return (
    <div className="fixed bottom-4 right-4 space-y-2 z-50">
      {queue.slice(-maxVisible).map(notification => (
        <NotificationCard key={notification.id} {...notification} />
      ))}
    </div>
  );
}
```

## Real-World Examples

### Phantom Wallet
- Shows in-app toast when transaction is signed
- Browser notification when transaction confirms (if tab not focused)
- Sound effect on success/failure

### Jupiter Aggregator
- Loading toast while routing
- Progress indicator during swap
- Success toast with explorer link
- Error toast with retry button

## Best Practices

1. Use toast notifications for transaction updates
2. Include explorer links in success notifications
3. Implement WebSocket subscriptions for real-time balance changes
4. Request browser notification permission for background updates
5. Add sound effects for important events (success, error)
6. Limit visible notifications to avoid UI clutter
7. Auto-dismiss notifications after 5-8 seconds
