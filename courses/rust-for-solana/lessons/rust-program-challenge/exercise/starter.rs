fn process_transfer(
    sender_balance: u64,
    recipient_balance: u64,
    amount: u64,
    fee: u64,
) -> Result<(u64, u64), String> {
    // TODO: Check sender can afford amount + fee

    // TODO: Calculate new sender balance (subtract amount and fee)

    // TODO: Calculate new recipient balance (add amount)

    // TODO: Return Ok((new_sender, new_recipient))
    todo!()
}
