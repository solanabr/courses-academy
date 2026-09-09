// A settlement receipt: read the vault's balance on BOTH sides of the CPI.
//
// One read per side, and the placement is forced by the borrow rather than by
// taste. `opening` sits above the `CpiContext` because that is the last point
// at which the vault is still unborrowed — and what it reads out is a `u64`
// copy on the stack, which holds no borrow of its own to collide with the
// handle built on the next line. `closing` sits below the `transfer_checked`
// call because that call consumes the `CpiContext`, and consuming it is what
// kills the handles; the read is legal again there, and it is already fresh,
// because there is no deserialized copy anywhere to have gone stale.
//
// Hoisting `closing` up beside `opening` also silences E0502 — and reports the
// pre-transfer balance twice. That is the v1 bug wearing a compiling face.

struct TokenAccount {
    amount: u64,
}

impl TokenAccount {
    /// A typed read of the account's live data: needs a shared borrow.
    fn amount(&self) -> u64 {
        self.amount
    }

    /// Hand this account to a CPI that may WRITE it: a mutable borrow.
    fn cpi_handle_mut(&mut self) -> CpiHandleMut<'_> {
        CpiHandleMut(self)
    }
}

struct Mint {
    decimals: u8,
}

impl Mint {
    fn decimals(&self) -> u8 {
        self.decimals
    }

    /// Hand this account to a CPI that only READS it: a shared borrow.
    fn cpi_handle(&self) -> CpiHandle<'_> {
        CpiHandle(self)
    }
}

struct CpiHandleMut<'a>(&'a mut TokenAccount);
struct CpiHandle<'a>(&'a Mint);

struct TransferChecked<'a> {
    from: CpiHandleMut<'a>,
    mint: CpiHandle<'a>,
    to: CpiHandleMut<'a>,
}

struct CpiContext<'a> {
    accounts: TransferChecked<'a>,
}

impl<'a> CpiContext<'a> {
    fn new(accounts: TransferChecked<'a>) -> Self {
        CpiContext { accounts }
    }
}

mod token_interface {
    /// Taking the `CpiContext` by value is what ends every borrow it carries.
    pub fn transfer_checked(cpi: super::CpiContext<'_>, amount: u64, decimals: u8) {
        let accounts = cpi.accounts;
        // The "checked" half: the caller states the mint's decimals and the
        // callee proves them against the mint account it was handed.
        assert_eq!(accounts.mint.0.decimals, decimals, "mint decimals mismatch");
        accounts.from.0.amount = accounts.from.0.amount
            .checked_sub(amount)
            .expect("settlement exceeds the vault balance");
        accounts.to.0.amount = accounts.to.0.amount
            .checked_add(amount)
            .expect("recipient balance overflow");
    }
}

struct Accounts {
    mint: Mint,
    recipient_ta: TokenAccount,
    vault_ta: TokenAccount,
}

struct Context {
    accounts: Accounts,
}

fn settle(vault_start: u64, recipient_start: u64, amount: u64) -> (u64, u64) {
    let mut ctx = Context {
        accounts: Accounts {
            mint: Mint { decimals: 6 },
            recipient_ta: TokenAccount { amount: recipient_start },
            vault_ta: TokenAccount { amount: vault_start },
        },
    };

    // GIVEN, do not move: the receipt must be read, not derived. The transfer
    // keeps the one copy of `amount` it needs, and then the arguments stop
    // existing — from here down `vault_start`, `recipient_start` and `amount`
    // are unit values, so a receipt written from arithmetic on them is a type
    // error, the same fate as a misplaced read.
    let transfer_amount = amount;
    #[allow(unused_variables)]
    let (vault_start, recipient_start, amount) = ((), (), ());

    // No handle exists yet, so this read is free — and the `u64` it copies out
    // borrows nothing once the statement ends.
    let opening = ctx.accounts.vault_ta.amount();

    let cpi = CpiContext::new(TransferChecked {
        from: ctx.accounts.vault_ta.cpi_handle_mut(),
        mint: ctx.accounts.mint.cpi_handle(),
        to: ctx.accounts.recipient_ta.cpi_handle_mut(),
    });

    token_interface::transfer_checked(cpi, transfer_amount, ctx.accounts.mint.decimals());

    // The handles died with the `CpiContext` on the line above, so the vault is
    // readable again — and what it reads is the post-transfer balance.
    let closing = ctx.accounts.vault_ta.amount();

    (opening, closing)
}
