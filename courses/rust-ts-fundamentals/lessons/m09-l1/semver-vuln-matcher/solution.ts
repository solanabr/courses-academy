/**
 * Semver-range vulnerability matcher (SOLUTION).
 *
 * isVulnerable(installed, range), range is one or more space-separated
 * comparators (>=, <=, >, <, =, or a bare exact version), ALL of which must
 * hold. Versions compare NUMERICALLY component by component, which is why
 * '0.3.9' < '0.3.10' here even though string comparison says otherwise.
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
  const partsA = a.split('.').map(Number);
  const partsB = b.split('.').map(Number);
  for (let i = 0; i < 3; i++) {
    const componentA = partsA[i] ?? 0;
    const componentB = partsB[i] ?? 0;
    if (componentA !== componentB) {
      return componentA < componentB ? -1 : 1;
    }
  }
  return 0;
}
