Title: Save format lacks versioning and migration strategy

Summary
- The `SaveGame` format in `gc_core::save` has no explicit versioning. As components/entities evolve, old saves may become incompatible without a migration path.

Details
- Location: `crates/gc_core/src/save.rs`.
- `SaveGame` and `EntityData` have no `version` field.
- `load_world` directly deserializes and spawns components without handling schema changes.

Impact
- Breaking changes to components or map structure will prevent loading older saves and complicate testing and user workflows.

Proposed Fix
- Add a `version: u32` to `SaveGame`.
- Define a migration module that can upgrade older versions to the current shape.
- Consider feature-gating serialization of optional fields to keep forwards-compat behavior.

Acceptance Criteria
- Old saves (v1) load via migration to v2 once versioning is added.
- New tests cover loading a prior-version save file.
