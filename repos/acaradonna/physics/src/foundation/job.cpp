#include "ape/job.h"

using namespace std;

namespace ape {

JobSystem::JobSystem(unsigned workers) {
    if (workers == 0) workers = 1;
    threads_.reserve(workers);
    for (unsigned i = 0; i < workers; i++) {
        threads_.emplace_back([this] { worker_loop(); });
    }
}

JobSystem::~JobSystem() {
    // Signal shutdown and wake all workers so they can exit promptly.
    quit_.store(true, memory_order_relaxed);
    cv_.notify_all();
    for (auto& t : threads_) if (t.joinable()) t.join();
}

void JobSystem::enqueue(function<void()> job) {
    {
        lock_guard<mutex> lock(mtx_);
        q_.push(std::move(job));
    }
    cv_.notify_one();
}

void JobSystem::wait_idle() {
    unique_lock<mutex> lock(mtx_);
    cv_.wait(lock, [this] { return q_.empty() && active_.load(memory_order_relaxed) == 0; });
}

void JobSystem::worker_loop() {
    while (!quit_.load(memory_order_relaxed)) {
        function<void()> job;
        {
            unique_lock<mutex> lock(mtx_);
            cv_.wait(lock, [this] { return quit_.load(memory_order_relaxed) || !q_.empty(); });
            if (quit_.load(memory_order_relaxed)) return;
            job = std::move(q_.front());
            q_.pop();
            active_.fetch_add(1, memory_order_relaxed);
        }
        // Run outside the lock to avoid blocking producers and other workers.
        job();
        active_.fetch_sub(1, memory_order_relaxed);
        cv_.notify_all();
    }
}

} // namespace ape
