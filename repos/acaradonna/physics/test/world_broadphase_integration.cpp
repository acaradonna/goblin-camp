#include "ape/ape.h"
#include <cassert>

int main(){
    ape::World w;
    // Two bodies close enough to overlap given r=0.5
    ape::RigidBodyDesc a{}; a.position = {0,0,0};
    ape::RigidBodyDesc b{}; b.position = {0.6f,0,0};
    w.createRigidBody(a);
    w.createRigidBody(b);
    w.step(0.0f); // compute AABBs and pairs
    assert(w.debug_broadphasePairCount() == 1);

    // Move them apart by velocity and step
    ape::RigidBodyDesc c{}; c.position = {10,0,0};
    w.createRigidBody(c);
    w.step(0.0f);
    // Still only first pair overlaps
    assert(w.debug_broadphasePairCount() == 1);
    return 0;
}
