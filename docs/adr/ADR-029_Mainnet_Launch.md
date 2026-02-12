# ADR-029: Mainnet Launch Protocol

## Status
Accepted / Implemented

## Context
Transitioning from Testnet to Mainnet introduces real economic risk (Gas Fees).

## Decision
1.  **Gas Cap**: Abort any transaction if `L1 Gas Price > 30 Gwei`.
2.  **Solvency Check**: Ensure `ETH Balance > 0.005` (approx $12.00) before operation.
3.  **Slippage**: Slippage tolerance set to 1% for Dex interactions (Future proofing).

## Usage
Implemented in `starknet_client.rs` (`get_network_status`) and `strategy_module.py`.
