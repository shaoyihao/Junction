// poll.h - support for epoll(), poll(), select() etc.

#pragma once

extern "C" {
#include <poll.h>
#include <sys/epoll.h>
}

#include <functional>
#include <vector>

#include "junction/base/arch.h"
#include "junction/base/finally.h"
#include "junction/base/intrusive_list.h"
#include "junction/bindings/sync.h"

namespace junction {

// ensure event types can be used interchangably for epoll() and poll()
static_assert(EPOLLIN == POLLIN);
static_assert(EPOLLOUT == POLLOUT);
static_assert(EPOLLERR == POLLERR);
static_assert(EPOLLRDHUP == POLLRDHUP);
static_assert(EPOLLHUP == POLLHUP);
static_assert(EPOLLPRI == POLLPRI);

// event types
constexpr unsigned int kPollIn = EPOLLIN;        // available for read()
constexpr unsigned int kPollOut = EPOLLOUT;      // available for write()
constexpr unsigned int kPollErr = EPOLLERR;      // error condition
constexpr unsigned int kPollRDHUp = EPOLLRDHUP;  // reader closed
constexpr unsigned int kPollHUp = EPOLLHUP;      // writer closed
constexpr unsigned int kPollPrio = EPOLLPRI;     // priority event (TCP URG)

namespace detail {
class EPollFile;
}
class PollSource;

// PollObserver provides a notification for each event from a PollSource.
class PollObserver {
 public:
  friend class PollSource;

  PollObserver() noexcept = default;
  virtual ~PollObserver() { assert(!is_attached()); }

  PollObserver(const PollObserver &o) noexcept : src_(nullptr) {}
  PollObserver &operator=(const PollObserver &o) {
    src_ = nullptr;
    return *this;
  }
  PollObserver(PollObserver &&o) noexcept : src_(nullptr) {
    assert(!is_attached());
  }
  PollObserver &operator=(PollObserver &&o) {
    assert(!is_attached());
    src_ = nullptr;
    return *this;
  }

  bool is_attached() const { return src_ != nullptr; }

  // Detach this observer from its PollSource.
  void Detach();

 private:
  // Notify informs the observer that the events have changed.
  virtual void Notify(unsigned int event_mask) = 0;

  IntrusiveListNode node_;
  PollSource *src_{nullptr};
};

// PollSource generates events and delivers them to each PollObserver.
class alignas(kCacheLineSize) PollSource {
 public:
  friend detail::EPollFile;

  PollSource() noexcept = default;
  ~PollSource() { assert(observers_.empty()); }

  unsigned int get_events() { return rt::read_once(event_mask_); }

  // Sets a mask of events and notifies (must be synchronized by caller).
  void Set(unsigned int event_mask);

  // Clears a mask of events and notifies (must be synchronized by caller).
  void Clear(unsigned int event_mask);

  // Registers an observer to receive events from this source.
  void Attach(PollObserver &o);

  // Unregisters an observer from this source.
  void Detach(PollObserver &o);

 private:
  void Notify();

  rt::Spin lock_;
  unsigned int event_mask_{0};
  IntrusiveList<PollObserver, &PollObserver::node_> observers_;
  IntrusiveList<PollObserver, &PollObserver::node_> epoll_observers_;
};

inline void PollSource::Set(unsigned int event_mask) {
  unsigned int cur = event_mask_;
  if ((cur & event_mask) == event_mask) return;
  event_mask_ = cur | event_mask;
  Notify();
}

inline void PollSource::Clear(unsigned int event_mask) {
  unsigned int cur = event_mask_;
  if ((cur & event_mask) == 0) return;
  event_mask_ = cur & ~event_mask;
  Notify();
}

inline void PollSource::Attach(PollObserver &o) {
  assert(o.src_ == nullptr);
  o.src_ = this;
  rt::SpinGuard g(lock_);
  observers_.push_back(o);
  unsigned int events = get_events();
  if (events != 0) o.Notify(events);
}

inline void PollSource::Detach(PollObserver &o) {
  assert(o.src_ != nullptr);
  o.src_ = nullptr;
  rt::SpinGuard g(lock_);
  observers_.erase(decltype(observers_)::s_iterator_to(o));
}

inline void PollObserver::Detach() { src_->Detach(*this); }

// Poller is a simple wrapper that runs a lambda when poll events trigger.
class Poller : public PollObserver {
 public:
  Poller() noexcept = default;
  explicit Poller(std::function<void(unsigned int)> func) noexcept
      : func_(func) {}
  ~Poller() = default;

  Poller(const Poller &p) noexcept : func_(p.func_) {}
  Poller &operator=(const Poller &p) {
    func_ = p.func_;
    return *this;
  }
  Poller(Poller &&p) noexcept : func_(std::move(p.func_)) {}
  Poller &operator=(Poller &&p) {
    func_ = std::move(p.func_);
    return *this;
  }

 private:
  void Notify(unsigned int event_mask) override { func_(event_mask); }

  std::function<void(unsigned int)> func_;
};

}  // namespace junction
