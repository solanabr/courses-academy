fn validate_account(
    account_owner: &str,
    expected_owner: &str,
    is_signer: bool,
    require_signer: bool,
    is_writable: bool,
    require_writable: bool,
) -> Result<String, String> {
    if account_owner != expected_owner {
        return Err("IncorrectProgramId".to_string());
    }
    if require_signer && !is_signer {
        return Err("MissingRequiredSignature".to_string());
    }
    if require_writable && !is_writable {
        return Err("AccountNotWritable".to_string());
    }
    Ok("Valid".to_string())
}
