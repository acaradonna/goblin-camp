use crate::components::{
    AssignedJob, Carriable, Carrier, DesignationLifecycle, DesignationState, Goblin, Inventory,
    Item, ItemType, Miner, Stockpile, VisionRadius, ZoneBounds,
};
use crate::designations::MineDesignation;
use crate::systems;
use crate::world::{GameMap, Name, Position, TileKind, Velocity};
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
// Cursor is only used inside decode_cbor

/// Sort entity records in a stable, deterministic order.
///
/// Ordering key: (name, pos, vel, item_type, carriable, role flags, zone/designation info)
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
        let carriable_ord = a.carriable.cmp(&b.carriable);
        if carriable_ord != Ordering::Equal {
            return carriable_ord;
        }
        let goblin_ord = a.goblin.cmp(&b.goblin);
        if goblin_ord != Ordering::Equal {
            return goblin_ord;
        }
        let miner_ord = a.miner.cmp(&b.miner);
        if miner_ord != Ordering::Equal {
            return miner_ord;
        }
        let carrier_ord = a.carrier.cmp(&b.carrier);
        if carrier_ord != Ordering::Equal {
            return carrier_ord;
        }
        let stockpile_ord = a.stockpile.cmp(&b.stockpile);
        if stockpile_ord != Ordering::Equal {
            return stockpile_ord;
        }
        let stockpile_accepts_ord = a.stockpile_accepts.cmp(&b.stockpile_accepts);
        if stockpile_accepts_ord != Ordering::Equal {
            return stockpile_accepts_ord;
        }
        let bounds_ord = a.zone_bounds.cmp(&b.zone_bounds);
        if bounds_ord != Ordering::Equal {
            return bounds_ord;
        }
        let mine_desig_ord = a.mine_designation.cmp(&b.mine_designation);
        if mine_desig_ord != Ordering::Equal {
            return mine_desig_ord;
        }
        a.designation_state.cmp(&b.designation_state)
    });
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGame {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<TileKind>,
    pub entities: Vec<EntityData>,
    // Determinism: persist tick timing and RNG seed
    // Note: RNG stream positions not yet persisted - reloading resets RNG to initial state
    // TODO: Serialize per-stream RNG state for full determinism across save/load
    #[serde(default = "default_tick_ms")]
    pub tick_ms: u64,
    #[serde(default)]
    pub ticks: u64,
    #[serde(default)]
    pub master_seed: u64,
}

fn default_tick_ms() -> u64 {
    100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityData {
    pub name: Option<String>,
    pub pos: Option<(i32, i32)>,
    pub vel: Option<(i32, i32)>,
    pub item_type: Option<ItemType>,
    pub carriable: bool,
    /// Marker for goblin agents (optional; not currently used by core systems)
    #[serde(default)]
    pub goblin: bool,
    /// Marker for miner agents (required for mining job execution)
    #[serde(default)]
    pub miner: bool,
    /// Marker for carrier agents (required for hauling job execution)
    #[serde(default)]
    pub carrier: bool,
    /// Marker for stockpile zone entities
    #[serde(default)]
    pub stockpile: bool,
    /// Optional stockpile acceptance list (None = accepts all)
    #[serde(default)]
    pub stockpile_accepts: Option<Vec<ItemType>>,
    /// Optional zone bounds for stockpiles and other rectangular zones
    #[serde(default)]
    pub zone_bounds: Option<ZoneBoundsData>,
    /// Marker for mine designation entities
    #[serde(default)]
    pub mine_designation: bool,
    /// Lifecycle state for designations (only meaningful when mine_designation=true)
    #[serde(default)]
    pub designation_state: Option<DesignationState>,
}

/// Serializable representation of `ZoneBounds` for save/load.
///
/// Stored as plain ints to avoid forcing serde derives on all ECS components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ZoneBoundsData {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
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
        Option<&Goblin>,
        Option<&Miner>,
        Option<&Carrier>,
        Option<&Stockpile>,
        Option<&ZoneBounds>,
        Option<&MineDesignation>,
        Option<&DesignationLifecycle>,
    )>();
    for (
        name,
        pos,
        vel,
        item,
        carriable,
        goblin,
        miner,
        carrier,
        stockpile,
        bounds,
        mine_desig,
        lifecycle,
    ) in q.iter(world)
    {
        let mine_designation = mine_desig.is_some();
        entities.push(EntityData {
            name: name.map(|n| n.0.clone()),
            pos: pos.map(|p| (p.0, p.1)),
            vel: vel.map(|v| (v.0, v.1)),
            item_type: item.map(|i| i.item_type),
            carriable: carriable.is_some(),
            goblin: goblin.is_some(),
            miner: miner.is_some(),
            carrier: carrier.is_some(),
            stockpile: stockpile.is_some(),
            stockpile_accepts: stockpile.and_then(|s| s.accepts.clone()),
            zone_bounds: bounds.map(|b| ZoneBoundsData {
                min_x: b.min_x,
                min_y: b.min_y,
                max_x: b.max_x,
                max_y: b.max_y,
            }),
            mine_designation,
            designation_state: if mine_designation {
                lifecycle.map(|l| l.0)
            } else {
                None
            },
        });
    }
    // Deterministic ordering across codecs and runs
    sort_entities_deterministically(&mut entities);
    // Persist determinism metadata (fallback to defaults if resources are absent)
    let (tick_ms, ticks) = match world.get_resource::<systems::Time>() {
        Some(time) => (time.tick_ms, time.ticks),
        None => (100, 0),
    };
    let master_seed = world
        .get_resource::<systems::DeterministicRng>()
        .map(|rng| rng.master_seed)
        .unwrap_or(0);

    SaveGame {
        width,
        height,
        tiles,
        entities,
        tick_ms,
        ticks,
        master_seed,
    }
}

pub fn load_world(save: SaveGame, world: &mut World) {
    world.insert_resource(GameMap {
        width: save.width,
        height: save.height,
        tiles: save.tiles,
    });
    // Restore deterministic time and RNG seed
    world.insert_resource(systems::Time {
        ticks: save.ticks,
        tick_ms: save.tick_ms,
    });
    world.insert_resource(systems::DeterministicRng::new(save.master_seed));
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
        if e.goblin {
            ec.insert(Goblin);
        }
        if e.miner {
            // Minimal required components for job systems to "see" this entity
            ec.insert((Miner, AssignedJob::default(), VisionRadius(8)));
        }
        if e.carrier {
            // Minimal required components for hauling systems to "see" this entity
            ec.insert((
                Carrier,
                Inventory::default(),
                AssignedJob::default(),
                VisionRadius(8),
            ));
        }
        if e.stockpile {
            ec.insert(Stockpile {
                accepts: e.stockpile_accepts,
            });
        }
        if let Some(bounds) = e.zone_bounds {
            ec.insert(ZoneBounds::new(
                bounds.min_x,
                bounds.min_y,
                bounds.max_x,
                bounds.max_y,
            ));
        }
        if e.mine_designation {
            ec.insert(MineDesignation);
            ec.insert(DesignationLifecycle(
                e.designation_state.unwrap_or(DesignationState::Active),
            ));
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
