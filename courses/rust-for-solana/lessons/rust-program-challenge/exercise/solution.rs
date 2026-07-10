fn process_transfer(
    sender_balance: u64,
    recipient_balance: u64,
    amount: u64,
    fee: u64,
) -> Result<(u64, u64), String> {
    let total_cost = amount.checked_add(fee)
        .ok_or_else(|| "Overflow".to_string())?;
    if sender_balance < total_cost {
        return Err("INSUFFICIENT_BALANCE".to_string());
    }
    let new_sender = sender_balance - total_cost;
    let new_recipient = recipient_balance + amount;
    Ok((new_sender, new_recipient))
}
