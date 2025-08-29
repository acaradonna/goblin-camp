// Tests for core library functionality - ActionLog and main exports

use gc_core::prelude::*;

#[test]
fn action_log_new_log_is_empty() {
    let log = ActionLog::default();
    assert!(log.events.is_empty());
}

#[test]
fn action_log_can_log_events() {
    let mut log = ActionLog::default();

    log.log("First event".to_string());
    log.log("Second event".to_string());

    assert_eq!(log.events.len(), 2);
    assert_eq!(log.events[0], "First event");
    assert_eq!(log.events[1], "Second event");
}

#[test]
fn action_log_clear_empties_events() {
    let mut log = ActionLog::default();

    log.log("Some event".to_string());
    log.log("Another event".to_string());
    assert_eq!(log.events.len(), 2);

    log.clear();
    assert!(log.events.is_empty());
}

#[test]
fn action_log_maintains_chronological_order() {
    let mut log = ActionLog::default();

    for i in 0..10 {
        log.log(format!("Event {}", i));
    }

    for (i, event) in log.events.iter().enumerate() {
        assert_eq!(event, &format!("Event {}", i));
    }
}

#[test]
fn prelude_exports_work() {
    // Test that we can use common types from the prelude
    let _pos = Position(5, 10);
    let _name = Name("Test".to_string());
    let _job = AssignedJob::default();

    // If compilation succeeds, prelude exports are working
    // This test validates that all prelude exports compile correctly
}
