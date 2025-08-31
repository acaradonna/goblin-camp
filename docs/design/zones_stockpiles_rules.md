# Zones and Stockpile Rules Design

This document specifies the zone system (stockpiles and activity zones) and the rules that govern item acceptance, priorities, links, and hauling policies. It builds on Mining/Items/Stockpiles and formalizes behaviors needed for scalable logistics, determinism, and clear player control.

Related:

- Mining & Stockpiles: mining_items_stockpiles.md
- Workshops & Production Chains: workshops_production_chains.md

## Goals

- Expressive zones: rectangular areas that define storage and activities
- Deterministic, fair hauling with stable priorities and tie-breakers
- Clear filters/policies to reduce micromanagement (feeder links, give/take)
- Efficient queries and updates at scale (thousands of items)
- Save/load stable schemas with versioning

Non-goals (MVP):

- Physics for bins/barrels; z-level fluid interactions; UI

## Zone Types

- Stockpile: accepts items by filter; has capacity/priority
- GarbageDump: single-tile or area, forces drop-on-tile; can act as “matter compression” zone but we avoid exploits by enforcing per-cell stacking caps
- PenPasture: occupancy of livestock (out of scope for MVP logic, schema only)
- Hospital: reserves medical supplies (schema placeholder)
- WaterSource/Fishing: activity hints for jobs (schema placeholder)

Only Stockpile and GarbageDump have item logistics in MVP.

## Core Data Schemas

Rust-like shapes; final names may vary to match gc_core conventions.

```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct ZoneId(pub u64);

#[derive(Serialize, Deserialize, Clone)]
pub struct ZoneBounds { pub min: IVec2, pub max: IVec2 }

#[derive(Serialize, Deserialize, Clone)]
pub enum ZoneKind {
    Stockpile(StockpilePolicy),
    GarbageDump,
    PenPasture,
    Hospital(HospitalPolicy),
    WaterSource,
    Fishing,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Zone {
    pub id: ZoneId,
    pub name: String,
    pub bounds: ZoneBounds,
    pub kind: ZoneKind,
    pub created_at_tick: u64,
    pub priority: u8, // 0..=9, higher means prefer earlier
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct StockpilePolicy {
  pub accepts: SmallVec<[ItemType; 8]>,   // inclusion filter; empty means accept all item types
    pub rejects: SmallVec<[ItemType; 8]>,   // exclusion wins over accepts
    pub max_per_cell: u8,                   // stacking cap per cell (default 1)
    pub links: StockpileLinks,              // give/take connections
    pub allow_take_from_anywhere: bool,     // if false, only from links
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct StockpileLinks {
    pub give_to: SmallVec<[ZoneId; 4]>,
    pub take_from: SmallVec<[ZoneId; 4]>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct HospitalPolicy { pub reserve: SmallVec<[ItemType; 8]> }
```

Notes:

- Filters: rejects override accepts; both empty = allow all
- Priority: stable tie-breaker with ZoneId for determinism (0..=9; default 5)
- max_per_cell prevents quantum-stockpile behavior without simulating containers (default 1)
- allow_take_from_anywhere default: false (linked stockpiles only unless enabled)

### Events

```rust
pub struct ZoneCreated { pub zone: Zone }
pub struct ZoneUpdated { pub id: ZoneId }
pub struct ZoneDeleted { pub id: ZoneId }

pub struct ItemEnteredZone { pub item: Entity, pub zone: ZoneId, pub cell: IVec2 }
pub struct ItemLeftZone   { pub item: Entity, pub zone: ZoneId, pub cell: IVec2 }

pub struct ReserveCell { pub zone: ZoneId, pub cell: IVec2, pub for_item: Entity }
pub struct ReleaseCell { pub zone: ZoneId, pub cell: IVec2, pub for_item: Entity }
```

Emit debounced ZoneUpdated when filters/links/priority change.

## Systems

### Zone Indexing

- Maintain a grid index mapping tile -> ZoneId for O(1) lookups.
- For Stockpiles, maintain per-cell occupancy counts (0..=max_per_cell).
- Rebuild lazily on ZoneCreated/Updated/Deleted.

