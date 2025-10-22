#include "ape/ape.h"
#include <iostream>

// Minimal example: one body under gravity, print final y position.
int main() {
    ape::World w; auto id = w.createRigidBody({{0, 1, 0}, {0, 0, 0}, 1});
    for (int i = 0; i < 120; i++) w.step(1.0f / 60.0f);
    auto p = w.getPosition(id);
    std::cout << p.y << "\n";
    return 0;
}
