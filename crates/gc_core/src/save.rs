use crate::components::{Carriable, Item, ItemType};
use crate::world::{GameMap, Name, Position, TileKind, Velocity};
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
// Cursor is only used inside decode_cbor

/// Sort entity records in a stable, deterministic order.
///
/// Ordering key: (name, pos, vel, item_type, carriable)
fn sort_entities_deterministically(entities: &mut [EntityData]) {
    use std::cmp::Ordering;
    entities.sort_by(|a, b| {
        let name_ord = a.name.cmp(&b.name);
        if name_ord != Ordering::Equal {
            return name_ord;
        }
        let pos_ord = a.pos.cmp(&b.pos);
        if pos_ord != Ordering::Equal {
            return pos_ord;
        }
        let vel_ord = a.vel.cmp(&b.vel);
        if vel_ord != Ordering::Equal {
            return vel_ord;
        }
        let item_ord = a.item_type.cmp(&b.item_type);
        if item_ord != Ordering::Equal {
            return item_ord;
        }
        a.carriable.cmp(&b.carriable)
    });
}

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
    // Deterministic ordering across codecs and runs
    sort_entities_deterministically(&mut entities);
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

// --- Minimal codec helpers (format-agnostic call sites) ---

/// Encode a SaveGame to JSON string
pub fn encode_json(save: &SaveGame) -> Result<String, serde_json::Error> {
    serde_json::to_string(save)
}

/// Decode a SaveGame from JSON string
pub fn decode_json(s: &str) -> Result<SaveGame, serde_json::Error> {
    serde_json::from_str(s)
}

/// Encode a SaveGame to RON string
pub fn encode_ron(save: &SaveGame) -> Result<String, ron::Error> {
    ron::ser::to_string(save)
}

/// Decode a SaveGame from RON string
pub fn decode_ron(s: &str) -> Result<SaveGame, ron::Error> {
    ron::de::from_str::<SaveGame>(s).map_err(ron::Error::from)
}

/// Encode a SaveGame to CBOR bytes
pub fn encode_cbor(save: &SaveGame) -> Result<Vec<u8>, ciborium::ser::Error<std::io::Error>> {
    let mut buf = Vec::new();
    ciborium::ser::into_writer(save, &mut buf)?;
    Ok(buf)
}

/// Decode a SaveGame from CBOR bytes
pub fn decode_cbor(bytes: &[u8]) -> Result<SaveGame, ciborium::de::Error<std::io::Error>> {
    use std::io::Cursor;
    let mut cur = Cursor::new(bytes);
    ciborium::de::from_reader(&mut cur)
}
