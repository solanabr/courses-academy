# Price Oracles for DeFi

DeFi protocols need reliable asset prices to function. Lending platforms must know collateral values, DEXs need reference prices for limit orders, and derivatives require settlement prices. Price oracles solve this problem by bringing off-chain price data on-chain.

## Oracle Mechanisms

**Push oracles** proactively write price updates to on-chain accounts on a schedule (e.g., every 60 seconds) or when prices deviate beyond a threshold. Protocols simply read the latest price from the oracle account. Switchboard uses this model with validator-operated data feeds.

**Pull oracles** store signed price attestations off-chain. Protocols fetch the latest attestation when needed and submit it with their transaction. The oracle program verifies the signature before allowing the price to be used. Pyth Network pioneered this model for Solana.

## Pyth Network

Pyth aggregates price data from over 90 first-party sources including exchanges, market makers, and trading firms. Each data provider signs their price contribution, and the on-chain program computes a confidence-weighted median.

**Key features**:
- Sub-second price updates for major assets
- Confidence intervals indicating price uncertainty
- Exponentially-weighted moving average (EMA) for smooth prices
- Publisher attribution for transparency

Pyth's pull model reduces costs by only updating on-chain prices when transactions need them, rather than constantly pushing updates.

## Time-Weighted Average Price (TWAP)

TWAP oracles compute the average price over a time window, making price manipulation expensive. An attacker would need to sustain artificial prices for the entire window, not just a single block.

Protocols can construct TWAP from Pyth EMA prices or compute it directly from DEX swap observations. TWAP is essential for lending protocols to prevent flash loan attacks that temporarily spike prices.

## Oracle Safety

Robust oracle integration requires:
- **Staleness checks**: Reject prices older than acceptable threshold (e.g., 60 seconds)
- **Confidence intervals**: Only use prices with acceptable confidence (low uncertainty)
- **Circuit breakers**: Halt operations if price deviates too far from recent values
- **Multi-oracle redundancy**: Compare prices from multiple sources

Poor oracle design has caused hundreds of millions in DeFi exploits. Always validate oracle data before using it in critical logic.
