#[derive(Debug, PartialEq)]
struct SolanaAccount {
    owner: String,
    balance: u64,
    is_frozen: bool,
}

impl SolanaAccount {
    fn new(owner: &str, balance: u64) -> Self {
        SolanaAccount {
            owner: owner.to_string(),
            balance,
            is_frozen: false,
        }
    }

    fn withdraw(&mut self, amount: u64) -> Result<u64, String> {
        if self.is_frozen {
            return Err("Account is frozen".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient balance".to_string());
        }
        self.balance -= amount;
        Ok(self.balance)
    }
}

fn create_and_withdraw(owner: &str, initial_balance: u64, withdraw_amount: u64) -> String {
    let mut account = SolanaAccount::new(owner, initial_balance);
    match account.withdraw(withdraw_amount) {
        Ok(remaining) => format!("OK:{}", remaining),
        Err(e) => format!("ERR:{}", e),
    }
}
