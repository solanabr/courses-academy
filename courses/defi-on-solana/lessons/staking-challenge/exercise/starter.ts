/**
 * Calculate staking rewards with compound interest
 * @param {number} stakeAmount - Initial stake in SOL
 * @param {number} apyPercent - Annual percentage yield (e.g., 7.5 for 7.5%)
 * @param {number} epochDurationDays - Days per epoch (e.g., 2.5)
 * @param {number} numEpochs - Number of epochs to calculate
 * @returns {object} - { finalBalance, effectiveApy, totalRewards }
 */
function calculateStakingRewards(stakeAmount, apyPercent, epochDurationDays, numEpochs) {
  // TODO: Calculate epoch rate from APY
  // epochsPerYear = 365 / epochDurationDays
  // epochRate = (1 + apyPercent/100)^(epochDurationDays/365) - 1
  
  // TODO: Apply compound interest over numEpochs
  // finalBalance = stakeAmount * (1 + epochRate)^numEpochs
  
  // TODO: Calculate effective APY from final balance
  // effectiveApy = ((finalBalance / stakeAmount)^(365/(epochDurationDays*numEpochs)) - 1) * 100
  
  // TODO: Calculate total rewards
  // totalRewards = finalBalance - stakeAmount
  
  return {
    finalBalance: 0,
    effectiveApy: 0,
    totalRewards: 0
  };
}

const result = calculateStakingRewards(100, 7.5, 2.5, 146);
console.log(result.finalBalance); // Should be ~107.5
console.log(result.effectiveApy); // Should be ~7.5
console.log(result.totalRewards); // Should be ~7.5
