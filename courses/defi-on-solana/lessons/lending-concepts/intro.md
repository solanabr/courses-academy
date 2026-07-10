# Lending & Borrowing Protocols

Lending protocols are core DeFi primitives that enable users to earn yield on deposits and borrow against collateral.

## How Lending Works

### Supplying (Lending)

1. User deposits tokens into a lending pool
2. User receives **pool tokens** (e.g., deposit SOL, receive cSOL)
3. Pool tokens earn interest (redeem for more SOL over time)
4. Can withdraw anytime (if liquidity available)

### Borrowing

1. User deposits collateral (e.g., SOL)
2. User borrows against collateral (e.g., borrow USDC)
3. User pays interest on borrowed amount
4. Must maintain **collateralization ratio** or face liquidation

## Interest Rates

Interest rates are **algorithmic** (set by supply/demand, not by humans).

### Utilization Rate

```
Utilization = Total Borrowed / Total Supplied

Example:
Total Supplied: 1,000 SOL
Total Borrowed: 700 SOL
Utilization: 70%
```

High utilization → higher borrow rates (to incentivize more supply)

### Interest Rate Model

Typical model has two slopes:

```
if Utilization < 80%:
  Borrow APY = Base + (Utilization × Multiplier)
else:
  Borrow APY = Base + (80% × Multiplier) + ((Utilization - 80%) × JumpMultiplier)

Example parameters:
Base = 2%
Multiplier = 5%
JumpMultiplier = 100%

At 70% utilization:
  Borrow APY = 2% + (70% × 5%) = 5.5%

At 90% utilization:
  Borrow APY = 2% + (80% × 5%) + (10% × 100%) = 16%
```

The "kink" at 80% prevents the pool from running out of liquidity.

### Supply APY Calculation

```
Supply APY = Borrow APY × Utilization × (1 - Reserve Factor)

Reserve Factor = 10% (protocol fee)
Utilization = 70%
Borrow APY = 8%

Supply APY = 8% × 70% × 90% = 5.04%
```

## Overcollateralization

Crypto lending requires **overcollateralization** (you can't borrow more than your collateral value).

```
User deposits: 10 SOL @ $100 = $1,000
Max LTV (Loan-to-Value): 75%
Max borrow: $1,000 × 75% = $750 USDC
```

Why overcollateralization?
- No credit scores in DeFi
- Instant liquidation if collateral drops
- Protects lenders from bad debt

## Collateralization Ratios

### LTV (Loan-to-Value)

Maximum you can borrow against collateral:

```
SOL LTV: 75%
Means: For every $100 SOL, borrow up to $75
```

### Liquidation Threshold

When your position can be liquidated:

```
SOL Liquidation Threshold: 80%
Means: If debt reaches 80% of collateral value → liquidation
```

Liquidation threshold > LTV provides a safety buffer.

### Health Factor

```
Health Factor = (Collateral Value × Liquidation Threshold) / Debt Value

Example:
Collateral: 10 SOL @ $100 = $1,000
Liquidation Threshold: 80%
Debt: $750 USDC

Health Factor = ($1,000 × 0.8) / $750 = 1.067

If Health Factor < 1.0 → Liquidatable
```

## Liquidations

When a position becomes undercollateralized, liquidators can repay the debt and seize collateral.

### Liquidation Process

1. Health factor drops below 1.0
2. Liquidator repays portion of debt (close factor)
3. Liquidator receives collateral + bonus
4. Remaining position (if any) is healthier

### Example

```
Borrower:
Collateral: 10 SOL @ $90 = $900
Debt: $750 USDC
Liquidation Threshold: 80%
Health Factor: ($900 × 0.8) / $750 = 0.96 ❌

Liquidation:
Close Factor: 50% (can liquidate up to 50% of debt)
Liquidator repays: $375 USDC
Collateral seized: $375 / $90 = 4.17 SOL
Liquidation Bonus: 5%
Actual seized: 4.17 × 1.05 = 4.38 SOL

After liquidation:
Borrower has: 5.62 SOL, $375 debt
Health Factor: ($506 × 0.8) / $375 = 1.08 ✅
```

### Close Factor

Limits how much of a position can be liquidated at once:

```
Close Factor = 50%
Means: Max 50% of debt can be repaid in one liquidation

Prevents: Liquidating entire position when only slightly undercollateralized
```

## Risk Parameters

Each asset has custom risk parameters:

| Asset | LTV | Liq Threshold | Liq Bonus |
|-------|-----|---------------|----------|
| SOL | 75% | 80% | 5% |
| USDC | 80% | 85% | 4% |
| BONK | 50% | 60% | 15% |

More volatile/risky assets → lower LTV, higher liquidation bonus.

## Isolated Pools

To manage risk, protocols create **isolated pools**:

```
Main Pool: SOL, USDC, USDT, ETH (blue-chip assets)
Isolated Pool 1: SOL + New Token X
Isolated Pool 2: USDC + Experimental Asset Y
```

If one isolated pool has bad debt, it doesn't affect others.

## Borrowing Strategies

### Leveraged Long

1. Deposit 10 SOL as collateral
2. Borrow 7 SOL worth of USDC
3. Buy 7 more SOL with USDC
4. Now have 17 SOL exposure with 10 SOL capital
5. If SOL ↑ 10%, profit is 17% (minus borrow costs)

### Yield Farming

1. Deposit stablecoins, borrow SOL
2. Use SOL in high-APY liquidity pool
3. Earn more from LP fees than borrow cost
4. Profit = LP APY - Borrow APY

### Short Position

1. Deposit USDC collateral
2. Borrow SOL
3. Sell SOL for USDC
4. If SOL drops, buy back cheaper and repay

## Oracle Dependency

Lending protocols rely on **oracles** for price feeds:

### Pyth Network

- Pull-based oracle (request price on-demand)
- Sub-second updates
- 350+ price feeds

### Switchboard

- Decentralized oracle network
- Customizable feeds
- TEE (Trusted Execution Environment) support

### Oracle Risks

- **Stale prices**: If oracle doesn't update, liquidations may fail or be unfair
- **Manipulation**: Flash loan attacks can manipulate some oracles
- **Downtime**: If oracle is down, protocol may pause

Mitigation: Use multiple oracles, TWAP (Time-Weighted Average Price), circuit breakers.

## Protocol Revenue

```
Reserve Factor: 10%
Total Interest Paid: $100,000
To Suppliers: $90,000
To Protocol: $10,000
```

Protocols earn from the spread between borrow and supply rates.

## Real-World Examples

### Solend

- TVL: ~$300M
- Main pool + multiple isolated pools
- Pyth oracles
- SLND token rewards

### MarginFi

- TVL: ~$600M
- Risk-adjusted portfolio margin
- Integrated with DEXs for leveraged trading

### Kamino Finance

- TVL: ~$1.8B
- Focuses on automated liquidity strategies
- Multiply (leveraged yield vaults)

## Next Challenge

You'll implement the constant product AMM formula with fees, price impact, and liquidity calculations.
