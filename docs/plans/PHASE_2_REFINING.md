# ADR-028: Phase 2 Execution — The Refining Spread Logic
**Status:** Proposed

## Context
We are moving into Phase 2 (Implementation). The goal is to transform the `stark_PyRust_Chain` from a connectivity framework into an active Supply Chain Orchestrator. This ADR defines the logic for the "Refining Spread" strategy, specifically targeting the **Iron → Steel** pipeline on Adalia.

## 1. Logic Mapping: The "Refining" Edge
To satisfy the Generalization and KISS protocols, the Rust core will handle the math, while Python handles the threshold policy.

- **Inputs**: 250 Iron Ore + 20 Fuel (Propellant).
- **Outputs**: 100 Steel.
- **Energy Cost**: 480kW (as mapped in ADR-024).

**Math Logic (LaTeX):**
$$ Profit = (P_{Steel} \times 100) - [(P_{Iron} \times 250) + (P_{Fuel} \times 20) + Fee_{Gas} + Cost_{Energy}] $$

## 2. Implementation Directive for PyPro-Systems
**Agent Directive:** Execute the Phase 2 Sprint exactly as outlined in the provided Implementation Plan.

### Rust Core (`rust-core/src/supply_chain.rs`)
- **Hardcode** the Iron to Steel recipe.
- **Implement** `calculate_profitability`. It must take current market prices as arguments and return a f64 profit value.

### Python Logic (`python-logic/strategy_module.py`)
- **Initialize** the `RefiningStrategy` class.
- **Protocol Check**: Ensure the "Poll Asteroid Inventory" step uses the `PyInfluenceClient` with its built-in Rate Limiter (5 req/s).
- **Headed-First Validation**: If `DRY_RUN=True`, log the raw JSON payload of the transaction to the console instead of submitting.

### Orchestrator (`python-logic/orchestrator.py`)
- **Bind** the `start` command to the `RefiningStrategy` loop.
- **Implement** a 60-second "Sleep" between market scans to maintain healthy API relations.

## 3. Security: Vault Integration
The `RefiningStrategy` must never touch the `.env` directly. It must call `PyVault.get_secret("STARKNET_PRIVATE_KEY")` or use the encrypted Session Key generated during the wizard phase.

## Context Health Check
- **Current Priority**: Iron to Steel Automated Spread.
- **Architecture**: Adjacency List (Rust) → Policy Layer (Python).
- **Safety**: Dry Run enabled by default.
- **Target**: 120-day ROI breakeven on $15 initial capital.
