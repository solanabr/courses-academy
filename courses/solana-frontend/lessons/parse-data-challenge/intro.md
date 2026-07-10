# Challenge: Parse Account Data Buffer

Write a function that parses a binary account data buffer into a structured object.

## Requirements

- Parse a `Uint8Array` containing the following fields (in order):
  1. **version** (u8, 1 byte): Account version number
  2. **level** (u32, 4 bytes, little-endian): User level
  3. **username** (string): Length-prefixed UTF-8 string (u32 length + bytes)
- Return an object with `{ version: number, level: number, username: string }`
- Use `DataView` for reading integers and `TextDecoder` for the string
