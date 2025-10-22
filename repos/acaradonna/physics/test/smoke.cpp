#include "ape/ape.h"
#include <cassert>
#include <iostream>

// Basic smoke test: create one body above the origin, step the world under
// gravity, and verify that its height decreased.
int main() {
    ape::World world;
    ape::RigidBodyDesc d{};
    d.position = {0, 10, 0};
    d.mass = 1.0f;
    auto id = world.createRigidBody(d);

    for (int i = 0; i < 60; i++) world.step(1.0f / 60.0f);
    auto p = world.getPosition(id);

    std::cout << "y=" << p.y << "\n";
    assert(p.y < 10.0f);
    return 0;
}
