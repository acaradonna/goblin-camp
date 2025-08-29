use anyhow::Result;
use bevy_ecs::prelude::*;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use gc_core::prelude::*;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Style},
    text::Text,
    widgets::Paragraph,
    Terminal,
};
use std::io::{stdout, Stdout};
use std::time::{Duration, Instant};

pub struct AppState {
    pub paused: bool,
    pub steps_per_frame: u32,
}

impl Default for AppState {
    fn default() -> Self {
        Self { paused: false, steps_per_frame: 1 }
    }
}

pub fn build_world(width: u32, height: u32, seed: u64) -> World {
    let mut world = World::new();
    world.insert_resource(systems::DeterministicRng::new(seed));

    // Map generation
    let gen = MapGenerator::new();
    let mapgen_seed = {
        let mut rng = world.resource_mut::<systems::DeterministicRng>();
        rng.mapgen_rng.gen::<u32>()
    };
    let map = gen.generate(width, height, mapgen_seed);
    world.insert_resource(map);

    // Other resources
    world.insert_resource(JobBoard::default());
    world.insert_resource(jobs::ItemSpawnQueue::default());
    world.insert_resource(jobs::ActiveJobs::default());
    world.insert_resource(designations::DesignationConfig { auto_jobs: true });
    world.insert_resource(systems::Time::new(100));

    // A simple agent at center
    let (cx, cy) = ((width as i32) / 2, (height as i32) / 2);
    world.spawn((
        Name("Agent".into()),
        Position(cx, cy),
        Velocity(0, 0),
        Miner,
        AssignedJob::default(),
        VisionRadius(8),
    ));

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

fn render_ascii_map(world: &World) -> String {
    let map = world.resource::<GameMap>();

    // Collect agent positions to overlay marker
    use std::collections::HashSet;
    let mut agent_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut q_agents = world.query_filtered::<&Position, With<Miner>>();
    for pos in q_agents.iter(world) {
        agent_positions.insert((pos.0, pos.1));
    }

    let mut out = String::with_capacity((map.width * (map.height + 1)) as usize);
    for y in 0..map.height as i32 {
        for x in 0..map.width as i32 {
            if agent_positions.contains(&(x, y)) {
                out.push('@');
            } else {
                let ch = match map.get_tile(x, y).unwrap_or(TileKind::Wall) {
                    TileKind::Floor => '.',
                    TileKind::Wall => '#',
                    TileKind::Water => '~',
                    TileKind::Lava => '^',
                };
                out.push(ch);
            }
        }
        out.push('\n');
    }
    out
}

fn draw(terminal: &mut Terminal<CrosstermBackend<Stdout>>, world: &World, app: &AppState) -> Result<()> {
    let text = render_ascii_map(world);
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(f.size());

        let header = Paragraph::new(Text::raw("Goblin Camp — TUI (q=quit, space=pause, .=step)"));
        let body = Paragraph::new(Text::raw(text)).style(Style::default());
        let footer = Paragraph::new(Text::raw(format!(
            "paused={}, steps/frame={}",
            app.paused, app.steps_per_frame
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

    // Main loop
    let tick = Duration::from_millis(16);
    let mut last = Instant::now();
    loop {
        // Draw
        draw(&mut terminal, &world, &app)?;

        // Input
        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            // Exit
                            cleanup_terminal()?;
                            return Ok(());
                        }
                        KeyCode::Char(' ') => app.paused = !app.paused,
                        KeyCode::Char('.') => {
                            // Single step
                            let prev = app.paused;
                            // Temporarily step once
                            // We'll run a single schedule tick below
                            // without unpausing permanently
                            // (implemented by calling run once directly)
                            let mut tmp = AppState { paused: false, steps_per_frame: 1 };
                            run_frame(&mut world, &mut schedule, &tmp);
                            app.paused = prev;
                        }
                        KeyCode::Char(d @ '1'..='9') => {
                            let n = (d as u8 - b'0') as u32;
                            app.steps_per_frame = n.max(1);
                        }
                        _ => {}
                    }
                }
            }
        }

        // Tick
        if last.elapsed() >= tick {
            run_frame(&mut world, &mut schedule, &app);
            last = Instant::now();
        }
    }
}

fn cleanup_terminal() -> Result<()> {
    disable_raw_mode()?;
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    Ok(())
}
