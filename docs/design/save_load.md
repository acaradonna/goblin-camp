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

## Troubleshooting Guide

### Common Issues and Solutions

#### Save File Corruption
**Symptoms**: Save file fails to load with "Invalid header" or "Checksum mismatch" errors.

**Causes**:
- File was truncated during write (power loss, disk full)
- File was edited manually and corrupted
- Different codec than expected

**Solutions**:
1. Check file size - should be non-zero and reasonable for your world size
2. Verify file extension matches codec: `.ron` for RON, `.cbor` for CBOR
3. Try loading with explicit codec: `--codec ron` or `--codec cbor`
4. Check for backup saves in the same directory

#### Schema Version Mismatch
**Symptoms**: "Schema version X not supported" or "Migration failed" errors.

**Causes**:
- Save was created with newer game version
- Save was created with older game version that needs migration
- Migration system has bugs

**Solutions**:
1. Update to latest game version for newer saves
2. Check migration logs for specific failure points
3. Use `--verbose` flag to see detailed migration steps
4. Report migration failures with save file and game version

#### Determinism Issues
**Symptoms**: Same seed produces different results after save/load.

**Causes**:
- RNG streams not properly serialized/restored
- Non-deterministic iteration order in systems
- Floating-point precision issues

**Solutions**:
1. Verify DeterministicRng state is saved in header
2. Check that entity iteration is stable (sorted by ID)
3. Use integer-only math where possible
4. Run determinism tests: `cargo test determinism`

#### Performance Issues
**Symptoms**: Save/load operations are very slow.

**Causes**:
- Large world with many entities
- Using RON format for large saves
- Inefficient serialization of components

**Solutions**:
1. Use CBOR format for large saves: `--codec cbor`
2. Profile serialization bottlenecks
3. Consider chunking large worlds (future feature)
4. Optimize component serialization

#### Mod Compatibility
**Symptoms**: "Mod mismatch" or "Content manifest invalid" errors.

**Causes**:
- Save created with different mods enabled
- Mod versions changed between save and load
- Mod conflicts or corruption

**Solutions**:
1. Check ContentManifest in save file for mod list
2. Enable same mods that were active when save was created
3. Update mods to compatible versions
4. Disable conflicting mods

### CLI Examples

#### Basic Save/Load Operations
```bash
# Save current world (default RON format)
cargo run -p gc_cli -- save-load --save my_world

# Save with CBOR format (more compact)
cargo run -p gc_cli -- save-load --save my_world --codec cbor

# Load a save file
cargo run -p gc_cli -- save-load --load my_world.ron

# Load with explicit codec
cargo run -p gc_cli -- save-load --load my_world.cbor --codec cbor
```

#### Debugging and Validation
```bash
# Validate save file integrity
cargo run -p gc_cli -- save-load --validate my_world.ron

# Show save file header information
cargo run -p gc_cli -- save-load --info my_world.ron

# Convert between formats
cargo run -p gc_cli -- save-load --convert my_world.ron --output my_world.cbor --codec cbor

# Run determinism test
cargo run -p gc_cli -- save-load --test-determinism --seed 12345 --steps 1000
```

#### Migration and Compatibility
```bash
# Force migration to latest schema
cargo run -p gc_cli -- save-load --load old_save.ron --migrate

# Check what migrations would be applied
cargo run -p gc_cli -- save-load --load old_save.ron --dry-run

# Create backup before migration
cargo run -p gc_cli -- save-load --load old_save.ron --backup --migrate
```

### Error Codes and Meanings

| Error Code | Meaning | Solution |
|------------|---------|----------|
| `SAVE_001` | Invalid header magic | File is corrupted or not a Goblin Camp save |
| `SAVE_002` | Unsupported schema version | Update game or use migration |
| `SAVE_003` | Checksum mismatch | File corruption - try backup |
| `SAVE_004` | Codec mismatch | Use correct `--codec` flag |
| `SAVE_005` | Migration failed | Check migration logs, report bug |
| `SAVE_006` | Mod compatibility error | Enable required mods |
| `SAVE_007` | Determinism violation | Check RNG state and iteration order |
| `SAVE_008` | File I/O error | Check disk space and permissions |

### Best Practices

#### For Developers
1. **Always derive Serialize/Deserialize** for new components
2. **Use stable iteration order** (sort by entity ID)
3. **Test migrations thoroughly** with real save files
4. **Keep schema changes backward-compatible** when possible
5. **Document breaking changes** in migration steps

#### For Users
1. **Regular backups** - save system creates automatic backups
2. **Use CBOR for large worlds** - better performance and smaller files
3. **Keep mods updated** - avoid version mismatches
4. **Report issues** with save files and error messages
5. **Test saves** after major game updates

#### For Modders
1. **Include version in mod manifest** - helps with compatibility
2. **Test with save/load cycles** - ensure mod state persists
3. **Handle migration gracefully** - provide fallbacks for missing data
4. **Document mod requirements** - list dependencies and versions
5. **Use stable component IDs** - avoid breaking existing saves

### Debugging Tools

#### Save File Inspector
```bash
# Examine save file structure
cargo run -p gc_cli -- save-load --inspect my_world.ron

# Show entity count and component distribution
cargo run -p gc_cli -- save-load --stats my_world.ron

# Validate specific components
cargo run -p gc_cli -- save-load --validate-components my_world.ron
```

#### Migration Debugging
```bash
# Show migration path
cargo run -p gc_cli -- save-load --migration-path old_save.ron

# Test migration without applying
cargo run -p gc_cli -- save-load --test-migration old_save.ron

# Show migration logs
cargo run -p gc_cli -- save-load --load old_save.ron --verbose
```

## Tracking

- [1/8] Schema core — Header, registries, ContentManifest: #149
- [2/8] RON codec — encoder/decoder and CLI integration: #151
- [3/8] CBOR codec — encoder/decoder and feature-flag: #158
- [4/8] Deterministic ordering utilities + tests: #153
- [5/8] Migration framework + v0→v1 example: #154
- [6/8] Persist DeterministicRng streams and tick: #155
- [7/8] Golden saves and compatibility tests: #156
- [8/8] Documentation and troubleshooting guide: #157
