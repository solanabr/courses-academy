/**
 * Convert base units to display units using decimals
 * @param {number} baseAmount - Amount in base units (e.g., 1_000_000)
 * @param {number} decimals - Token decimals (e.g., 6 for USDC)
 * @returns {number} - Amount in display units (e.g., 1.0)
 */
function baseToDisplay(baseAmount, decimals) {
  // TODO: Divide baseAmount by 10^decimals
  return 0;
}

/**
 * Calculate token price from pool reserves
 * @param {number} reserveA - Reserve of token A
 * @param {number} reserveB - Reserve of token B
 * @returns {number} - Price of token A in terms of token B
 */
function calculatePrice(reserveA, reserveB) {
  // TODO: Price of A = reserveB / reserveA
  // Handle division by zero
  return 0;
}

/**
 * Calculate user's LP token share percentage
 * @param {number} userLpTokens - User's LP token balance
 * @param {number} totalLpTokens - Total LP tokens in pool
 * @returns {number} - Percentage share (0-100)
 */
function calculateLpShare(userLpTokens, totalLpTokens) {
  // TODO: (userLpTokens / totalLpTokens) * 100
  // Handle division by zero
  return 0;
}

console.log(baseToDisplay(1000000, 6)); // Should output: 1
console.log(calculatePrice(100000, 200000)); // Should output: 2
console.log(calculateLpShare(500, 10000)); // Should output: 5
