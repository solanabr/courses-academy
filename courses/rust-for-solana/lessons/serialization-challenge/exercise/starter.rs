/// Serialize account data to bytes (Borsh-like layout):
/// [4-byte owner_len][owner UTF-8 bytes][8-byte balance LE][1-byte frozen]
fn serialize_account(owner: &str, balance: u64, frozen: bool) -> Vec<u8> {
    // TODO: Create a new Vec<u8>
    // TODO: Write owner length as u32 little-endian (4 bytes)
    // TODO: Write owner string as UTF-8 bytes
    // TODO: Write balance as u64 little-endian (8 bytes)
    // TODO: Write frozen as 0 or 1 (1 byte)
    todo!()
}
