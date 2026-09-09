// Wavelength Records: drop blink
// Reference solution: assemble a spec-conformant ActionPostResponse.
// Called positionally: buildActionPostResponse(transactionBase64, message?, nextActionHref?)

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
  const response: ActionPostResponse = {
    type: 'transaction',
    transaction: transactionBase64,
  };

  if (message != null) {
    response.message = message;
  }

  if (nextActionHref != null) {
    response.links = { next: { type: 'post', href: nextActionHref } };
  }

  return response;
}
