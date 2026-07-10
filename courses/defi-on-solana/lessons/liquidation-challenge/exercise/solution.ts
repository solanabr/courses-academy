function checkLiquidation(collateral, debt, collateralFactor) {
  if (debt === 0) {
    return { isLiquidatable: false, healthFactor: Infinity };
  }
  
  const healthFactor = collateral / debt;
  const isLiquidatable = healthFactor < collateralFactor;
  
  return { isLiquidatable, healthFactor };
}

console.log(checkLiquidation(100, 50, 1.5));
console.log(checkLiquidation(100, 80, 1.5));
