function calculateStakingRewards(stakeAmount, apyPercent, epochDurationDays, numEpochs) {
  const epochsPerYear = 365 / epochDurationDays;
  const annualMultiplier = 1 + (apyPercent / 100);
  const epochRate = Math.pow(annualMultiplier, epochDurationDays / 365) - 1;
  
  const finalBalance = stakeAmount * Math.pow(1 + epochRate, numEpochs);
  
  const totalDays = epochDurationDays * numEpochs;
  const effectiveApy = (Math.pow(finalBalance / stakeAmount, 365 / totalDays) - 1) * 100;
  
  const totalRewards = finalBalance - stakeAmount;
  
  return {
    finalBalance: finalBalance,
    effectiveApy: effectiveApy,
    totalRewards: totalRewards
  };
}

const result = calculateStakingRewards(100, 7.5, 2.5, 146);
console.log(result.finalBalance);
console.log(result.effectiveApy);
console.log(result.totalRewards);
