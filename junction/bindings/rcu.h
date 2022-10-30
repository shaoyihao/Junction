// rcu.h - support for Read Copy Update (RCU) synchronization

#pragma once

extern "C" {
#include <runtime/rcu.h>
}

#include <atomic>
#include <memory>

#include "junction/bindings/sync.h"
#include "junction/bindings/thread.h"

namespace junction::rt {

// Usage Example:
// std::unique_ptr<foo> p;
// rt::RCUPtr<foo> rcu_p;
//
// Reader:
// {
//   rt::RCURead l;
//   rt::RCUReadGuard g(&l);
//   auto a = rcu_p.get();
//   // read the data in a
// }
//
// Writer:
// std::unique_ptr<foo> tmp = std::make_unique<foo>(*p); // make a copy
// // modify tmp as desired (update)
// rcu_p.set(tmp.get());
// rt::RCUFree(std::move(p));
// p = std::move(tmp);

// An RCU Reader Lock.
using RCURead = rt::Preempt;
// A scoped handler for the above RCU reader Lock.
using RCUReadGuard = rt::PreemptGuard;

// RCUPtr is a raw pointer to an object that readers can safely dereference
// without locking, even though the pointer could be concurrently modified by
// the writer.
//
// RCUPtr does not manage the lifetime of the object; this is the
// responsibility of the writer.
//
// It is also fine to use an atomic pointer directly (rather than an RCUPtr) if
// a more customized use of RCU is required.
template <typename T>
class RCUPtr {
 public:
  explicit RCUPtr(T *ptr) noexcept : ptr_(ptr) {}
  ~RCUPtr() = default;

  // disable copy and move
  RCUPtr(RCUPtr &&) = delete;
  RCUPtr &operator=(RCUPtr &&) = delete;
  RCUPtr(const RCUPtr &) = delete;
  RCUPtr &operator=(const RCUPtr &) = delete;

  // Set the pointer (as a writer).
  void set(T *ptr) { ptr_.store(ptr, std::memory_order_release); }

  // Get the pointer (as a reader). Preemption must be disabled (normally via
  // RCUReadGuard), and the pointer cannot be held past when preemption is
  // re-enabled.
  [[nodiscard]] const T *get() const {
    assert_preempt_disabled();
    return ptr_.load(std::memory_order_consume);
  }

  // Get the pointer (as a writer).
  [[nodiscard]] T *get_locked() const {
    return ptr_.load(std::memory_order_relaxed);
  }

 private:
  std::atomic<T *> ptr_;
};

// Blocks the calling thread and waits until a quiescent period has elapsed.
inline void RCUSynchronize() { synchronize_rcu(); }

// Frees a raw pointer after a quiescent period.
// Does not block. Allocates memory (for a thread). Can take a custom deleter.
template <typename T, typename D = std::default_delete<T>>
inline void RCUFree(T *p, D d = {}) {
  // TODO(amb): eliminate thread spawning for better performance.
  rt::Spawn([p, d = std::move(d)]() {
    RCUSynchronize();
    d(p);
  });
}

// Takes ownership of a unique pointer and frees it after a quiescent period.
// Does not block. Allocates memory (for a thread).
template <typename T>
inline void RCUFree(std::unique_ptr<T> ptr) {
  // TODO(amb): eliminate thread spawning for better performance.
  rt::Spawn([p = std::move(ptr)]() mutable { RCUSynchronize(); });
}

}  // namespace junction::rt
