use crate::components::{Carriable, Item, ItemType};
use crate::world::{GameMap, Name, Position, TileKind, Velocity};
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SaveGame {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<TileKind>,
    pub entities: Vec<EntityData>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityData {
    pub name: Option<String>,
    pub pos: Option<(i32, i32)>,
    pub vel: Option<(i32, i32)>,
    pub item_type: Option<ItemType>,
    pub carriable: bool,
}

pub fn save_world(world: &mut World) -> SaveGame {
    // Clone map data first to avoid overlapping borrows with query construction
    let (width, height, tiles) = {
        let map = world.resource::<GameMap>();
        (map.width, map.height, map.tiles.clone())
    };

    let mut entities = Vec::new();
    let mut q = world.query::<(
        Option<&Name>,
        Option<&Position>,
        Option<&Velocity>,
        Option<&Item>,
        Option<&Carriable>,
    )>();
    for (name, pos, vel, item, carriable) in q.iter(world) {
        entities.push(EntityData {
            name: name.map(|n| n.0.clone()),
            pos: pos.map(|p| (p.0, p.1)),
            vel: vel.map(|v| (v.0, v.1)),
            item_type: item.map(|i| i.item_type),
            carriable: carriable.is_some(),
        });
    }
    SaveGame {
        width,
        height,
        tiles,
        entities,
    }
}

pub fn load_world(save: SaveGame, world: &mut World) {
    world.insert_resource(GameMap {
        width: save.width,
        height: save.height,
        tiles: save.tiles,
    });
    for e in save.entities {
        let mut ec = world.spawn(());
        if let Some(name) = e.name {
            ec.insert(Name(name));
        }
        if let Some((x, y)) = e.pos {
            ec.insert(Position(x, y));
        }
        if let Some((vx, vy)) = e.vel {
            ec.insert(Velocity(vx, vy));
        }
        if let Some(item_type) = e.item_type {
            ec.insert(Item { item_type });
        }
        if e.carriable {
            ec.insert(Carriable);
        }
    }
}
