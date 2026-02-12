# ADR-035: Cross-Chain Onramp

## Status
Accepted / Implemented

## Context
Manual funding is inefficient. Need a programmatic bridge from Coinbase to Starknet.

## Decision
1.  **Bridge Logic**: Simulate/Integrate Coinbase CDP (Stargate/Layerswap).
2.  **Validation**: Enforce Regex Validation (`^0x[a-fA-F0-9]{60,66}$`) on destination address.
3.  **Safety**: Dry-run verification of source USDC balance.

## Usage
Implemented in `onramp.py`.
