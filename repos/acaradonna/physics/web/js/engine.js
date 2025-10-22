// Minimal engine loader for demos. In the future, load WASM build of APE.
// For now, provide a deterministic JS fallback with the same surface we need.

class JsWorld {
  constructor(){
    this.bodies = [];
    this.g = {x:0, y:-9.80665, z:0};
  }
  createRigidBody(desc){
    const d = { position: {...(desc.position||{x:0,y:0,z:0})}, velocity: {...(desc.velocity||{x:0,y:0,z:0})}, mass: desc.mass ?? 1 };
    const id = this.bodies.length;
    this.bodies.push(d);
    return id;
  }
  setGravity(g){ this.g = {...g}; }
  getPosition(id){ const b = this.bodies[id]; return b ? {...b.position} : {x:0,y:0,z:0}; }
  step(dt){
    for (const b of this.bodies){
      b.velocity.x += this.g.x * dt; b.velocity.y += this.g.y * dt; b.velocity.z += this.g.z * dt;
      b.position.x += b.velocity.x * dt; b.position.y += b.velocity.y * dt; b.position.z += b.velocity.z * dt;
    }
  }
}

async function tryLoadWasm() {
  // Convention: ape_wasm.js (Emscripten MODULARIZE=1) + ape_wasm.wasm in the same folder
  try {
    const modFactory = (await import('./ape_wasm.js')).default;
    const Module = await modFactory({ locateFile: (p) => p });
    // Minimal wrapper around C ABI via ccall/cwrap
    const cwrap = Module.cwrap.bind(Module);
    const _world_create = cwrap('ape_world_create', 'number', []);
    const _world_destroy = cwrap('ape_world_destroy', null, ['number']);
    const _world_step = cwrap('ape_world_step', null, ['number','number']);
    const _world_create_rb_p = cwrap('ape_world_create_rigidbody_p', 'number', ['number','number']);
    const _world_get_pos_out = cwrap('ape_world_get_position_out', null, ['number','number','number']);
    const _world_set_g_p = cwrap('ape_world_set_gravity_p', null, ['number','number']);
    const sizeVec3 = 12; // 3 * float32
    const sizeDesc = 28; // two vec3 + float
    class WasmWorld {
      constructor(){ this.ptr = _world_create(); }
      createRigidBody(desc){
        const ptr = Module._malloc(sizeDesc);
        const f32 = new Float32Array(Module.HEAPF32.buffer, ptr, sizeDesc/4);
        // position (0..2), velocity (3..5), mass (6 as float32 after padding); layout matches ape_rigidbody_desc
        f32[0]=desc.position?.x||0; f32[1]=desc.position?.y||0; f32[2]=desc.position?.z||0;
        f32[3]=desc.velocity?.x||0; f32[4]=desc.velocity?.y||0; f32[5]=desc.velocity?.z||0;
        // Mass placed at byte offset 24 => index 6
        f32[6]=desc.mass??1;
        const id = _world_create_rb_p(this.ptr, ptr);
        Module._free(ptr);
        return id>>>0;
      }
      setGravity(g){
        const ptr = Module._malloc(sizeVec3);
        const f32 = new Float32Array(Module.HEAPF32.buffer, ptr, 3);
        f32[0]=g.x||0; f32[1]=g.y||0; f32[2]=g.z||0;
        _world_set_g_p(this.ptr, ptr);
        Module._free(ptr);
      }
      getPosition(id){
        const ptr = Module._malloc(sizeVec3);
        _world_get_pos_out(this.ptr, id>>>0, ptr);
        const f32 = new Float32Array(Module.HEAPF32.buffer, ptr, 3);
        const out = {x:f32[0], y:f32[1], z:f32[2]};
        Module._free(ptr);
        return out;
      }
      step(dt){ _world_step(this.ptr, dt); }
      destroy(){ if(this.ptr){ _world_destroy(this.ptr); this.ptr=0; } }
    }
    return {
      World: WasmWorld,
      version: { major: Module.ccall('ape_version_major','number',[],[]), minor: Module.ccall('ape_version_minor','number',[],[]), patch: Module.ccall('ape_version_patch','number',[],[]) }
    };
  } catch (e) {
    return null;
  }
}

export async function createEngine(){
  const wasm = await tryLoadWasm();
  if (wasm) {
    const status = document.getElementById('engine-status');
    if (status) status.textContent = `Engine: WASM v${wasm.version.major}.${wasm.version.minor}.${wasm.version.patch}`;
    return wasm;
  }
  const engine = { World: JsWorld, version: {major:0, minor:0, patch:1} };
  const status = document.getElementById('engine-status');
  if (status) status.textContent = `Engine: JS fallback v${engine.version.major}.${engine.version.minor}.${engine.version.patch}`;
  return engine;
}
