# Parsing Account Data

Account data on Solana is stored as raw bytes (`Uint8Array`). To use this data in your application, you must **deserialize** it into structured JavaScript objects.

## Why Binary Encoding?

Solana stores data as compact binary to:
- **Minimize storage costs** (rent is based on account size)
- **Maximize throughput** (smaller data = faster network transmission)
- **Enable cross-language compatibility** (Rust programs, JS clients, Python bots all read the same bytes)

## Borsh Encoding Standard

Most Solana programs use **Borsh** (Binary Object Representation Serializer for Hashing):

```typescript
import { serialize, deserialize, Schema } from 'borsh';

// Define schema
class UserProfile {
  username: string = '';
  level: number = 0;
  xp: bigint = 0n;

  constructor(fields: { username: string; level: number; xp: bigint }) {
    Object.assign(this, fields);
  }
}

const schema: Schema = {
  struct: {
    username: 'string',
    level: 'u8',
    xp: 'u64',
  },
};

// Deserialize
const accountData = await connection.getAccountInfo(profilePubkey);
const profile = deserialize(schema, UserProfile, accountData.data);

console.log(profile.username); // "SolDev123"
console.log(profile.level);    // 5
console.log(profile.xp);       // 1200n
```

## Manual Parsing with DataView

For simple data structures, you can parse manually:

```typescript
function parseTokenAccount(data: Uint8Array) {
  const view = new DataView(data.buffer);

  return {
    mint: new PublicKey(data.slice(0, 32)),
    owner: new PublicKey(data.slice(32, 64)),
    amount: view.getBigUint64(64, true), // little-endian
    delegateOption: view.getUint32(72, true),
    state: view.getUint8(108),
  };
}
```

## Common Data Types

| Borsh Type | JavaScript | Bytes | Range |
|------------|------------|-------|-------|
| `u8` | number | 1 | 0 to 255 |
| `u32` | number | 4 | 0 to 4,294,967,295 |
| `u64` | bigint | 8 | 0 to 2^64-1 |
| `string` | string | variable | Length prefix + UTF-8 |
| `[u8; 32]` | Uint8Array | 32 | Fixed-size byte array |
| `Vec<T>` | Array<T> | variable | Length prefix + items |

## String Encoding

Strings are encoded as:
1. **4-byte length prefix** (u32, little-endian)
2. **UTF-8 bytes**

```typescript
function parseString(data: Uint8Array, offset: number): [string, number] {
  const view = new DataView(data.buffer);
  const length = view.getUint32(offset, true); // little-endian
  
  const stringBytes = data.slice(offset + 4, offset + 4 + length);
  const str = new TextDecoder().decode(stringBytes);
  
  return [str, offset + 4 + length]; // [parsed string, new offset]
}
```

## Real-World Example: Metaplex NFT Metadata

Metaplex stores NFT metadata on-chain:

```typescript
import { Metadata } from '@metaplex-foundation/mpl-token-metadata';

const metadataPDA = await Metadata.getPDA(mintPublicKey);
const accountInfo = await connection.getAccountInfo(metadataPDA);

const metadata = Metadata.deserialize(accountInfo.data);

console.log('Name:', metadata[0].data.name);
console.log('Symbol:', metadata[0].data.symbol);
console.log('URI:', metadata[0].data.uri);
```

## Anchor Discriminators

Anchor programs use an 8-byte **discriminator** at the start of each account:

```typescript
const discriminator = data.slice(0, 8);
// Hash of "account:YourAccountName"
```

This helps identify the account type. Anchor's TypeScript client automatically handles discriminators.

## Debugging Tips

### View Raw Bytes

```typescript
console.log('Raw bytes:', Array.from(data).map(b => b.toString(16).padStart(2, '0')).join(' '));
// 01 00 00 00 0a 00 00 00 48 65 6c 6c 6f ...
```

### Use Solana Explorer

Paste your account address into [Solana Explorer](https://explorer.solana.com) → "Account Data" tab to view hex dump.

## Best Practices

1. **Use existing parsers** when available (Metaplex, SPL Token, Anchor)
2. **Handle endianness** consistently (Solana uses little-endian)
3. **Validate data length** before parsing to avoid out-of-bounds errors
4. **Cache parsed data** (parsing is CPU-intensive; don't re-parse on every render)

## Next Steps

In the next challenge, you'll manually parse a binary data buffer into structured fields.
