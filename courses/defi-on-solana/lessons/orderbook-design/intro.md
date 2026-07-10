# On-Chain Orderbooks on Solana

Orderbook-based decentralized exchanges (DEXs) bring the familiar trading experience of centralized exchanges to DeFi. Solana's high throughput and low latency make it uniquely suited for on-chain orderbooks.

## Central Limit Order Books (CLOBs)

A CLOB maintains separate bid (buy) and ask (sell) order lists sorted by price. When a new order arrives, the matching engine searches for counterparty orders at compatible prices. Matching happens on a price-time priority basis: the best price gets filled first, and among orders at the same price, earlier orders have priority.

**Key protocols**:
- **Phoenix**: Next-generation CLOB with optimized matching and reduced memory footprint
- **OpenBook**: Community fork of Serum's original CLOB implementation
- **Zeta Markets**: Options and futures orderbook specialized for derivatives

## Orderbooks vs AMMs

Automated Market Makers (AMMs) like Raydium and Orca use liquidity pools with algorithmic pricing (x*y=k). Orderbooks offer several advantages:

**Capital efficiency**: Liquidity providers can specify exact prices instead of providing liquidity across the entire price curve. A market maker can place tight bid-ask spreads at current market price without locking capital in extreme price ranges.

**Price discovery**: Order flow directly determines market price rather than relying on arbitrageurs to rebalance pools. This enables better price discovery, especially for less liquid assets.

**Professional trading tools**: Limit orders, stop losses, fill-or-kill orders, and post-only orders give traders precise execution control.

## Solana's Advantages

Traditional blockchain throughput (15-30 TPS) makes on-chain orderbooks impractical due to slow settlement and high fees. Solana's 400ms block times and 65,000 TPS capacity enable:

- Real-time order updates visible to all participants
- Sub-second trade settlement and confirmation
- Low fees enabling profitable market making with tight spreads
- Atomic order matching preventing front-running

Orderbooks represent the future of professional DeFi trading, combining CEX user experience with DEX trustlessness.
