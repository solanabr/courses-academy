// The Almanac collection's Royalties-plugin pre-flight check.
//
// Before you attach a Core Royalties plugin to an asset or collection, the
// config has to be internally consistent or the mint reverts. A Core Royalties
// plugin carries three things:
//   - basisPoints: the royalty rate, an integer 0..10000 (10000 = 100%)
//   - creators:    a split list; each creator's percentage is 0..100 and the
//                  percentages MUST sum to exactly 100, with no duplicate
//                  creator addresses
//   - ruleSet:     one of "None" | "ProgramAllowList" | "ProgramDenyList"
//
// The grader calls your validator with three positional arguments:
//   validateRoyalties(basisPoints, ruleSet, creatorSpec)
// where creatorSpec encodes the split list as one string of
// semicolon-separated "address=percentage" entries, e.g.
//   'Farm1er1111111111111111111111111111111111=70;Farm2er2222222222222222222222222222222222=30'
// The parsing into Creator objects is already done for you below.
//
// Return { ok: true, reason: "ok" } when the config is valid, else
// { ok: false, reason: <which rule failed> }.
//
// TODO: the starter only checks the shares sum. That is the single most common
// thing people remember and the four things they forget: basisPoints range,
// the ruleSet variant, duplicate creators, per-share range. Fix it.

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
  const creators: Creator[] = creatorSpec
    .split(";")
    .filter((entry) => entry.length > 0)
    .map((entry) => {
      const [address, pct] = entry.split("=");
      return { address, percentage: Number(pct) };
    });

  // TODO: replace this shares-only stub with the full guard.
  const sum = creators.reduce((acc, c) => acc + c.percentage, 0);
  if (sum !== 100) {
    return { ok: false, reason: "creator shares must sum to 100" };
  }
  return { ok: true, reason: "ok" };
}
