# Building APE for WebAssembly

This project can be compiled to WebAssembly to power the browser demos.

Quick notes

- Toolchain: Emscripten SDK (emsdk) latest
- Target: C ABI (`ape_c`) built with `-s MODULARIZE=1 -s EXPORT_NAME=Module`
- Output: `web/js/ape_wasm.js` and `web/js/ape_wasm.wasm`

Minimal build sketch (to adapt into CMake):

- Activate emsdk and use `em++`.
- Compile sources of `ape_core` and `cbindings/ape_c.cpp` into a single module export.
- Export required C functions: `ape_world_create`, `ape_world_destroy`, `ape_world_step`, `ape_world_create_rigidbody_p`, `ape_world_get_position_out`, `ape_world_set_gravity_p`, `ape_version_*`.

Example command (illustrative):

```bash
em++ \
  -O3 -s MODULARIZE=1 -s EXPORT_NAME=Module \
  -s ENVIRONMENT=web \
  -s EXPORTED_FUNCTIONS='[_ape_world_create,_ape_world_destroy,_ape_world_step,_ape_world_create_rigidbody_p,_ape_world_get_position_out,_ape_world_set_gravity_p,_ape_version_major,_ape_version_minor,_ape_version_patch]' \
  -s EXPORTED_RUNTIME_METHODS='[cwrap,ccall,HEAPF32,_malloc,_free]' \
  -Iinclude \
  src/ape.cpp src/foundation/job.cpp src/collision/broadphase.cpp cbindings/ape_c.cpp \
  -o web/js/ape_wasm.js
```

Then open `web/index.html`. The loader will auto‑use WASM if present, otherwise fall back to the JS shim.

> Ensure the memory layout of `ape_rigidbody_desc` matches the JS code (two vec3 then float mass).
