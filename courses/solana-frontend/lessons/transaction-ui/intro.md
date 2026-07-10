# Transaction UI Patterns

Sending transactions is the core interaction in any Solana dApp. Great UX requires clear transaction flows, status tracking, and helpful feedback.

## Multi-Step Confirmation Flow

Break complex transactions into clear steps:

```typescript
enum TxStep {
  IDLE = 'idle',
  CONFIRMING = 'confirming',
  SIGNING = 'signing',
  SENDING = 'sending',
  CONFIRMING_TX = 'confirming_tx',
  SUCCESS = 'success',
  ERROR = 'error',
}

function SwapInterface() {
  const [step, setStep] = useState<TxStep>(TxStep.IDLE);
  const [signature, setSignature] = useState<string>('');

  async function handleSwap() {
    try {
      setStep(TxStep.CONFIRMING);
      // User reviews swap details

      setStep(TxStep.SIGNING);
      const { signature } = await sendTransaction(tx, connection);
      setSignature(signature);

      setStep(TxStep.SENDING);
      // Transaction sent to network

      setStep(TxStep.CONFIRMING_TX);
      await connection.confirmTransaction(signature, 'confirmed');

      setStep(TxStep.SUCCESS);
    } catch (err) {
      setStep(TxStep.ERROR);
    }
  }

  return (
    <div>
      {step === TxStep.CONFIRMING && <ReviewModal />}
      {step === TxStep.SIGNING && <WalletSigningPrompt />}
      {step === TxStep.SENDING && <SendingAnimation />}
      {step === TxStep.CONFIRMING_TX && <ConfirmingAnimation signature={signature} />}
      {step === TxStep.SUCCESS && <SuccessMessage signature={signature} />}
      {step === TxStep.ERROR && <ErrorMessage />}
    </div>
  );
}
```

## Status Indicators

Show clear visual feedback for each stage:

```typescript
function TransactionStatus({ step }: { step: TxStep }) {
  const stages = [
    { label: 'Review', step: TxStep.CONFIRMING },
    { label: 'Sign', step: TxStep.SIGNING },
    { label: 'Send', step: TxStep.SENDING },
    { label: 'Confirm', step: TxStep.CONFIRMING_TX },
  ];

  return (
    <div className="flex gap-2">
      {stages.map((stage, idx) => (
        <div key={idx} className="flex items-center">
          <div className={cn(
            "w-8 h-8 rounded-full flex items-center justify-center",
            step === stage.step && "bg-purple-600 text-white",
            idx < stages.findIndex(s => s.step === step) && "bg-green-600",
          )}>
            {idx < stages.findIndex(s => s.step === step) ? '✓' : idx + 1}
          </div>
          <span className="ml-2">{stage.label}</span>
        </div>
      ))}
    </div>
  );
}
```

## Explorer Links

Always provide links to view transactions on-chain:

```typescript
function ExplorerLink({
  signature,
  cluster = 'mainnet-beta',
}: {
  signature: string;
  cluster?: string;
}) {
  const url = `https://explorer.solana.com/tx/${signature}${cluster !== 'mainnet-beta' ? `?cluster=${cluster}` : ''}`;

  return (
    <a
      href={url}
      target="_blank"
      rel="noopener noreferrer"
      className="text-purple-600 hover:underline flex items-center gap-1"
    >
      View on Solana Explorer
      <ExternalLinkIcon className="w-4 h-4" />
    </a>
  );
}
```

## Estimated Cost Display

Show users the transaction cost before signing:

```typescript
async function estimateTransactionCost(
  transaction: Transaction,
  connection: Connection
): Promise<number> {
  const { blockhash } = await connection.getLatestBlockhash();
  transaction.recentBlockhash = blockhash;
  
  // Estimate fee
  const fee = await transaction.getEstimatedFee(connection);
  return fee || 5000; // Default fallback
}

function TransactionPreview({ transaction }: { transaction: Transaction }) {
  const { connection } = useConnection();
  const [fee, setFee] = useState<number | null>(null);

  useEffect(() => {
    estimateTransactionCost(transaction, connection).then(setFee);
  }, [transaction, connection]);

  return (
    <div className="bg-gray-100 p-4 rounded">
      <p className="text-sm text-gray-600">Estimated network fee</p>
      <p className="text-lg font-bold">
        {fee ? `${(fee / LAMPORTS_PER_SOL).toFixed(6)} SOL` : 'Calculating...'}
      </p>
    </div>
  );
}
```

## Retry Logic

Handle failed transactions gracefully:

```typescript
function useTransactionSender() {
  const { connection } = useConnection();
  const { sendTransaction } = useWallet();
  const [attempts, setAttempts] = useState(0);

  async function sendWithRetry(
    transaction: Transaction,
    maxRetries = 3
  ): Promise<string> {
    for (let i = 0; i < maxRetries; i++) {
      setAttempts(i + 1);

      try {
        const signature = await sendTransaction(transaction, connection);
        await connection.confirmTransaction(signature, 'confirmed');
        return signature;
      } catch (err) {
        if (i === maxRetries - 1) throw err;
        
        // Wait before retry (exponential backoff)
        await new Promise(resolve => setTimeout(resolve, 1000 * Math.pow(2, i)));
      }
    }

    throw new Error('Transaction failed after retries');
  }

  return { sendWithRetry, attempts };
}
```

## Transaction History

Show users their past transactions:

```typescript
function TransactionHistory() {
  const { publicKey } = useWallet();
  const { connection } = useConnection();
  const [txs, setTxs] = useState<ParsedTransactionWithMeta[]>([]);

  useEffect(() => {
    if (!publicKey) return;

    connection.getSignaturesForAddress(publicKey, { limit: 10 })
      .then(async (sigs) => {
        const txPromises = sigs.map(sig => 
          connection.getParsedTransaction(sig.signature, {
            maxSupportedTransactionVersion: 0,
          })
        );
        const txs = await Promise.all(txPromises);
        setTxs(txs.filter(tx => tx !== null));
      });
  }, [publicKey, connection]);

  return (
    <div>
      {txs.map((tx, idx) => (
        <TransactionCard key={idx} tx={tx} />
      ))}
    </div>
  );
}
```

## Real-World Examples

### Jupiter Swap Flow
1. Review swap (token amounts, slippage, price impact)
2. Approve transaction in wallet
3. Show "Swapping..." animation with explorer link
4. Confirm on-chain (with retry logic)
5. Show success with new token balance

### Magic Eden NFT Purchase
1. Show NFT details and price
2. Check buyer has sufficient SOL
3. Request wallet signature
4. Stream transaction status updates
5. Display purchase confirmation with NFT preview

## Best Practices

1. Always show transaction cost before signing
2. Provide explorer links for every transaction
3. Use optimistic updates for instant feedback
4. Implement retry logic for network hiccups
5. Clear error messages (insufficient funds, rejected, timeout)
6. Show multi-step progress for complex flows
