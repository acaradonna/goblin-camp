// C bindings for the APE C++ API. These functions adapt the C++ types to a
// stable C ABI, using an opaque `ape_world` and plain-old-data structs.
#include "ape/ape_c.h"
#include "ape/ape.h"
#include "ape/version.h"
#include <cstdlib>

// Opaque world wrapper for the C ABI. Allocated with malloc/free to avoid
// mixing allocation domains across language boundaries.
struct ape_world { ape::World* w; };

extern "C" {

uint32_t ape_version_major(void) { return APE_VERSION_MAJOR; }
uint32_t ape_version_minor(void) { return APE_VERSION_MINOR; }
uint32_t ape_version_patch(void) { return APE_VERSION_PATCH; }

ape_world* ape_world_create(void) {
    ape_world* h = static_cast<ape_world*>(std::malloc(sizeof(ape_world)));
    h->w = new ape::World();
    return h;
}

void ape_world_destroy(ape_world* h) {
    if (!h) return;
    delete h->w;
    std::free(h);
}

uint32_t ape_world_create_rigidbody(ape_world* h, ape_rigidbody_desc desc) {
    if (!h || !h->w) return UINT32_MAX;
    ape::RigidBodyDesc d;
    d.position = {desc.position.x, desc.position.y, desc.position.z};
    d.velocity = {desc.velocity.x, desc.velocity.y, desc.velocity.z};
    d.mass = desc.mass;
    return h->w->createRigidBody(d);
}

uint32_t ape_world_create_rigidbody_p(ape_world* h, const ape_rigidbody_desc* desc) {
    if (!h || !h->w || !desc) return UINT32_MAX;
    ape::RigidBodyDesc d;
    d.position = {desc->position.x, desc->position.y, desc->position.z};
    d.velocity = {desc->velocity.x, desc->velocity.y, desc->velocity.z};
    d.mass = desc->mass;
    return h->w->createRigidBody(d);
}

void ape_world_step(ape_world* h, float dt) { if (h && h->w) h->w->step(dt); }

ape_vec3 ape_world_get_position(const ape_world* h, uint32_t id) {
    if (!h || !h->w) return ape_vec3{0,0,0};
    auto p = h->w->getPosition(id);
    return ape_vec3{p.x, p.y, p.z};
}

void ape_world_get_position_out(const ape_world* h, uint32_t id, ape_vec3* out) {
    if (!out) return;
    if (!h || !h->w) { out->x = out->y = out->z = 0.0f; return; }
    auto p = h->w->getPosition(id);
    out->x = p.x; out->y = p.y; out->z = p.z;
}

void ape_world_set_gravity(ape_world* h, ape_vec3 g) {
    if (h && h->w) h->w->setGravity({g.x, g.y, g.z});
}

void ape_world_set_gravity_p(ape_world* h, const ape_vec3* g) {
    if (h && h->w && g) h->w->setGravity({g->x, g->y, g->z});
}

ape_vec3 ape_world_get_gravity(const ape_world* h) {
    if (!h || !h->w) return ape_vec3{0,0,0};
    auto g = h->w->getGravity();
    return ape_vec3{g.x, g.y, g.z};
}

void ape_world_get_gravity_out(const ape_world* h, ape_vec3* out) {
    if (!out) return;
    if (!h || !h->w) { out->x = out->y = out->z = 0.0f; return; }
    auto g = h->w->getGravity();
    out->x = g.x; out->y = g.y; out->z = g.z;
}

} // extern "C"
