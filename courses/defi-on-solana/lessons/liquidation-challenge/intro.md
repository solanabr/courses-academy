# Liquidation Engine

Build a liquidation engine for a lending protocol.

Lending platforms like Solend and MarginFi allow users to borrow against collateral. If the collateral value drops too much, the position becomes unhealthy and must be liquidated to protect the protocol.

**Health Factor**: `(collateral_value * collateral_factor) / debt_value`
- Health > 1.0: Position is safe
- Health < 1.0: Position is liquidatable

**Liquidation**: A liquidator repays the debt and receives the collateral plus a bonus (typically 5-10%).

Your challenge: implement health factor calculation, identify liquidatable positions, and execute liquidations with proper accounting.
