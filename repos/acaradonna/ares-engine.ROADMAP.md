# Roadmap

Last updated: 2025-10-22

## Milestone 0: Spikes
- [ ] Call `ape_c` from Rust; print version
- [ ] `wgpu` + `winit` triangle
- [ ] ECS loop scaffold

## Milestone 1: Foundations
- [ ] Game loop, ECS, structured logging
- [ ] Rendering MVP with instancing; free-fly camera
- [ ] Physics desktop integration; batch pose readback via `ape_sys`
- [ ] Sync physics→render; debug overlay/lines

## Milestone 2: Physics features
- [ ] Shapes (sphere/box/capsule), materials, events, queries
- [ ] Sleep/awake; determinism harness
- [ ] Simple constraints/joints pass-through from APE

## Milestone 3: Tooling
- [ ] Asset watcher, shader hot-reload, profiling, CI
- [ ] Editor/debug UI panel (egui) with toggles for overlays

## Milestone 4: Web path exploration
- [ ] WASM viability for APE with threads+SIMD; decision doc
- [ ] Minimal Web demo parity (camera + instanced draws)
