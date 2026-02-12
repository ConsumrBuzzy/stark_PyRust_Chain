# ADR-038: Pulse Automation Architecture

## Status
Accepted / Implemented

## Context
Local scripts rely on "uptime". We need cloud-native persistence.

## Decision
1.  **GitHub Actions**: Use `schedule` event for 30-minute intervals.
2.  **Jitter**: Schedule at `27,57` minutes to avoid congestion.
3.  **Secrets**: Inject sensitive keys via GitHub Secrets (`STARKNET_PRIVATE_KEY`).
4.  **Masking**: Custom logger masks secrets in stdout.

## Usage
Implemented in `.github/workflows/pulse.yml` and `orchestrator.py`.
