use crate::world::{GameMap, TileKind};
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn is_opaque(kind: TileKind) -> bool {
    matches!(kind, TileKind::Wall)
}

// Bresenham line of sight check between two points, inclusive
pub fn los_visible(map: &GameMap, x0: i32, y0: i32, x1: i32, y1: i32) -> bool {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        if let Some(i) = map.idx(x0, y0) {
            if is_opaque(map.tiles[i]) && !(x0 == x1 && y0 == y1) {
                return false;
            }
        } else {
            return false;
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
    true
}

#[derive(Resource, Default, Debug, Clone)]
pub struct Visibility {
    pub per_entity: HashMap<Entity, HashSet<(i32, i32)>>,
}

pub fn compute_visibility_system(
    map: Res<GameMap>,
    mut vis: ResMut<Visibility>,
    q: Query<(
        Entity,
        &crate::world::Position,
        Option<&crate::components::VisionRadius>,
    )>,
) {
    let mut per = HashMap::new();
    for (e, pos, vr) in q.iter() {
        let mut visible = HashSet::new();
        let r = vr.map(|v| v.0).unwrap_or(8);
        for dy in -r..=r {
            for dx in -r..=r {
                let nx = pos.0 + dx;
                let ny = pos.1 + dy;
                if !map.in_bounds(nx, ny) {
                    continue;
                }
                if (dx * dx + dy * dy) as f32 <= (r as f32 * r as f32)
                    && los_visible(&map, pos.0, pos.1, nx, ny)
                {
                    visible.insert((nx, ny));
                }
            }
        }
        per.insert(e, visible);
    }
    vis.per_entity = per;
}
