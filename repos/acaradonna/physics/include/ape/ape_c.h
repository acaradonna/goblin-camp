#pragma once
#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

/**
 * @file ape/ape_c.h
 * @brief Stable C ABI for the APE physics prototype.
 *
 * This header is designed to be safe to call from C and other languages via
 * FFI. All objects are opaque, and value types use trivially copyable structs.
 * Where practical, pointer-based variants are provided to avoid struct-by-value
 * crossings that can be problematic for some ABIs (notably WASM and certain
 * foreign language interop layers).
 */

/** @brief Trivial 3D vector value type. */
typedef struct ape_vec3 { float x, y, z; } ape_vec3;

/**
 * @brief Rigid body construction parameters.
 *
 * Defaults are not encoded in the C ABI; callers should fill all fields.
 */
typedef struct ape_rigidbody_desc {
    ape_vec3 position;  /**< Initial world-space position */
    ape_vec3 velocity;  /**< Initial linear velocity */
    float mass;         /**< Mass in kilograms (must be positive) */
} ape_rigidbody_desc;

/** @brief Opaque world object; use the functions below to manage it. */
typedef struct ape_world ape_world; // opaque

/** @name Versioning
 *  Engine semantic version split into major.minor.patch.
 *  @{ */
uint32_t ape_version_major(void);
uint32_t ape_version_minor(void);
uint32_t ape_version_patch(void);
/** @} */

/** @name World lifetime
 *  Create and destroy a simulation world.
 *  @{ */
ape_world* ape_world_create(void);
void ape_world_destroy(ape_world* w);
/** @} */

/** @name Simulation API
 *  Core operations: create a body, step the world, query state.
 *  @{ */
uint32_t ape_world_create_rigidbody(ape_world* w, ape_rigidbody_desc desc);
void ape_world_step(ape_world* w, float dt);
ape_vec3 ape_world_get_position(const ape_world* w, uint32_t id);
/** @} */

/** @name Global parameters
 *  Get/set world gravity.
 *  @{ */
void ape_world_set_gravity(ape_world* w, ape_vec3 g);
ape_vec3 ape_world_get_gravity(const ape_world* w);
/** @} */

/** @name Pointer-based variants
 *  FFI-friendly overloads that avoid passing structs by value across the ABI.
 *  These are recommended for WASM and some language bindings.
 *  @{ */
uint32_t ape_world_create_rigidbody_p(ape_world* w, const ape_rigidbody_desc* desc);
void ape_world_get_position_out(const ape_world* w, uint32_t id, ape_vec3* out);
void ape_world_set_gravity_p(ape_world* w, const ape_vec3* g);
void ape_world_get_gravity_out(const ape_world* w, ape_vec3* out);
/** @} */

#ifdef __cplusplus
}
#endif
