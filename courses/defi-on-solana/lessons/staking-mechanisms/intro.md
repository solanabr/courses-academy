# Staking Mechanisms on Solana

Staking is fundamental to Solana's proof-of-stake consensus and DeFi ecosystem. Understanding staking mechanisms is essential for building yield-generating applications.

## Native Staking

Solana's native staking allows SOL holders to delegate their tokens to validators. Validators process transactions and secure the network, earning rewards that are shared with delegators. Staking requires locking SOL for the duration of an epoch (approximately 2-3 days), during which it cannot be transferred.

**Key mechanics**:
- **Delegation**: Users create a stake account and delegate to a validator
- **Activation**: Stakes activate at the next epoch boundary
- **Deactivation**: Unstaking requires waiting for the current epoch to end
- **Rewards**: Automatically compounded into the stake account

## Liquid Staking Tokens

Liquid staking protocols like Marinade (mSOL) and Jito (jitoSOL) solve the liquidity problem. Users deposit SOL and receive liquid staking tokens (LSTs) that represent their staked position plus accrued rewards. LSTs can be traded, used as collateral, or deployed in DeFi while still earning staking yields.

**Advantages**:
- Instant liquidity without waiting for epoch changes
- Automatic validator diversification across multiple validators
- Compound staking rewards with DeFi yields
- Exchange rate appreciation as rewards accrue

## Stake Pools

The Solana Program Library includes the Stake Pool program, which enables permissionless creation of staking pools. Pool managers can add validators, set fees, and manage pool operations. The pool issues pool tokens that track the stake account value.

**Rewards calculation**: Staking APY varies by validator commission (typically 5-10%) and network inflation rate. Effective APY = (1 + epoch_rate)^epochs_per_year - 1, accounting for compound interest.

Understanding these mechanisms is critical for building staking UIs, yield aggregators, and advanced DeFi protocols that leverage staked SOL as collateral.
