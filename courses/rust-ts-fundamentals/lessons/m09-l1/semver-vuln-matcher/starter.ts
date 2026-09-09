/**
 * Semver-range vulnerability matcher (STARTER, one function is lying to you).
 *
 * A security advisory names a VULNERABLE RANGE, e.g. ">=0.3.0 <0.3.11".
 * An auditor's core question is: does MY installed version sit inside it?
 *
 * Range grammar (the AND form advisories use):
 *   - one or more comparators separated by single spaces; ALL must hold
 *   - comparator = optional operator (>=, <=, >, <, =) + a version "x.y.z"
 *   - a bare version means exact match ("1.2.3" === "=1.2.3")
 *
 * isVulnerable('0.3.10', '>=0.3.0 <0.3.11') -> true   (the arrayref window)
 * isVulnerable('0.3.11', '>=0.3.0 <0.3.11') -> false  (the patched release)
 *
 * The range parsing below is done. The bug is in compareVersions: it compares
 * version strings as STRINGS, and lexicographically, '0.3.9' > '0.3.10',
 * because '9' > '1'. Fix compareVersions to compare numerically, component
 * by component (major, then minor, then patch). Pure, no imports.
 */
function isVulnerable(installed: string, vulnerableRange: string): boolean {
  const comparators = vulnerableRange.trim().split(/\s+/);
  for (const comparator of comparators) {
    if (!satisfiesComparator(installed, comparator)) {
      return false;
    }
  }
  return true;
}

function satisfiesComparator(version: string, comparator: string): boolean {
  const match = comparator.match(/^(>=|<=|>|<|=)?(\d.*)$/);
  if (!match) {
    return false;
  }
  const op = match[1] ?? '=';
  const target = match[2];
  const cmp = compareVersions(version, target);
  switch (op) {
    case '>':
      return cmp > 0;
    case '>=':
      return cmp >= 0;
    case '<':
      return cmp < 0;
    case '<=':
      return cmp <= 0;
    default:
      return cmp === 0;
  }
}

function compareVersions(a: string, b: string): number {
  // TODO: this is the bug. String comparison orders '0.3.10' BEFORE '0.3.9'
  // (character '1' < character '9'), so double-digit components misorder.
  // Split each version on '.', convert the components to numbers, and compare
  // major, then minor, then patch. Return -1 / 0 / 1.
  if (a === b) {
    return 0;
  }
  return a < b ? -1 : 1;
}
