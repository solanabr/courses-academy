# Sign-In With Solana (SIWS)

Sign-In With Solana (SIWS) is a standard for authenticating users using their Solana wallet, similar to "Sign-In With Ethereum" (SIWE).

## Why Message Signing?

Traditional web apps use passwords. Web3 apps use **cryptographic signatures**:

1. Server generates a unique message
2. User signs the message with their wallet
3. Server verifies the signature matches the public key
4. User is authenticated without ever sharing a password

**Benefits**:
- No password management
- Wallet handles all cryptography
- Cannot be phished (signature is specific to the message)
- Works across all Solana dApps

## SIWS Message Format

A SIWS message typically includes:

```
academy.courses wants you to sign in with your Solana account:
7C4jsPZpht42Tw6MjXWF56Q5RQUocjBBmciEjDa8HRtp

Sign in to Superteam Academy to access your courses and track your progress.

URI: https://academy.courses
Version: 1
Nonce: k3j2n4k5j6h7g8f9d
Issued At: 2025-02-09T15:30:00.000Z
Expiration Time: 2025-02-09T15:45:00.000Z
```

**Key Components**:
- **Domain**: Which site is requesting the signature
- **Address**: User's public key
- **Statement**: Human-readable purpose
- **Nonce**: Prevents replay attacks (must be unique)
- **Issued At/Expiration**: Time-bound authentication

## Signing Flow

```typescript
import { useWallet } from '@solana/wallet-adapter-react';
import bs58 from 'bs58';

async function signInWithSolana() {
  const { publicKey, signMessage } = useWallet();
  if (!publicKey || !signMessage) throw new Error('Wallet not connected');

  // 1. Get nonce from server
  const { nonce } = await fetch('/api/auth/nonce').then(r => r.json());

  // 2. Build message
  const message = `
academy.courses wants you to sign in with your Solana account:
${publicKey.toBase58()}

Sign in to Superteam Academy.

Nonce: ${nonce}
Issued At: ${new Date().toISOString()}
  `.trim();

  // 3. Sign message
  const encodedMessage = new TextEncoder().encode(message);
  const signature = await signMessage(encodedMessage);

  // 4. Send to server for verification
  await fetch('/api/auth/verify', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      message,
      signature: bs58.encode(signature),
      publicKey: publicKey.toBase58(),
    }),
  });
}
```

## Server-Side Verification

The server verifies the signature using the Ed25519 algorithm:

```typescript
import { PublicKey } from '@solana/web3.js';
import nacl from 'tweetnacl';
import bs58 from 'bs58';

function verifySignature(
  message: string,
  signature: string,
  publicKey: string
): boolean {
  const messageBytes = new TextEncoder().encode(message);
  const signatureBytes = bs58.decode(signature);
  const publicKeyBytes = new PublicKey(publicKey).toBytes();

  return nacl.sign.detached.verify(
    messageBytes,
    signatureBytes,
    publicKeyBytes
  );
}
```

## Security Considerations

### Nonce Management
- Generate cryptographically random nonces (use `crypto.randomBytes(32)`)
- Store nonces in Redis/memory with TTL (5-15 minutes)
- Delete nonce after successful verification (prevent replay)

### Message Validation
- Verify message format matches expected structure
- Check `Issued At` is recent (within 5 minutes)
- Verify domain matches your application
- Ensure `Expiration Time` hasn't passed

### Example: Nonce Replay Attack

Without nonce verification:
```
Attacker intercepts a valid signature
Attacker replays it multiple times
Server accepts each replay as valid ❌
```

With nonce verification:
```
Attacker intercepts a valid signature
Server marks nonce as used
Attacker replays signature
Server rejects (nonce already used) ✅
```

## Real-World Implementation

Most Solana dApps use SIWS for:
- Gating premium content (Dialect, Squads)
- Personalized dashboards (Jupiter, Raydium)
- User settings and preferences
- Subscription verification

## Next Steps

In the next challenge, you'll construct a SIWS message with all required fields.
