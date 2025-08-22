use bevy_ecs::prelude::*;
use gc_core::prelude::*;
use gc_core::{designations, jobs, systems};

/// Test that designations start with Active state by default
#[test]
fn designation_lifecycle_defaults_to_active() {
    let mut world = World::new();
    let entity = world
        .spawn((
            designations::MineDesignation,
            Position(5, 5),
            DesignationLifecycle::default(),
        ))
        .id();

    let lifecycle = world.get::<DesignationLifecycle>(entity).unwrap();
    assert_eq!(lifecycle.0, DesignationState::Active);
}

/// Test that a single designation remains active after dedup system runs
#[test]
fn single_designation_remains_active() {
    let mut world = World::new();
    world.insert_resource(designations::DesignationConfig { auto_jobs: false });

    let entity = world
        .spawn((
            designations::MineDesignation,
            Position(5, 5),
            DesignationLifecycle::default(),
        ))
        .id();

    // Run dedup system
    let mut schedule = Schedule::default();
    schedule.add_systems(designations::designation_dedup_system);
    schedule.run(&mut world);

    let lifecycle = world.get::<DesignationLifecycle>(entity).unwrap();
    assert_eq!(lifecycle.0, DesignationState::Active);
}

/// Test that duplicate designations at the same position are marked as Ignored
#[test]
fn duplicate_designations_marked_ignored() {
    let mut world = World::new();
    world.insert_resource(designations::DesignationConfig { auto_jobs: false });

    // Spawn multiple designations at the same position
    let entity1 = world
        .spawn((
            designations::MineDesignation,
            Position(5, 5),
            DesignationLifecycle::default(),
        ))
        .id();

    let entity2 = world
        .spawn((
            designations::MineDesignation,
            Position(5, 5),
            DesignationLifecycle::default(),
        ))
        .id();

    let entity3 = world
        .spawn((
            designations::MineDesignation,
            Position(5, 5),
            DesignationLifecycle::default(),
        ))
        .id();

    // Run dedup system
    let mut schedule = Schedule::default();
    schedule.add_systems(designations::designation_dedup_system);
    schedule.run(&mut world);

    // First entity should remain active
    let lifecycle1 = world.get::<DesignationLifecycle>(entity1).unwrap();
    assert_eq!(lifecycle1.0, DesignationState::Active);

    // Second and third entities should be ignored
    let lifecycle2 = world.get::<DesignationLifecycle>(entity2).unwrap();
    assert_eq!(lifecycle2.0, DesignationState::Ignored);

    let lifecycle3 = world.get::<DesignationLifecycle>(entity3).unwrap();
    assert_eq!(lifecycle3.0, DesignationState::Ignored);
}

/// Test that designations at different positions remain active
#[test]
fn different_positions_remain_active() {
    let mut world = World::new();
    world.insert_resource(designations::DesignationConfig { auto_jobs: false });

    let entity1 = world
        .spawn((
            designations::MineDesignation,
            Position(5, 5),
            DesignationLifecycle::default(),
        ))
        .id();

    let entity2 = world
        .spawn((
            designations::MineDesignation,
            Position(6, 5),
            DesignationLifecycle::default(),
        ))
        .id();

    let entity3 = world
        .spawn((
            designations::MineDesignation,
            Position(5, 6),
            DesignationLifecycle::default(),
        ))
        .id();

    // Run dedup system
    let mut schedule = Schedule::default();
    schedule.add_systems(designations::designation_dedup_system);
    schedule.run(&mut world);

    // All entities should remain active since they're at different positions
    let lifecycle1 = world.get::<DesignationLifecycle>(entity1).unwrap();
    assert_eq!(lifecycle1.0, DesignationState::Active);

    let lifecycle2 = world.get::<DesignationLifecycle>(entity2).unwrap();
    assert_eq!(lifecycle2.0, DesignationState::Active);

    let lifecycle3 = world.get::<DesignationLifecycle>(entity3).unwrap();
    assert_eq!(lifecycle3.0, DesignationState::Active);
}

