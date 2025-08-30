use anyhow::Result;
use bevy_ecs::prelude::*;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use gc_core::bootstrap::{
    build_default_schedule as core_build_default_schedule, build_standard_world, WorldOptions,
};
use gc_core::fov;
use gc_core::prelude::*;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::Text,
    widgets::Paragraph,
    Terminal,
};
use std::collections::HashSet;
use std::io::{stdout, Stdout};
use std::time::{Duration, Instant};

pub struct AppState {
    pub paused: bool,
    pub steps_per_frame: u32,
    pub show_vis: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            paused: false,
            steps_per_frame: 1,
            show_vis: false,
        }
    }
}

pub fn build_world(width: u32, height: u32, seed: u64) -> World {
    let mut world = build_standard_world(
        width,
        height,
        seed,
        WorldOptions {
            populate_demo_scene: true,
            tick_ms: 100,
        },
    );
    // Field of view and overlay cache are TUI responsibilities
    world.insert_resource(fov::Visibility::default());
    world.insert_resource(OverlayCache::default());
    // Track a player agent for camera center; fallback to center if absent
    let center = ((width as i32) / 2, (height as i32) / 2);
    // First, search for an existing Miner with an immutable borrow
    let existing_miner = {
        let mut q = world.query_filtered::<Entity, With<Miner>>();
        q.iter(&world).next()
    };
    // After the immutable borrow ends, optionally spawn a new agent
    let player = match existing_miner {
        Some(e) => e,
        None => world
            .spawn((
                Name("Agent".into()),
                Position(center.0, center.1),
                Velocity(0, 0),
                Miner,
                AssignedJob::default(),
                VisionRadius(8),
            ))
            .id(),
    };
    world.insert_resource(PlayerAgent(player));
    world
}

pub fn build_schedule() -> Schedule {
    let mut schedule = core_build_default_schedule();
    // Keep visibility up-to-date as entities move
    schedule.add_systems((fov::compute_visibility_system,));
    schedule
}

/// Cache for the visibility overlay.
///
/// Holds the union of all entities' visible tiles so rendering can check
/// visibility in O(1) per tile without recomputing per frame. The `dirty`
/// flag indicates the cache must be recomputed (e.g., after sim updates or
/// when the overlay toggle changes).
#[derive(Resource, Default)]
struct OverlayCache {
    /// Union of all currently visible tiles across entities. Used by the
    /// renderer for constant-time visibility checks per tile.
    union_vis: HashSet<(i32, i32)>,
    /// Marks that `union_vis` is stale and must be rebuilt before the next
    /// render when the visibility overlay is enabled.
    dirty: bool,
}

/// Handle to the player agent entity for fast lookups during rendering.
#[derive(Resource, Clone, Copy)]
struct PlayerAgent(Entity);

/// Get the `(x, y)` position for a specific entity if it exists.
fn entity_position(world: &World, entity: Entity) -> Option<(i32, i32)> {
    world
        .get_entity(entity)
        .and_then(|e| e.get::<Position>())
        .map(|pos| (pos.0, pos.1))
}

fn render_ascii_map(world: &World, show_vis: bool) -> String {
    let map = world.resource::<GameMap>();
    let cache = world.get_resource::<OverlayCache>();

    // Query the actual agent position if present; fallback to center
    let center = ((map.width as i32) / 2, (map.height as i32) / 2);
    let agent_pos = world
        .get_resource::<PlayerAgent>()
        .and_then(|pa| entity_position(world, pa.0))
        .unwrap_or(center);

    // If overlay enabled, check cached union of visible tiles
    let union_vis = if show_vis {
        cache.map(|c| &c.union_vis)
    } else {
        None
    };

    let mut out = String::with_capacity((map.width * (map.height + 1)) as usize);
    for y in 0..map.height as i32 {
        for x in 0..map.width as i32 {
            if (x, y) == agent_pos {
                out.push('@');
            } else {
                // If visibility overlay enabled and this tile is visible by any entity, draw '*'
                let visible = union_vis.map(|u| u.contains(&(x, y))).unwrap_or(false);
                let ch = if visible {
                    '*'
                } else {
                    match map.get_tile(x, y).unwrap_or(TileKind::Wall) {
                        TileKind::Floor => '.',
                        TileKind::Wall => '#',
                        TileKind::Water => '~',
                        TileKind::Lava => '^',
                    }
                };
                out.push(ch);
            }
        }
        out.push('\n');
    }
    out
}

