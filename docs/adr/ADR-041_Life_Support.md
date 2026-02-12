# ADR-041: Life Support & Status Guardrails

## Status
Accepted / Implemented

## Context
"Silent Leaks" occur when Crew is Starving (Efficiency Drop) or Busy (Gas Waste).

## Decision
1.  **Busy Gate**: Check `get_crew_status`. If `Busy`, exit Pulse immediately (Save Gas).
2.  **Food Check**: If `Food < 550kg`, trigger Warning/Restock.
3.  **Data Source**: Use `InfluenceAPI` mock/indexer data.

## Usage
Implemented in `influence_api.rs` and `strategy_module.py`.
