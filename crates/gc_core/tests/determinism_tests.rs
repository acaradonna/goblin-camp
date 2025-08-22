use bevy_ecs::prelude::*;
use gc_core::prelude::*;
use gc_core::{designations, jobs, systems};
use rand::Rng;

/// Test that identical seeds produce identical behavior across map generation and job creation
#[test]
fn deterministic_behavior_across_systems() {
    fn create_world_and_run(seed: u64) -> (String, Vec<u8>) {
        let mut world = World::new();

        // Insert deterministic RNG resource
        world.insert_resource(systems::DeterministicRng::new(seed));

        // Generate map using centralized RNG
        let gen = MapGenerator::new();
        let mapgen_seed = {
            let mut rng = world.resource_mut::<systems::DeterministicRng>();
            rng.mapgen_rng.gen::<u32>()
        };
        let map = gen.generate(10, 10, mapgen_seed);
        world.insert_resource(map);

        // Set up other resources
        world.insert_resource(JobBoard::default());
        world.insert_resource(designations::DesignationConfig { auto_jobs: true });

        // Create a designation that will generate a job
        world.spawn((
            designations::MineDesignation,
            Position(5, 5),
            DesignationLifecycle::default(),
        ));

        // Run designation to jobs system
        let mut schedule = Schedule::default();
        schedule.add_systems(designations::designation_to_jobs_system);
        schedule.run(&mut world);

        // Extract deterministic data for comparison
        let job_board = world.resource::<jobs::JobBoard>();
        let job_id_string = if job_board.0.is_empty() {
            "no_jobs".to_string()
        } else {
            job_board.0[0].id.0.to_string()
        };

        let map = world.resource::<GameMap>();
        let map_tiles: Vec<u8> = map.tiles.iter().map(|t| *t as u8).collect();

        (job_id_string, map_tiles)
    }

    // Run with same seed twice
    let (job_id1, map_tiles1) = create_world_and_run(12345);
    let (job_id2, map_tiles2) = create_world_and_run(12345);

    // Results should be identical
    assert_eq!(
        job_id1, job_id2,
        "Job IDs should be identical with same seed"
    );
    assert_eq!(
        map_tiles1, map_tiles2,
        "Map tiles should be identical with same seed"
    );

    // Run with different seed
    let (job_id3, map_tiles3) = create_world_and_run(54321);

    // Results should be different
    assert_ne!(
        job_id1, job_id3,
        "Job IDs should be different with different seeds"
    );
    assert_ne!(
        map_tiles1, map_tiles3,
        "Map tiles should be different with different seeds"
    );
}

/// Test that the DeterministicRng resource produces consistent sequences
#[test]
fn deterministic_rng_consistent_sequences() {
    let mut rng1 = systems::DeterministicRng::new(42);
    let mut rng2 = systems::DeterministicRng::new(42);

    // Generate sequences from both RNGs
    let seq1: Vec<u32> = (0..10).map(|_| rng1.mapgen_rng.gen()).collect();
    let seq2: Vec<u32> = (0..10).map(|_| rng2.mapgen_rng.gen()).collect();

    assert_eq!(
        seq1, seq2,
        "Identical seeds should produce identical sequences"
    );

    // Test job RNG stream is independent
    let job_vals1: Vec<u32> = (0..5).map(|_| rng1.job_rng.gen()).collect();
    let job_vals2: Vec<u32> = (0..5).map(|_| rng2.job_rng.gen()).collect();

    assert_eq!(
        job_vals1, job_vals2,
        "Job RNG streams should also be identical"
    );
}
