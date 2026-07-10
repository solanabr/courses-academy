# Challenge: Constant Product AMM

Implement a constant product AMM with swap, liquidity, and price impact calculations.

## Requirements

Implement three functions:

### 1. `swap(reserveIn, reserveOut, inputAmount, fee)`

Calculate output amount for a swap:
- Apply fee (e.g., 0.3% means fee = 0.003)
- Use constant product formula: `x * y = k`
- Return `{ outputAmount, newReserveIn, newReserveOut, priceImpact }`

### 2. `addLiquidity(reserveA, reserveB, depositA, depositB, totalLPTokens)`

Calculate LP tokens minted:
- If pool is empty (totalLPTokens === 0), mint `sqrt(depositA * depositB)`
- Otherwise, mint proportional to deposit: `min(depositA/reserveA, depositB/reserveB) * totalLPTokens`
- Return `{ lpTokens, finalDepositA, finalDepositB }`

### 3. `calculatePriceImpact(inputAmount, reserveIn, reserveOut)`

Calculate price impact percentage:
- Spot price: `reserveOut / reserveIn`
- Execution price: `outputAmount / inputAmount`
- Price impact: `((executionPrice / spotPrice) - 1) * 100`
- Return price impact as a percentage (negative value means worse than spot)
