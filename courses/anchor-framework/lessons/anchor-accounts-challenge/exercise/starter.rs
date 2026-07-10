fn validate_account(
    account_owner: &str,
    expected_owner: &str,
    is_signer: bool,
    require_signer: bool,
    is_writable: bool,
    require_writable: bool,
) -> Result<String, String> {
    // TODO: Check owner matches expected_owner
    //       -> Err("IncorrectProgramId")

    // TODO: Check signer requirement
    //       -> Err("MissingRequiredSignature")

    // TODO: Check writable requirement
    //       -> Err("AccountNotWritable")

    // TODO: All checks passed -> Ok("Valid")
    todo!()
}
