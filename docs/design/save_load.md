# Save/Load

Requirements:

- Versioned, forward-compatible saves
- Deterministic RNG w/ seeded streams

Format:

- JSON for MVP; switch to RON/CBOR later for perf

Strategy:

- Snapshot ECS world; custom (de)serialize components/resources
- Content manifest records versions and mod set
