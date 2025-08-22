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
    world.insert_resource(jobs::ItemSpawnQueue::default());
    world.insert_resource(jobs::ActiveJobs::default());
    world.insert_resource(designations::DesignationConfig { auto_jobs: true });
    // Deterministic fixed-step time resource (10 Hz reference)
    world.insert_resource(systems::Time::new(100));

<<<<<<< HEAD
    // A test goblin (carrier)
=======
    // A test goblin miner positioned at the mining location for demo
>>>>>>> 82525fb (Implement M2 hauling job execution to stockpile system)
    world.spawn((
        Name("Grak".into()),
        Position(5, 5),
        Velocity(0, 0),
        Miner,
        AssignedJob::default(),
        VisionRadius(8),
    ));

<<<<<<< HEAD
    // A test miner goblin
    world.spawn((
        Name("Thok".into()),
        Position(5, 5), // Start at mine designation position
        Velocity(0, 0),
        Miner,
=======
    // A test goblin carrier
    world.spawn((
        Name("Urok".into()),
        Position(5, 5), // Start at mining location to pick up items
        Velocity(0, 0),
        Carrier,
        Inventory::default(),
>>>>>>> 82525fb (Implement M2 hauling job execution to stockpile system)
        AssignedJob::default(),
        VisionRadius(8),
    ));

<<<<<<< HEAD
=======
    // A test stockpile
    world.spawn((
        Name("Stockpile".into()),
        Position(10, 10),
        Stockpile { accepts_any: true },
    ));

>>>>>>> 82525fb (Implement M2 hauling job execution to stockpile system)
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
            jobs::job_assignment_system,
        )
            .chain(),
<<<<<<< HEAD
        jobs::mine_job_assignment_system,
        jobs::job_assignment_system,
        jobs::mine_job_execution_system,
=======
        (
            systems::mining_job_execution_system,
            systems::hauling_job_execution_system,
            systems::auto_haul_job_system,
        ),
>>>>>>> 82525fb (Implement M2 hauling job execution to stockpile system)
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

<<<<<<< HEAD
    // Ensure there's a wall at position (5,5) for mining
=======
    // Set a wall tile at (5,5) for mining
>>>>>>> 82525fb (Implement M2 hauling job execution to stockpile system)
    {
        let mut map = world.resource_mut::<GameMap>();
        map.set_tile(5, 5, TileKind::Wall);
    }

<<<<<<< HEAD
    // Initialize action log
    world.insert_resource(ActionLog::default());

    // Add some stockpiles for demonstration
    let _stockpile1 = world.spawn(StockpileBundle::new(10, 10, 15, 15)).id();
    let _stockpile2 = world.spawn(StockpileBundle::new(25, 5, 30, 10)).id();

=======
>>>>>>> 82525fb (Implement M2 hauling job execution to stockpile system)
    // Add a mine designation which will auto-spawn a job
    world.spawn((
        designations::MineDesignation,
        Position(5, 5),
        DesignationLifecycle::default(),
    ));

    println!(
        "Before mining: tile at (5,5) = {:?}",
        world.resource::<GameMap>().get_tile(5, 5)
    );

    // Spawn some stone items as if they were mined
    world.spawn((
        Name("Stone Chunk A".into()),
        Position(6, 6),
        Item::stone(),
        Carriable,
    ));
    
    world.spawn((
        Name("Stone Chunk B".into()),
        Position(7, 7),
        Item::stone(),
        Carriable,
    ));

    // Log initial state
    {
        let mut log = world.resource_mut::<ActionLog>();
        log.log("=== Jobs Demo Started ===".to_string());
        log.log("Created mine designation at (5, 5)".to_string());
    }

<<<<<<< HEAD
    // Run sim steps with logging
    let mut schedule = build_default_schedule();
    for step in 0..args.steps {
        // Capture state before systems run
        let state_before = StateSnapshot::capture(&mut world);

        schedule.run(&mut world);

        // Capture state after systems run and log changes
        let state_after = StateSnapshot::capture(&mut world);

        log_step_changes(&mut world, step + 1, &state_before, &state_after);
    }

    // Print mining results
    println!(
        "After mining: tile at (5,5) = {:?}",
        world.resource::<GameMap>().get_tile(5, 5)
    );

    let item_queue = world.resource::<jobs::ItemSpawnQueue>();
    println!("Items spawned: {} stone items", item_queue.requests.len());
    for req in &item_queue.requests {
=======
    // Print assignments and results
    let mut q = world.query::<(&Name, &AssignedJob)>();
    for (name, aj) in q.iter(&world) {
>>>>>>> 82525fb (Implement M2 hauling job execution to stockpile system)
        println!(
            "  {:?} at ({}, {})",
            req.item_type, req.position.0, req.position.1
        );
    }

<<<<<<< HEAD
    // Print action log
    let log = world.resource::<ActionLog>();
    println!("\n=== Action Log ===");
    for event in &log.events {
        println!("{}", event);
    }

    // Print assignment summary
    println!("\n=== Assignment Summary ===");
    let mut q = world.query::<(&Name, &AssignedJob)>();
    for (name, aj) in q.iter(&world) {
        let job_status =
            aj.0.map(|id| format!("Job ID: {}", id.0))
                .unwrap_or_else(|| "No job assigned".to_string());
        println!("{}: {}", name.0, job_status);
    }
    
    // Print items in the world
    let mut item_q = world.query::<(&Name, &Position, &Item, &Carriable)>();
    let items: Vec<_> = item_q.iter(&world).collect();
    if !items.is_empty() {
        println!("\nItems in world:");
        for (name, pos, item, _carriable) in items {
            println!("  {} ({:?}) at ({}, {})", name.0, item.item_type, pos.0, pos.1);
        }
    }

    // Show stockpile information
    println!("\nStockpiles:");
    let mut stockpile_query =
        world.query_filtered::<(Entity, &Position, &ZoneBounds), With<Stockpile>>();
    for (entity, pos, bounds) in stockpile_query.iter(&world) {
        println!(
            "  Stockpile {:?} at center ({}, {}) bounds ({},{}) to ({},{})",
            entity, pos.0, pos.1, bounds.min_x, bounds.min_y, bounds.max_x, bounds.max_y
        );
    }

    // Demonstrate nearest stockpile query from a few test positions
    let test_positions = [(8, 8), (20, 3), (0, 0)];
    for (x, y) in test_positions {
        match find_nearest_stockpile(&mut world, x, y) {
            Some((entity, distance_sq)) => {
                let distance = (distance_sq as f32).sqrt();
                println!(
                    "Nearest stockpile to ({}, {}): {:?} (distance: {:.1})",
                    x, y, entity, distance
                );
            }
            None => {
                println!("No stockpiles found from ({}, {})", x, y);
            }
        }
    }

    // Print final state summary
    let mut designations_count = std::collections::HashMap::new();
    for d in world.query::<&DesignationLifecycle>().iter(&world) {
        *designations_count.entry(d.0).or_insert(0) += 1;
    }
    println!("\n=== Final State Summary ===");
    println!(
        "Designations: Active={}, Ignored={}, Consumed={}",
        designations_count
            .get(&DesignationState::Active)
            .unwrap_or(&0),
        designations_count
            .get(&DesignationState::Ignored)
            .unwrap_or(&0),
        designations_count
            .get(&DesignationState::Consumed)
            .unwrap_or(&0)
    );
    println!("Jobs on board: {}", world.resource::<JobBoard>().0.len());
=======
    // Print miner and carrier positions
    let mut q_miners = world.query_filtered::<(&Name, &Position), With<Miner>>();
    for (name, pos) in q_miners.iter(&world) {
        println!("{} (Miner) at: ({}, {})", name.0, pos.0, pos.1);
    }
    let mut q_carriers = world.query_filtered::<(&Name, &Position, &Inventory), With<Carrier>>();
    for (name, pos, inv) in q_carriers.iter(&world) {
        println!(
            "{} (Carrier) at: ({}, {}) carrying {} items",
            name.0,
            pos.0,
            pos.1,
            inv.items.len()
        );
    }

    // Print items created
    let mut q_items = world.query::<(&Position, &Stone)>();
    let item_count = q_items.iter(&world).count();
    println!("Stone items in world: {}", item_count);
    for (pos, _) in q_items.iter(&world) {
        println!("  Stone at: ({}, {})", pos.0, pos.1);
    }

    // Print haul jobs created
    let job_board = world.resource::<JobBoard>();
    let haul_jobs = job_board
        .0
        .iter()
        .filter(|j| matches!(j.kind, JobKind::Haul { .. }))
        .count();
    println!("Haul jobs queued: {}", haul_jobs);

    // Check if mined tile is now floor
    let map = world.resource::<GameMap>();
    match map.get_tile(5, 5) {
        Some(TileKind::Floor) => println!("Mining successful: (5, 5) is now Floor"),
        Some(TileKind::Wall) => println!("Mining not yet complete: (5, 5) is still Wall"),
        Some(other) => println!("Tile (5, 5) is: {:?}", other),
        None => println!("Tile (5, 5) is out of bounds"),
    }

>>>>>>> 82525fb (Implement M2 hauling job execution to stockpile system)
    Ok(())
}

/// State snapshot for change detection
struct StateSnapshot {
    designations: std::collections::HashMap<Entity, (DesignationState, Position)>,
    jobs_count: usize,
    assignments: std::collections::HashMap<Entity, Option<String>>,
}

impl StateSnapshot {
    fn capture(world: &mut World) -> Self {
        Self {
            designations: Self::capture_designation_states(world),
            jobs_count: world.resource::<JobBoard>().0.len(),
            assignments: Self::capture_assignments(world),
        }
    }

    fn capture_designation_states(
        world: &mut World,
    ) -> std::collections::HashMap<Entity, (DesignationState, Position)> {
        let mut query = world.query::<(Entity, &DesignationLifecycle, &Position)>();
        query
            .iter(world)
            .map(|(entity, lifecycle, pos)| (entity, (lifecycle.0, *pos)))
            .collect()
    }

    /// Capture current job assignments
    fn capture_assignments(world: &mut World) -> std::collections::HashMap<Entity, Option<String>> {
        let mut query = world.query::<(Entity, &AssignedJob)>();
        query
            .iter(world)
            .map(|(entity, assigned)| (entity, assigned.0.map(|id| id.0.to_string())))
            .collect()
    }
}

/// Log changes that occurred during a simulation step
fn log_step_changes(world: &mut World, step: u32, before: &StateSnapshot, after: &StateSnapshot) {
    let has_changes = before.designations != after.designations
        || before.jobs_count != after.jobs_count
        || before.assignments != after.assignments;

    if !has_changes {
        return;
    }

    // Collect entity names first to avoid borrowing conflicts
    let entity_names: std::collections::HashMap<Entity, String> = world
        .query::<(Entity, &Name)>()
        .iter(world)
        .map(|(entity, name)| (entity, name.0.clone()))
        .collect();

    let mut log = world.resource_mut::<ActionLog>();
    log.log(format!("--- Step {} ---", step));

    // Log designation state changes
    for (entity, (state_after, pos_after)) in &after.designations {
        if let Some((state_before, _pos_before)) = before.designations.get(entity) {
            if state_before != state_after {
                log.log(format!(
                    "Designation at ({}, {}): {} -> {}",
                    pos_after.0,
                    pos_after.1,
                    format_designation_state(*state_before),
                    format_designation_state(*state_after)
                ));
            }
        }
    }

    // Log job changes
    if before.jobs_count != after.jobs_count {
        if after.jobs_count > before.jobs_count {
            log.log(format!(
                "Jobs created: {} (total: {})",
                after.jobs_count - before.jobs_count,
                after.jobs_count
            ));
        } else if after.jobs_count < before.jobs_count {
            log.log(format!(
                "Jobs assigned: {} (remaining: {})",
                before.jobs_count - after.jobs_count,
                after.jobs_count
            ));
        }
    }

    // Log assignment changes
    for (entity, assignment_after) in &after.assignments {
        if let Some(assignment_before) = before.assignments.get(entity) {
            if assignment_before != assignment_after {
                if let Some(name) = entity_names.get(entity) {
                    match (assignment_before.as_ref(), assignment_after.as_ref()) {
                        (None, Some(job_id)) => {
                            log.log(format!("{} assigned job: {}", name, job_id));
                        }
                        (Some(old_job), Some(new_job)) if old_job != new_job => {
                            log.log(format!("{} reassigned: {} -> {}", name, old_job, new_job));
                        }
                        (Some(old_job), None) => {
                            log.log(format!("{} job completed: {}", name, old_job));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

/// Format designation state for display
fn format_designation_state(state: DesignationState) -> &'static str {
    match state {
        DesignationState::Active => "Active",
        DesignationState::Ignored => "Ignored",
        DesignationState::Consumed => "Consumed",
    }
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
