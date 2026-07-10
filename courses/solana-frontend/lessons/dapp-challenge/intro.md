# Challenge: SOL Transfer Form

Build the validation and transaction construction logic for a SOL transfer form.

## Requirements

- Create a function that validates a transfer request and constructs the transaction
- Accept `senderPublicKey` (PublicKey), `recipientAddress` (string), `amountInSol` (number), and `connection` (Connection)
- Validate:
  1. Recipient address is a valid PublicKey (use try/catch)
  2. Amount is positive and non-zero
  3. Sender has sufficient balance (check with `connection.getBalance()`)
- If validation fails, return `{ success: false, error: string }`
- If validation passes, construct a Transaction with SystemProgram.transfer and return `{ success: true, transaction: Transaction }`
