#pragma once
#include <functional>
#include <vector>
#include <thread>
#include <atomic>
#include <condition_variable>
#include <queue>

/**
 * @file ape/job.h
 * @brief Tiny thread-pool style job system used by the engine.
 *
 * This is a straightforward, minimal job system that provides background
 * worker threads, a FIFO queue of std::function jobs, and a way to wait until
 * the queue drains. It is not designed for low-latency or high-throughput
 * workloads; it exists to enable early experiments and parallelize trivial
 * tasks. API and implementation are intentionally kept simple.
 */

namespace ape {

class JobSystem {
public:
    /**
     * @brief Construct a pool with `workers` threads.
     * If `workers` is zero, a single worker is created.
     */
    explicit JobSystem(unsigned workers = std::thread::hardware_concurrency());
    /** @brief Join all workers and destroy the pool. */
    ~JobSystem();

    /** @brief Enqueue a job for execution by a worker thread. */
    void enqueue(std::function<void()> job);
    /** @brief Block until the queue is empty and all workers are idle. */
    void wait_idle();

private:
    void worker_loop();

    std::vector<std::thread> threads_;      ///< Worker threads
    std::mutex mtx_;                         ///< Protects queue and condition
    std::condition_variable cv_;             ///< Signals queue or shutdown
    std::queue<std::function<void()>> q_;    ///< FIFO of pending jobs
    std::atomic<bool> quit_{false};          ///< Cooperative shutdown flag
    std::atomic<int> active_{0};             ///< Number of jobs currently running
};

} // namespace ape
