Title: Mining execution allows diagonal adjacency; clarify or restrict adjacency rule

Summary
- `mining_execution_system` permits mining when the miner is diagonally adjacent (3x3 AoE via `dx <= 1 && dy <= 1`). This may be intended, but it diverges from 4-directional movement constraints elsewhere (pathfinding) and could need clarification or restriction to cardinal adjacency.

Details
- Location: `crates/gc_core/src/systems.rs`, lines where `dx`/`dy` are computed and `dx <= 1 && dy <= 1` gate mining.
- Pathfinding uses 4-directional movement (no diagonals) per `crates/gc_core/src/path.rs`.
- Mining thus can occur from diagonals even though an agent may be unable to step onto that diagonal in a single move.

Impact
- Potential design inconsistency; could enable mining through diagonal corners the agent cannot reach orthogonally.

Options
- Keep as-is but document explicitly that mining allows diagonal adjacency.
- Restrict to cardinal adjacency by requiring `(dx + dy) <= 1`.

Acceptance Criteria
- Decision documented in code comments and README/docs.
- If restricting: update tests to reflect cardinal-only adjacency and ensure miners must be at same tile or N/S/E/W neighbor to mine.
