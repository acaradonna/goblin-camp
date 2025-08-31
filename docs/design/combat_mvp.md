# Combat MVP: Simple Injuries and Death

This document proposes an initial, deterministic combat system to support simple melee interactions, injuries, and death. It aligns with Goblin Camp’s goals: deterministic ECS simulation, integer math, reproducible seeds, and headless CLI demos. The MVP keeps scope tight but extensible.

## Goals and non-goals

Goals

- Deterministic, integer-only damage and death with seeded RNG stream (pre-wired in `Systems`)
- Minimal components to attach combat capabilities to existing actors
- Single-tile melee attacks with adjacency checks and simple targeting
- Injury model: Hit Points (HP) + Wounds counter for flavor; no per-limb yet
- Death: entity flagged Dead; body becomes an Item (corpse) on the ground
- Pathfinding integration: avoid attacking through impassable tiles; simple chase when in sight
- Save/Load support for new components
- CLI demo rendering encounters and outcomes in ASCII

Non-goals

- Ranged combat, armor/weapon stats, damage types, body parts, bleeding, medicine
- Squad AI, morale, formations
- Complex perception; keep to simple FOV LOS queries already available

## Determinism and performance constraints

- Fixed tick cadence: damage resolution occurs in a Combat phase system after movement
- Single RNG stream `combat_rng` already present in `gc_core::systems::Systems`
- Integer math only; HP is small u16; damage is u8
- Bounded work per tick: cap attacks processed and chase path recomputations

## Data model (ECS components/resources)

Components

- Combatant { hp: u16, max_hp: u16, attack: u8, defense: u8, attack_cooldown: u8, cooldown_left: u8 }
- Faction { id: u8 } // 0 = Neutral, 1 = Player, 2 = Hostile, etc.
- Dead // marker; implies entity is no longer active in jobs/AI
- Corpse // marker on item spawned from a dead actor
- Aggro { target: Option<Entity>, last_seen: Option<IVec2> }

Resources

- CombatConfig { chase_max_steps: u16, attack_ap_cost: u8, max_attacks_per_tick: u16 }
- CombatEvents (Vec<CombatLogEvent>) for demo and tests

Enums

- CombatLogEvent { Spotted(Entity, Entity), MovedTo(Entity, IVec2), Attacked { attacker: Entity, defender: Entity, hit: bool, dmg: u8, hp_after: u16 }, Died(Entity) }

## Systems and flow

Order in schedule (post-movement, pre-jobs resolution):

1. AcquireTargets: For each Combatant with a Faction, find nearest enemy in LOS within N tiles; set Aggro.target
1. ChaseTargets: If target exists and not adjacent, request a path (bounded), step toward target; emit MovedTo
1. ResolveMelee: If adjacent and cooldown_left == 0, roll to-hit using attack vs defense and RNG; on hit, apply damage; set cooldown
1. ApplyDeaths: For any Combatant with hp == 0 and not Dead, mark Dead; despawn actor and spawn corpse item; emit Died
1. CooldownsAndCleanup: decrement cooldown_left; clear Aggro if target invalid or dead; trim events buffer

Notes

- Use existing FOV utilities to gate targeting/aggro
- Use existing pathfinding demo plumbing for chase, with conservative timeouts

## Algorithms

- Targeting: Manhattan nearest enemy within radius R; ties broken deterministically by entity id
- To-Hit: deterministic linear formula with clamp
  - base = 50 + (attack as i16 - defense as i16) * 5
  - hit if (rng_u8 % 100) < clamp(base, 5, 95)
- Damage: roll 1..=attack (rng_u8 % attack + 1); apply saturating_sub to hp
- Cooldown: fixed 1 tick for MVP

## CLI demo

- Subcommand: `combat-mvp` (hyphenated to match existing patterns like `save-load`, `path-batch`)
- Respects global flags (placed before the subcommand): `--seed`, `--width`, `--height`, `--steps`, `--ascii-map`
- Map: open arena with a few walls; defaults `width=40`, `height=20` but accepts overrides via global flags
- Spawn: 3 goblins (Faction=Player) vs 3 trolls (Faction=Hostile) with different stats
- Render per tick: ASCII with G/T for actors, corpses as x, last attack shown as * glyph overlay, and a rolling text log below
- Stop when a side is wiped or max 200 ticks; print summary (survivors, deaths, total attacks)

Examples

```bash
# Default sized arena (40x20), deterministic seed
cargo run -p gc_cli -- --seed 123 --steps 200 combat-mvp

# Custom map size with more space
cargo run -p gc_cli -- --seed 123 --width 60 --height 20 --steps 300 combat-mvp
```

## Save/Load

- Derive Serialize/Deserialize for new components/resources
- Add to snapshot in core save/load paths
- Golden test: save mid-fight, load, continue; outcome must be identical

## Tests

- Unit: to-hit math boundaries (5%/95%), damage saturation, cooldown behavior
- Integration: 1v1 deterministic duel with fixed seed produces exact log sequence
- Integration: team-vs-team ends with deterministic survivor counts and total attacks
- Persistence: golden snapshot mid-fight resumes to identical end state

## Data shapes (Rust)

Pseudo-structs (actual code will live under `crates/gc_core`):

- struct Combatant { hp: u16, max_hp: u16, attack: u8, defense: u8, attack_cooldown: u8, cooldown_left: u8 }
- struct Faction(pub u8)
- struct Aggro { target: Option<Entity>, last_seen: Option<IVec2> }
- struct CombatConfig { chase_max_steps: u16, attack_ap_cost: u8, max_attacks_per_tick: u16 }
- enum CombatLogEvent { Spotted(Entity, Entity), MovedTo(Entity, IVec2), Attacked { attacker: Entity, defender: Entity, hit: bool, dmg: u8, hp_after: u16 }, Died(Entity) }

## Story breakdown (sequenced issues)

1. Design doc (this) and index link

- Add `docs/design/combat_mvp.md`; link from docs index

1. Core types and components

- Add Combatant, Faction, Aggro, Dead, Corpse components in `gc_core`
- Add CombatConfig resource with defaults; wire into setup
- Derive serde; add to save/load schemas

1. Target acquisition system

- LOS-gated nearest-enemy selection; deterministic tie-breakers
- Populate Aggro.target and last_seen
- Emit Spotted events

1. Chase movement system

- If target not adjacent, compute bounded step toward last_seen using existing path API
- Emit MovedTo events

1. Melee resolution system

- Adjacency check; cooldown gate; to-hit + damage; HP apply
- Emit Attacked events

1. Death handling

- Detect hp==0; mark Dead; despawn actor; spawn corpse item at tile
- Remove from queries; emit Died events

1. CLI demo: combat

- `gc_cli` subcommand; render arena, entities, overlay, and text log

1. Save/Load integration

- Include new components/resources; golden snapshot test mid-fight

1. Tests: units + integrations

- Boundary math unit tests; deterministic logs for 1v1 and squad vs squad

1. Docs polish

- Update docs with CLI usage and demo screenshots (ASCII)

## Risks and mitigations

- ECS query conflicts: keep write access localized; separate systems by stages
- Path thrash: cap per-tick chases; reuse last path when possible (future)
- Determinism: single RNG stream; avoid time-based randomness

---

Author: Copilot

References: bevy_ecs 0.14 patterns; existing path/fov demos
