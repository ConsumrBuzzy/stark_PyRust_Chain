# ADR-043: Full-Spectrum Logistic & Mechanical Guardrails

## Status
Accepted / Implemented

## Context
Finalizing the "Decade Device" for long-term viability.

## Decision
1.  **Class Affinity**: Enforce "Engineer" class to prevent 50% speed penalty.
2.  **Address Validation**: Regex (`^0x[a-fA-F0-9]{60,66}$`) for Onramp.
3.  **Documentation**: Formalize all ADRs in `docs/ADR/`.

## Usage
Implemented in `verify_status.py`, `onramp.py`, and Documentation.
