use bevy_ecs::prelude::*;
use gc_core::prelude::*;
use gc_core::{designations, jobs, systems};

/// Helper function to create a test world with a map
fn create_test_world_with_map(width: u32, height: u32) -> World {
    let mut world = World::new();
    world.insert_resource(designations::DesignationConfig { auto_jobs: true });
    world.insert_resource(jobs::JobBoard::default());
    world.insert_resource(systems::DeterministicRng::new(42));

    // Create a simple map with floor tiles
    let mut map = GameMap::new(width, height);
    for y in 0..height as i32 {
        for x in 0..width as i32 {
            map.set_tile(x, y, TileKind::Floor);
        }
    }
    world.insert_resource(map);

    world
}

/// Test that construction designation creates build job when active
#[test]
fn construction_designation_creates_build_job() {
    let mut world = create_test_world_with_map(10, 10);

    // Create construction designation for a wall
    world.spawn((
        ConstructionDesignation {
            buildable: BuildableKind::Wall,
            position: (5, 5),
            orientation: None,
        },
        DesignationLifecycle(DesignationState::Active),
    ));

    // Run construction designation to jobs system
    let mut schedule = Schedule::default();
    schedule.add_systems(designations::construction_designation_to_jobs_system);
    schedule.run(&mut world);

    // Check that a build job was created
    let job_board = world.resource::<jobs::JobBoard>();
    assert_eq!(job_board.0.len(), 1);

    // Verify the job is for the construction designation
    let job = &job_board.0[0];
    match &job.kind {
        jobs::JobKind::Build { x, y, .. } => {
            assert_eq!(*x, 5);
            assert_eq!(*y, 5);
        }
        _ => panic!("Expected Build job"),
    }
}

/// Test that construction designation adds material reservation component
#[test]
fn construction_designation_adds_material_reservation() {
    let mut world = create_test_world_with_map(10, 10);

    // Create construction designation for a door
    let entity = world
        .spawn((
            ConstructionDesignation {
                buildable: BuildableKind::Door,
                position: (5, 5),
                orientation: Some(Orientation::North),
            },
            DesignationLifecycle(DesignationState::Active),
        ))
        .id();

    // Run construction designation to jobs system
    let mut schedule = Schedule::default();
    schedule.add_systems(designations::construction_designation_to_jobs_system);
    schedule.run(&mut world);

    // Check that MaterialReservation component was added
    assert!(
        world.get::<MaterialReservation>(entity).is_some(),
        "MaterialReservation component should be added"
    );

    // Check that it starts empty (materials not yet gathered)
    let reservation = world.get::<MaterialReservation>(entity).unwrap();
    assert_eq!(
        reservation.reserved_items.len(),
        0,
        "Should start with no reserved items"
    );
}

/// Test that construction designation marks designation as consumed
#[test]
fn construction_designation_marks_consumed() {
    let mut world = create_test_world_with_map(10, 10);

    // Create construction designation
    let entity = world
        .spawn((
            ConstructionDesignation {
                buildable: BuildableKind::Floor,
                position: (5, 5),
                orientation: None,
            },
            DesignationLifecycle(DesignationState::Active),
        ))
        .id();

    // Run construction designation to jobs system
    let mut schedule = Schedule::default();
    schedule.add_systems(designations::construction_designation_to_jobs_system);
    schedule.run(&mut world);

    // Check that designation is now consumed
    let lifecycle = world.get::<DesignationLifecycle>(entity).unwrap();
    assert_eq!(
        lifecycle.0,
        DesignationState::Consumed,
        "Designation should be consumed after job creation"
    );

    // Run the system again and verify no new jobs are created
    schedule.run(&mut world);

    let job_board = world.resource::<jobs::JobBoard>();
    assert_eq!(
        job_board.0.len(),
        1,
        "Should still have only 1 job after second run"
    );
}

