fn simulate_transfer_test(
    sender_initial: u64,
    transfer_amount: u64,
    fee: u64,
) -> String {
    let receiver_initial: u64 = 0;

    if sender_initial < transfer_amount + fee {
        return "FAIL:insufficient_funds".to_string();
    }

    let sender_final = sender_initial - transfer_amount - fee;
    let receiver_final = receiver_initial + transfer_amount;

    format!("PASS:sender={},receiver={}", sender_final, receiver_final)
}
