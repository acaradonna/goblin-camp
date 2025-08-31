# Needs, Moods, and Traits

Goal: implement a deterministic, data-driven model for colonist motivations (needs), short-term affect (mood via thoughts/memories), and longer-term personality (traits) that influences job choice, movement, combat morale, and social behaviors.

This document defines data shapes, constants, ECS components/systems, update cadence, save/load schema, CLI demos, and an incremental plan to deliver an MVP that is fast, predictable, and fun to tune.

Scope for MVP

- Needs: hunger, thirst, sleep, warmth, hygiene, social, fun
- Thoughts: small, decaying modifiers from recent events (e.g., ate good meal, slept in barracks, saw corpse)
- Mood: aggregate scalar from needs + thoughts, with thresholds → statuses (e.g., Low, Neutral, High)
- Traits: static modifiers per entity that tweak rates/weights and unlock special behaviors (e.g., Glutton, NightOwl, Brave)
- Integration: job scoring bias toward need-satisfying tasks; pathfinding cost hooks; combat morale gates

Not in MVP (tracked for later epics)

- Complex relationships (friendship/romance), grudges, memories-of-others
- Deep conversation/skills systems
- Long-term mental breaks/tantrum cascades (we’ll add a minimal “refusal/short-break” state only)

## Determinism principles

- Fixed-timestep updates in a dedicated schedule set executed once per simulation tick before jobs/fov/pathfinding.
- All randomness via RNG seeded from world seed + tick + entity ID to ensure reproducibility.
- Bounded work: per-entity O(Needs+Thoughts) with cap on active thoughts to avoid unbounded vectors.

## Data model (no floats)

- Scale: Needs and Mood use permille S1k fixed scale for clarity: 0..=1000.
  - Pros: human-readable, simple math, easy thresholds, fewer overflows than S10.
- Types:
  - NeedValue = u16 (0..=1000)
  - NeedRate = i16 (delta per T_NEED_TICK)
  - MoodValue = i16 (-1000..=+1000)
  - ThoughtImpact = i16 (-500..=+500 typical)
  - TraitId = u16 (indexed into registry)

## Constants

```text
// Tick cadence
T_NEED_TICK: u16 = 10     // needs update every 10 sim ticks
T_THOUGHT_TICK: u16 = 10  // thought decay cadence (can share with needs)

// Saturation bounds
NEED_MIN: NeedValue = 0
NEED_MAX: NeedValue = 1000
MOOD_MIN: MoodValue = -1000
MOOD_MAX: MoodValue = 1000

// Mood thresholds (inclusive ranges)
MOOD_LOW: MoodValue = -300
MOOD_HIGH: MoodValue = 300

// Thought decay half-life in ticks (piecewise power-of-two decay)
THOUGHT_HALF_LIFE: u16 = 600 // ~60 need ticks if T_NEED_TICK=10

// Per-need base decay rates per T_NEED_TICK (negative is decay)
BASE_DECAY: {
  Hunger:  -3,
  Thirst:  -4,
  Sleep:   -2,
  Warmth:  context-dependent (see Environment),
  Hygiene: -1,
  Social:  -1,
  Fun:     -1,
}

// Satiation recovery when performing a satisfying action (typical values)
RECOVERY_PER_ACTION: {
  EatMeal:  +350,
  Drink:    +500,
  Sleep:    +700,
  WarmUp:   +250,
  Bathe:    +300,
  Chat:     +200,
  Play:     +250,
}

// Weighting for mood composition (sum to ~100 for readability)
MOOD_WEIGHTS: {
  NeedsVector: 70,
  Thoughts:    30,
}

// Traits influence ranges (applied as multipliers or offsets)
TRAIT_RATE_MULT_MIN: i16 = 50   // 50% speed (slower decay)
TRAIT_RATE_MULT_MAX: i16 = 150  // 150% speed (faster decay)
```

## ECS components and resources

### Components

- Needs { hunger, thirst, sleep, warmth, hygiene, social, fun: NeedValue }
- NeedRates { per-need i16 effective deltas, post-traits/environment }
- Thoughts { entries: fixed-cap `Vec<Thought>` with ring index }
  - Thought { impact: ThoughtImpact, ttl: u16, half_life: u16 }
- Mood { value: MoodValue, status: MoodStatus }
  - MoodStatus = Low | Neutral | High
- Traits { trait_ids: SmallVec<TraitId, N=8> }

### Resources

- TraitRegistry { map<TraitId, TraitDef> }
  - TraitDef { name, desc, need_rate_mult: Option<HashMap<Need, i16>>, mood_bias: i16, flags }
- NeedConfig/MoodConfig for tunables (weights, rates); loaded at startup
- EnvironmentFeed { per-tile warmth_modifier, shelter flags, etc. }

## Systems and scheduling

Order within `ScheduleSet::NeedsAndMood` (runs once per tick; gated by every_n_ticks where noted):

