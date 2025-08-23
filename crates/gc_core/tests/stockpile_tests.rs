use bevy_ecs::prelude::*;
use gc_core::prelude::*;

#[test]
fn stockpile_bundle_creates_with_correct_center() {
    let bundle = StockpileBundle::new(5, 5, 10, 10);

    // Center should be calculated correctly
    assert_eq!(bundle.position.0, 7); // (5 + 10) / 2 = 7
    assert_eq!(bundle.position.1, 7); // (5 + 10) / 2 = 7

    // Bounds should be set correctly
    assert_eq!(bundle.bounds.min_x, 5);
    assert_eq!(bundle.bounds.min_y, 5);
    assert_eq!(bundle.bounds.max_x, 10);
    assert_eq!(bundle.bounds.max_y, 10);

    // MVP stockpile accepts any (accepts=None)
    assert!(bundle.stockpile.accepts.is_none());
}

#[test]
fn zone_bounds_contains_works_correctly() {
    let bounds = ZoneBounds::new(5, 5, 10, 10);

    // Points inside bounds
    assert!(bounds.contains(5, 5)); // min corner
    assert!(bounds.contains(10, 10)); // max corner
    assert!(bounds.contains(7, 7)); // center
    assert!(bounds.contains(5, 10)); // edge
    assert!(bounds.contains(10, 5)); // edge

    // Points outside bounds
    assert!(!bounds.contains(4, 7)); // left of bounds
    assert!(!bounds.contains(11, 7)); // right of bounds
    assert!(!bounds.contains(7, 4)); // below bounds
    assert!(!bounds.contains(7, 11)); // above bounds
    assert!(!bounds.contains(4, 4)); // completely outside
}

#[test]
fn zone_bounds_center_calculation() {
    let bounds1 = ZoneBounds::new(0, 0, 10, 10);
    assert_eq!(bounds1.center(), (5, 5));

    let bounds2 = ZoneBounds::new(5, 5, 15, 15);
    assert_eq!(bounds2.center(), (10, 10));

    // Test with odd dimensions
    let bounds3 = ZoneBounds::new(0, 0, 9, 9);
    assert_eq!(bounds3.center(), (4, 4));
}

#[test]
fn find_nearest_stockpile_returns_closest() {
    let mut world = World::new();

    // Create multiple stockpiles
    let stockpile1 = world.spawn(StockpileBundle::new(0, 0, 5, 5)).id(); // center (2, 2)
    let stockpile2 = world.spawn(StockpileBundle::new(10, 10, 15, 15)).id(); // center (12, 12)
    let stockpile3 = world.spawn(StockpileBundle::new(20, 0, 25, 5)).id(); // center (22, 2)

    // Test point close to stockpile1
    let nearest = find_nearest_stockpile(&mut world, 3, 3);
    assert!(nearest.is_some());
    let (entity, _distance) = nearest.unwrap();
    assert_eq!(entity, stockpile1);

    // Test point close to stockpile2
    let nearest = find_nearest_stockpile(&mut world, 13, 13);
    assert!(nearest.is_some());
    let (entity, _distance) = nearest.unwrap();
    assert_eq!(entity, stockpile2);

    // Test point close to stockpile3
    let nearest = find_nearest_stockpile(&mut world, 23, 1);
    assert!(nearest.is_some());
    let (entity, _distance) = nearest.unwrap();
    assert_eq!(entity, stockpile3);
}

#[test]
fn find_nearest_stockpile_returns_none_when_empty() {
    let mut world = World::new();

    let nearest = find_nearest_stockpile(&mut world, 5, 5);
    assert!(nearest.is_none());
}

#[test]
fn position_in_stockpile_detects_membership() {
    let mut world = World::new();

    // Create stockpiles
    world.spawn(StockpileBundle::new(0, 0, 5, 5));
    world.spawn(StockpileBundle::new(10, 10, 15, 15));

    // Points inside stockpiles
    assert!(position_in_stockpile(&mut world, 2, 2)); // in first stockpile
    assert!(position_in_stockpile(&mut world, 12, 12)); // in second stockpile
    assert!(position_in_stockpile(&mut world, 0, 0)); // edge of first stockpile

    // Points outside all stockpiles
    assert!(!position_in_stockpile(&mut world, 7, 7)); // between stockpiles
    assert!(!position_in_stockpile(&mut world, 20, 20)); // completely outside
}

#[test]
fn find_stockpiles_at_position_returns_all_overlapping() {
    let mut world = World::new();

    // Create overlapping stockpiles
    let stockpile1 = world.spawn(StockpileBundle::new(0, 0, 10, 10)).id();
    let stockpile2 = world.spawn(StockpileBundle::new(5, 5, 15, 15)).id();
    let stockpile3 = world.spawn(StockpileBundle::new(20, 20, 25, 25)).id();

    // Position in overlap of stockpile1 and stockpile2
    let stockpiles = find_stockpiles_at_position(&mut world, 7, 7);
    assert_eq!(stockpiles.len(), 2);
    assert!(stockpiles.contains(&stockpile1));
    assert!(stockpiles.contains(&stockpile2));

    // Position only in stockpile1
    let stockpiles = find_stockpiles_at_position(&mut world, 2, 2);
    assert_eq!(stockpiles.len(), 1);
    assert!(stockpiles.contains(&stockpile1));

    // Position only in stockpile3
    let stockpiles = find_stockpiles_at_position(&mut world, 22, 22);
    assert_eq!(stockpiles.len(), 1);
    assert!(stockpiles.contains(&stockpile3));

    // Position in no stockpiles
    let stockpiles = find_stockpiles_at_position(&mut world, 100, 100);
    assert_eq!(stockpiles.len(), 0);
}

#[test]
fn stockpile_integration_with_ecs() {
    let mut world = World::new();

    // Create a stockpile entity
    let entity = world.spawn(StockpileBundle::new(5, 5, 10, 10)).id();

    // Verify entity has all required components
    assert!(world.get::<Stockpile>(entity).is_some());
    assert!(world.get::<Position>(entity).is_some());
    assert!(world.get::<ZoneBounds>(entity).is_some());

    // Verify component values
    let position = world.get::<Position>(entity).unwrap();
    assert_eq!(position.0, 7);
    assert_eq!(position.1, 7);

    let bounds = world.get::<ZoneBounds>(entity).unwrap();
    assert_eq!(bounds.min_x, 5);
    assert_eq!(bounds.max_x, 10);

    let stockpile = world.get::<Stockpile>(entity).unwrap();
    assert!(stockpile.accepts.is_none());
}
