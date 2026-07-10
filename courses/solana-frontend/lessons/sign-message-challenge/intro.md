# Challenge: Build a SIWS Message

Construct a properly formatted Sign-In With Solana (SIWS) message that can be used for authentication.

## Requirements

- Create a function that accepts `domain` (string), `publicKey` (PublicKey), `nonce` (string), and `statement` (string)
- Generate an ISO timestamp for "Issued At" (use `new Date().toISOString()`)
- Return a formatted SIWS message string following this structure:
  ```
  {domain} wants you to sign in with your Solana account:
  {publicKey}
  
  {statement}
  
  URI: https://{domain}
  Version: 1
  Nonce: {nonce}
  Issued At: {issuedAt}
  ```
