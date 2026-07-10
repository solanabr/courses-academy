fn serialize_account(owner: &str, balance: u64, frozen: bool) -> Vec<u8> {
    let mut buf = Vec::new();
    // Write owner length as u32 little-endian
    buf.extend_from_slice(&(owner.len() as u32).to_le_bytes());
    // Write owner as UTF-8 bytes
    buf.extend_from_slice(owner.as_bytes());
    // Write balance as u64 little-endian
    buf.extend_from_slice(&balance.to_le_bytes());
    // Write frozen flag
    buf.push(if frozen { 1 } else { 0 });
    buf
}
