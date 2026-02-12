# ADR-027: stark_PyRust_Chain — Framework Consolidation & Session Handshake

**Status:** Accepted

## Context
We have completed the foundational scaffolding for `stark_PyRust_Chain`. The architecture successfully bridges a high-performance Rust Core (AES-256 Vault, Multicall-capable Starknet Client, and Influence SAGE API) with a Python Logic Layer (Typer/Rich Orchestrator). This ADR codifies the current state and provides the "Source Truth" for the Pair Programming Agent to begin strategy implementation.

## 1. Architectural Decision: The Hybrid "Chain" Pattern
To satisfy the KISS principle while maintaining Technical Visionary performance, we have established the following boundaries:

### Rust Layer (The Engine)
- **`vault.rs`**: Handles AES-256-GCM encryption for master secrets.
- **`starknet_client.rs`**: Asynchronous batching of JSON-RPC calls via `starknet-rs`.
- **`session_keys.rs`**: Native Account Abstraction (AA) support to generate temporary ECDSA keys, reducing the need for constant master-key access.
- **`influence_api.rs`**: Direct integration with Influence (SAGE Labs) for sub-second market and asteroid state polling.

### Python Layer (The Policy)
- **`orchestrator.py`**: A wizard-driven CLI that simplifies the "Introduction Player" experience.
- **`strategy_module.py`**: A generalized base class where "Resource Spreads" (e.g., Iron to Steel) are defined.

## 2. The "Session Handshake" Protocol
To avoid the security risk of storing master private keys in `.env` for the "Decade Device" lifecycle, we have implemented the Session Key Wizard:
1.  **Generation**: Rust generates an ephemeral ECDSA key pair.
2.  **Authorization**: Python generates a JSON payload for the user to sign via their Argent/Braavos wallet.
3.  **Persistence**: The signed authorization is encrypted in the Vault and used by the bot for all game-actions (Mining/Refining) for the next 24 hours.

## 3. Operations & Verification
### Verification Checklist (Health Check)
| Component | Status | Verification Path |
| :--- | :--- | :--- |
| **Rust Vault** | VERIFIED | Encrypts/Decrypts correctly via `PyVault`. |
| **API Rate Limiter** | ACTIVE | Throttles to 5 req/s via `governor` crate. |
| **Session Key Gen** | VERIFIED | Produces valid Starknet-compatible signing payloads. |
| **Orchestrator Wizard** | READY | Guided setup for RPC and API keys is functional. |

## 4. Context Health Check
- **Current State**: Framework consolidated; Installation verified.
- **Targeting**: First automated "Refining" action on Mainnet.
- **Safety**: `DRY_RUN=true` remains the default in `strategy_module.py`.
