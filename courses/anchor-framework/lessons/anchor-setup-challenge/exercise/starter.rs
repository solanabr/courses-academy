#[derive(Debug)]
struct AccountMeta {
    pubkey: String,
    is_signer: bool,
    is_writable: bool,
}

fn build_instruction(program_id: &str, user_key: &str, instruction_name: &str) -> String {
    // TODO: Encode instruction_name as bytes and get its length

    // TODO: Build account metas Vec with 2 entries:
    //   - user_key: signer=true, writable=true
    //   - system program: signer=false, writable=false

    // TODO: Return "program:<id>,accounts:<count>,data_len:<len>"
    todo!()
}
