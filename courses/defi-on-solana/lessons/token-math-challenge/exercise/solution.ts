function baseToDisplay(baseAmount, decimals) {
  return baseAmount / Math.pow(10, decimals);
}

function calculatePrice(reserveA, reserveB) {
  if (reserveA === 0) return 0;
  return reserveB / reserveA;
}

function calculateLpShare(userLpTokens, totalLpTokens) {
  if (totalLpTokens === 0) return 0;
  return (userLpTokens / totalLpTokens) * 100;
}

console.log(baseToDisplay(1000000, 6));
console.log(calculatePrice(100000, 200000));
console.log(calculateLpShare(500, 10000));
