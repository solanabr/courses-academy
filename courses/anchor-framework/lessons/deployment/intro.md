# Deploying Solana Programs

Deploying to Solana is straightforward with Anchor, but there are important considerations for security and upgrades.

## Building Your Program

```bash
anchor build
```

This compiles your program and generates:
- `target/deploy/my_program.so` - The compiled program
- `target/idl/my_program.json` - The Interface Definition Language file
- `target/types/my_program.ts` - TypeScript types for your program

## Deploying to Devnet

1. **Configure your CLI for devnet**:

```bash
solana config set --url devnet
```

2. **Ensure you have SOL**:

```bash
solana balance
solana airdrop 2  # If needed
```

3. **Deploy**:

```bash
anchor deploy
```

Anchor will:
- Upload the program binary
- Create the program account
- Set you as the upgrade authority

## Understanding Program IDs

Your program ID is declared in `lib.rs`:

```rust
declare_id!("YourProgramIDHere");
```

This ID is generated when you run `anchor init` and stored in `target/deploy/my_program-keypair.json`.

**Important**: Once deployed, the program ID is permanent for that deployment. If you want a new ID, you need to create a new keypair.

## The IDL (Interface Definition Language)

The IDL is a JSON file describing your program's interface:

```json
{
  "version": "0.1.0",
  "name": "my_program",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        { "name": "myAccount", "isMut": true, "isSigner": true },
        { "name": "user", "isMut": true, "isSigner": true },
        { "name": "systemProgram", "isMut": false, "isSigner": false }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "MyAccount",
      "type": {
        "kind": "struct",
        "fields": [
          { "name": "data", "type": "u64" }
        ]
      }
    }
  ]
}
```

Clients use the IDL to interact with your program.

## Uploading the IDL

```bash
anchor idl init -f target/idl/my_program.json <PROGRAM_ID>
```

This stores the IDL on-chain, making it easier for tools like Anchor Explorer to understand your program.

Update an existing IDL:

```bash
anchor idl upgrade -f target/idl/my_program.json <PROGRAM_ID>
```

## Program Upgrades

Solana programs are **upgradeable** by default. The upgrade authority can replace the program code:

```bash
anchor upgrade target/deploy/my_program.so --program-id <PROGRAM_ID>
```

### Checking Upgrade Authority

```bash
solana program show <PROGRAM_ID>
```

Output:
```
Program Id: YourProgramID
Owner: BPFLoaderUpgradeab1e11111111111111111111111
ProgramData Address: ...
Authority: YourWalletPublicKey
Last Deployed In Slot: 123456789
Data Length: 200000 bytes
```

### Transferring Upgrade Authority

```bash
solana program set-upgrade-authority <PROGRAM_ID> --new-upgrade-authority <NEW_AUTHORITY>
```

### Revoking Upgrade Authority (Immutable Program)

For maximum security, you can make a program **immutable**:

```bash
solana program set-upgrade-authority <PROGRAM_ID> --final
```

**Warning**: This is irreversible! No one can ever upgrade the program again.

## Multisig Upgrade Authority

For production programs, use a multisig as the upgrade authority:

```bash
# Create a multisig with Squads Protocol
# Set multisig as upgrade authority
solana program set-upgrade-authority <PROGRAM_ID> --new-upgrade-authority <MULTISIG_ADDRESS>
```

This requires multiple team members to approve upgrades.

## Verifiable Builds

Verifiable builds let anyone verify your deployed program matches your source code:

```bash
anchor build --verifiable
```

This builds your program in a Docker container with a deterministic environment.

Verify a deployed program:

```bash
anchor verify <PROGRAM_ID>
```

## Deploying to Mainnet

1. **Switch to mainnet**:

```bash
solana config set --url mainnet-beta
```

2. **Fund your wallet** (deploying costs ~2-5 SOL depending on program size)

3. **Audit your code** (get a professional audit for production programs)

4. **Deploy**:

```bash
anchor deploy
```

5. **Upload IDL**:

```bash
anchor idl init -f target/idl/my_program.json <PROGRAM_ID>
```

6. **Set up multisig authority** or **revoke authority** if appropriate

## Best Practices

1. **Test on devnet first** - Always deploy and test on devnet before mainnet
2. **Use verifiable builds** - Builds trust with users
3. **Store keypairs securely** - Never commit `target/deploy/*-keypair.json` to Git
4. **Plan your upgrade strategy** - Multisig for most cases, immutable for critical infrastructure
5. **Monitor program accounts** - Track rent, data size, and usage
6. **Document your IDL** - Add comments to your Rust code; they appear in the IDL

## Common Deployment Issues

### Insufficient SOL

```
Error: Account <PROGRAM_ID> has insufficient funds
```

Solution: Airdrop more SOL (devnet) or fund your wallet (mainnet)

### Program Already Deployed

```
Error: Program already exists
```

Solution: Use `anchor upgrade` instead of `anchor deploy`

### Wrong Upgrade Authority

```
Error: Upgrade authority mismatch
```

Solution: Ensure the wallet you're using matches the upgrade authority

## Next Challenge

You'll write a test that simulates a program execution scenario.