1) ComputeEffectiveNeedRates (every T_NEED_TICK)
  - rates = BASE_DECAY +/- environment +/- trait multipliers (clamped)
2) ApplyNeedRates (every T_NEED_TICK)
  - needs[i] = clamp(needs[i] + rates[i])
3) DecayThoughts (every T_THOUGHT_TICK)
  - power-of-two style decay: impact → impact - sign(impact)*max(1, |impact| >> shift)
  - decrement ttl; drop when ttl==0 or |impact| < epsilon
4) ComputeMood (every T_THOUGHT_TICK)
  - needs_score = average(map need→ (need - 500)) mapped to -500..+500
  - thoughts_score = sum(active thought impacts, clamped -500..+500)
  - mood_raw = w_needs*needs_score + w_thoughts*thoughts_score normalized to -1000..+1000
  - set Mood.value, Mood.status by thresholds
5) PropagateMoodFlags
  - set lightweight flags for downstream systems (e.g., LowMood → job_score_bias, move_speed_mul)
6) IngestEvents → Thoughts
  - subscribe to domain events and enqueue Thought entries with configured impacts/ttls

## Determinism guards

- Fixed ordering: stable entity iteration (e.g., by Entity index) and no parallel mutation within the same component archetype pass.
- RNG seeded as rng(entity_id, tick, SYSTEM_KIND) to resolve equal-choice ties identically.

## Events → Thoughts mapping (examples)

- Ate(MealQuality) → +100..+300
- Slept(LocationQuality) → +50..+250
- SoakedInRain → -80
- WitnessedCorpse → -250 (stack-limited per day)
- TookDamage(Severe) → -200
- WonCombat → +200

## Job scoring integration

- Provide JobBias { hunger_urgency, thirst_urgency, sleep_urgency, …, mood_bias } resource from Needs/Mood.
- Job selection multiplies base desirability by (1 + k * urgency) and adds a small tie-breaker from RNG seed.
- Hard blocks: below critical thresholds (e.g., sleep < 100) some jobs become ineligible except self-help.

## Combat and movement hooks (MVP)

- Movement: Low mood → move_speed_mul = 0.9; High mood → 1.05 (tunable).
- Combat: Low mood → flee_chance +p; High mood → bravery +p; Traits (Brave/Cowardly) push these further.

## Persistence (serde)

- Components Needs, Thoughts, Mood, Traits are serde Serialize/Deserialize.
- TraitRegistry loads from static table for now (later: external data packs).
- Backward/forward compatibility: versioned enums and tables; avoid reordering variants.

## CLI demo

```bash
cargo run -p gc_cli -- needs
```

- Spawns 1–5 goblins with different trait sets.
- Prints per-tick (or every 10 ticks) compact bars: HUNGER: ███░ … MOOD: -123 (Low)
- Triggers scripted events (eat/sleep/see corpse) to visualize mood swings deterministically.

## Tests (deterministic)

- needs_decay_bounds: after N need ticks, each need within expected range for given rates
- thought_decay_half_life: impact halves approximately after THOUGHT_HALF_LIFE
- mood_composition_weights: altering weights shifts mood predictably
- traits_modify_rates: Glutton speeds hunger decay; NightOwl slows sleep decay at night window
- event_to_thought: events enqueue thoughts with correct impact and ttl

## Implementation plan (slices)

1) Core types and configs
  - Enums, newtypes, constants, NeedConfig/MoodConfig, TraitId/registry def
2) Components and basic systems
  - Needs, NeedRates; ComputeEffectiveNeedRates + ApplyNeedRates
3) Thoughts storage and decay
  - Fixed-cap ring buffer per entity; DecayThoughts
4) Mood computation and statuses
  - ComputeMood, PropagateMoodFlags
5) Trait registry and modifiers
  - Load defaults; apply multipliers in effective rates and mood bias
6) Events → thoughts ingestion
  - Minimal event types and mapping fn; enqueue on entities
7) Job scoring integration (bias + hard blocks)
  - Expose JobBias; integrate with job assignment
8) CLI demo (needs)
  - Add subcommand; pretty ASCII; deterministic script
9) Save/Load support
  - serde for components; snapshot roundtrip test
10) Tests and polish

- unit + integration + doc tuning; config constants review

## Risks and mitigations

- Overhead on large populations → keep per-entity work O(1), tick every 10 sim ticks, fixed-cap thoughts.
- Tuning complexity → configs live in resource; CLI demo to visualize and iterate.
- Non-determinism → stable iteration, seeded RNG, no floating point.

## Appendix: Example parameterization

- Trait: Glutton { need_rate_mult[Hunger] = 130 }
- Trait: NightOwl { sleep_decay slower at night: contextual multiplier via EnvironmentFeed time-of-day }
- Trait: Brave { mood_bias +50; flee threshold reduced }
