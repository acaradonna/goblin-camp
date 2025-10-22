# Acaradonna Physics Engine (APE)

A next‑generation, high‑performance, high‑fidelity, and highly‑extensible physics engine core designed for multi‑language, multi‑platform use.

Highlights

- Performance-first, data‑oriented C++20 core with SIMD and tasking
- Stable C ABI for fast bindings to Python, Node, C#, Java, and WebAssembly
- Deterministic fixed‑step simulation with robust collision & constraint pipeline
- Modular subsystems (rigid, soft, cloth, fluids) with plugin extension API

Status

- Incubating. This repo currently provides design docs and a minimal skeleton that builds and runs a smoke test.

Quick start (build)

- Requires: CMake ≥3.20, a C++20 compiler

See `docs/ARCHITECTURE.md` and `docs/ROADMAP.md` for the big picture and milestones. See `docs/BINDINGS.md` for language interop plans.

Deep planning

- `docs/ARCHITECTURE.md`: system design and invariants
- `docs/ROADMAP.md`: phased delivery and milestones
- `docs/BINDINGS.md`: interop strategy and ABI plan

Web playground & wiki

- A lightweight, browser‑deployable site with live demos is in `web/`.
- Serve locally (any static server) and open `web/index.html`.

Example (using Python 3 http.server):

```bash
cd web
python3 -m http.server 8080
# open http://localhost:8080
```

## WASM (browser) note

- The loader will try to import `web/js/ape_wasm.js` (Emscripten MODULARIZE build) alongside `ape_wasm.wasm`.
- If not present, it falls back to the JS engine shim.
