#include "ape/ape.h"
#include <chrono>
#include <iostream>

// Micro-benchmark: create 10k bodies and step 600 frames at 120 Hz.
int main() {
    ape::World w; ape::RigidBodyDesc d{}; d.mass = 1.0f; d.position = {0, 10, 0};
    for (int i = 0; i < 10000; i++) w.createRigidBody(d);
    auto start = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < 600; i++) w.step(1.f / 120.f);
    auto end = std::chrono::high_resolution_clock::now();
    double ms = std::chrono::duration<double, std::milli>(end - start).count();
    std::cout << "frames=600 bodies=10000 time_ms=" << ms << "\n";
    return 0;
}
