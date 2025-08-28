use anyhow::Result;
use bevy_ecs::prelude::*;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use gc_core::prelude::*;
use gc_core::{designations, jobs, systems};
use rand::Rng;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Basic app state for the TUI
#[derive(Default)]
pub struct AppState {
    pub paused: bool,
    pub show_vis: bool,
    pub steps_per_frame: u32,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            paused: false,
            show_vis: false,
            steps_per_frame: 1,
        }
    }
}

/// Build a default world similar to CLI demos
pub fn build_world(width: u32, height: u32, seed: u64) -> World {
    let mut world = World::new();
    world.insert_resource(systems::DeterministicRng::new(seed));

    // Mapgen with deterministic RNG stream
    let gen = MapGenerator::new();
    let mapgen_seed = {
        let mut rng = world.resource_mut::<systems::DeterministicRng>();
        rng.mapgen_rng.gen::<u32>()
    };
    let map = gen.generate(width, height, mapgen_seed);
    world.insert_resource(map);

    // Core resources
    world.insert_resource(JobBoard::default());
    world.insert_resource(jobs::ItemSpawnQueue::default());
    world.insert_resource(jobs::ActiveJobs::default());
    world.insert_resource(designations::DesignationConfig { auto_jobs: true });
    world.insert_resource(systems::Time::new(100)); // 10 ms per tick (100 Hz logical tick)

    // Example agents
    world.spawn((
        Name("Grak".into()),
        Position(5, 5),
        Velocity(0, 0),
        Miner,
        AssignedJob::default(),
        VisionRadius(8),
    ));
    world.spawn((
        Name("Urok".into()),
        Position(5, 5),
        Velocity(0, 0),
        Carrier,
        Inventory::default(),
        AssignedJob::default(),
        VisionRadius(8),
    ));

    // Example stockpile
    world
        .spawn(gc_core::stockpiles::StockpileBundle::new(9, 9, 11, 11))
        .insert(Name("Stockpile".into()));

    world
}

pub fn build_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems((
        systems::movement,
        systems::confine_to_map,
        (
            designations::designation_dedup_system,
            designations::designation_to_jobs_system,
            jobs::job_assignment_system,
        )
            .chain(),
        (
            jobs::mine_job_execution_system,
            systems::hauling_execution_system,
            systems::auto_haul_system,
        ),
        systems::advance_time,
    ));
    schedule
}

/// One frame of simulation if not paused
fn step_sim(world: &mut World, schedule: &mut Schedule, app: &AppState) {
    if app.paused {
        return;
    }
    for _ in 0..app.steps_per_frame {
        schedule.run(world);
    }
}

/// Draw the map and overlays
fn draw(frame: &mut Frame, world: &World, app: &AppState) {
    let areas = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .split(frame.area());

    // Title
    frame.render_widget(
        Block::default()
            .title("Goblin Camp — TUI Prototype")
            .borders(Borders::BOTTOM),
        areas[0],
    );

    // Main map area: render a simple ASCII map using Paragraph buffer for now
    let map = world.resource::<GameMap>();
    let mut lines = Vec::with_capacity(map.height as usize);

    // Optional visibility overlay: compute union of visible tiles into a set if enabled
    let mut visible: Option<std::collections::HashSet<(i32, i32)>> = None;
    if app.show_vis {
        if world.contains_resource::<gc_core::fov::Visibility>() {
            let vis = world.resource::<gc_core::fov::Visibility>();
            let mut set = std::collections::HashSet::new();
            for tiles in vis.per_entity.values() {
                set.extend(tiles.iter().copied());
            }
            visible = Some(set);
        }
    }

    for y in 0..map.height as i32 {
        let mut row = String::with_capacity(map.width as usize);
        for x in 0..map.width as i32 {
            let mut ch = match map.get_tile(x, y).unwrap_or(TileKind::Wall) {
                TileKind::Floor => '.',
                TileKind::Wall => '#',
                TileKind::Water => '~',
                TileKind::Lava => '^',
            };

            if let Some(v) = &visible {
                if v.contains(&(x, y)) {
                    ch = '*';
                }
            }
            row.push(ch);
        }
        lines.push(row);
    }

    let text = lines.join("\n");
    let para = Paragraph::new(text).style(Style::default().fg(Color::White));
    frame.render_widget(para, areas[1]);

    // Status bar
    let status = format!(
        "[q] quit  [space] {}  [.] step  [v] vis={}  [1-9] spf={}",
        if app.paused { "resume" } else { "pause" },
        app.show_vis,
        app.steps_per_frame
    );
    frame.render_widget(
        Paragraph::new(Span::raw(status)).block(Block::default().borders(Borders::TOP)),
        areas[2],
    );
}

/// Run the interactive TUI loop. Returns when the user quits.
pub fn run(width: u32, height: u32, seed: u64) -> Result<()> {
    // Build world and schedule
    let mut world = build_world(width, height, seed);
    world.insert_resource(gc_core::fov::Visibility::default());
    let mut schedule = build_schedule();

    // Initialize terminal
    let mut terminal = ratatui::init();

    let mut app = AppState::new();

    // Main loop
    loop {
        // Draw
        terminal.draw(|f| draw(f, &world, &app))?;

        // Handle input (non-blocking small poll)
        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char(' ') => app.paused = !app.paused,
                    KeyCode::Char('.') => {
                        let once = AppState {
                            steps_per_frame: 1,
                            ..app
                        };
                        step_sim(&mut world, &mut schedule, &once);
                    }
                    KeyCode::Char('v') => app.show_vis = !app.show_vis,
                    KeyCode::Char(d) if d.is_ascii_digit() && d != '0' => {
                        app.steps_per_frame = d.to_digit(10).unwrap_or(1);
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Step sim
        step_sim(&mut world, &mut schedule, &app);

        // Recompute visibility each frame if overlay is enabled
        if app.show_vis {
            let mut fov_sched = Schedule::default();
            fov_sched.add_systems((gc_core::fov::compute_visibility_system,));
            fov_sched.run(&mut world);
        }
    }

    // Restore terminal
    ratatui::restore();
    Ok(())
}
