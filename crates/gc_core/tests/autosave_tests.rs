use gc_core::autosave::*;
use gc_core::bootstrap::{build_default_schedule, build_standard_world, WorldOptions};
use gc_core::save;
use std::fs;
use std::path::PathBuf;

fn temp_autosave_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "goblin-camp-autosave-test-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    fs::create_dir_all(&dir).expect("create temp autosave dir");
    dir
}

#[test]
fn autosave_manager_writes_rotating_slots() {
    let dir = temp_autosave_dir();

    let mut world = build_standard_world(
        20,
        10,
        42,
        WorldOptions {
            populate_demo_scene: true,
            tick_ms: 100,
        },
    );
    let mut schedule = build_default_schedule();

    let config = AutosaveConfig {
        every_ticks: 2,
        slots: 2,
        codec: SaveCodec::Json,
        base_name: "autosave".to_string(),
    };
    let mut mgr = AutosaveManager::new(&dir, config.clone()).expect("create autosave manager");

    for _ in 0..5 {
        schedule.run(&mut world);
        let _ = mgr.maybe_autosave(&mut world).expect("autosave tick");
    }

    let slot0 = autosave_slot_path(&dir, "autosave", 0, SaveCodec::Json);
    let slot1 = autosave_slot_path(&dir, "autosave", 1, SaveCodec::Json);
    assert!(slot0.exists(), "slot0 should exist");
    assert!(slot1.exists(), "slot1 should exist");

    // Newest valid autosave should correspond to tick=4 (saved every 2 ticks).
    let recovered = recover_latest_autosave(&dir, &config)
        .expect("recover latest")
        .expect("some autosave should exist");
    assert_eq!(recovered.save.ticks, 4);

    // Validate that core gameplay entities are captured by the save snapshot.
    assert!(recovered.save.entities.iter().any(|e| e.miner));
    assert!(recovered.save.entities.iter().any(|e| e.carrier));
    assert!(recovered.save.entities.iter().any(|e| e.stockpile));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn recovery_uses_backup_when_primary_corrupt() {
    let dir = temp_autosave_dir();

    let config = AutosaveConfig {
        every_ticks: 1,
        slots: 2,
        codec: SaveCodec::Json,
        base_name: "autosave".to_string(),
    };

    // Create a valid save (tick=10) and write to slot 0.
    let mut world = build_standard_world(3, 3, 42, WorldOptions::default());
    world.resource_mut::<gc_core::systems::Time>().ticks = 10;
    let save10 = save::save_world(&mut world);
    let slot0 = autosave_slot_path(&dir, "autosave", 0, SaveCodec::Json);
    fs::write(&slot0, save::encode_json(&save10).expect("encode json")).expect("write slot0");

    // Slot 1 is corrupted, but its backup is valid and newer (tick=20).
    let slot1 = autosave_slot_path(&dir, "autosave", 1, SaveCodec::Json);
    fs::write(&slot1, "{not valid json").expect("write corrupt slot1");
    world.resource_mut::<gc_core::systems::Time>().ticks = 20;
    let save20 = save::save_world(&mut world);
    let slot1_bak = autosave_slot_backup_path(&dir, "autosave", 1, SaveCodec::Json);
    fs::write(&slot1_bak, save::encode_json(&save20).expect("encode json"))
        .expect("write slot1 backup");

    let recovered = recover_latest_autosave(&dir, &config)
        .expect("recover latest")
        .expect("some autosave should exist");
    assert_eq!(recovered.save.ticks, 20);
    assert_eq!(recovered.path, slot1_bak);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn golden_fixtures_load_and_recover() {
    let dir = temp_autosave_dir();

    let config = AutosaveConfig {
        every_ticks: 1,
        slots: 2,
        codec: SaveCodec::Json,
        base_name: "autosave".to_string(),
    };

    let good10: save::SaveGame =
        save::decode_json(include_str!("fixtures/autosave/good-10.json")).expect("decode good10");
    let good20: save::SaveGame =
        save::decode_json(include_str!("fixtures/autosave/good-20.json")).expect("decode good20");

    // Write fixtures into expected slot layout.
    fs::write(
        autosave_slot_path(&dir, "autosave", 0, SaveCodec::Json),
        save::encode_json(&good10).expect("encode good10"),
    )
    .expect("write slot0");

    fs::write(
        autosave_slot_path(&dir, "autosave", 1, SaveCodec::Json),
        include_str!("fixtures/autosave/corrupt.json"),
    )
    .expect("write corrupt slot1");

    fs::write(
        autosave_slot_backup_path(&dir, "autosave", 1, SaveCodec::Json),
        save::encode_json(&good20).expect("encode good20"),
    )
    .expect("write slot1 backup");

    let recovered = recover_latest_autosave(&dir, &config)
        .expect("recover latest")
        .expect("some autosave should exist");
    assert_eq!(recovered.save.ticks, 20);

    let _ = fs::remove_dir_all(&dir);
}
