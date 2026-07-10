function swap(reserveIn, reserveOut, inputAmount, fee) {
  // Apply fee
  const inputAfterFee = inputAmount * (1 - fee);

  // Constant product: k = x * y
  const k = reserveIn * reserveOut;

  // New reserve in
  const newReserveIn = reserveIn + inputAfterFee;

  // Calculate new reserve out from constant product
  const newReserveOut = k / newReserveIn;

  // Output amount
  const outputAmount = reserveOut - newReserveOut;

  // Price impact
  const spotPrice = reserveOut / reserveIn;
  const executionPrice = outputAmount / inputAmount;
  const priceImpact = ((executionPrice / spotPrice) - 1) * 100;

  return {
    outputAmount,
    newReserveIn: reserveIn + inputAmount,  // Include fee in reserves
    newReserveOut,
    priceImpact,
  };
}

function addLiquidity(reserveA, reserveB, depositA, depositB, totalLPTokens) {
  let lpTokens;
  let finalDepositA = depositA;
  let finalDepositB = depositB;

  if (totalLPTokens === 0) {
    // First liquidity provider
    lpTokens = Math.sqrt(depositA * depositB);
  } else {
    // Calculate proportional LP tokens
    const ratioA = depositA / reserveA;
    const ratioB = depositB / reserveB;

    // Use minimum ratio to prevent imbalance
    const ratio = Math.min(ratioA, ratioB);
    lpTokens = ratio * totalLPTokens;

    // Adjust deposits to match pool ratio
    finalDepositA = ratio * reserveA;
    finalDepositB = ratio * reserveB;
  }

  return {
    lpTokens,
    finalDepositA,
    finalDepositB,
  };
}

function calculatePriceImpact(inputAmount, reserveIn, reserveOut) {
  const spotPrice = reserveOut / reserveIn;

  // Calculate output (assuming 0 fee for price impact)
  const k = reserveIn * reserveOut;
  const newReserveIn = reserveIn + inputAmount;
  const newReserveOut = k / newReserveIn;
  const outputAmount = reserveOut - newReserveOut;

  const executionPrice = outputAmount / inputAmount;
  const priceImpact = ((executionPrice / spotPrice) - 1) * 100;

  return priceImpact;
}
