#include "ape/ape_c.h"
#include "ape/ape.h"
#include <stdlib.h>

// C source variant of the C bindings for environments that prefer .c files.
// Kept minimal and consistent with the C++ implementation in ape_c.cpp.

struct ape_world { ape::World* w; };

extern "C" {

ape_world* ape_world_create(void) {
    ape_world* h = (ape_world*)malloc(sizeof(ape_world));
    h->w = new ape::World();
    return h;
}

void ape_world_destroy(ape_world* h) {
    if (!h) return;
    delete h->w;
    free(h);
}

uint32_t ape_world_create_rigidbody(ape_world* h, ape_rigidbody_desc desc) {
    ape::RigidBodyDesc d;
    d.position = {desc.position.x, desc.position.y, desc.position.z};
    d.velocity = {desc.velocity.x, desc.velocity.y, desc.velocity.z};
    d.mass = desc.mass;
    return h->w->createRigidBody(d);
}

void ape_world_step(ape_world* h, float dt) { h->w->step(dt); }

ape_vec3 ape_world_get_position(const ape_world* h, uint32_t id) {
    auto p = h->w->getPosition(id);
    return (ape_vec3){p.x, p.y, p.z};
}

} // extern "C"
