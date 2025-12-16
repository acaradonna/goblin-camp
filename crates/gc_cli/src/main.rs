use anyhow::Result;
use bevy_ecs::prelude::*;
use clap::{Parser, Subcommand};
use gc_core::autosave::{recover_latest_autosave, AutosaveConfig, AutosaveManager, SaveCodec};
use gc_core::bootstrap::{
    build_default_schedule as core_build_default_schedule, build_standard_world, WorldOptions,
};
use gc_core::prelude::*;
use gc_core::{designations, save};
use std::fs;
use std::io::IsTerminal;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

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
    /// TUI Prototype
    Tui,
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

    /// Codec for save/load demo: json|ron|cbor (default: json)
    #[arg(long, default_value = "json")]
    codec: String,

    /// Enable autosave every N simulation ticks (0 disables autosave)
    #[arg(long, default_value_t = 0)]
    autosave_every: u64,

    /// Number of rotating autosave slots to keep
    #[arg(long, default_value_t = 3)]
    autosave_slots: usize,

    /// Directory to store autosave files (used when autosave is enabled)
    #[arg(long, default_value = "autosaves")]
    autosave_dir: String,

    /// Codec for autosaves: json|ron|cbor (default: json)
    #[arg(long, default_value = "json")]
    autosave_codec: String,

    /// Load the newest autosave (if present) before running the demo
    #[arg(long, default_value_t = false)]
    load_autosave: bool,

    /// Disable interactive crash-recovery prompt on startup
    #[arg(long, default_value_t = false)]
    no_recovery_prompt: bool,

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

fn build_fresh_world(args: &Args) -> World {
    build_standard_world(
        args.width,
        args.height,
        args.seed,
        WorldOptions {
            populate_demo_scene: true,
            tick_ms: 100,
        },
    )
}

fn build_world_from_save(save_game: save::SaveGame) -> World {
    // Build a canonical world with required resources, but do not spawn demo entities.
    // The saved snapshot will spawn entities and override map/time/rng resources.
    let mut world = build_standard_world(
        save_game.width,
        save_game.height,
        save_game.master_seed,
        WorldOptions {
            populate_demo_scene: false,
            tick_ms: save_game.tick_ms,
        },
    );
    save::load_world(save_game, &mut world);
    world
}

fn parse_save_codec(s: &str) -> Result<SaveCodec> {
    match s {
        "json" => Ok(SaveCodec::Json),
        "ron" => Ok(SaveCodec::Ron),
        "cbor" => Ok(SaveCodec::Cbor),
        other => anyhow::bail!("Unknown codec '{}'. Use one of: json|ron|cbor", other),
    }
}

struct AutosaveSessionLock {
    path: PathBuf,
}

impl AutosaveSessionLock {
    fn lock_path(dir: &Path) -> PathBuf {
        dir.join("autosave.lock")
    }

    fn acquire(dir: &Path) -> Result<Self> {
        fs::create_dir_all(dir)?;
        let path = Self::lock_path(dir);
        fs::write(&path, format!("pid={}\n", std::process::id()))?;
        Ok(Self { path })
    }
}

impl Drop for AutosaveSessionLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn prompt_yes_no(prompt: &str) -> bool {
    print!("{prompt}");
    let _ = io::stdout().flush();
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_ok() {
        matches!(buf.trim().to_lowercase().as_str(), "y" | "yes")
    } else {
        false
    }
}

fn build_default_schedule() -> Schedule {
    core_build_default_schedule()
}

fn run_demo_mapgen(args: &Args, world: &World) -> Result<()> {
    let map = world.resource::<GameMap>();
    if args.ascii_map {
        print_ascii_map(map);
    }
    println!("Map {}x{} generated.", map.width, map.height);
    Ok(())
}

