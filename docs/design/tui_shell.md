# M3 — TUI Shell (ratatui + crossterm), Update Loop, Overlays

Status: Draft
Owner: @acaradonna
Refs: docs/plan/MASTER_PLAN.md, issue #27

## Goals

- Provide an interactive TUI shell to visualize the headless simulation.
- Keep core determinism intact; TUI is purely a view/controller around `gc_core`.
- Implement a minimal update loop with pause/resume and step.
- Support simple overlays (visibility/FOV) and responsive layout.
- Keep CLI demos as CI canonical paths; TUI is for local dev and UX prototyping.

Non-goals (M3):

- Full-featured input rebinding or mouse-driven UI.
- Complex widgets; we favor a simple ASCII map first.
- ECS in TUI crate; ECS remains in `gc_core`.

## Crate Layout

- `crates/gc_tui/` — TUI crate using `ratatui` + `crossterm`.
  - `src/lib.rs` exposes `run(width, height, seed) -> Result<()>` plus helpers:
    - `build_world(width, height, seed) -> World`
    - `build_schedule() -> Schedule`
    - `AppState { paused, show_vis, steps_per_frame }`
    - Internal `draw(frame, world, app)` and `step_sim(world, schedule, app)`
- `crates/gc_cli/` adds a `tui` subcommand that calls `gc_tui::run`.

This mirrors the headless-first approach: `gc_core` owns data/systems, TUI/CLI are thin shells.

## Rendering Plan

- Renderer: Start with simple ASCII map using `Paragraph` and join of rows.
- Layout: Header, main area, status/footer using `Layout::vertical([1, Min(0), 1])`.
- Overlays: Optional visibility overlay draws `*` where visible.
- Agents/Entities: Future M3+ iteration could render entity markers on top.

## Update Loop

- Tick source: `gc_core::systems::Time::new(100)` fixed-tick resource.
- Per-frame behavior:
  - Draw frame.
  - Poll input for ~16ms to avoid blocking.
  - Apply input: toggle pause, step (`.`), toggle vis (`v`), change steps-per-frame via `1-9`.
  - If not paused, run `steps_per_frame` iterations of `schedule.run(&mut world)`.
  - If visibility is enabled, run FOV system pass per frame.
- Exit on `q` or `Esc`.

## Input Map (initial)

- `q`/`Esc`: quit
- `Space`: pause/resume
- `.`: single-step once
- `v`: toggle visibility overlay
- `1`..`9`: set steps-per-frame

## Determinism

- Seeded RNG resource `systems::DeterministicRng` injected.
- All random operations done through the resource; no `std::time` in sim.
- TUI does not affect sim determinism across runs when receiving the same input sequence.

## Error Handling and Exit

- Use `ratatui::init()`/`ratatui::restore()` to manage alternate screen/raw mode and panic hook.
- Clean restore on normal exit and errors.

## Testing Strategy

- Keep core integration tests unchanged; TUI is not tested in CI yet.
- Optional future: a golden-buffer snapshot test using `ratatui::buffer::Buffer` + a fake backend to assert ASCII output for a known small map.

## Performance Notes

- Drawing with `Paragraph` is simple but not optimal; when needed, move to a custom `Widget` that writes directly to the frame buffer for better performance.
- Event poll duration is small; can be adjusted or moved to a timer approach.

## Future Extensions

- Agent markers and selection cursor; highlight tile details.
- Sidebar panels (jobs, entities, stockpiles) using a split layout.
- Input abstraction and keybind configuration.
- Snapshot tests for rendering.
- Frame pacing controls (vsync-like sleeps) and FPS display.

## Risks

- Terminal compatibility differences; use crossterm defaults and avoid non-portable features.
- Performance for large maps; keep ASCII path and measure.

## Acceptance Criteria

- `cargo run -p gc_cli -- tui` launches TUI in alternate screen and displays the map.
- Can pause/resume, step once, toggle visibility overlay, adjust steps-per-frame.
- Exits cleanly with terminal restored.

## References

- Ratatui docs: <https://docs.rs/ratatui>
- Event handling with crossterm: <https://docs.rs/crossterm/latest/crossterm/event>
- Application patterns: <https://ratatui.rs/concepts/application-patterns/>
