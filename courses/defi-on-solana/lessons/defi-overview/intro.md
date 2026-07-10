# DeFi on Solana Overview

Decentralized Finance (DeFi) on Solana has exploded in growth thanks to the chain's speed, low costs, and composability. Let's explore the landscape.

## Why Solana for DeFi?

### Speed
- **400ms block time** - Near-instant transaction finality
- **65,000+ TPS** - Handles high-frequency trading
- **Parallel execution** - Multiple programs run simultaneously

### Cost
- **$0.00025 average transaction** - Makes micro-transactions viable
- **No gas wars** - Predictable fees even during congestion

### Composability
- **Single global state** - All programs share the same state
- **Atomic composability** - Multi-program transactions are atomic
- **No bridges needed** - Everything is native

## The Solana DeFi Ecosystem

### Decentralized Exchanges (DEXs)

**Jupiter** - Aggregates liquidity from all Solana DEXs
- Routes trades through multiple DEXs for best price
- 15+ DEX integrations
- Limit orders, DCA, perpetuals

**Raydium** - Automated Market Maker (AMM)
- Concentrated liquidity (like Uniswap v3)
- OpenBook integration (hybrid AMM/orderbook)
- Top 3 by volume on Solana

**Orca** - User-friendly AMM
- Whirlpools (concentrated liquidity)
- Clean UX, popular for beginners

**Phoenix** - Central Limit Order Book (CLOB)
- Fully on-chain orderbook
- Sub-second matching
- Successor to Serum

### Lending Protocols

**Solend** - Algorithmic lending/borrowing
- Supply assets, earn yield
- Borrow against collateral
- Isolated pools for risk management

**MarginFi** - Decentralized margin trading
- Leveraged positions
- Portfolio margin
- Integrated with major DEXs

**Kamino Finance** - Automated liquidity strategies
- Vaults for concentrated liquidity
- Leveraged yield farming
- Risk-adjusted returns

### Liquid Staking

**Marinade (mSOL)** - Largest liquid staking protocol
- Stake SOL, receive mSOL
- mSOL accrues staking rewards
- Can use mSOL in DeFi while earning staking yield

**Jito (jitoSOL)** - MEV-enhanced staking
- Distributes MEV rewards to stakers
- Higher APY than native staking

**BlazeStake (bSOL)** - Focuses on decentralization
- Delegates to smaller validators
- Supports Solana decentralization

### Perpetuals & Derivatives

**Drift Protocol** - Decentralized perpetuals
- Up to 10x leverage
- Virtual AMM design
- Insurance fund for trader protection

**Zeta Markets** - Options and futures
- Undercollateralized DeFi options
- On-chain orderbook

## Total Value Locked (TVL)

As of 2025, Solana DeFi TVL:
- **Total**: ~$7B USD
- **Jito**: $2.5B (liquid staking)
- **Kamino**: $1.8B (lending/vaults)
- **Marinade**: $1.2B (liquid staking)
- **Raydium**: $1.5B (DEX)

Solana is the 4th largest DeFi ecosystem after Ethereum, Tron, and BNB Chain.

## DeFi Primitives on Solana

### 1. Automated Market Makers (AMMs)

Constant product formula: `x * y = k`

```
Pool: 1000 SOL × 100,000 USDC = 100,000,000
Price: 100,000 / 1000 = 100 USDC per SOL
```

### 2. Concentrated Liquidity

Liquidity providers choose a price range:

```
Standard AMM: Capital spread across 0 → ∞
Concentrated: Capital focused on $95-$105 range
Result: 10-100x capital efficiency
```

### 3. Lending Pools

Supply rate = Borrow rate × Utilization × (1 - Reserve factor)

```
Total supplied: 1000 SOL
Total borrowed: 700 SOL
Utilization: 70%
Borrow APY: 8%
Supply APY: 8% × 70% × 90% = 5.04%
```

### 4. Liquidations

```
Health Factor = Collateral Value / (Debt Value / LTV)
If Health Factor < 1.0 → Liquidatable
```

## Composability in Action

**Example: Leveraged Yield Farming**

1. Deposit SOL to Kamino
2. Borrow USDC against SOL collateral
3. Swap USDC → SOL on Jupiter
4. Deposit SOL to liquidity pool on Raydium
5. Earn swap fees + USDC rewards

All of this can happen in **one atomic transaction**.

## Security Considerations

### Smart Contract Risk
- Programs can have bugs (test thoroughly, get audits)
- Upgradeable programs can be changed by authority
- Check if program authority is a multisig or revoked

### Oracle Risk
- Price feeds can be manipulated or go stale
- Use multiple oracles (Pyth, Switchboard)
- Implement staleness checks and circuit breakers

### Liquidation Risk
- Solana's speed helps (faster liquidations = less bad debt)
- But volatility can still cause cascading liquidations

### Protocol Composability Risk
- A bug in one protocol can cascade to others
- Isolated pools mitigate this

## The Future of Solana DeFi

**Token Extensions (Token-2022)**
- Transfer hooks (fees, restrictions)
- Confidential transfers (privacy)
- Interest-bearing tokens

**Firedancer**
- New validator client → 1M+ TPS
- Even faster DeFi execution

**Cross-chain**
- Wormhole, Allbridge for bridging
- But Solana's goal is to be self-contained

## Next Lessons

You'll dive deep into AMM mechanics, lending protocols, and build DeFi primitives yourself.
