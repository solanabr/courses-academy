// tests/swap_fixture.rs — the Mollusk account fixture for R4, the swap.
//
// Drop this file in at `programs/token-ticket-swap/tests/swap_fixture.rs` and
// declare `mod swap_fixture;` at the top of any test that uses it, exactly as
// m06-l1's `cu_baseline.rs` and m06-l2's `cu_swap_regression.rs` do. Its full
// surface: keys(), build_swap_accounts(), build_swap_ix(), build_init_accounts(),
// build_init_ix().
//
// It is ordinary SPL fixture construction with nothing V2-specific in it: real
// 82-byte mint layouts, real 165-byte token-account layouts, packed with the
// same `spl_token` state types the on-chain program reads, plus the pool at its
// real PDA. Mollusk runs one instruction against exactly the accounts you hand
// it — there is no validator to create them for you — so a fixture's whole job
// is to lay out that account set the way the pool's init would have. The one
// thing an account row cannot do is make a program EXECUTABLE: the swap CPIs
// into the token program, so the test registers it in Mollusk's program cache
// with `mollusk_svm_programs_token::token::add_program(&mut mollusk)` right
// after `Mollusk::new`, and this fixture supplies the matching account row.
//
// If your R4's context diverged from the worked example, adapt the field names
// here; the shape of the construction does not change.
#![allow(dead_code)]

use anchor_lang::{Discriminator, InstructionData, ToAccountMetas};
use solana_sdk::{account::Account, instruction::Instruction, pubkey::Pubkey};
use spl_token::solana_program::{program_option::COption, program_pack::Pack};

use token_ticket_swap::{Pool, POOL_SEED};

/// Every fixture account is funded far above the rent-exempt line, so rent
/// never decides a measurement.
const FUNDED: u64 = 1_000_000_000;

/// The starting reserves: a balanced 1,000,000 / 1,000,000 pool, the same
/// depth m05-l2 quoted by hand, so the numbers you can predict are the
/// numbers the trade sees.
const RESERVE_DEPTH: u64 = 1_000_000;
/// What the trader walks up with.
const TRADER_ARCADE: u64 = 1_000_000;
/// Each mint's recorded supply is exactly the sum of the balances the fixture
/// deals out, so the books balance the way a real mint's would.
const ARCADE_SUPPLY: u64 = RESERVE_DEPTH + TRADER_ARCADE;
const TICKET_SUPPLY: u64 = RESERVE_DEPTH;
const DECIMALS: u8 = 6;

pub struct Keys {
    pub trader: Pubkey,
    pub pool: Pubkey,
    pub mint_arcade: Pubkey,
    pub mint_ticket: Pubkey,
    pub reserve_arcade: Pubkey,
    pub reserve_ticket: Pubkey,
    pub trader_arcade: Pubkey,
    pub trader_ticket: Pubkey,
}

/// One address per account in the swap. Only the pool is derived — it is the
/// program's own PDA, seeded exactly as R4 declares it — and everything else
/// is a fresh unique key, because nothing constrains where a mint or a token
/// account lives.
pub fn keys(program_id: &Pubkey) -> Keys {
    Keys {
        trader: Pubkey::new_unique(),
        pool: Pubkey::find_program_address(&[POOL_SEED], program_id).0,
        mint_arcade: Pubkey::new_unique(),
        mint_ticket: Pubkey::new_unique(),
        reserve_arcade: Pubkey::new_unique(),
        reserve_ticket: Pubkey::new_unique(),
        trader_arcade: Pubkey::new_unique(),
        trader_ticket: Pubkey::new_unique(),
    }
}

/// The account set for one trade, laid out as pool-init left it: the pool PDA
/// holding its state, both mints live, the two reserves owned by the pool PDA
/// and funded to depth, the trader's arcade account funded and ticket account
/// empty. Returned as Mollusk's (Pubkey, Account) pairs.
pub fn build_swap_accounts(k: &Keys) -> Vec<(Pubkey, Account)> {
    vec![
        (k.trader, system_account(FUNDED)),
        (k.pool, pool_account(k)),
        (k.mint_arcade, mint_account(ARCADE_SUPPLY)),
        (k.mint_ticket, mint_account(TICKET_SUPPLY)),
        (k.reserve_arcade, token_account(&k.mint_arcade, &k.pool, RESERVE_DEPTH)),
        (k.reserve_ticket, token_account(&k.mint_ticket, &k.pool, RESERVE_DEPTH)),
        (k.trader_arcade, token_account(&k.mint_arcade, &k.trader, TRADER_ARCADE)),
        (k.trader_ticket, token_account(&k.mint_ticket, &k.trader, 0)),
        // The token program's account row, shaped the way the cache entry
        // expects it (executable, loader-owned). The pair travels together:
        // add_program puts the program in the cache, this row puts it in the
        // instruction's account list.
        mollusk_svm_programs_token::token::keyed_account(),
    ]
}

