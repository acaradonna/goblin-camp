#include "ape/broadphase.h"

namespace ape {

// See header for contract and semantics.
void broadphase_naive(const AABB* boxes, std::size_t count, std::vector<Pair>& out) {
    out.clear();
    if (!boxes || count < 2) return;

    // Reserve a heuristic capacity to reduce reallocations. In the worst case
    // (all boxes overlap), the number of pairs is count*(count-1)/2; we avoid
    // allocating that upfront to keep memory usage reasonable for tests.
    out.reserve(count);

    for (std::size_t i = 0; i + 1 < count; ++i) {
        for (std::size_t j = i + 1; j < count; ++j) {
            if (aabb_overlaps(boxes[i], boxes[j])) {
                out.push_back(Pair{static_cast<std::uint32_t>(i), static_cast<std::uint32_t>(j)});
            }
        }
    }
}

} // namespace ape
