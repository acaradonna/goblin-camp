# Performance notes (early draft)

- Use SoA for bodies and contacts to improve memory bandwidth.
- Hot loops: compute with float32 and SIMD (SSE/NEON) where safe.
- Task graph per step to maximize parallelism; avoid false sharing with padding.
- Broadphase: incremental SAP + BVH hybrid; cache coherence across frames.
- Narrowphase: vectorized GJK support mapping; batch pairs.
- Solver: warm-start, contact caching, split impulses; clamping for stability.
- Determinism: fixed traversal order; stable sorts; explicit seeds.
- Profiling: zone macros; per-subsystem timers + counters.
