#[derive(Debug)]
struct AccountMeta {
    pubkey: String,
    is_signer: bool,
    is_writable: bool,
}

fn build_instruction(program_id: &str, user_key: &str, instruction_name: &str) -> String {
    let data_len = instruction_name.as_bytes().len();

    let accounts = vec![
        AccountMeta {
            pubkey: user_key.to_string(),
            is_signer: true,
            is_writable: true,
        },
        AccountMeta {
            pubkey: "11111111111111111111111111111111".to_string(),
            is_signer: false,
            is_writable: false,
        },
    ];

    format!("program:{},accounts:{},data_len:{}", program_id, accounts.len(), data_len)
}
