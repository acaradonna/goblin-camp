use anyhow::Result;
use bevy_ecs::prelude::*;
use clap::{Parser, Subcommand};
use gc_core::prelude::*;
use gc_core::{designations, jobs, save, systems};
use rand::Rng;
use std::io::{self, Write};

#[derive(Subcommand, Debug, Clone)]
enum Demo {
    /// Interactive menu
    Menu,
    /// Show generated map and basic info
    Mapgen,
    /// Line-of-sight/FOV demo
    Fov,
    /// A* pathfinding demo
    Path,
    /// Job board + designation assignment demo
    Jobs,
    /// Save/Load snapshot demo
    SaveLoad,
    /// Batched pathfinding with LRU cache
    PathBatch,
}

#[derive(Parser, Debug)]
#[command(name = "goblin-camp", version, about = "Goblin Camp headless sim")]
struct Args {
    /// Map width
    #[arg(long, default_value_t = 80)]
    width: u32,
    /// Map height
    #[arg(long, default_value_t = 50)]
    height: u32,
    /// Steps to run (where applicable)
    #[arg(long, default_value_t = 10)]
    steps: u32,
    /// RNG seed for mapgen
    #[arg(long, default_value_t = 42)]
    seed: u64,
    /// Print ASCII map on start (demos that render maps)
    #[arg(long, default_value_t = true)]
    ascii_map: bool,
    /// Show visibility overlay in FOV demo
    #[arg(long, default_value_t = false)]
    show_vis: bool,

    /// Choose a demo to run. If omitted or set to `menu`, an interactive picker is shown.
    #[command(subcommand)]
    demo: Option<Demo>,
}

fn print_ascii_map(map: &GameMap) {
    for y in 0..map.height as i32 {
        let mut line = String::with_capacity(map.width as usize);
        for x in 0..map.width as i32 {
            let ch = match map.get_tile(x, y).unwrap_or(TileKind::Wall) {
                TileKind::Floor => '.',
                TileKind::Wall => '#',
                TileKind::Water => '~',
                TileKind::Lava => '^',
            };
            line.push(ch);
        }
        println!("{}", line);
    }
}

fn print_ascii_map_with_path(map: &GameMap, path: &[(i32, i32)]) {
    use std::collections::HashSet;
    let set: HashSet<(i32, i32)> = path.iter().copied().collect();
    for y in 0..map.height as i32 {
        let mut line = String::with_capacity(map.width as usize);
        for x in 0..map.width as i32 {
            let ch = if set.contains(&(x, y)) {
                'o'
            } else {
                match map.get_tile(x, y).unwrap_or(TileKind::Wall) {
                    TileKind::Floor => '.',
                    TileKind::Wall => '#',
                    TileKind::Water => '~',
                    TileKind::Lava => '^',
                }
            };
            line.push(ch);
        }
        println!("{}", line);
    }
}

fn build_world(args: &Args) -> World {
    let mut world = World::new();

    // Insert deterministic RNG resource first
    world.insert_resource(systems::DeterministicRng::new(args.seed));

    // Map generation using centralized RNG
    let gen = MapGenerator::new();
    let mapgen_seed = {
        let mut rng = world.resource_mut::<systems::DeterministicRng>();
        rng.mapgen_rng.gen::<u32>()
    };
    let map = gen.generate(args.width, args.height, mapgen_seed);
    world.insert_resource(map);

    // Other resources
    world.insert_resource(JobBoard::default());
    world.insert_resource(designations::DesignationConfig { auto_jobs: true });
    // Deterministic fixed-step time resource (10 Hz reference)
    world.insert_resource(systems::Time::new(100));

    // A test goblin
    world.spawn((
        Name("Grak".into()),
        Position(1, 1),
        Velocity(1, 0),
        Carrier,
        AssignedJob::default(),
        VisionRadius(8),
    ));

    world
}

fn build_default_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems((
        systems::movement,
        systems::confine_to_map,
        (
            designations::designation_dedup_system,
            designations::designation_to_jobs_system,
        )
            .chain(),
        jobs::job_assignment_system,
        systems::advance_time,
    ));
    schedule
}

fn run_demo_mapgen(args: &Args) -> Result<()> {
    let world = build_world(args);
    let map = world.resource::<GameMap>();
    if args.ascii_map {
        print_ascii_map(map);
    }
    println!("Map {}x{} generated.", map.width, map.height);
    Ok(())
}

