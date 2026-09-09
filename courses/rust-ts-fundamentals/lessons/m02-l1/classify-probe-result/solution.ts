// Pulse Station, probe classifier (m02-l1), reference solution
//
// The (kind, value) pair is parsed ONCE at the boundary into a
// discriminated union; classification is an exhaustive switch whose
// default arm is a never-check, so adding a variant without handling it
// becomes a compile error, not a silent 'down'. Verdict stays the
// three-member union; 'invalid' is a parse fact, not a fourth verdict,
// and the return type says so.

type Verdict = 'up' | 'degraded' | 'down';

type ProbeResult =
  | { kind: 'ok'; latencyMs: number }
  | { kind: 'timeout'; budgetMs: number }
  | { kind: 'http-error'; status: number };

function classifyProbe(kind: string, value: number): Verdict | 'invalid' {
  const result = parseProbe(kind, value);
  if (result === null) {
    return 'invalid';
  }
  switch (result.kind) {
    case 'ok':
      if (result.latencyMs > 1000) {
        return 'down';
      }
      return result.latencyMs >= 400 ? 'degraded' : 'up';
    case 'timeout':
      return 'down';
    case 'http-error':
      // 429 means the target answered, you are being rate-limited, it is
      // not dead. Every other error status counts as down.
      return result.status === 429 ? 'degraded' : 'down';
    default:
      return assertNever(result);
  }
}

function parseProbe(kind: string, value: number): ProbeResult | null {
  if (kind === 'ok') {
    return { kind: 'ok', latencyMs: value };
  }
  if (kind === 'timeout') {
    return { kind: 'timeout', budgetMs: value };
  }
  if (kind === 'http-error') {
    return { kind: 'http-error', status: value };
  }
  return null;
}

function assertNever(x: never): never {
  throw new Error('unhandled probe variant: ' + JSON.stringify(x));
}
