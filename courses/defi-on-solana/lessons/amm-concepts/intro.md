# Automated Market Makers (AMMs)

Automated Market Makers revolutionized DeFi by replacing traditional orderbooks with mathematical formulas. Let's understand how they work.

## The Constant Product Formula

The most popular AMM design uses the **constant product formula**:

```
x * y = k
```

Where:
- `x` = reserves of token A
- `y` = reserves of token B
- `k` = constant (product of reserves)

### Example

```
Pool: 100 SOL × 10,000 USDC = 1,000,000 (k)
Price: 10,000 / 100 = 100 USDC per SOL
```

## How Swaps Work

When someone swaps, they add to one reserve and remove from the other, keeping `k` constant.

**User wants to buy 10 SOL:**

```
Before: 100 SOL × 10,000 USDC = 1,000,000
After: 90 SOL × y USDC = 1,000,000
y = 1,000,000 / 90 = 11,111.11 USDC

User pays: 11,111.11 - 10,000 = 1,111.11 USDC
Effective price: 1,111.11 / 10 = 111.11 USDC per SOL
```

Notice the price increased from 100 to 111.11 - this is **slippage**.

## Slippage

Slippage is the difference between expected and executed price:

```
Expected price: 100 USDC/SOL
Actual price: 111.11 USDC/SOL
Slippage: (111.11 - 100) / 100 = 11.11%
```

**Larger trades = more slippage**. This is why AMMs work best for smaller trades relative to pool size.

## Providing Liquidity

Liquidity providers (LPs) deposit both tokens in proportion to current reserves:

```
Pool: 100 SOL × 10,000 USDC
LP deposits: 10 SOL + 1,000 USDC

LP share: 10 / (100 + 10) = 9.09%
```

LPs receive **LP tokens** representing their share. When they withdraw, they burn LP tokens and receive their share of the pool.

### LP Rewards

LPs earn fees from swaps:

```
Swap fee: 0.3%
Swap volume: $1,000,000/day
Daily fees: $3,000

LP with 9.09% share earns: $3,000 × 9.09% = $272.70/day
```

## Impermanent Loss

Impermanent loss (IL) occurs when token prices diverge.

**Example:**

LP deposits when 1 SOL = 100 USDC:
- 10 SOL + 1,000 USDC = $2,000 total value

SOL price doubles to 200 USDC:
- Arbitrageurs rebalance pool
- New pool: 7.07 SOL × 1,414.21 USDC
- LP share: 7.07 SOL × 200 USDC = $1,414.21 + $1,414.21 = $2,828.42

If LP had just held:
- 10 SOL × 200 = $2,000 + $1,000 = $3,000

Impermanent loss: $3,000 - $2,828.42 = **$171.58 (5.7%)**

### IL Formula

```
IL = (2 × sqrt(price_ratio)) / (1 + price_ratio) - 1

Price 2x: IL = 5.7%
Price 3x: IL = 13.4%
Price 5x: IL = 25.5%
```

IL is "impermanent" because it only realizes if you withdraw. If price returns to original, IL disappears.

## Fee Structures

### Uniswap v2 Style (Raydium Standard Pools)

- **0.3%** swap fee
- All fees go to LPs
- Simple, battle-tested

### Uniswap v3 Style (Raydium CLMM, Orca Whirlpools)

- **0.01%, 0.05%, 0.3%, 1%** fee tiers
- Different tiers for different volatility pairs
- Stable pairs (USDC/USDT): 0.01%
- Volatile pairs (SOL/BONK): 1%

## Concentrated Liquidity

Standard AMMs spread liquidity across the entire price curve (0 → ∞). Concentrated liquidity lets LPs choose a range:

```
Standard AMM: $10,000 spread across $0 - $∞
Concentrated: $10,000 focused on $95 - $105

Result: In the $95-$105 range, concentrated liquidity acts like $100,000+
```

### Benefits

- **Capital efficiency**: Same liquidity with less capital
- **Higher LP rewards**: More fees per dollar

### Risks

- **Out of range**: If price exits your range, you stop earning fees
- **More IL**: Concentrated positions amplify impermanent loss
- **Active management**: Need to rebalance ranges

## AMM Variants

### StableSwap (for stablecoins)

Raydium and Orca use StableSwap for USDC/USDT pairs:

```
A × x × y = k (for prices near 1:1)

Result: Much lower slippage for stable pairs
```

### Weighted Pools (Balancer Style)

Pools with custom weights:

```
80% SOL / 20% USDC

Provides more SOL exposure with less IL
```

## Price Impact Calculation

Price impact is the slippage from your trade:

```javascript
function calculatePriceImpact(inputAmount, reserveIn, reserveOut) {
  const constantProduct = reserveIn * reserveOut;
  const newReserveIn = reserveIn + inputAmount;
  const newReserveOut = constantProduct / newReserveIn;
  const outputAmount = reserveOut - newReserveOut;

  const executionPrice = inputAmount / outputAmount;
  const spotPrice = reserveOut / reserveIn;

  const priceImpact = (executionPrice / spotPrice) - 1;
  return priceImpact * 100; // As percentage
}
```

## MEV and AMMs

AMMs are vulnerable to MEV (Maximal Extractable Value):

### Sandwich Attacks

1. Bot sees your large buy order in mempool
2. Bot front-runs with a buy (pushes price up)
3. Your transaction executes (at worse price)
4. Bot back-runs with a sell (profits from price increase)

**Mitigation**: Slippage tolerance, private RPCs (Jito), batch auctions

### Just-in-Time (JIT) Liquidity

1. LP sees large swap incoming
2. LP adds concentrated liquidity in that block
3. LP earns most of the swap fee
4. LP withdraws liquidity immediately

This is actually beneficial (provides liquidity when needed), but can be seen as unfair to long-term LPs.

## Real-World AMM Strategies

### Passive LP

- Deposit to stable pairs (USDC/USDT)
- Wide range or full-range
- Low IL, steady fees

### Active LP

- Concentrated positions on volatile pairs
- Rebalance ranges frequently
- Higher risk, higher reward

### Liquidity Mining

- Provide liquidity to incentivized pools
- Earn swap fees + token rewards
- Watch for impermanent loss vs. rewards

## Next Challenge

You'll implement the constant product AMM formula and calculate swaps, liquidity, and price impact.
