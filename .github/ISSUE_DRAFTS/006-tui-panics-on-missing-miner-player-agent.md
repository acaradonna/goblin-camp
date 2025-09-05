Title: TUI panics when no Miner exists to seed PlayerAgent

Summary
- `gc_tui::build_world` expects a Miner to exist (from `populate_demo_scene=true`) and calls `.expect(...)`. If the world is built without a Miner or demo scene disabled, the TUI will panic at startup.

Details
- Location: `crates/gc_tui/src/lib.rs` near selection of `player` entity using `query_filtered::<Entity, With<Miner>>()` then `.next().expect(...)`.

Impact
- Reduces robustness; prevents running TUI with custom or minimal worlds.

Proposed Fix
- Fallback to a virtual camera or center point if no Miner found.
- Alternatively, spawn a default camera/miner if absent.

Acceptance Criteria
- TUI starts without panic even when no Miner exists; `PlayerAgent` is handled gracefully.
