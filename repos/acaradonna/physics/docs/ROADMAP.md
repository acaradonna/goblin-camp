# Roadmap

Phase 0: Incubation (this repo)

- [x] Minimal C++ core skeleton + C ABI
- [x] CMake build + smoke test
- [x] Job system foundation (threads, queues)
- [x] SoA body storage + handles
- [ ] Broadphase (AABB, SAP)
- [ ] Narrowphase (GJK/EPA baseline)
- [ ] Iterative solver (PGS) with warm-start
- [ ] Deterministic step guarantees + tests
- [ ] Profiling + logging infra
- [ ] Fuzz tests, sanitizer CI

Phase 1: Rigid body MVP

- [ ] Basic shapes (sphere, box, capsule)
- [ ] Continuous collision detection (swept volumes)
- [ ] Joints (distance, hinge, fixed)
- [ ] Sleep/awake, island management
- [ ] Python and C# bindings alpha

Phase 2: Advanced features

- [ ] Mesh collisions, convex decomposition
- [ ] XPBD constraints, cloth and soft bodies
- [ ] Fluids (APIC/FLIP) experiments
- [ ] GPU broadphase experiments
- [ ] WebAssembly build with threads + SIMD

Phase 3: Production hardening

- [ ] Deterministic replay/rollback API
- [ ] Extensive benchmarks suite
- [ ] Documentation site + examples gallery