fn run_demo_fov(args: &Args) -> Result<()> {
    let mut world = build_world(args);
    world.insert_resource(gc_core::fov::Visibility::default());

    // Compute visibility
    let mut schedule = Schedule::default();
    schedule.add_systems((gc_core::fov::compute_visibility_system,));
    schedule.run(&mut world);

    // Print result
    let map = world.resource::<GameMap>();
    if args.ascii_map {
        if args.show_vis {
            let vis = world.resource::<gc_core::fov::Visibility>();
            // Show union of all visible tiles for simplicity
            use std::collections::HashSet;
            let mut all: HashSet<(i32, i32)> = HashSet::new();
            for s in vis.per_entity.values() {
                all.extend(s.iter().copied());
            }
            for y in 0..map.height as i32 {
                let mut line = String::with_capacity(map.width as usize);
                for x in 0..map.width as i32 {
                    let ch = if all.contains(&(x, y)) {
                        '*'
                    } else {
                        match map.get_tile(x, y).unwrap_or(TileKind::Wall) {
                            TileKind::Floor => '.',
                            TileKind::Wall => '#',
                            TileKind::Water => '~',
                            TileKind::Lava => '^',
                        }
                    };
                    line.push(ch);
                }
                println!("{}", line);
            }
        } else {
            print_ascii_map(map);
        }
    }
    println!(
        "LOS from (1,1) to bottom-right-1: {}",
        los_visible(map, 1, 1, args.width as i32 - 2, args.height as i32 - 2)
    );
    Ok(())
}

fn run_demo_path(args: &Args) -> Result<()> {
    let world = build_world(args);
    let map = world.resource::<GameMap>();
    let start = (1, 1);
    let goal = (args.width as i32 - 2, args.height as i32 - 2);
    match astar_path(map, start, goal) {
        Some((path, cost)) => {
            println!("Path found: length={}, cost={}", path.len(), cost);
            if args.ascii_map {
                print_ascii_map_with_path(map, &path);
            }
        }
        None => println!("No path found from {:?} to {:?}", start, goal),
    }
    Ok(())
}

fn run_demo_path_batch(args: &Args) -> Result<()> {
    let world = build_world(args);
    let map = world.resource::<GameMap>();
    let mut svc = gc_core::path::PathService::new(256);

    let starts = [(1, 1), (2, 2), (3, 3), (4, 4)];
    let goal = (args.width as i32 - 2, args.height as i32 - 2);
    let mut reqs = Vec::new();
    for s in starts {
        reqs.push(gc_core::path::PathRequest { start: s, goal });
    }
    // Repeat to exercise cache hits
    for s in starts {
        reqs.push(gc_core::path::PathRequest { start: s, goal });
    }

    let results = svc.batch(map, &reqs);
    let (hits, misses) = svc.stats();
    println!(
        "Batched {} requests. Cache hits={}, misses={}",
        results.len(),
        hits,
        misses
    );

    if args.ascii_map {
        if let Some(Some((path, _))) = results.first() {
            print_ascii_map_with_path(map, path);
        }
    }
    Ok(())
}

fn run_demo_jobs(args: &Args) -> Result<()> {
    let mut world = build_world(args);
    // Add a mine designation which will auto-spawn a job
    world.spawn((
        designations::MineDesignation,
        Position(5, 5),
        DesignationLifecycle::default(),
    ));

    // Run sim steps
    let mut schedule = build_default_schedule();
    for _ in 0..args.steps {
        schedule.run(&mut world);
    }

    // Print assignments
    let mut q = world.query::<(&Name, &AssignedJob)>();
    for (name, aj) in q.iter(&world) {
        println!(
            "{} assigned: {}",
            name.0,
            aj.0.map(|id| id.0.to_string())
                .unwrap_or_else(|| "none".into())
        );
    }
    Ok(())
}

fn run_demo_save(args: &Args) -> Result<()> {
    let mut world = build_world(args);
    let save = save_world(&mut world);
    let json = serde_json::to_string(&save)?;
    println!("Serialized save length: {} bytes", json.len());
    let parsed: save::SaveGame = serde_json::from_str(&json)?;
    let mut world2 = World::new();
    load_world(parsed, &mut world2);
    println!(
        "Reloaded world with {}x{} map.",
        world2.resource::<GameMap>().width,
        world2.resource::<GameMap>().height
    );
    Ok(())
}

fn interactive_pick() -> Demo {
    println!("Goblin Camp — Demo Menu");
    println!("1) Mapgen");
    println!("2) FOV/LOS");
    println!("3) Pathfinding");
    println!("4) Jobs & Designations");
    println!("5) Save/Load");
    println!("6) Path Batch + Cache");
    print!("Select [1-6]: ");
    let _ = io::stdout().flush();

    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_ok() {
        match buf.trim() {
            "1" => Demo::Mapgen,
            "2" => Demo::Fov,
            "3" => Demo::Path,
            "4" => Demo::Jobs,
            "5" => Demo::SaveLoad,
            "6" => Demo::PathBatch,
            _ => Demo::Mapgen,
        }
    } else {
        Demo::Mapgen
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let chosen = match args.demo.clone().unwrap_or(Demo::Menu) {
        Demo::Menu => interactive_pick(),
        other => other,
    };

    match chosen {
        Demo::Mapgen => run_demo_mapgen(&args),
        Demo::Fov => run_demo_fov(&args),
        Demo::Path => run_demo_path(&args),
        Demo::Jobs => run_demo_jobs(&args),
        Demo::SaveLoad => run_demo_save(&args),
        Demo::PathBatch => run_demo_path_batch(&args),
        Demo::Menu => Ok(()),
    }
}
