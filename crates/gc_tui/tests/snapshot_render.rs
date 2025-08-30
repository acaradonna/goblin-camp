use bevy_ecs::prelude::*;
use gc_core::bootstrap::{build_default_schedule, build_standard_world, WorldOptions};
use gc_core::fov;
use gc_tui::render_ascii_snapshot;
use std::env;
use std::fs;
use std::path::PathBuf;

/// Build a deterministic world suitable for TUI snapshot rendering tests.
fn build_test_world(width: u32, height: u32, seed: u64) -> World {
    let mut world = build_standard_world(
        width,
        height,
        seed,
        WorldOptions {
            populate_demo_scene: true,
            tick_ms: 100,
        },
    );
    // TUI-specific resources used by renderer
    world.insert_resource(fov::Visibility::default());

    // Note: TUI renderer falls back to map center if no PlayerAgent resource
    // exists, so we don't insert the private PlayerAgent here.

    // Compute initial FOV once for deterministic visibility state
    let mut schedule = build_default_schedule();
    schedule.add_systems(fov::compute_visibility_system);
    schedule.run(&mut world);

    world
}

fn snapshot_path(name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("__snapshots__");
    fs::create_dir_all(&p).expect("create snapshots dir");
    p.push(name);
    p
}

#[test]
fn tui_ascii_snapshot_20x10_seed42_no_vis() {
    let mut world = build_test_world(20, 10, 42);
    // Ensure any overlay caching inside TUI is reset/deterministic
    // The renderer path used here does not depend on runtime cache mutations.

    let actual = render_ascii_snapshot(&mut world, false);

    let path = snapshot_path("tui_20x10_seed42_no_vis.txt");

    if env::var("UPDATE_SNAPSHOTS").is_ok() || !path.exists() {
        fs::write(&path, actual.as_bytes()).expect("write snapshot");
    }

    let expected = fs::read_to_string(&path).expect("read snapshot");
    assert_eq!(
        actual, expected,
        "ASCII snapshot mismatch; run with UPDATE_SNAPSHOTS=1 to refresh"
    );
}

#[test]
fn tui_ascii_snapshot_20x10_seed42_with_vis() {
    let mut world = build_test_world(20, 10, 42);

    let actual = render_ascii_snapshot(&mut world, true);

    let path = snapshot_path("tui_20x10_seed42_with_vis.txt");

    if env::var("UPDATE_SNAPSHOTS").is_ok() || !path.exists() {
        fs::write(&path, actual.as_bytes()).expect("write snapshot");
    }

    let expected = fs::read_to_string(&path).expect("read snapshot");
    assert_eq!(
        actual, expected,
        "ASCII snapshot mismatch; run with UPDATE_SNAPSHOTS=1 to refresh"
    );
}