/// Test that ignored construction designations don't create jobs
#[test]
fn ignored_construction_designations_skip_job_creation() {
    let mut world = create_test_world_with_map(10, 10);

    // Create ignored construction designation
    world.spawn((
        ConstructionDesignation {
            buildable: BuildableKind::Wall,
            position: (5, 5),
            orientation: None,
        },
        DesignationLifecycle(DesignationState::Ignored),
    ));

    // Run construction designation to jobs system
    let mut schedule = Schedule::default();
    schedule.add_systems(designations::construction_designation_to_jobs_system);
    schedule.run(&mut world);

    // Check that no jobs were created
    let job_board = world.resource::<jobs::JobBoard>();
    assert_eq!(
        job_board.0.len(),
        0,
        "Ignored designations should not create jobs"
    );
}

/// Test that auto_jobs config controls job creation
#[test]
fn auto_jobs_config_disables_job_creation() {
    let mut world = create_test_world_with_map(10, 10);

    // Override auto_jobs to false
    world.insert_resource(designations::DesignationConfig { auto_jobs: false });

    // Create active construction designation
    world.spawn((
        ConstructionDesignation {
            buildable: BuildableKind::Wall,
            position: (5, 5),
            orientation: None,
        },
        DesignationLifecycle(DesignationState::Active),
    ));

    // Run construction designation to jobs system
    let mut schedule = Schedule::default();
    schedule.add_systems(designations::construction_designation_to_jobs_system);
    schedule.run(&mut world);

    // Check that no jobs were created when auto_jobs is false
    let job_board = world.resource::<jobs::JobBoard>();
    assert_eq!(
        job_board.0.len(),
        0,
        "No jobs should be created when auto_jobs is false"
    );
}

/// Test multiple construction designations create multiple jobs
#[test]
fn multiple_construction_designations_create_multiple_jobs() {
    let mut world = create_test_world_with_map(10, 10);

    // Create multiple construction designations
    world.spawn((
        ConstructionDesignation {
            buildable: BuildableKind::Wall,
            position: (5, 5),
            orientation: None,
        },
        DesignationLifecycle(DesignationState::Active),
    ));

    world.spawn((
        ConstructionDesignation {
            buildable: BuildableKind::Door,
            position: (6, 5),
            orientation: Some(Orientation::East),
        },
        DesignationLifecycle(DesignationState::Active),
    ));

    world.spawn((
        ConstructionDesignation {
            buildable: BuildableKind::Floor,
            position: (5, 6),
            orientation: None,
        },
        DesignationLifecycle(DesignationState::Active),
    ));

    // Run construction designation to jobs system
    let mut schedule = Schedule::default();
    schedule.add_systems(designations::construction_designation_to_jobs_system);
    schedule.run(&mut world);

    // Check that 3 build jobs were created
    let job_board = world.resource::<jobs::JobBoard>();
    assert_eq!(job_board.0.len(), 3, "Should create 3 build jobs");

    // Verify all jobs are Build jobs
    for job in &job_board.0 {
        assert!(
            matches!(job.kind, jobs::JobKind::Build { .. }),
            "All jobs should be Build jobs"
        );
    }
}

/// Test that build job references designation entity
#[test]
fn build_job_references_designation_entity() {
    let mut world = create_test_world_with_map(10, 10);

    // Create construction designation
    let designation_entity = world
        .spawn((
            ConstructionDesignation {
                buildable: BuildableKind::Wall,
                position: (5, 5),
                orientation: None,
            },
            DesignationLifecycle(DesignationState::Active),
        ))
        .id();

    // Run construction designation to jobs system
    let mut schedule = Schedule::default();
    schedule.add_systems(designations::construction_designation_to_jobs_system);
    schedule.run(&mut world);

    // Check that the build job references the designation entity
    let job_board = world.resource::<jobs::JobBoard>();
    let job = &job_board.0[0];
    match &job.kind {
        jobs::JobKind::Build { designation, .. } => {
            assert_eq!(
                *designation, designation_entity,
                "Build job should reference the designation entity"
            );
        }
        _ => panic!("Expected Build job"),
    }
}

