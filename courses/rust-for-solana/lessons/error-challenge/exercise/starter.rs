fn validate_transfer(
    sender: &str,
    recipient: &str,
    amount: u64,
    sender_balance: u64,
) -> Result<(), String> {
    // TODO: Check if recipient is empty -> Err("INVALID_RECIPIENT")

    // TODO: Check if amount is 0 -> Err("ZERO_AMOUNT")

    // TODO: Check if sender_balance < amount -> Err("INSUFFICIENT_BALANCE")

    // TODO: Check if sender == recipient -> Err("SAME_ACCOUNT")

    // TODO: All checks passed -> Ok(())
    todo!()
}
