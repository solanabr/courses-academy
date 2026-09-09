// Wavelength Records: drop blink
// Build the ActionPostResponse a blink returns from its POST handler.
//
// The Actions spec: after a wallet POSTs { account } to your action endpoint,
// you return an ActionPostResponse. At minimum it carries the base64 transaction
// the wallet will sign. Optionally it carries:
//   - `message`: a human-readable confirmation string
//   - `links.next`: a chained action (a POST callback, same-origin) that runs
//     after this transaction confirms; this is how a blink shows a "thanks"
//     step or a follow-up mint.
//
// The grader calls this function with POSITIONAL arguments:
//   buildActionPostResponse(transactionBase64, message?, nextActionHref?)
// An argument that is omitted or passed as `null` means "not provided".
//
// TODO: return the FULL response:
//   - always set `type: 'transaction'` and `transaction`
//   - include `message` only when one was provided (not null, not undefined)
//   - include `links.next` ({ type: 'post', href }) only when nextActionHref was provided
// Do NOT emit `message` or `links` keys when their inputs are absent.

interface ActionPostResponse {
  type: 'transaction';
  transaction: string;
  message?: string;
  links?: { next: { type: 'post'; href: string } };
}

function buildActionPostResponse(
  transactionBase64: string,
  message?: string | null,
  nextActionHref?: string | null,
): ActionPostResponse {
  // Incomplete: drops message and chaining.
  return {
    type: 'transaction',
    transaction: transactionBase64,
  };
}
