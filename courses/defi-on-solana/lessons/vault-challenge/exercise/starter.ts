const vault = {
  totalAssets: 0,
  totalShares: 0,
  users: {} // username -> shares balance
};

/**
 * Deposit assets into vault and mint shares
 * @param {string} user - Username
 * @param {number} amount - Amount to deposit
 */
function deposit(user, amount) {
  // TODO: Calculate shares to mint
  // If vault is empty: shares = amount (1:1 ratio)
  // Otherwise: shares = (amount * totalShares) / totalAssets
  
  // TODO: Update vault state
  // vault.totalAssets += amount
  // vault.totalShares += shares
  
  // TODO: Update user balance
  // vault.users[user] = (vault.users[user] || 0) + shares
  
  return 0;
}

/**
 * Withdraw assets from vault by burning shares
 * @param {string} user - Username
 * @param {number} shares - Number of shares to burn
 * @returns {number} - Amount of assets withdrawn
 */
function withdraw(user, shares) {
  // TODO: Calculate assets to return
  // assets = (shares * totalAssets) / totalShares
  
  // TODO: Update vault state
  // vault.totalShares -= shares
  // vault.totalAssets -= assets
  
  // TODO: Update user balance
  // vault.users[user] -= shares
  
  return 0;
}

/**
 * Simulate yield generation
 * @param {number} amount - Yield amount to add
 */
function addYield(amount) {
  // TODO: Add to totalAssets (increases exchange rate)
  vault.totalAssets += amount;
}
