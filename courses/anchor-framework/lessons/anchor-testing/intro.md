# Testing Anchor Programs

Testing is critical for Solana programs. Bugs in smart contracts can lead to loss of funds, so comprehensive testing is non-negotiable.

## Anchor Test Setup

Anchor projects come with a `tests/` folder and pre-configured testing:

```bash
anchor test
```

This command:
1. Builds your program
2. Deploys to a local validator
3. Runs your TypeScript tests
4. Shuts down the validator

## Basic Test Structure

```typescript
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { MyProgram } from '../target/types/my_program';
import { expect } from 'chai';

describe('my-program', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MyProgram as Program<MyProgram>;

  it('Initializes the program', async () => {
    const myAccount = anchor.web3.Keypair.generate();

    await program.methods
      .initialize()
      .accounts({
        myAccount: myAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([myAccount])
      .rpc();

    const account = await program.account.myAccount.fetch(
      myAccount.publicKey
    );

    expect(account.data.toNumber()).to.equal(0);
  });
});
```

## Bankrun: Fast Local Testing

Bankrun is a lightweight Solana test validator that runs in-process:

```typescript
import { BankrunProvider } from 'anchor-bankrun';
import { startAnchor } from 'solana-bankrun';

describe('fast tests', () => {
  let context;
  let provider;
  let program;

  before(async () => {
    context = await startAnchor(
      '',
      [{ name: 'my_program', programId: PROGRAM_ID }],
      []
    );

    provider = new BankrunProvider(context);
    program = new Program<MyProgram>(IDL, provider);
  });

  it('runs 10x faster', async () => {
    // Your test
  });
});
```

Bankrun is **much faster** than spinning up a full validator for each test.

## Testing Patterns

### 1. Test Fixtures

```typescript
const createUser = async () => {
  const user = anchor.web3.Keypair.generate();

  // Airdrop SOL
  await provider.connection.requestAirdrop(
    user.publicKey,
    2 * anchor.web3.LAMPORTS_PER_SOL
  );

  return user;
};

const createTokenAccount = async (owner, mint) => {
  // Create token account for testing
};
```

### 2. Account Assertions

```typescript
const account = await program.account.userAccount.fetch(userPDA);

expect(account.authority.toBase58()).to.equal(
  user.publicKey.toBase58()
);
expect(account.balance.toNumber()).to.equal(1000);
expect(account.isActive).to.be.true;
```

### 3. Error Testing

```typescript
try {
  await program.methods
    .withdraw(new anchor.BN(10000))
    .accounts({ user: userPDA })
    .rpc();

  // Should not reach here
  expect.fail('Expected error was not thrown');
} catch (err) {
  expect(err.error.errorCode.code).to.equal('InsufficientFunds');
}
```

### 4. Event Testing

Anchor can emit events:

```rust
#[event]
pub struct TransferEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}

emit!(TransferEvent {
    from: ctx.accounts.from.key(),
    to: ctx.accounts.to.key(),
    amount,
});
```

Test events:

```typescript
const listener = program.addEventListener('TransferEvent', (event) => {
  console.log('Transfer:', event);
  expect(event.amount.toNumber()).to.equal(100);
});

await program.methods.transfer(new anchor.BN(100)).rpc();

program.removeEventListener(listener);
```

## Test Organization

```
tests/
├── setup.ts              # Shared setup logic
├── fixtures.ts           # Test data generators
├── initialize.test.ts    # Initialization tests
├── transfer.test.ts      # Transfer logic tests
└── errors.test.ts        # Error condition tests
```

## Mocking Accounts

For unit-testing complex logic:

```typescript
const mockAccount = {
  authority: user.publicKey,
  balance: new anchor.BN(1000),
  lastUpdate: new anchor.BN(Date.now() / 1000),
};

// Serialize and write to test validator
const accountData = program.coder.accounts.encode(
  'UserAccount',
  mockAccount
);
```

## Best Practices

1. **Test happy path first**, then edge cases
2. **Test all error conditions** (unauthorized access, insufficient funds, etc.)
3. **Use descriptive test names**: `it('rejects withdrawal when balance is insufficient')`
4. **Clean up state** between tests (or use Bankrun snapshots)
5. **Test with realistic data** (not just 0s and 1s)
6. **Measure code coverage** (use `solana-program-test` for Rust unit tests)

## Continuous Integration

```yaml
# .github/workflows/test.yml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - name: Install Solana
        run: sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
      - name: Install Anchor
        run: cargo install --git https://github.com/coral-xyz/anchor avm --force
      - name: Run tests
        run: anchor test
```

## Next Lesson

You'll learn how to define custom errors in Anchor programs.
