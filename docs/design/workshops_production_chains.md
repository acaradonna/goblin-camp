# Epic #32: Workshops and Production Chains

Status: Draft (design for discussion and iteration)

Owner: Core simulation (gc_core) with CLI demo (gc_cli) and optional TUI overlays (gc_tui)

Scope: Minimal viable production system with extensible data model and ECS systems to support multi-step crafting, hauling integration, and automation knobs.

## Goals

- Represent workstations (workshops) that transform inputs to outputs over time.
- Define recipes (inputs, optional containers/fuel, outputs, byproducts, work time, skill) decoupled from code.
- Generate work orders (player-queued or auto) that spawn jobs to haul ingredients and operate the station.
- Integrate with existing ECS: designations/jobs/pathfinding/stockpiles/visibility.
- Deterministic, testable simulation with reproducible outcomes.
- Save/load compatibility with versioned schemas.

Non-goals (MVP):

- No power network or heat simulation (can stub “fuel” as a consumable item type, no temperature).
- No item quality; no workshop ownership; no squad priorities beyond existing job weights.
- No multi-z fluid/power; no UI for complex management beyond CLI demo.

## Concepts and Data Model

Terminology kept generic to map cleanly to Dwarf Fortress/RimWorld patterns.

- WorkshopStation (entity): a placed workstation that can run one recipe at a time.

  Components:

  - Station: station_id (string), state (Idle | WaitingForHaul | Ready | Working), current_order (`Option<WorkOrderId>`), position.
  - StationInventory: input_slots (`Vec<Slot>`), output_slots (`Vec<Slot>`), reserved (`Vec<Reservation>`). Slots hold stackable items by item kind.
  - StationPolicy: product_destination (DropOnFloor | TakeToBestStockpile | TakeTo(StorageGroupId)), ingredient_radius (u32, tiles), auto_start (bool).
  - StationLinks (optional): stockpile links (accept-from / give-to) for tighter control.

- Recipe (asset/registry entry):

  - id: string
  - station_ids: [string] (which stations can run it)
  - inputs: list of IngredientSpec
  - containers: optional reusable items required (e.g., bag, jug) with return policy
  - fuel: optional fuel item requirement per craft (consumed)
  - outputs: list of ProductSpec (kind, count, quality=None for MVP)
  - byproducts: optional outputs flagged as byproduct (e.g., ash)
  - work_time_ticks: u32 (simulation ticks)
  - skill: optional skill id that scales speed/quality (ignored for MVP speed only)
  - categories/tags: for UI/grouping and automation rules

- WorkOrder (entity):

  - recipe_id, count (DoTimes N | DoForever | UntilHave N with counters)
  - filters: allowed ingredients/materials, min/max skill for worker (MVP: off)
  - destination policy override (optional)
  - progress: remaining, suspended (bool), metrics
  - station_target: Station entity or any station of allowed type

- Job types (integration with gc_core):

  - HaulIngredientsToStation: reserve N of ingredient kind(s), path, deliver to input slot(s)
  - OperateStation: occupy station, run work_time, produce outputs, place per policy
  - ClearOutputs: haul outputs to destination if policy != DropOnFloor (may be indirect via stockpile logic)

- Inventory and Reservations:

  - Extend Item reservation/reservation-guard to prevent double-claim across haulers.
  - Slot model ensures all inputs are present before OperateStation can start.

## Systems and Flow

High-level tick loop for one station:

1. Pull Orders: If Idle and station has queued WorkOrder, choose next recipe instance.
2. Material Check: Query inventory/stockpiles within ingredient_radius honoring StationLinks; compute missing inputs.
3. Haul Scheduling: For each missing ingredient, enqueue HaulIngredientsToStation jobs with reservations; station moves to WaitingForHaul.
4. Ready Gate: When all inputs present in input slots, transition to Ready.
5. Operate: Spawn OperateStation job; worker pathfinds, occupies station, runs for work_time_ticks; on completion, consume inputs/fuel, emit outputs/byproducts to output slots or floor; advance WorkOrder progress.
6. Post-process: If destination policy requires, create hauling jobs for outputs.
7. Loop: Continue until order complete or suspended; then pick next.

Scheduling style: push-based from WorkOrder to Job Board (existing system). Hauling and operation are normal jobs with priorities fit into the current job selection.

Triggers:

- Inventory change events (item spawned/despawned/reserved/released) should wake stations in WaitingForHaul to re-evaluate missing inputs.
- WorkOrder state changes (suspend/resume/completed) wake stations.

## MVP Stations and Recipes

- Carpenter’s Bench (station_id: carpenter):
  - Recipe: logs_to_planks: inputs: 1x Log -> outputs: 4x Plank, time: 50 ticks

- Mason’s Workshop (station_id: mason):
  - Recipe: stone_to_blocks: inputs: 1x Stone -> outputs: 1x Block, time: 50 ticks

No containers/fuel/quality for MVP; Product destination default: DropOnFloor to keep hauling decoupled in first iteration.

## ECS Integration Details

### Components in gc_core