fn draw(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    world: &World,
    app: &AppState,
) -> Result<()> {
    let text = render_ascii_map(world, app.show_vis);
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(f.size());

        let header = Paragraph::new(Text::raw(
            "Goblin Camp — TUI (q:quit, space:pause, .:step, v:vis)",
        ));
        let body = Paragraph::new(Text::raw(text)).style(Style::default());
        let footer = Paragraph::new(Text::raw(format!(
            "paused={}, steps/frame={}, vis={}",
            app.paused, app.steps_per_frame, app.show_vis
        )));

        f.render_widget(header, chunks[0]);
        f.render_widget(body, chunks[1]);
        f.render_widget(footer, chunks[2]);
    })?;
    Ok(())
}

fn run_frame(world: &mut World, schedule: &mut Schedule, app: &AppState) {
    if !app.paused {
        for _ in 0..app.steps_per_frame {
            schedule.run(world);
            mark_overlay_dirty(world);
        }
    }
}

pub fn run(width: u32, height: u32, seed: u64) -> Result<()> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App state and world
    let mut app = AppState::default();
    let mut world = build_world(width, height, seed);
    let mut schedule = build_schedule();
    // Ensure initial visibility buffer is computed before first draw
    schedule.run(&mut world);
    mark_overlay_dirty(&mut world);

    // Main loop
    let tick = Duration::from_millis(16);
    let mut last = Instant::now();
    loop {
        // Prepare overlay cache before drawing
        prepare_overlay_cache(&mut world, app.show_vis);
        // Draw
        draw(&mut terminal, &world, &app)?;

        // Input
        while event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        // Exit
                        cleanup_terminal()?;
                        return Ok(());
                    }
                    KeyCode::Char(' ') => app.paused = !app.paused,
                    KeyCode::Char('.') => {
                        // Single step: run the schedule once without changing paused state
                        schedule.run(&mut world);
                        mark_overlay_dirty(&mut world);
                    }
                    KeyCode::Char('v') => {
                        // Toggle visibility overlay
                        app.show_vis = !app.show_vis;
                        mark_overlay_dirty(&mut world);
                    }
                    KeyCode::Char(d @ '1'..='9') => {
                        let n = (d as u8 - b'0') as u32;
                        app.steps_per_frame = n.max(1);
                    }
                    _ => {}
                },
                Event::Resize(_, _) => {
                    // No-op; next draw will adapt to the new size
                }
                _ => {}
            }
        }

        // Tick
        if last.elapsed() >= tick {
            run_frame(&mut world, &mut schedule, &app);
            last = Instant::now();
        }
    }
}

fn prepare_overlay_cache(world: &mut World, show_vis: bool) {
    // If the overlay is disabled, nothing to do. Leaving the cache as-is is
    // intentional so toggling back on is instant unless marked dirty.
    if !show_vis {
        return;
    }
    // Check dirtiness without taking a mutable borrow yet
    if let Some(cache) = world.get_resource::<OverlayCache>() {
        if !cache.dirty {
            return;
        }
    }

    // Build the union in a local buffer first
    let mut union: HashSet<(i32, i32)> = HashSet::new();
    if let Some(vis) = world.get_resource::<fov::Visibility>() {
        for set in vis.per_entity.values() {
            // Extend the union with all points from this entity's visibility set
            union.extend(set.iter().copied());
        }
    }

    // Now update the cache
    if let Some(mut cache) = world.get_resource_mut::<OverlayCache>() {
        cache.union_vis = union;
        cache.dirty = false;
    } else {
        // Handle the missing-cache case gracefully by inserting a fresh cache.
        world.insert_resource(OverlayCache {
            union_vis: union,
            dirty: false,
        });
    }
}

/// Mark the overlay cache as needing recomputation on the next draw.
///
/// Call this after simulation steps or input toggles that can change which
/// tiles are visible, so `prepare_overlay_cache` knows to rebuild the union.
fn mark_overlay_dirty(world: &mut World) {
    if let Some(mut cache) = world.get_resource_mut::<OverlayCache>() {
        cache.dirty = true;
    }
}

fn cleanup_terminal() -> Result<()> {
    disable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}
