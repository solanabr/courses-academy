/**
 * Check if a lending position is liquidatable
 *
 * Health Factor = collateral / debt
 * A position is liquidatable when healthFactor < collateralFactor (threshold)
 *
 * In DeFi lending (Solend, MarginFi), borrowers post collateral.
 * If collateral value drops below the threshold, liquidators can
 * repay the debt and seize collateral + a bonus.
 *
 * @param {number} collateral - Collateral value in USD
 * @param {number} debt - Debt value in USD
 * @param {number} collateralFactor - Liquidation threshold (e.g., 1.5 = 150%)
 * @returns {object} - { isLiquidatable, healthFactor }
 */
function checkLiquidation(collateral, debt, collateralFactor) {
  // TODO: Handle edge case where debt is 0 (no debt = never liquidatable)
  
  // TODO: Calculate health factor
  // healthFactor = collateral / debt
  
  // TODO: Determine if position is liquidatable
  // isLiquidatable = healthFactor < collateralFactor
  
  return {
    isLiquidatable: false,
    healthFactor: 0
  };
}

// Test cases
console.log(checkLiquidation(100, 50, 1.5));
// Expected: { isLiquidatable: false, healthFactor: 2.0 } (100/50 = 2.0 > 1.5)
console.log(checkLiquidation(100, 80, 1.5));
// Expected: { isLiquidatable: true, healthFactor: 1.25 } (100/80 = 1.25 < 1.5)
