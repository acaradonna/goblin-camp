#pragma once

/**
 * @file ape/broadphase.h
 * @brief Minimal broadphase scaffolding for early engine validation.
 *
 * Provides an axis-aligned bounding box (AABB) primitive, a simple overlap
 * predicate, and a naive O(n^2) pair finder. This is intentionally simplistic
 * and will be replaced by a more scalable approach (e.g., sweep-and-prune or a
 * BVH). The goal is to validate the end-to-end simulation pipeline and provide
 * testability hooks.
 */

#include <cstddef>
#include <cstdint>
#include <vector>

namespace ape {

/** @brief Axis-aligned bounding box in world space. */
struct AABB {
    float min_x, min_y, min_z; ///< Lower corner
    float max_x, max_y, max_z; ///< Upper corner
};

/**
 * @brief Inclusive AABB overlap test on all three axes.
 *
 * Two boxes are considered overlapping if their projections intersect on X, Y,
 * and Z. The comparison is inclusive so that touching faces count as overlap.
 */
inline bool aabb_overlaps(const AABB& a, const AABB& b) {
    return (a.min_x <= b.max_x && a.max_x >= b.min_x) &&
           (a.min_y <= b.max_y && a.max_y >= b.min_y) &&
           (a.min_z <= b.max_z && a.max_z >= b.min_z);
}

/** @brief Pair of indices (a < b) into the input box array. */
struct Pair { std::uint32_t a, b; };

/**
 * @brief Naive all-pairs broadphase.
 *
 * Iterates i<j over the input array, emitting all overlapping pairs to `out`.
 * The output vector is cleared and may reallocate; callers may reserve in
 * advance to avoid churn. Indices refer to the original array positions.
 */
void broadphase_naive(const AABB* boxes, std::size_t count, std::vector<Pair>& out);

} // namespace ape
