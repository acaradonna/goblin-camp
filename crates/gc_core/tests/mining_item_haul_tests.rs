use bevy_ecs::prelude::*;
use gc_core::prelude::*;
use gc_core::world::{GameMap, Name, Position, TileKind};
use gc_core::{jobs, systems};

#[test]
fn mining_item_haul_end_to_end() {
    let mut world = World::new();

    // Setup resources
    world.insert_resource(GameMap::new(10, 10));
    world.insert_resource(JobBoard::default());
    world.insert_resource(DesignationConfig { auto_jobs: true });
    world.insert_resource(systems::DeterministicRng::new(42));
    world.insert_resource(jobs::ItemSpawnQueue::default());
    world.insert_resource(jobs::ActiveJobs::default());

    // Get mutable reference to map and place a wall
    {
        let mut map = world.resource_mut::<GameMap>();
        map.set_tile(5, 5, TileKind::Wall);
    }

    // Spawn a miner
    let _miner = world
        .spawn((
            Name("Miner".to_string()),
            Position(4, 5),
            Miner,
            AssignedJob::default(),
        ))
        .id();

    // Spawn a hauler (carrier)
    let _hauler = world
        .spawn((
            Name("Hauler".to_string()),
            Position(6, 5),
            Carrier,
            AssignedJob::default(),
            Inventory::default(),
        ))
        .id();

    // Create a stockpile at position (8, 8)
    let _stockpile = world
        .spawn((
            Name("Stockpile".to_string()),
            Position(8, 8),
            Stockpile { accepts: None },
        ))
        .id();

    // Create a mine designation at the wall position
    world.spawn(DesignationBundle {
        pos: Position(5, 5),
        kind: MineDesignation,
        lifecycle: DesignationLifecycle::default(),
    });

    // Verify initial state - wall exists, no items
    {
        let map = world.resource::<GameMap>();
        assert_eq!(map.get_tile(5, 5), Some(TileKind::Wall));
    }

    let item_count_before = world.query::<&Item>().iter(&world).count();
    assert_eq!(item_count_before, 0, "Should start with no items");

    // Run designation systems
    let mut designation_schedule = Schedule::default();
    designation_schedule.add_systems((designation_dedup_system, designation_to_jobs_system));
    designation_schedule.run(&mut world);

    // Assign mine job to miner
    let mut mining_job_schedule = Schedule::default();
    mining_job_schedule.add_systems(mining_job_assignment_system);
    mining_job_schedule.run(&mut world);

    // Execute mining
    let mut mining_schedule = Schedule::default();
    mining_schedule.add_systems(mining_execution_system);
    mining_schedule.run(&mut world);

    // Verify mining results - wall becomes floor, item spawned
    {
        let map = world.resource::<GameMap>();
        assert_eq!(
            map.get_tile(5, 5),
            Some(TileKind::Floor),
            "Wall should be converted to floor"
        );
    }

    let item_count_after_mining = world.query::<&Item>().iter(&world).count();
    assert_eq!(
        item_count_after_mining, 1,
        "Should have one stone item after mining"
    );

    // Verify item is at mining location
    let mut item_found = false;
    let mut item_position = (0, 0);
    for (_, pos, _) in world.query::<(Entity, &Position, &Item)>().iter(&world) {
        item_found = true;
        item_position = (pos.0, pos.1);
        break;
    }

    assert!(item_found, "Should find stone item after mining");
    assert_eq!(item_position.0, 5, "Stone should be at mining x position");
    assert_eq!(item_position.1, 5, "Stone should be at mining y position");

    // Create haul job automatically
    let mut auto_haul_schedule = Schedule::default();
    auto_haul_schedule.add_systems(auto_haul_system);
    auto_haul_schedule.run(&mut world);

    // Assign haul job to hauler
    let mut haul_job_assignment_schedule = Schedule::default();
    haul_job_assignment_schedule.add_systems(job_assignment_system);
    haul_job_assignment_schedule.run(&mut world);

    // Execute hauling
    let mut hauling_schedule = Schedule::default();
    hauling_schedule.add_systems(hauling_execution_system);
    hauling_schedule.run(&mut world);

    // Verify hauling results - item moved to stockpile
    let mut final_item_found = false;
    let mut final_item_position = (0, 0);
    for (_, pos, _) in world.query::<(Entity, &Position, &Item)>().iter(&world) {
        final_item_found = true;
        final_item_position = (pos.0, pos.1);
        break;
    }

    assert!(
        final_item_found,
        "Should still find stone item after hauling"
    );
    assert_eq!(
        final_item_position.0, 8,
        "Stone should be moved to stockpile x position"
    );
    assert_eq!(
        final_item_position.1, 8,
        "Stone should be moved to stockpile y position"
    );

    // Final item count check
    let final_item_count = world.query::<&Item>().iter(&world).count();
    assert_eq!(
        final_item_count, 1,
        "Should still have exactly one item after hauling"
    );

    // Verify the item is a stone
    let stone_item = world
        .query::<&Item>()
        .iter(&world)
        .next()
        .expect("Should have stone item");

    assert_eq!(
        stone_item.item_type,
        gc_core::components::ItemType::Stone,
        "Item should be stone type"
    );
}

#[test]
fn mining_without_wall_does_nothing() {
    let mut world = World::new();

    // Setup resources
    world.insert_resource(GameMap::new(10, 10));
    world.insert_resource(JobBoard::default());
    world.insert_resource(DesignationConfig { auto_jobs: true });
    world.insert_resource(systems::DeterministicRng::new(42));
    world.insert_resource(jobs::ItemSpawnQueue::default());
    world.insert_resource(jobs::ActiveJobs::default());

    // Note: No wall placed - position (5,5) will be Floor by default

    // Spawn a miner
    let _miner = world
        .spawn((
            Name("Miner".to_string()),
            Position(4, 5),
            Miner,
            AssignedJob::default(),
        ))
        .id();

    // Create a mine designation at position with no wall
    world.spawn(DesignationBundle {
        pos: Position(5, 5),
        kind: MineDesignation,
        lifecycle: DesignationLifecycle::default(),
    });

    // Verify initial state - floor exists, no items
    {
        let map = world.resource::<GameMap>();
        assert_eq!(map.get_tile(5, 5), Some(TileKind::Floor));
    }

    let item_count_before = world.query::<&Item>().iter(&world).count();
    assert_eq!(item_count_before, 0, "Should start with no items");

    // Run designation systems
    let mut designation_schedule = Schedule::default();
    designation_schedule.add_systems((designation_dedup_system, designation_to_jobs_system));
    designation_schedule.run(&mut world);

    // Assign mine job to miner
    let mut job_schedule = Schedule::default();
    job_schedule.add_systems(job_assignment_system);
    job_schedule.run(&mut world);

    // Execute mining
    let mut mining_schedule = Schedule::default();
    mining_schedule.add_systems(mining_execution_system);
    mining_schedule.run(&mut world);

    // Verify no changes - still floor, no items spawned
    {
        let map = world.resource::<GameMap>();
        assert_eq!(
            map.get_tile(5, 5),
            Some(TileKind::Floor),
            "Should remain floor"
        );
    }

    let item_count_after = world.query::<&Item>().iter(&world).count();
    assert_eq!(
        item_count_after, 0,
        "Should still have no items - no wall to mine"
    );
}
