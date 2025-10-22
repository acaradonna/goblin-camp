# Language bindings plan

Goals

- Keep C ABI tiny and stable. Prefer opaque handles and flat structs.
- Zero-copy where safe; explicit lifetimes.
- Avoid per-call allocation and locks.

Targets and notes

- Python: pybind11 or CPython C API with midspeed path; numpy views for SoA buffers.
- Node.js: N-API; expose minimal classes; use worker threads for stepping.
- C#: P/Invoke + source generators; Span\<T\> over pinned buffers when possible.
- Java: JNI thin layer; direct byte buffers for arrays.
- Rust: bindgen + safe wrapper; no panics across FFI.
- Web: Emscripten + WASM SIMD/threads; transferable memory for data snapshots.

Versioning & features

- ABI version in `ape_c.h`; query at runtime.
- Feature flags for optional subsystems.
