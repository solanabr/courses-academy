// The Almanac collection's Royalties-plugin pre-flight check (reference solution).
//
// Grader calling convention: validateRoyalties(basisPoints, ruleSet, creatorSpec)
// where creatorSpec is one string of semicolon-separated "address=percentage"
// entries.

export interface Creator {
  address: string;
  percentage: number;
}

export interface RoyaltyCheck {
  ok: boolean;
  reason: string;
}

function validateRoyalties(
  basisPoints: number,
  ruleSet: string,
  creatorSpec: string,
): RoyaltyCheck {
  const fail = (reason: string): RoyaltyCheck => ({ ok: false, reason });

  const RULE_SETS = new Set<string>([
    "None",
    "ProgramAllowList",
    "ProgramDenyList",
  ]);

  const creators: Creator[] = creatorSpec
    .split(";")
    .filter((entry) => entry.length > 0)
    .map((entry) => {
      const [address, pct] = entry.split("=");
      return { address, percentage: Number(pct) };
    });

  if (
    !Number.isInteger(basisPoints) ||
    basisPoints < 0 ||
    basisPoints > 10000
  ) {
    return fail(`basisPoints out of range: ${basisPoints}`);
  }

  if (!RULE_SETS.has(ruleSet)) {
    return fail(`unknown ruleSet: ${ruleSet}`);
  }

  if (creators.length === 0) {
    return fail("creators list is empty");
  }

  const seen = new Set<string>();
  let sum = 0;
  for (const c of creators) {
    if (seen.has(c.address)) {
      return fail(`duplicate creator: ${c.address}`);
    }
    seen.add(c.address);
    if (!Number.isInteger(c.percentage) || c.percentage < 0 || c.percentage > 100) {
      return fail(`creator share out of range: ${c.percentage}`);
    }
    sum += c.percentage;
  }

  if (sum !== 100) {
    return fail(`creator shares must sum to 100, got ${sum}`);
  }

  return { ok: true, reason: "ok" };
}
