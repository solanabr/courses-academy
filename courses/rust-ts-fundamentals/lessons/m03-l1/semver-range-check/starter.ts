/**
 * Semver range check (STARTER, finish the three TODOs).
 *
 * package.json ranges are a contract: "which future versions of this dep do I
 * trust?" You just read helius-sdk's peerDependencies pin a whole major line
 * with one caret. Implement the checker so the contract is something you can
 * COMPUTE, not squint at.
 *
 * Rules to implement (this file's spec is the grader's spec):
 *   ">=X.Y.Z"  any version at or above the base.
 *   "^X.Y.Z"   at or above the base, same MAJOR, unless major is 0: then the
 *              MINOR is the breaking slot, so same major AND same minor
 *              (the caret-zero rule: ^0.3.9 admits 0.3.10, never 0.4.0).
 *   "~X.Y.Z"   at or above the base, same MAJOR and same MINOR.
 *   "X.Y.Z"    exact match only (already done below).
 *
 * Pure logic, no imports, no npm, no semver package. parseSemver and compare
 * are provided underneath; note that parseSemver returns null on anything that
 * is not three dot-separated non-negative integers (so feed it the range with
 * its operator ALREADY sliced off).
 */
function satisfiesRange(version: string, range: string): boolean {
  const v = parseSemver(version);
  if (v === null) return false;

  // TODO 1: handle ">=X.Y.Z", slice off the operator, parse the base,
  //         return whether compare(v, base) >= 0.

  // TODO 2: handle "^X.Y.Z", at or above the base, same major;
  //         caret-zero rule when the base major is 0 (same minor required).

  // TODO 3: handle "~X.Y.Z", at or above the base, same major AND minor.

  // Exact match (done for you):
  const exact = parseSemver(range);
  if (exact === null) return false;
  return compare(v, exact) === 0;
}

function parseSemver(s: string): [number, number, number] | null {
  const parts = s.trim().split(".");
  if (parts.length !== 3) return null;
  const nums = parts.map((p) => Number(p));
  if (nums.some((n) => !Number.isInteger(n) || n < 0)) return null;
  return [nums[0], nums[1], nums[2]];
}

function compare(
  a: [number, number, number],
  b: [number, number, number]
): number {
  for (let i = 0; i < 3; i++) {
    if (a[i] !== b[i]) return a[i] < b[i] ? -1 : 1;
  }
  return 0;
}
