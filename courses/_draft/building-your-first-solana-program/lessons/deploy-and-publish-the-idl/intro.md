# Deploy and Publish the IDL

> **Version stamp — checked 2026-07-26.** `anchor-lang 1.1.2` · `borsh` resolves to **1.8.0** · Agave ≥ 3.1.10 · rustc 1.89+ · IDL spec 0.1.0 (Anchor 0.30+ shape).

This is it. The code block below is your vault as a deploy artifact: the Course 2 core, plus the three instructions that move lamports or create state — `initialize_vault`, `deposit`, `withdraw` — assembled into one file. (`transfer_between_vaults` from the last lesson is not in it, and the header comment says why.) Build it, then put it on a public network.

Two things come out of this lesson, and both of them are artifacts you keep:

1. **A program id.** A 32-byte address that anyone, anywhere, can look up without your permission and without asking you for anything.
2. **An IDL published on-chain at that id.** The machine-readable description of your instructions and accounts — which is what lets a client call your program having been told nothing but the id.

Course 4 consumes exactly those two. Keep the id somewhere you will find it again.

## The Deployment Process

Deploying a Solana program involves 4 steps:

1. **Compile**: Your Rust code → `.so` binary (done by the build server)
2. **Create Buffer**: A temporary on-chain account to hold the binary during upload
3. **Upload Chunks**: The binary is uploaded in ~1000-byte chunks (200+ transactions for a typical program)
4. **Finalize**: Link the buffer to a new program account, making it executable

This follows the **BPF Loader Upgradeable** protocol — the standard way all Anchor programs are deployed.

Your vault compiles to roughly 229 KB, so expect the full 200-plus transactions. The chunking is not a platform quirk: a Solana transaction is capped near 1232 bytes, which is three orders of magnitude smaller than the binary, so a large upload has no other shape available to it.

## What You'll See

- **1 wallet popup** to create the buffer account
- **Batch signing** every ~30 chunks (to avoid blockhash expiry)
- **A real-time transaction log** showing each chunk being confirmed
- **Your Program ID** when deployment completes

## Instructions

1. Click **Build** to compile the program
2. When the build succeeds, click **Deploy to Devnet**
3. Approve the transactions in your wallet
4. Watch the deployment progress in real time!

> **If deployment fails**: Don't worry! The system saves your progress. Click **Resume** to pick up where you left off — you won't re-upload chunks that already succeeded.

## Publishing the IDL: what changed in Anchor 1.0

Your program is on-chain. Nothing yet knows what its instructions are called.

An IDL is a JSON document describing every instruction, its 8-byte discriminator, the accounts it expects, the seeds of any PDA among them, your account layouts, and your error codes. It is the difference between an address a client can *see* and an address a client can *use*.

For years Anchor put that JSON on-chain through instructions built into your own program — the mechanism behind `anchor idl init`. **Anchor 1.0 removed those instructions.** Every tutorial that tells you to run `anchor idl init <PROGRAM_ID>` is describing a mechanism that no longer exists, and there are a great many of them.

One bridge before you write the verb off entirely: the Anchor **V2** release-candidate CLI reuses the `anchor idl` verb names — init, upgrade, fetch — reimplemented on top of the Program Metadata Program. If you meet them later in **Master Anchor V2**, that is the same PMP mechanism under the old names, not the removed legacy instructions coming back — and not a contradiction of this lesson.

The replacement is the **Program Metadata Program**, a separate on-chain program that stores metadata for *any* program, keyed by program id. It is not Anchor-specific and it is not part of your binary — which is the improvement: your program no longer carries instructions whose only purpose is to describe itself.

Publishing to it is one command:

```bash
npx @solana-program/program-metadata write idl <program-id> ./idl.json
```

In a local Anchor repo `anchor deploy` does this for you as part of deploying, and `--no-idl` opts out. Deployment here happens in the browser and does not, so publishing is the separate step above — run against the id the panel just gave you, with the IDL from this lesson.

Two honest notes:

- **The authority matters.** Writing metadata for a program id requires the program's upgrade authority, which is your wallet. Nobody else can publish an IDL for your program, and you cannot publish one for someone else's.
- **Explorer support is uneven.** Not every explorer resolves and renders Program Metadata IDLs yet; several still look only for the legacy account that Anchor 1.0 stopped writing. If an explorer shows your program without an IDL, that is a gap in the explorer, not a failed publish. Clients that read the metadata PDA directly — Codama among them, which is what Course 4 uses — get it either way.

The property worth internalising: **once published, the IDL is reachable from the program id alone.** No repo, no npm package, no README. Hand someone 32 bytes and they can generate a typed client. That is the whole reason the next course can start from your program instead of its own.

## Interacting with your live program

The panel below reads the IDL, derives every account it can, and builds real transactions against **your** deployed program. Work through it in order:

1. **`initialize_vault`** — no arguments. Watch the account list: `vault` resolves as a PDA, derived from `[b"vault", your_wallet]` under your program id, and `user` resolves to your wallet. Nothing was typed in; the seeds in the IDL were enough. After it confirms, the account view shows `owner` (your address), `balance` (0) and `bump` (probably 255, occasionally lower).
2. **`deposit`** — pass an amount in lamports. `1000000000` is 1 SOL; start smaller. Two things change and you should check both: the vault account's lamport balance on the explorer, and the `balance` field in the account view. They are separate writes — the System Program moved the lamports, your Course 2 `deposit` method recorded the number — and a program where they disagree is a program with a bug.
3. **`withdraw`** — pass a smaller amount than you deposited. Look at the account list before you send: there are **two** accounts, not three. No `system_program`, because `withdraw` makes no CPI — the account being debited is the vault, your program owns it, and a program may move lamports out of an account it owns without asking anyone. That absence in the account list is module 3's hardest finding made visible.
4. **Read the bytes.** Open the vault address on [Solana Explorer](https://explorer.solana.com) with `?cluster=devnet` and look at the raw account data. The first 8 bytes are the discriminator; then 32 bytes of `owner`; then `balance` as a little-endian `u64`, least significant byte first; then one byte of `bump`. 49 bytes, exactly the arithmetic you did in module 2, in front of you.

Then try two errors on purpose, because a live program is the only place you can watch your own error strings travel.

**Call `deposit` with `0`.** The transaction fails with custom error `6002`, and the message attached is the `#[msg("Amount must be greater than zero")]` string you wrote — travelling from your `require!` through the runtime into a UI that had never heard of your program before this lesson. That is what a named error buys you, and it is why Course 2 refused to let you `unwrap`.

**Then call `withdraw` for more than the vault holds** — or for an amount that would drop it below its rent floor. You get `6001`, `InsufficientFunds`, from your own guard, rather than the runtime's `InsufficientFundsForRent`. Both stop the transaction. Only one of them tells the caller what to do about it, and the difference is the rent-floor check you wrote instead of letting the runtime find out the hard way.
