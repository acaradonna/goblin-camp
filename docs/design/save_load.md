# Save/Load v2 — Versioned Schemas, Formats, and Migrations (Epic #38)

Status: Draft → Ready for implementation
Owner: @acaradonna
Related: worldgen, fluids, combat, zones, jobs, TUI

## Goals

- Forward- and backward-compatible saves with explicit schema versions
- Deterministic roundtrips (serialize → deserialize → serialize equals original) for stable seeds
- Pluggable formats: RON (human-readable) and CBOR (compact, fast) with the same logical schema
- Mod/content manifest recorded with save to validate compatibility
- Safe migrations between versions with test coverage

Non-goals (MVP): live hot-reload of saves; streaming large-world chunks

## Format Options

- RON: default in debug/dev; easier to diff and review
- CBOR: default in release; compact, faster IO
- Common logical model in code; only the encoder/decoder changes

File layout (per save slot):

- save.ron/save.cbor: header + world snapshot
- meta.json: ContentManifest { game_version, schema_version, mods[], created_at, seed, map_size }
- thumbnails/optional ascii frames for quick preview (future)

## Header and Versioning

Header { magic: "GCSAVE", version: u16 (schema), codec: enum(RON, CBOR), checksum: u32 }

- Schema version increments on any breaking change to the logical model
- Component/resource registries map types to stable IDs; IDs persisted, not Rust paths

## Logical Model (Snapshot)

- WorldMeta { seed, ticks, width, height }
- Entities: array of { id: u64, components: map<ComponentId, bytes> }
- Resources: map<ResourceId, bytes>
- Indices: optional generated indices omitted from save to reduce size

Serialization rules:

- Component/Resource derives Serialize/Deserialize with serde; wrap with versioned enums when needed
- Use small-int enums and fixed-size numeric types; avoid floats where possible
- Collections serialized in stable order (sorted keys/IDs) for determinism

## Migrations

- Migration table: for each schema N->N+1, define up() and optional down() steps
- Steps operate on deserialized intermediate structs or on serde_value trees (format-agnostic)
- Version gate at load: if save.version < current, apply migrations sequentially; if > current, reject with clear error

Examples:

- Health v0 { hp: u8 } -> v1 { hp: u16, max: u16 }: up maps hp to both fields
- Inventory v1 adds slots[]: up creates empty default

## Determinism

- DeterministicRng state is serialized per-stream; upon load, streams resume exactly
- Tick counter saved; schedule restart from consistent stage order
- Maps serialized row-major; entities sorted by id; components sorted by ComponentId

## CLI

- Subcommands: `save-load` (exists) extended to support `--codec ron|cbor` and print header info
- Example: `cargo run -p gc_cli -- save-load --codec cbor`

## Tests

- Roundtrip: create world, serialize (RON/CBOR), deserialize, reserialize → byte-equal for each codec
- Golden saves: store a few small snapshots and ensure they load as schemas evolve (migrate if needed)
- Fuzz small randomized worlds to catch edge cases

## Implementation Plan (Stories)

1. Schema core: Header, ComponentId/ResourceId registries, ContentManifest
2. RON encoder/decoder for logical model; wire into existing save-load demo
3. CBOR encoder/decoder; feature-flag selection and CLI `--codec`
4. Deterministic ordering utilities (sort entities/components); tests
5. Migration framework with registry; implement v0→v1 example migration
6. Persist DeterministicRng streams and tick; verify determinism
7. Golden saves and compatibility tests
8. Documentation and troubleshooting guide

## Risks & Mitigations

- Drift between codecs: centralize logical model; tests run both codecs
- Large saves: adopt simple RLE for large zero regions; switch to CBOR in release
- Mod breakage: include mod hashes in ContentManifest; refuse to load with mismatched mods (or warn if safe)

## Tracking

- [1/8] Schema core — Header, registries, ContentManifest: #149
- [2/8] RON codec — encoder/decoder and CLI integration: #151
- [3/8] CBOR codec — encoder/decoder and feature-flag: #158
- [4/8] Deterministic ordering utilities + tests: #153
- [5/8] Migration framework + v0→v1 example: #154
- [6/8] Persist DeterministicRng streams and tick: #155
- [7/8] Golden saves and compatibility tests: #156
- [8/8] Documentation and troubleshooting guide: #157
