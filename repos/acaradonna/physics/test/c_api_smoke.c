#include "ape/ape_c.h"
#include <assert.h>
#include <stdio.h>

// Smoke test for the C ABI: create a body, run a few steps, and ensure it
// moved downward under gravity.
int main() {
    printf("ape %u.%u.%u\n", ape_version_major(), ape_version_minor(), ape_version_patch());
    ape_world* w = ape_world_create();
    ape_rigidbody_desc d;
    d.position = (ape_vec3){0, 5, 0};
    d.velocity = (ape_vec3){0, 0, 0};
    d.mass = 1.0f;
    unsigned id = ape_world_create_rigidbody(w, d);
    for (int i = 0; i < 60; i++) ape_world_step(w, 1.0f / 60.0f);
    ape_vec3 p = ape_world_get_position(w, id);
    printf("y=%f\n", p.y);
    assert(p.y < 5.0f);
    ape_world_destroy(w);
    return 0;
}
