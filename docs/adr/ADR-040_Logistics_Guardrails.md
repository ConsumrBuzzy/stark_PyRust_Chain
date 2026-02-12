# ADR-040: Logistics & Nonce Guardrails

## Status
Accepted / Implemented

## Context
-   **Nonce**: Simultaneous runs cause collisions.
-   **Logistics**: Distance eats profit.

## Decision
1.  **Nonce Persistence**: Fetch `get_nonce()` from RPC at start of every Pulse.
2.  **Logistics Penalty**: Subtract `15 SWAY` per Lot of distance from Projected Profit.
3.  **Inventory**: Check Output Capacity before refining.

## Usage
Implemented in `starknet_client.rs` and `pre_check.py`.