/// Test that wall construction on water tile is rejected
#[test]
fn wall_on_water_rejected() {
    let mut world = create_test_world_with_map(10, 10);

    // Set tile to water
    {
        let mut map = world.resource_mut::<GameMap>();
        map.set_tile(5, 5, TileKind::Water);
    }

    // Create construction designation for a wall on water
    let entity = world
        .spawn((
            ConstructionDesignation {
                buildable: BuildableKind::Wall,
                position: (5, 5),
                orientation: None,
            },
            DesignationLifecycle(DesignationState::Active),
        ))
        .id();

    // Run construction designation to jobs system
    let mut schedule = Schedule::default();
    schedule.add_systems(designations::construction_designation_to_jobs_system);
    schedule.run(&mut world);

    // Check that no job was created
    let job_board = world.resource::<jobs::JobBoard>();
    assert_eq!(
        job_board.0.len(),
        0,
        "Should not create job for wall on water"
    );

    // Check that designation was marked ignored
    let lifecycle = world.get::<DesignationLifecycle>(entity).unwrap();
    assert_eq!(
        lifecycle.0,
        DesignationState::Ignored,
        "Invalid placement should mark designation as ignored"
    );
}

/// Test that door construction requires floor tile
#[test]
fn door_requires_floor_tile() {
    let mut world = create_test_world_with_map(10, 10);

    // Set tile to wall
    {
        let mut map = world.resource_mut::<GameMap>();
        map.set_tile(5, 5, TileKind::Wall);
    }

    // Create construction designation for a door on wall
    let entity = world
        .spawn((
            ConstructionDesignation {
                buildable: BuildableKind::Door,
                position: (5, 5),
                orientation: Some(Orientation::North),
            },
            DesignationLifecycle(DesignationState::Active),
        ))
        .id();

    // Run construction designation to jobs system
    let mut schedule = Schedule::default();
    schedule.add_systems(designations::construction_designation_to_jobs_system);
    schedule.run(&mut world);

    // Check that no job was created
    let job_board = world.resource::<jobs::JobBoard>();
    assert_eq!(
        job_board.0.len(),
        0,
        "Should not create job for door on wall"
    );

    // Check that designation was marked ignored
    let lifecycle = world.get::<DesignationLifecycle>(entity).unwrap();
    assert_eq!(
        lifecycle.0,
        DesignationState::Ignored,
        "Invalid placement should mark designation as ignored"
    );
}

/// Test that floor construction works on passable tiles
#[test]
fn floor_on_passable_tile() {
    let mut world = create_test_world_with_map(10, 10);

    // Create construction designation for a floor on existing floor (passable)
    world.spawn((
        ConstructionDesignation {
            buildable: BuildableKind::Floor,
            position: (5, 5),
            orientation: None,
        },
        DesignationLifecycle(DesignationState::Active),
    ));

    // Run construction designation to jobs system
    let mut schedule = Schedule::default();
    schedule.add_systems(designations::construction_designation_to_jobs_system);
    schedule.run(&mut world);

    // Check that job was created
    let job_board = world.resource::<jobs::JobBoard>();
    assert_eq!(
        job_board.0.len(),
        1,
        "Should create job for floor on passable tile"
    );
}

/// Test that construction out of bounds is rejected
#[test]
fn out_of_bounds_rejected() {
    let mut world = create_test_world_with_map(10, 10);

    // Create construction designation for a wall outside map bounds
    let entity = world
        .spawn((
            ConstructionDesignation {
                buildable: BuildableKind::Wall,
                position: (100, 100),
                orientation: None,
            },
            DesignationLifecycle(DesignationState::Active),
        ))
        .id();

    // Run construction designation to jobs system
    let mut schedule = Schedule::default();
    schedule.add_systems(designations::construction_designation_to_jobs_system);
    schedule.run(&mut world);

    // Check that no job was created
    let job_board = world.resource::<jobs::JobBoard>();
    assert_eq!(
        job_board.0.len(),
        0,
        "Should not create job for out of bounds placement"
    );

    // Check that designation was marked ignored
    let lifecycle = world.get::<DesignationLifecycle>(entity).unwrap();
    assert_eq!(
        lifecycle.0,
        DesignationState::Ignored,
        "Out of bounds should mark designation as ignored"
    );
}