- Station, StationInventory, StationPolicy, StationLinks, WorkOrder, RecipeRef
- Marker components for station type (Carpenter, Mason) may be useful for queries

### Systems in gc_core

- station_pick_next_order_system
- station_compute_missing_inputs_system (produces HaulIngredients jobs)
- station_ready_gate_system
- station_operate_system (spawns OperateStation job)
- station_output_dispatch_system (optional for non-DropOnFloor)
- inventory_reservation_system (extend existing)

### Job Board

- Define JobKinds: HaulToStation, OperateStation
- Haul job similar to existing hauling but with target slot; if partial stacks, allow multiple trips.
- Operate job requires station occupancy (a simple mutex/Occupied component on station).

### Pathfinding

- Station occupies its tile; ensure interaction cell matches map conventions. For MVP, stations are 1x1 floor-tile “benches”.

## Data and Assets

Recipe Registry:

- Provide a serde-friendly JSON/RON registry file in gc_core/resources/recipes.json (or embed via include_str!).
- Load on startup into a `Resource<RecipeRegistry>`.
- For MVP, hardcode two recipes; config support eases later expansion.

Example (illustrative):
{
  "recipes": [
    {"id":"logs_to_planks","stations":["carpenter"],"inputs":[{"item":"Log","count":1}],"outputs":[{"item":"Plank","count":4}],"work_time_ticks":50},
    {"id":"stone_to_blocks","stations":["mason"],"inputs":[{"item":"Stone","count":1}],"outputs":[{"item":"Block","count":1}],"work_time_ticks":50}
  ]
}

Note: exact item names map to existing item kinds in items/stockpiles design.

## Policies and Automation (MVP settings)

- WorkOrder modes supported:
  - DoTimes(N)
  - DoForever
  - UntilHave(N) [planned; requires colony inventory counting and destination handling]

- Destination:
  - Start with DropOnFloor; later support TakeToBestStockpile and TakeTo(StorageGroupId).

- Ingredient radius:
  - Respect StationPolicy. For MVP, radius can be large to avoid deadlocks.

## Edge Cases and Error Modes

- Missing ingredients: Haul jobs may fail due to reservation conflicts; system retries on inventory change.
- Partial stacks: Allow accumulating across multiple hauls until required count met per slot.
- Output overflow: If output slots full and DropOnFloor blocked, spill to adjacent free tiles; if none, block Operate until space available.
- Worker preemption: If Operate job interrupted, keep inputs in slots, station returns to Ready; outputs not produced until completion.
- Save/load mid-operation: Persist remaining work_time_ticks and slot contents deterministically.

## Save/Load

- Add versioned serde for Station*, WorkOrder, and RecipeRegistry ID references.
- Snapshot test for a simple world: place station, enqueue order, run N ticks, verify outputs.

## Performance

- Avoid O(N_items) scans every tick by maintaining indices:
  - Inventory index by ItemKind -> locations with counts (existing stockpile system may expose this).
  - Event-driven updates to recompute missing inputs only when inventory changes or orders update.
- Batch path requests for hauling similar to existing path-batch demo; reuse LRU cache.

## CLI Demo (gc_cli)

- Subcommand: `workshops`
  - Generate a small map with wood and stone items nearby, spawn one carpenter and one mason station, enqueue: 1) 2x logs_to_planks, 2) 3x stone_to_blocks.
  - Print ascii frames every few ticks showing station state [I/W/R/•], item counts, and job queue summary.

## Tests

- Unit: recipe registry parse/validate.
- Integration: deterministic run that completes a fixed order set and asserts final inventory counts (e.g., planks=8, blocks=3) and station idle.
- Save/Load: serialize mid-operation and resume to completion.

## Phased Stories (ordered)

1) Core types and registry: Recipe, IngredientSpec, ProductSpec, RecipeRegistry; parsing and validation; tests.
2) Station entity and components: Station, StationInventory, StationPolicy; spawn helpers; tests.
3) WorkOrder entity and simple queueing (DoTimes, DoForever); attach to stations.
4) Haul to station job: reservations, input slots fill; integration with existing hauling.
5) Operate station job: consume inputs over time, produce outputs; DropOnFloor only.
6) MVP stations and recipes: carpenter/mason and two example recipes; registry wires.
7) CLI demo: workshops; ascii summary output; docs on usage.
8) Integration tests: end-to-end counts; snapshot of ascii summary optional.
9) Save/Load: persist new components; round-trip test.
10) Optional: UntilHave(N) mode with colony inventory counting; output hauling to stockpiles.

## Open Questions

- Should station placement be via designation and construction job first (vs. test helpers)? MVP may spawn stations programmatically for demos/tests; full build pipeline can come later.
- Where to model containers (reusable bags/jugs) in items now vs. later?
- Job priority tuning vs. existing job system weights.

## Acceptance Criteria (MVP)

- Given a map with logs and stone, and one carpenter + one mason station, when I enqueue orders (2 planks crafts, 3 blocks crafts), the system hauls inputs, runs operation, and produces outputs deterministically within a bounded time. Final counts match the expected outputs.
- All new types serialize/deserialize and integration tests pass.
- Demo runs and prints a clear textual progress summary.
