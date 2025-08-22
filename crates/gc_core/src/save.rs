use bevy_ecs::prelude::*;
use serde::{Serialize, Deserialize};
use crate::world::{GameMap, TileKind, Position, Velocity, Name};

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
    pub pos: Option<(i32,i32)>,
    pub vel: Option<(i32,i32)>,
}

pub fn save_world(world: &mut World) -> SaveGame {
    // Clone map data first to avoid overlapping borrows with query construction
    let (width, height, tiles) = {
        let map = world.resource::<GameMap>();
        (map.width, map.height, map.tiles.clone())
    };

    let mut entities = Vec::new();
    let mut q = world.query::<(Option<&Name>, Option<&Position>, Option<&Velocity>)>();
    for (name, pos, vel) in q.iter(world) {
        entities.push(EntityData {
            name: name.map(|n| n.0.clone()),
            pos: pos.map(|p| (p.0, p.1)),
            vel: vel.map(|v| (v.0, v.1)),
        });
    }
    SaveGame { width, height, tiles, entities }
}

pub fn load_world(save: SaveGame, world: &mut World) {
    world.insert_resource(GameMap { width: save.width, height: save.height, tiles: save.tiles });
    for e in save.entities {
        let mut ec = world.spawn(());
        if let Some(name) = e.name { ec.insert(Name(name)); }
        if let Some((x,y)) = e.pos { ec.insert(Position(x,y)); }
        if let Some((vx,vy)) = e.vel { ec.insert(Velocity(vx,vy)); }
    }
}
