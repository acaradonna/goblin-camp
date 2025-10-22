#include "ape/ape.h"
#include <cassert>
#include <vector>
#include <cstdint>

static float run_once(uint32_t seed) {
    ape::World w;
    // deterministic setup from seed
    ape::RigidBodyDesc d{};
    d.mass = 1.0f;
    for (int i=0;i<10;i++) {
        d.position = {0.1f * (seed + i), 2.0f + i, 0.0f};
        d.velocity = {0.0f, 0.0f, 0.0f};
        w.createRigidBody(d);
    }
    for (int i=0;i<600;i++) w.step(1.0f/120.0f);
    auto p = w.getPosition(0);
    return p.y;
}

int main(){
    float a = run_once(42);
    float b = run_once(42);
    assert(a == b);
    return 0;
}
