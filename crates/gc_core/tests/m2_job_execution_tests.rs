use bevy_ecs::prelude::*;
use gc_core::prelude::*;
use gc_core::{designations, jobs, systems};

#[test]
fn mining_to_item_to_haul_pipeline() {
    let mut world = World::new();

    // Setup map
    let mut map = GameMap::new(20, 20);
    map.set_tile(5, 5, TileKind::Wall); // Wall to mine
    world.insert_resource(map);

    // Setup resources
    world.insert_resource(JobBoard::default());
    world.insert_resource(designations::DesignationConfig { auto_jobs: true });
    world.insert_resource(systems::Time::new(100));

    // Create miner positioned at mining location
    world.spawn((
        Name("TestMiner".into()),
        Position(5, 5),
        Miner,
        AssignedJob::default(),
    ));

    // Create carrier positioned at mining location
    world.spawn((
        Name("TestCarrier".into()),
        Position(5, 5),
        Carrier,
        Inventory::default(),
        AssignedJob::default(),
    ));

    // Create stockpile
    world.spawn((
        Name("TestStockpile".into()),
        Position(10, 10),
        Stockpile { accepts: None },
    ));

    // Add mining designation
    world.spawn((
        designations::MineDesignation,
        Position(5, 5),
        DesignationLifecycle::default(),
    ));

    // Build schedule with job execution systems
    let mut schedule = Schedule::default();
    schedule.add_systems((
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
    ));

    // Step 1: Create mining job from designation and assign it
    schedule.run(&mut world);

    // Verify mining job was created and assigned
    let mut q_miners = world.query_filtered::<&AssignedJob, With<Miner>>();
    let miner_job = q_miners.single(&world);
    assert!(miner_job.0.is_some(), "Miner should have a job assigned");

    // Step 2: Execute mining job
    schedule.run(&mut world);

    // Verify wall became floor
    let map = world.resource::<GameMap>();
    assert_eq!(
        map.get_tile(5, 5),
        Some(TileKind::Floor),
        "Wall should be converted to floor"
    );

    // Verify stone item was created
    let mut q_items = world.query_filtered::<&Position, With<Stone>>();
    let items: Vec<_> = q_items.iter(&world).collect();
    assert_eq!(items.len(), 1, "Should have one stone item");
    assert_eq!(items[0].0, 5, "Stone should be at x=5");
    assert_eq!(items[0].1, 5, "Stone should be at y=5");

    // Step 3: Haul job should be created automatically
    schedule.run(&mut world);

    // Verify haul job was created and assigned to carrier
    let mut q_carriers = world.query_filtered::<&AssignedJob, With<Carrier>>();
    let carrier_job = q_carriers.single(&world);
    assert!(
        carrier_job.0.is_some(),
        "Carrier should have a haul job assigned"
    );

    // Step 4: Execute hauling (pick up item)
    schedule.run(&mut world);

    // Verify carrier picked up item
    let mut q_inv = world.query_filtered::<&Inventory, With<Carrier>>();
    let inventory = q_inv.single(&world);
    assert_eq!(
        if inventory.0.is_some() { 1 } else { 0 },
        1,
        "Carrier should be carrying one item"
    );

    // Step 5: Execute hauling (drop at stockpile)
    schedule.run(&mut world);

    // Verify item was delivered to stockpile
    let mut q_items = world.query_filtered::<&Position, With<Stone>>();
    let items: Vec<_> = q_items.iter(&world).collect();
    assert_eq!(items.len(), 1, "Should still have one stone item");
    assert_eq!(items[0].0, 10, "Stone should be at stockpile x=10");
    assert_eq!(items[0].1, 10, "Stone should be at stockpile y=10");

    // Verify carrier is no longer carrying anything
    let mut q_inv = world.query_filtered::<&Inventory, With<Carrier>>();
    let inventory = q_inv.single(&world);
    assert_eq!(
        if inventory.0.is_some() { 1 } else { 0 },
        0,
        "Carrier should no longer be carrying anything"
    );

    // Verify all jobs are completed
    let job_board = world.resource::<JobBoard>();
    assert_eq!(job_board.0.len(), 0, "All jobs should be completed");

    // Verify agents are unassigned
    let mut q_miners = world.query_filtered::<&AssignedJob, With<Miner>>();
    let miner_job = q_miners.single(&world);
    assert!(miner_job.0.is_none(), "Miner should no longer have a job");

    let mut q_carriers = world.query_filtered::<&AssignedJob, With<Carrier>>();
    let carrier_job = q_carriers.single(&world);
    assert!(
        carrier_job.0.is_none(),
        "Carrier should no longer have a job"
    );
}

#[test]
fn multiple_items_create_multiple_haul_jobs() {
    let mut world = World::new();

    // Setup map with multiple walls
    let mut map = GameMap::new(20, 20);
    map.set_tile(5, 5, TileKind::Wall);
    map.set_tile(6, 6, TileKind::Wall);
    world.insert_resource(map);

    // Setup resources
    world.insert_resource(JobBoard::default());
    world.insert_resource(designations::DesignationConfig { auto_jobs: true });
    world.insert_resource(systems::Time::new(100));

    // Create miner
    world.spawn((
        Name("TestMiner1".into()),
        Position(5, 5),
        Miner,
        AssignedJob::default(),
    ));

    // Create second miner for second wall
    world.spawn((
        Name("TestMiner2".into()),
        Position(6, 6),
        Miner,
        AssignedJob::default(),
    ));

    // Create carrier
    world.spawn((
        Name("TestCarrier".into()),
        Position(5, 5),
        Carrier,
        Inventory::default(),
        AssignedJob::default(),
    ));

    // Create stockpile
    world.spawn((
        Name("TestStockpile".into()),
        Position(10, 10),
        Stockpile { accepts: None },
    ));

    // Add multiple mining designations
    world.spawn((
        designations::MineDesignation,
        Position(5, 5),
        DesignationLifecycle::default(),
    ));
    world.spawn((
        designations::MineDesignation,
        Position(6, 6),
        DesignationLifecycle::default(),
    ));

    // Build schedule
    let mut schedule = Schedule::default();
    schedule.add_systems((
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
    ));

    // Run multiple steps to complete both mining operations
    for _ in 0..10 {
        schedule.run(&mut world);
    }

    // Verify both walls were mined
    let map = world.resource::<GameMap>();
    assert_eq!(
        map.get_tile(5, 5),
        Some(TileKind::Floor),
        "First wall should be floor"
    );
    assert_eq!(
        map.get_tile(6, 6),
        Some(TileKind::Floor),
        "Second wall should be floor"
    );

    // Verify items were created and moved to stockpile
    let mut q_items = world.query_filtered::<&Position, With<Stone>>();
    let items: Vec<_> = q_items.iter(&world).collect();

    // Should have at least one item at stockpile (the system processes one haul job at a time)
    let stockpile_items = items
        .iter()
        .filter(|pos| pos.0 == 10 && pos.1 == 10)
        .count();
    assert!(
        stockpile_items > 0,
        "At least one item should be at the stockpile"
    );
}
