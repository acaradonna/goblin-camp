#include "ape/ape.h"
#include <cassert>

// Validate that out-of-range or mismatched-generation handles are ignored.
int main() {
    ape::World w;
    ape::RigidBodyDesc d{}; d.position = {1, 2, 3};
    std::uint32_t h = w.createRigidBody(d);

    // Valid handle should return the set position
    auto p = w.getPosition(h);
    assert(p.x == 1 && p.y == 2 && p.z == 3);

    // Corrupt index
    std::uint32_t bad_idx = (h & 0xFFFF0000u) | 0x00FFu; // likely different index
    auto p2 = w.getPosition(bad_idx);
    assert(p2.x == 0 && p2.y == 0 && p2.z == 0);

    // Corrupt generation
    std::uint32_t bad_gen = ((h + (1u<<16)) & 0xFFFF0000u) | (h & 0xFFFFu);
    auto p3 = w.getPosition(bad_gen);
    assert(p3.x == 0 && p3.y == 0 && p3.z == 0);

    return 0;
}
