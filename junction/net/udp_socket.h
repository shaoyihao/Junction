// udp_socket.h - UDP socket
#pragma once

#include <atomic>
#include <memory>
#include <span>

#include "junction/base/error.h"
#include "junction/bindings/net.h"
#include "junction/net/socket.h"

namespace junction {

class UDPSocket : public Socket {
 public:
  UDPSocket() noexcept : Socket() {}
  UDPSocket(rt::UDPConn conn) noexcept : Socket(), conn_(std::move(conn)) {}
  ~UDPSocket() override = default;

  Status<std::shared_ptr<Socket>> Bind(netaddr addr) override {
    if (unlikely(conn_.is_valid())) return MakeError(EINVAL);
    Status<rt::UDPConn> ret = rt::UDPConn::Listen(addr);
    if (unlikely(!ret)) return MakeError(ret);
    return std::make_shared<UDPSocket>(std::move(*ret));
  }

  Status<std::shared_ptr<Socket>> Connect(netaddr addr) override {
    netaddr laddr;
    if (conn_.is_valid()) {
      netaddr remote = conn_.RemoteAddr();
      if (unlikely(remote.ip || remote.port)) return MakeError(EINVAL);
      laddr = conn_.LocalAddr();
    } else {
      laddr = {0, 0};
    }
    Status<rt::UDPConn> ret = rt::UDPConn::Dial(laddr, addr);
    if (unlikely(!ret)) return MakeError(ret);

    if (IsPollSourceSetup() && conn_.is_valid())
      conn_.InstallPollSource(nullptr, nullptr, 0);

    return std::make_shared<UDPSocket>(std::move(*ret));
  }

  Status<size_t> Read(std::span<std::byte> buf,
                      [[maybe_unused]] off_t *off) override {
    if (unlikely(!conn_.is_valid())) return MakeError(EINVAL);
    return conn_.Read(buf);
  }

  Status<size_t> Write(std::span<const std::byte> buf,
                       [[maybe_unused]] off_t *off) override {
    if (unlikely(!conn_.is_valid())) return MakeError(EDESTADDRREQ);
    return conn_.Write(buf);
  }

  Status<size_t> ReadFrom(std::span<std::byte> buf, netaddr *raddr) override {
    if (unlikely(!conn_.is_valid())) return MakeError(EINVAL);
    return conn_.ReadFrom(buf, raddr);
  }

  Status<size_t> WriteTo(std::span<const std::byte> buf,
                         const netaddr *raddr) override {
    if (!conn_.is_valid()) {
      Status<rt::UDPConn> ret = rt::UDPConn::Listen({0, 0});
      if (unlikely(!ret)) return MakeError(ret);
      conn_ = std::move(*ret);
      if (IsPollSourceSetup()) SetupPollSource();
    }
    return conn_.WriteTo(buf, raddr);
  }

  Status<void> Shutdown([[maybe_unused]] int how) override {
    if (unlikely(!conn_.is_valid())) return MakeError(EINVAL);
    bool shutdown = false;
    if (is_shut_.compare_exchange_strong(shutdown, true)) conn_.Shutdown();
    return {};
  }

  Status<netaddr> RemoteAddr() override {
    if (unlikely(!conn_.is_valid())) return MakeError(EINVAL);
    return conn_.RemoteAddr();
  }

  Status<netaddr> LocalAddr() override {
    if (unlikely(!conn_.is_valid())) return MakeError(EINVAL);
    return conn_.LocalAddr();
  }

 private:
  virtual void SetupPollSource() override {
    if (!conn_.is_valid()) return;
    conn_.InstallPollSource(PollSourceSet, PollSourceClear,
                            reinterpret_cast<unsigned long>(&poll_));
  }

  // This may or may not be valid. If UDPSocket is created without a rt::UDPConn
  // then this will be invalid until WriteTo is called.
  // Otherwise, UDPSocket will be created with a valid rt::UDPConn which will be
  // stored here (as a result of Bind/Connect calls).
  rt::UDPConn conn_;
  std::atomic_bool is_shut_{false};
};

}  // namespace junction