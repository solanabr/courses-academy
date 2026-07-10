function parseAccountData(data: Uint8Array): {
  version: number;
  level: number;
  username: string;
} {
  const view = new DataView(data.buffer, data.byteOffset, data.byteLength);
  
  // Parse version (u8, 1 byte)
  const version = view.getUint8(0);
  
  // Parse level (u32, 4 bytes, little-endian)
  const level = view.getUint32(1, true);
  
  // Parse string (u32 length prefix + UTF-8 bytes)
  const stringLength = view.getUint32(5, true);
  const stringBytes = data.slice(9, 9 + stringLength);
  const username = new TextDecoder().decode(stringBytes);
  
  return { version, level, username };
}
