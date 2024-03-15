extern "C" {
#include <sys/ioctl.h>
#include <sys/stat.h>
}

#include <syscall.h>

#include <memory>
#include <string>

#include "junction/base/error.h"
#include "junction/filesystem/linuxfile.h"
#include "junction/kernel/ksys.h"
#include "junction/syscall/strace.h"

namespace junction {

LinuxFile::LinuxFile(Token, int fd, int flags, mode_t mode,
                     std::string &&pathname) noexcept
    : File(FileType::kNormal, flags, mode, std::move(pathname)), fd_(fd) {}

LinuxFile::LinuxFile(Token, int fd, int flags, mode_t mode,
                     std::string_view pathname) noexcept
    : File(FileType::kNormal, flags, mode, pathname), fd_(fd) {}

LinuxFile::~LinuxFile() { ksys_close(fd_); }

std::shared_ptr<LinuxFile> LinuxFile::Open(std::string_view pathname, int flags,
                                           mode_t mode) {
  int fd = ksys_open(pathname.data(), flags, mode);
  if (fd < 0) return nullptr;
  return std::make_shared<LinuxFile>(Token{}, fd, flags, mode, pathname);
}

void TouchPages(std::span<std::byte> buf) {
  uintptr_t start = PageAlignDown(reinterpret_cast<uintptr_t>(buf.data()));
  char *pg = reinterpret_cast<char *>(start);
  char *end = reinterpret_cast<char *>(buf.data() + buf.size_bytes());
  [[maybe_unused]] volatile char c;
  for (; pg < end; pg += kPageSize) c = access_once(*pg);
}

Status<size_t> LinuxFile::Read(std::span<std::byte> buf, off_t *off) {
  // If we are tracing page accesses, we need to fault the pages in before
  // passing them to the kernel since the page fault handler won't be invoked
  // by the kernel in this case.
  // TODO(jf): consider gating this with a compile flag.
  if (IsJunctionThread() && unlikely(myproc().get_mem_map().TraceEnabled()))
    TouchPages(buf);
  ssize_t ret = ksys_pread(fd_, buf.data(), buf.size_bytes(), *off);
  if (ret < 0) {
    if (ret == -EINTR) return MakeError(ERESTARTSYS);
    return MakeError(-ret);
  }
  *off += ret;
  return ret;
}

Status<size_t> LinuxFile::Write(std::span<const std::byte> buf, off_t *off) {
  ssize_t ret = ksys_pwrite(fd_, buf.data(), buf.size_bytes(), *off);
  if (ret < 0) return MakeError(-ret);
  *off += ret;
  return ret;
}

Status<off_t> LinuxFile::Seek(off_t off, SeekFrom origin) {
  switch (origin) {
    case SeekFrom::kStart:
      return off;
    case SeekFrom::kCurrent:
      return get_off_ref() + off;
    case SeekFrom::kEnd:
      return get_size() + off;
    default:
      return MakeError(EINVAL);
  }
}

Status<void *> LinuxFile::MMap(void *addr, size_t length, int prot, int flags,
                               off_t off) {
  assert(!(flags & MAP_ANONYMOUS));
  intptr_t ret = ksys_mmap(addr, length, prot, flags, fd_, off);
  if (ret < 0) return MakeError(-ret);
  return reinterpret_cast<void *>(ret);
}

Status<void> LinuxFile::Stat(struct stat *statbuf, int flags) {
  char empty_path[1] = {'\0'};
  assert(flags & AT_EMPTY_PATH);
  int ret = ksys_newfstatat(fd_, empty_path /* pathname */, statbuf, flags);
  if (ret) return MakeError(-ret);
  return {};
}

Status<int> LinuxFile::GetDents(void *dirp, unsigned int count) {
  int ret = ksys_getdents(fd_, dirp, count);
  if (ret < 0) return MakeError(-ret);
  return ret;
}

Status<int> LinuxFile::GetDents64(void *dirp, unsigned int count) {
  int ret = ksys_getdents64(fd_, dirp, count);
  if (ret < 0) return MakeError(-ret);
  return ret;
}

Status<void> LinuxFile::Ioctl(unsigned long request,
                              [[maybe_unused]] char *argp) {
  if (request == FIOCLEX) {
    // Equivalent to: fcntl(fd, F_SETFD, FD_CLOEXEC)
    set_flags(get_flags() | FD_CLOEXEC);
    return {};
  }
  return MakeError(EINVAL);
}

}  // namespace junction