/// One swap instruction: amount_in of 100, and a min_out of 0 so the slippage
/// guard never decides the measurement for you (m06-l1's rule for the fixture).
pub fn build_swap_ix(program_id: &Pubkey, k: &Keys) -> Instruction {
    use token_ticket_swap::accounts::SwapArcadeForTickets as SwapAccounts;
    use token_ticket_swap::instruction::SwapArcadeForTickets as SwapArgs;

    let metas = SwapAccounts {
        trader: k.trader,
        pool: k.pool,
        mint_arcade: k.mint_arcade,
        mint_ticket: k.mint_ticket,
        reserve_arcade: k.reserve_arcade,
        reserve_ticket: k.reserve_ticket,
        trader_arcade: k.trader_arcade,
        trader_ticket: k.trader_ticket,
        token_program: spl_token::ID,
    }
    .to_account_metas(None);
    let data = SwapArgs {
        amount_in: 100,
        min_out: 0,
    }
    .data();
    Instruction {
        program_id: *program_id,
        accounts: metas,
        data,
    }
}

/// The account set for pool-init. It differs from the swap's in exactly the
/// way m06-l2 says it must: the pool and both reserves arrive *uninitialized*
/// — empty, system-owned accounts the init's create-account CPIs will fill —
/// while the payer arrives funded and the mints arrive live.
pub fn build_init_accounts(k: &Keys) -> Vec<(Pubkey, Account)> {
    vec![
        (k.trader, system_account(FUNDED)),
        (k.pool, Account::default()),
        (k.mint_arcade, mint_account(ARCADE_SUPPLY)),
        (k.mint_ticket, mint_account(TICKET_SUPPLY)),
        (k.reserve_arcade, Account::default()),
        (k.reserve_ticket, Account::default()),
        mollusk_svm_programs_token::token::keyed_account(),
        (system_program_id(), Account::default()),
    ]
}

/// One init_pool instruction, paid for by the trader keypair standing in as
/// the deployer.
pub fn build_init_ix(program_id: &Pubkey, k: &Keys) -> Instruction {
    use token_ticket_swap::accounts::InitPool as InitAccounts;
    use token_ticket_swap::instruction::InitPool as InitArgs;

    let metas = InitAccounts {
        payer: k.trader,
        pool: k.pool,
        mint_arcade: k.mint_arcade,
        mint_ticket: k.mint_ticket,
        reserve_arcade: k.reserve_arcade,
        reserve_ticket: k.reserve_ticket,
        token_program: spl_token::ID,
        system_program: system_program_id(),
    }
    .to_account_metas(None);
    let data = InitArgs {}.data();
    Instruction {
        program_id: *program_id,
        accounts: metas,
        data,
    }
}

// ---------------------------------------------------------------------------
// The builders below are the "ordinary SPL fixture construction" part: each
// one writes the byte layout the owning program would have written on-chain.
// ---------------------------------------------------------------------------

/// A plain funded system account (the trader, a payer).
fn system_account(lamports: u64) -> Account {
    Account {
        lamports,
        ..Account::default()
    }
}

/// The system program's well-known id, spelled out so this file adds no
/// dependency beyond the ones m06-l1's Cargo.toml already declares.
fn system_program_id() -> Pubkey {
    "11111111111111111111111111111111".parse().unwrap()
}

/// A live SPL mint: the real 82-byte layout, packed by the same `spl_token`
/// state type the token program itself uses. No mint authority — the fixture
/// never mints after setup, it just declares the balances directly.
fn mint_account(supply: u64) -> Account {
    let mut data = vec![0u8; spl_token::state::Mint::LEN];
    spl_token::state::Mint {
        mint_authority: COption::None,
        supply,
        decimals: DECIMALS,
        is_initialized: true,
        freeze_authority: COption::None,
    }
    .pack_into_slice(&mut data);
    Account {
        lamports: FUNDED,
        data,
        owner: spl_token::ID,
        ..Account::default()
    }
}

/// A live SPL token account: the real 165-byte layout, holding `amount` of
/// `mint`, controlled by `owner` — the pool PDA for the reserves, the trader
/// for the trader-side pair.
fn token_account(mint: &Pubkey, owner: &Pubkey, amount: u64) -> Account {
    let mut data = vec![0u8; spl_token::state::Account::LEN];
    spl_token::state::Account {
        mint: *mint,
        owner: *owner,
        amount,
        delegate: COption::None,
        state: spl_token::state::AccountState::Initialized,
        is_native: COption::None,
        delegated_amount: 0,
        close_authority: COption::None,
    }
    .pack_into_slice(&mut data);
    Account {
        lamports: FUNDED,
        data,
        owner: spl_token::ID,
        ..Account::default()
    }
}

/// The pool PDA exactly as init_pool wrote it: the 8-byte account
/// discriminator, then the Pod body — both mints and the stored canonical
/// bump — owned by the swap program.
fn pool_account(k: &Keys) -> Account {
    let program_id = token_ticket_swap::ID;
    let (_, bump) = Pubkey::find_program_address(&[POOL_SEED], &program_id);

    let disc = Pool::DISCRIMINATOR;
    let mut data = vec![0u8; disc.len() + core::mem::size_of::<Pool>()];
    data[..disc.len()].copy_from_slice(disc);
    let body = disc.len();
    // Pod layout, straight offsets: arcade_mint, ticket_mint, bump, padding.
    data[body..body + 32].copy_from_slice(k.mint_arcade.as_ref());
    data[body + 32..body + 64].copy_from_slice(k.mint_ticket.as_ref());
    data[body + 64] = bump;

    Account {
        lamports: FUNDED,
        data,
        owner: program_id,
        ..Account::default()
    }
}
