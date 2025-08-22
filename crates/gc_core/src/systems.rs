use crate::world::*;
use bevy_ecs::prelude::*;

/// Fixed-step time resource for deterministic ticks
#[derive(Resource, Debug, Clone, Copy)]
pub struct Time {
    /// Accumulated tick count
    pub ticks: u64,
    /// Duration of a tick in milliseconds (for reference/logging)
    pub tick_ms: u64,
}

impl Time {
    pub fn new(tick_ms: u64) -> Self {
        Self { ticks: 0, tick_ms }
    }
}

/// Movement system (runs early)
pub fn movement(mut q: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in q.iter_mut() {
        pos.0 += vel.0;
        pos.1 += vel.1;
    }
}

/// Confine positions to map bounds (runs after movement)
pub fn confine_to_map(map: Res<GameMap>, mut q: Query<&mut Position>) {
    for mut pos in q.iter_mut() {
        pos.0 = pos.0.clamp(0, map.width as i32 - 1);
        pos.1 = pos.1.clamp(0, map.height as i32 - 1);
    }
}

/// Increments the tick counter; place at the end of the schedule for clarity
pub fn advance_time(mut time: ResMut<Time>) {
    time.ticks = time.ticks.saturating_add(1);
}
