#pragma once

/**
 * @file ape/ape.h
 * @brief Public C++ API surface for the APE physics prototype.
 *
 * This header exposes a minimal, header-only C++ interface for constructing a
 * physics `World`, creating rigid bodies, stepping the simulation, and reading
 * state back. The stable, language-agnostic ABI is provided separately in
 * `ape_c.h`; prefer that for FFI and early bindings. The C++ API is intended to
 * remain small and easy to reason about while the engine evolves.
 */

#include <cstdint>
#include <array>

namespace ape {

/**
 * @brief 3D vector with single-precision components.
 *
 * This is intentionally a trivial POD used across the public API, so it can be
 * passed by value and copied cheaply. No operators are provided here to keep
 * the public surface minimal and predictable.
 */
struct Vec3 {
    float x, y, z;
};

/**
 * @brief Construction-time description of a rigid body.
 *
 * The prototype engine currently supports position, velocity, and mass. As the
 * engine grows, additional properties (shape, inertia, damping, etc.) will be
 * added here. Defaults are chosen to produce a stationary unit-mass body at the
 * origin when fields are not set.
 */
struct RigidBodyDesc {
    Vec3 position{0,0,0};
    Vec3 velocity{0,0,0};
    float mass{1.0f};
};

/**
 * @brief Simulation world owning bodies and global state.
 *
 * This class uses the PIMPL pattern to keep ABI stable while internals evolve.
 * Object identity is represented by 32-bit handles that remain valid across
 * frames until the corresponding body is destroyed (destruction not yet
 * implemented in the prototype). Methods are designed to be straightforward and
 * deterministic: given a fixed input sequence, results are reproducible.
 */
class World {
public:
    /** @brief Construct an empty world. */
    World();
    /** @brief Destroy the world and all contained bodies. */
    ~World();

    /**
     * @brief Create a new rigid body and return its opaque handle.
     *
     * The returned handle encodes an index and a generation counter to guard
     * against stale references. If the world runs out of indices, the function
     * returns `UINT32_MAX`.
     */
    std::uint32_t createRigidBody(const RigidBodyDesc& desc);

    /**
     * @brief Advance the simulation by a time step `dt` (seconds).
     *
     * The prototype integrates velocity under constant global gravity and then
     * integrates position using a simple (semi-implicit) Euler update. A naive
     * broadphase is executed to validate the pipeline; no narrowphase/contact
     * resolution is performed yet.
     */
    void step(float dt);

    /**
     * @brief Query the current position of a body by handle.
     *
     * If the handle is invalid (index out of range, slot not alive, or
     * generation mismatch) a zero vector is returned.
     */
    Vec3 getPosition(std::uint32_t id) const;

    /**
     * @brief Set global gravity vector (default {0, -9.80665, 0}).
     */
    void setGravity(const Vec3& g);
    /** @brief Read the current global gravity vector. */
    Vec3 getGravity() const;

    /**
     * @brief Number of broadphase candidate pairs found in the last step.
     *
     * This is a temporary debug/telemetry helper to sanity-check the pipeline
     * and performance. It will likely be replaced by a proper diagnostics API.
     */
    std::uint32_t debug_broadphasePairCount() const;

private:
    struct Impl;           ///< Opaque implementation details (PIMPL)
    Impl* impl;            ///< Owned pointer to implementation
};

} // namespace ape
