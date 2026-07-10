#[derive(Debug, PartialEq)]
struct SolanaAccount {
    owner: String,
    balance: u64,
    is_frozen: bool,
}

impl SolanaAccount {
    fn new(owner: &str, balance: u64) -> Self {
        // TODO: Create a new account (not frozen by default)
        todo!()
    }

    fn withdraw(&mut self, amount: u64) -> Result<u64, String> {
        // TODO: Return Err if frozen
        // TODO: Return Err if insufficient balance
        // TODO: Subtract amount and return Ok(remaining)
        todo!()
    }
}

fn create_and_withdraw(owner: &str, initial_balance: u64, withdraw_amount: u64) -> String {
    let mut account = SolanaAccount::new(owner, initial_balance);
    match account.withdraw(withdraw_amount) {
        Ok(remaining) => format!("OK:{}", remaining),
        Err(e) => format!("ERR:{}", e),
    }
}