/// Test that only active designations create jobs
#[test]
fn only_active_designations_create_jobs() {
    let mut world = World::new();
    world.insert_resource(designations::DesignationConfig { auto_jobs: true });
    world.insert_resource(jobs::JobBoard::default());
    world.insert_resource(systems::DeterministicRng::new(42));

    // Create one active and one ignored designation
    world.spawn((
        designations::MineDesignation,
        Position(5, 5),
        DesignationLifecycle(DesignationState::Active),
    ));

    world.spawn((
        designations::MineDesignation,
        Position(6, 6),
        DesignationLifecycle(DesignationState::Ignored),
    ));

    // Run job creation system
    let mut schedule = Schedule::default();
    schedule.add_systems(designations::designation_to_jobs_system);
    schedule.run(&mut world);

    // Check that only one job was created
    let job_board = world.resource::<jobs::JobBoard>();
    assert_eq!(job_board.0.len(), 1);

    // Verify the job is for the active designation
    let job = &job_board.0[0];
    match &job.kind {
        jobs::JobKind::Mine { x, y } => {
            assert_eq!(*x, 5);
            assert_eq!(*y, 5);
        }
        _ => panic!("Expected Mine job"),
    }
}

/// Test full dedup and job creation pipeline
#[test]
fn full_pipeline_dedup_then_jobs() {
    let mut world = World::new();
    world.insert_resource(designations::DesignationConfig { auto_jobs: true });
    world.insert_resource(jobs::JobBoard::default());
    world.insert_resource(systems::DeterministicRng::new(42));

    // Create multiple designations, some duplicates
    world.spawn((
        designations::MineDesignation,
        Position(5, 5),
        DesignationLifecycle::default(),
    ));

    world.spawn((
        designations::MineDesignation,
        Position(5, 5), // Duplicate
        DesignationLifecycle::default(),
    ));

    world.spawn((
        designations::MineDesignation,
        Position(6, 6), // Different position
        DesignationLifecycle::default(),
    ));

    world.spawn((
        designations::MineDesignation,
        Position(6, 6), // Another duplicate
        DesignationLifecycle::default(),
    ));

    // Run dedup system first
    let mut dedup_schedule = Schedule::default();
    dedup_schedule.add_systems(designations::designation_dedup_system);
    dedup_schedule.run(&mut world);

    // Check states after dedup
    let mut active_count = 0;
    let mut ignored_count = 0;
    let query_result: Vec<_> = world
        .query::<&DesignationLifecycle>()
        .iter(&world)
        .collect();
    for lifecycle in query_result {
        match lifecycle.0 {
            DesignationState::Active => active_count += 1,
            DesignationState::Ignored => ignored_count += 1,
            _ => {}
        }
    }
    assert_eq!(
        active_count, 2,
        "Should have 2 active designations after dedup"
    );
    assert_eq!(
        ignored_count, 2,
        "Should have 2 ignored designations after dedup"
    );

    // Run job creation system
    let mut job_schedule = Schedule::default();
    job_schedule.add_systems(designations::designation_to_jobs_system);
    job_schedule.run(&mut world);

    // Should have exactly 2 jobs (one for each unique position)
    let job_board = world.resource::<jobs::JobBoard>();
    assert_eq!(job_board.0.len(), 2);

    // Verify the job positions
    let mut job_positions: Vec<(i32, i32)> = job_board
        .0
        .iter()
        .map(|job| match &job.kind {
            jobs::JobKind::Mine { x, y } => (*x, *y),
            _ => panic!("Expected Mine job"),
        })
        .collect();
    job_positions.sort();

    assert_eq!(job_positions, vec![(5, 5), (6, 6)]);
}

/// Test that ignored designations don't become active again after multiple runs
#[test]
fn ignored_designations_stay_ignored() {
    let mut world = World::new();
    world.insert_resource(designations::DesignationConfig { auto_jobs: false });

    // Create duplicate designations
    let entity1 = world
        .spawn((
            designations::MineDesignation,
            Position(5, 5),
            DesignationLifecycle::default(),
        ))
        .id();

    let entity2 = world
        .spawn((
            designations::MineDesignation,
            Position(5, 5),
            DesignationLifecycle::default(),
        ))
        .id();

    // Run dedup system multiple times
    let mut schedule = Schedule::default();
    schedule.add_systems(designations::designation_dedup_system);

    // First run
    schedule.run(&mut world);

    let lifecycle1_after_first = world.get::<DesignationLifecycle>(entity1).unwrap().0;
    let lifecycle2_after_first = world.get::<DesignationLifecycle>(entity2).unwrap().0;

    // Second run
    schedule.run(&mut world);

    let lifecycle1_after_second = world.get::<DesignationLifecycle>(entity1).unwrap().0;
    let lifecycle2_after_second = world.get::<DesignationLifecycle>(entity2).unwrap().0;

    // States should remain the same
    assert_eq!(lifecycle1_after_first, lifecycle1_after_second);
    assert_eq!(lifecycle2_after_first, lifecycle2_after_second);

    // One should be active, one should be ignored
    assert_eq!(lifecycle1_after_second, DesignationState::Active);
    assert_eq!(lifecycle2_after_second, DesignationState::Ignored);
}
