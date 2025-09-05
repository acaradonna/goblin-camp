Title: Pathfinding cache eviction semantics and stats reset clarity

Summary
- `PathService` uses `lru::LruCache` with a capacity but does not document eviction impacts on hit/miss stats. `reset_stats` resets counters but not cache content, which can confuse benchmarks.

Details
- Location: `crates/gc_core/src/path.rs`.

Proposed Improvements
- Document that `reset_stats` does not clear the cache; consider adding `clear_cache` and/or `reset_all`.
- Add a test for eviction behavior at small capacities.

Acceptance Criteria
- Docs and tests updated; optional new API added.