fn run_demo_fov(args: &Args, world: &mut World) -> Result<()> {
    world.insert_resource(gc_core::fov::Visibility::default());

    // Compute visibility
    let mut schedule = Schedule::default();
    schedule.add_systems((gc_core::fov::compute_visibility_system,));
    schedule.run(world);

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

fn run_demo_path(args: &Args, world: &World) -> Result<()> {
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

fn run_demo_path_batch(args: &Args, world: &World) -> Result<()> {
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

fn run_demo_jobs(
    args: &Args,
    world: &mut World,
    autosave: &mut Option<AutosaveManager>,
) -> Result<()> {
    // Set a wall tile at (5,5) for mining
    {
        let mut map = world.resource_mut::<GameMap>();
        map.set_tile(5, 5, TileKind::Wall);
    }

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

    // Capture pre-mining tile state for mined count reporting
    let pre_tile = {
        let map = world.resource::<GameMap>();
        map.get_tile(5, 5)
    };

    // Run simulation for the specified steps
    let mut schedule = build_default_schedule();
    for _step in 0..args.steps {
        schedule.run(world);
        if let Some(mgr) = autosave.as_mut() {
            let _ = mgr.maybe_autosave(world).map_err(|e| anyhow::anyhow!(e))?;
        }
    }

    // Print assignments and results
    let mut q = world.query::<(&Name, &AssignedJob)>();
    for (name, aj) in q.iter(world) {
        if let Some(job_id) = aj.0 {
            println!("{} assigned: {}", name.0, job_id.0);
        } else {
            println!("{}: No job assigned", name.0);
        }
    }

    // Print miner and carrier positions
    let mut q_miners = world.query_filtered::<(&Name, &Position), With<Miner>>();
    for (name, pos) in q_miners.iter(world) {
        println!("{} (Miner) at: ({}, {})", name.0, pos.0, pos.1);
    }
    let mut q_carriers = world.query_filtered::<(&Name, &Position, &Inventory), With<Carrier>>();
    for (name, pos, inv) in q_carriers.iter(world) {
        println!(
            "{} (Carrier) at: ({}, {}) carrying {}",
            name.0,
            pos.0,
            pos.1,
            if inv.0.is_some() { "1 item" } else { "0 items" }
        );
    }

    // Print items created
    let mut q_items = world.query::<(&Position, &Stone)>();
    let item_count = q_items.iter(world).count();
    println!("Stone items in world: {}", item_count);
    for (pos, _) in q_items.iter(world) {
        println!("  Stone at: ({}, {})", pos.0, pos.1);
    }

    // Count items hauled to stockpiles (items whose positions are inside any stockpile bounds)
    // Collect zone bounds first to avoid borrowing conflicts and extra item position allocation
    let bounds: Vec<gc_core::components::ZoneBounds> = {
        let mut q_bounds =
            world.query_filtered::<&gc_core::components::ZoneBounds, With<Stockpile>>();
        q_bounds.iter(world).cloned().collect()
    };
    let mut hauled_count = 0usize;
    for (pos, _) in q_items.iter(world) {
        if bounds.iter().any(|b| b.contains(pos.0, pos.1)) {
            hauled_count += 1;
        }
    }
    println!("Items hauled to stockpiles: {}", hauled_count);

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
    // Compute mined tile count (1 if (5,5) changed from Wall->Floor, else 0)
    let mined_count = match (pre_tile, map.get_tile(5, 5)) {
        (Some(TileKind::Wall), Some(TileKind::Floor)) => 1,
        _ => 0,
    };
    println!("Mined tiles: {}", mined_count);

    match map.get_tile(5, 5) {
        Some(TileKind::Floor) => println!("Mining successful: (5, 5) is now Floor"),
        Some(TileKind::Wall) => println!("Mining not yet complete: (5, 5) is still Wall"),
        Some(other) => println!("Tile (5, 5) is: {:?}", other),
        None => println!("Tile (5, 5) is out of bounds"),
    }

    Ok(())
}

fn run_demo_save(args: &Args, world: &mut World) -> Result<()> {
    let save = save_world(world);
    match args.codec.as_str() {
        "json" => {
            let data = save::encode_json(&save)?;
            println!("Serialized (json) length: {} bytes", data.len());
            let parsed: save::SaveGame = save::decode_json(&data)?;
            let mut world2 = World::new();
            load_world(parsed, &mut world2);
            println!(
                "Reloaded world with {}x{} map.",
                world2.resource::<GameMap>().width,
                world2.resource::<GameMap>().height
            );
        }
        "ron" => {
            let data = save::encode_ron(&save).map_err(|e| anyhow::anyhow!(e))?;
            println!("Serialized (ron) length: {} bytes", data.len());
            let parsed: save::SaveGame = save::decode_ron(&data).map_err(|e| anyhow::anyhow!(e))?;
            let mut world2 = World::new();
            load_world(parsed, &mut world2);
            println!(
                "Reloaded world with {}x{} map.",
                world2.resource::<GameMap>().width,
                world2.resource::<GameMap>().height
            );
        }
        "cbor" => {
            let bytes = save::encode_cbor(&save).map_err(|e| anyhow::anyhow!(e))?;
            println!("Serialized (cbor) length: {} bytes", bytes.len());
            let parsed: save::SaveGame =
                save::decode_cbor(&bytes).map_err(|e| anyhow::anyhow!(e))?;
            let mut world2 = World::new();
            load_world(parsed, &mut world2);
            println!(
                "Reloaded world with {}x{} map.",
                world2.resource::<GameMap>().width,
                world2.resource::<GameMap>().height
            );
        }
        other => {
            println!("Unknown codec '{}'", other);
            println!("Use one of: json|ron|cbor (default json)");
        }
    }
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
    println!("7) TUI Prototype");
    print!("Select [1-7]: ");
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
            "7" => Demo::Tui,
            _ => Demo::Mapgen,
        }
    } else {
        Demo::Mapgen
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let autosave_dir = PathBuf::from(&args.autosave_dir);
    let autosave_codec = parse_save_codec(&args.autosave_codec)?;
    let autosave_config = AutosaveConfig {
        every_ticks: args.autosave_every,
        slots: args.autosave_slots,
        codec: autosave_codec,
        base_name: "autosave".to_string(),
    };

    let interactive = io::stdin().is_terminal() && io::stdout().is_terminal();
    let crashed = AutosaveSessionLock::lock_path(&autosave_dir).exists();

    // Decide whether to load a recovery autosave.
    let mut recovered_save: Option<save::SaveGame> = None;
    let mut want_load_autosave = args.load_autosave;

    if crashed && interactive && !args.no_recovery_prompt && !want_load_autosave {
        if let Some(candidate) = recover_latest_autosave(&autosave_dir, &autosave_config)
            .map_err(|e| anyhow::anyhow!("autosave recovery scan failed: {e}"))?
        {
            println!(
                "⚠️ Detected an unclean shutdown. Latest autosave: {} (ticks={})",
                candidate.path.display(),
                candidate.save.ticks
            );
            if prompt_yes_no("Recover this autosave? [y/N]: ") {
                want_load_autosave = true;
                recovered_save = Some(candidate.save);
            }
        }
    }

    if want_load_autosave && recovered_save.is_none() {
        recovered_save = recover_latest_autosave(&autosave_dir, &autosave_config)
            .map_err(|e| anyhow::anyhow!("autosave recovery scan failed: {e}"))?
            .map(|c| c.save);
        if recovered_save.is_none() {
            println!("No valid autosave found in {}", autosave_dir.display());
        }
    }

    let chosen = match args.demo.clone().unwrap_or(Demo::Menu) {
        Demo::Menu => interactive_pick(),
        other => other,
    };

    let mut world = match recovered_save {
        Some(save_game) => build_world_from_save(save_game),
        None => build_fresh_world(&args),
    };

    // If autosave writing is enabled, acquire a session lock and initialize a manager.
    let mut autosave_mgr: Option<AutosaveManager> = None;
    let _autosave_lock = if autosave_config.enabled() {
        let lock = AutosaveSessionLock::acquire(&autosave_dir)?;
        let mut mgr =
            AutosaveManager::new(&autosave_dir, autosave_config).map_err(|e| anyhow::anyhow!(e))?;
        mgr.sync_last_saved_tick_from_world(&world)
            .map_err(|e| anyhow::anyhow!(e))?;
        autosave_mgr = Some(mgr);
        Some(lock)
    } else {
        None
    };

    match chosen {
        Demo::Mapgen => run_demo_mapgen(&args, &world),
        Demo::Fov => run_demo_fov(&args, &mut world),
        Demo::Path => run_demo_path(&args, &world),
        Demo::Jobs => run_demo_jobs(&args, &mut world, &mut autosave_mgr),
        Demo::SaveLoad => run_demo_save(&args, &mut world),
        Demo::PathBatch => run_demo_path_batch(&args, &world),
        Demo::Tui => gc_tui::run(args.width, args.height, args.seed),
        Demo::Menu => Ok(()),
    }
}