### Acceptance and Reservation

Contract:

- Input: item_type, target zone
- Output: Allow/Reject and optional reserved cell
- Error modes: No capacity, filter reject, links disallow take/give

Algorithm:

1) Filter: rejects.contains(item) ⇒ Reject; accepts empty or contains ⇒ Accept
2) Capacity: scan cells in stable order (row-major, min→max) for count < max_per_cell
3) ReserveCell event for chosen cell to avoid races; release on failure or completion

Stable order + ZoneId tie-breakers keep determinism.

### Hauling Job Generation (Stockpile → Stockpile, Ground → Stockpile)

For each OnGround item:

- Pick candidate zones that Accept, sorted by (priority desc, distance asc, zone_id asc)
- Try reservation; if ok, create Haul job to that cell.

For give/take links:

- If a zone has give_to links, items inside it become sources for linked targets.
- If allow_take_from_anywhere=false, only create haul jobs from take_from sources.

### Item Movement and Zone Updates

- On pickup, send ItemLeftZone for prior cell (if any) and decrement occupancy
- On drop, send ItemEnteredZone and increment occupancy
- If drop outside bounds due to pathing change, release reservation and retry

### Rebalancing

- Optional low-frequency system (every N ticks): if a zone overflows or links changed, propose moves respecting priorities and links; bounded work per tick.

## Determinism and Fairness

- Stable iteration orders: items by Entity id, cells row-major, zones by (priority desc, id asc)
- No RNG in selection; distance ties broken by (zone_id, cell)
- One reservation per item; timeouts release after K ticks if agent stuck

## CLI Demo (MVP)

- cargo run -p gc_cli -- zones
- Generates map with two stockpiles: Source(accept all, priority 3), Target(accept Stone, priority 5, take_from=[Source])
- Spawns scattered Stone; shows ASCII overlay with Z for zones, s for stones, arrows for planned hauls
- Prints counts per zone and per-cell occupancy

## Save/Load

- Versioned struct: ZoneV1 { id, name, bounds, kind, priority }
- Migration adds StockpilePolicyV1 { accepts, rejects, max_per_cell, links, allow_take_from_anywhere }
- Items store last_zone: Option\<ZoneId\> for quick diffs; recompute indexes on load
- Items store last_zone: `Option<ZoneId>` for quick diffs; recompute indexes on load

Example JSON (conceptual):

```json
{
  "zones": [
    {
      "id": 101,
      "name": "Target Store",
      "bounds": { "min": [10, 10], "max": [14, 12] },
      "priority": 5,
      "kind": {
        "Stockpile": {
          "accepts": ["Stone"],
          "rejects": [],
          "max_per_cell": 1,
          "links": { "give_to": [], "take_from": [100] },
          "allow_take_from_anywhere": false
        }
      }
    }
  ]
}
```

## Edge Cases

- Item type not listed anywhere: default allow; can be globally disabled by gameplay rules
- No capacity available after reservation due to race: release and retry next tick
- Overlapping zones: forbid all overlaps in MVP; later: consider z-index/precedence rules where different kinds may overlap safely
- GarbageDump: still uses max_per_cell; large max simulates compaction without infinite stacks

## Implementation Stories

1) Core types and storage: Zone, ZoneBounds, ZoneKind, StockpilePolicy, events
2) Zone indexer: tile→ZoneId map, occupancy counters, rebuild on changes
3) Filters and acceptance API with stable iteration and tests
4) Reservation system and events with timeouts
5) Haul job generation: Ground→Stockpile honoring priority and distance
6) Give/Take link logic between stockpiles; allow_take_from_anywhere flag
7) CLI demo: zones overlay and summary
8) Save/Load: schemas + migration tests
9) Rebalancing periodic system (bounded work)
10) Docs: player guidance and examples

## References

- DF Stockpiles: categories, feeder designs, lag considerations
- RimWorld Stockpile Zones: shelves and refill behavior near workbenches
- Oxygen Not Included: pressure/stacking inspiration (we cap per cell)
