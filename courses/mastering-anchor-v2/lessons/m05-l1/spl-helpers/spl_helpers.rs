// tests/spl_helpers.rs — the thin SPL setup wrapper m05-l1's fixture leans on.
//
// Drop this file in at `programs/quarter-vault/tests/spl_helpers.rs`. You never
// declare `mod spl_helpers;` yourself: `tests/spl_setup.rs` pulls it in by
// `#[path]`, because cargo compiles every `tests/*.rs` as its own crate root and
// a sibling `mod` would not resolve otherwise (the lesson has the full note).
//
// Everything here is ordinary SPL client code with nothing V2-specific in it:
// four helpers that stand up a mint, an ATA, and a balance, using the same two
// crates the lesson's `cargo add` line installed (`spl-token@9` and
// `spl-associated-token-account@8`). The imports ride `anchor_lang` and
// `anchor_v2_testing` and reach past neither — `system_instruction` comes
// through anchor_lang's v1-compatible re-export, so no solana crate is named
// that the Cargo.toml never declared.
#![allow(dead_code)]

use anchor_lang::{
    prelude::Address,
    solana_program::{instruction::Instruction, system_instruction},
};
use anchor_v2_testing::{
    Keypair, LiteSVM, Message, Signer, VersionedMessage, VersionedTransaction,
};
// Mint::LEN lives on the Pack trait; spl_token re-exports it for exactly this use.
use spl_token::solana_program::program_pack::Pack;

/// Create a new SPL mint with `authority` as its mint authority, no freeze
/// authority, and the given decimals. Returns the mint's address.
pub fn create_mint(svm: &mut LiteSVM, authority: &Keypair, decimals: u8) -> Address {
    let mint = Keypair::new();

    // Two instructions, the classic pair: the System Program allocates the
    // 82-byte mint account and assigns it to the token program, then
    // initialize_mint2 writes the mint state into it. The new account signs its
    // own creation, which is why `mint` rides along as a second signer.
    let space = spl_token::state::Mint::LEN;
    let create = system_instruction::create_account(
        &authority.pubkey(),
        &mint.pubkey(),
        svm.minimum_balance_for_rent_exemption(space),
        space as u64,
        &spl_token::ID,
    );
    let init = spl_token::instruction::initialize_mint2(
        &spl_token::ID,
        &mint.pubkey(),
        &authority.pubkey(),
        None, // no freeze authority; the fixture never freezes anything
        decimals,
    )
    .expect("initialize_mint2 builds");

    send(svm, authority, &[authority, &mint], &[create, init]);
    mint.pubkey()
}

/// Create the Associated Token Account for (`mint`, `owner`), rent paid by
/// `payer`. Returns the ATA's address. The owner does not sign — creating
/// someone's canonical token account is not an authority action.
pub fn create_ata(svm: &mut LiteSVM, payer: &Keypair, mint: &Address, owner: &Address) -> Address {
    let ix = spl_associated_token_account::instruction::create_associated_token_account(
        &payer.pubkey(),
        owner,
        mint,
        &spl_token::ID,
    );
    send(svm, payer, &[payer], &[ix]);
    ata_address(mint, owner)
}

/// Mint `amount` base units of `mint` into `dest`. The mint authority signs;
/// create_mint above made `authority` that authority.
pub fn mint_to(svm: &mut LiteSVM, authority: &Keypair, mint: &Address, dest: &Address, amount: u64) {
    let ix = spl_token::instruction::mint_to(
        &spl_token::ID,
        mint,
        dest,
        &authority.pubkey(),
        &[], // no multisig; the single authority is the signer
        amount,
    )
    .expect("mint_to builds");
    send(svm, authority, &[authority], &[ix]);
}

/// The canonical ATA address for (`mint`, `owner`): a PDA of the associated
/// token program, derived from exactly these three seeds, in this order.
pub fn ata_address(mint: &Address, owner: &Address) -> Address {
    Address::find_program_address(
        &[owner.as_ref(), spl_token::ID.as_ref(), mint.as_ref()],
        &spl_associated_token_account::ID,
    )
    .0
}

// The same send shape spl_setup.rs uses, kept private here so this file stands
// alone. Fixture setup failing is a test failure, so it panics with context
// instead of returning the error for the caller to thread.
fn send(svm: &mut LiteSVM, payer: &Keypair, signers: &[&Keypair], ixs: &[Instruction]) {
    let bh = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(ixs, Some(&payer.pubkey()), &bh);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), signers)
        .expect("fixture tx signs");
    svm.send_transaction(tx)
        .map(|_| ())
        .unwrap_or_else(|e| panic!("spl_helpers setup tx failed: {:?}", e.err));
}
