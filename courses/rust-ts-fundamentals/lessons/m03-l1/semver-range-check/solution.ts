/**
 * Semver range check (SOLUTION).
 *
 * ">=X.Y.Z"  any version at or above the base.
 * "^X.Y.Z"   at or above the base, same MAJOR, unless major is 0: then the
 *            MINOR is the breaking slot (caret-zero rule).
 * "~X.Y.Z"   at or above the base, same MAJOR and same MINOR.
 * "X.Y.Z"    exact match only.
 */
function satisfiesRange(version: string, range: string): boolean {
  const v = parseSemver(version);
  if (v === null) return false;

  if (range.startsWith(">=")) {
    const base = parseSemver(range.slice(2));
    if (base === null) return false;
    return compare(v, base) >= 0;
  }

  if (range.startsWith("^")) {
    const base = parseSemver(range.slice(1));
    if (base === null) return false;
    if (compare(v, base) < 0) return false;
    if (base[0] > 0) return v[0] === base[0];
    // caret-zero rule: ^0.Y.Z, the minor is the breaking slot
    return v[0] === 0 && v[1] === base[1];
  }

  if (range.startsWith("~")) {
    const base = parseSemver(range.slice(1));
    if (base === null) return false;
    if (compare(v, base) < 0) return false;
    return v[0] === base[0] && v[1] === base[1];
  }

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
