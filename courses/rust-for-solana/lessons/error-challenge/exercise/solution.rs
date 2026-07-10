fn validate_transfer(
    sender: &str,
    recipient: &str,
    amount: u64,
    sender_balance: u64,
) -> Result<(), String> {
    if recipient.is_empty() {
        return Err("INVALID_RECIPIENT".to_string());
    }
    if amount == 0 {
        return Err("ZERO_AMOUNT".to_string());
    }
    if sender_balance < amount {
        return Err("INSUFFICIENT_BALANCE".to_string());
    }
    if sender == recipient {
        return Err("SAME_ACCOUNT".to_string());
    }
    Ok(())
}
