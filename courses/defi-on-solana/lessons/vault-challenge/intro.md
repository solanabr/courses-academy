# DeFi Vault Simulator

Build a simplified DeFi vault that tracks deposits, shares, and exchange rates.

Your vault accepts deposits of an underlying token and mints vault shares. As the vault generates yield, the exchange rate between shares and underlying increases. Users can redeem shares for their proportional amount of underlying assets.

**Key concepts**:
- Track total underlying assets and total shares issued
- Calculate exchange rate when depositing and withdrawing
- Ensure the first depositor doesn't get unfair advantages

This is the core mechanic behind yield aggregators like Francium and Tulip Protocol.
