# ADR-031: Ghost Scanner Methodology

## Status
Accepted / Implemented

## Context
Blind execution leads to negative ROI due to market volatility.

## Decision
1.  **Pre-Validation**: Fetch Market Order Book (Iron, Steel, Propellant).
2.  **Spread Calculation**: `(Steel * 100) - (Iron * 250 + Propellant * 20 + Lease)`.
3.  **Gatekeeper**: Require `Net Profit > 150 SWAY` to proceed.

## Usage
Implemented in `pre_check.py` and `influence_api.rs`.
