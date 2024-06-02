
#include "junction/fs/linuxfs/linuxfs.h"

#include "junction/fs/linuxfs/linuxfile.h"

namespace junction::linuxfs {

// A file descriptor pointing to the root of the linux filesystem.
KernelFile linux_root_fd;
// TODO(jfried): integrate with chroot.
// The absolute path for the root mount of the linux filesystem.
std::string linux_root = "/";
struct statfs linux_statfs;
std::set<dev_t> allowed_devs;

Status<std::shared_ptr<File>> LinuxInode::Open(uint32_t flags, mode_t mode) {
  Status<KernelFile> f = linux_root_fd.OpenAt(get_path(), flags, mode);
  if (!f) return MakeError(f);
  return std::make_shared<LinuxFile>(std::move(*f), flags, mode, get_path(),
                                     shared_from_base<LinuxInode>());
}

Status<std::shared_ptr<IDir>> MountLinux(std::string_view path) {
  struct stat buf;
  int ret = ksys_newfstatat(AT_FDCWD, path.data(), &buf, AT_EMPTY_PATH);
  if (ret) return MakeError(-ret);
  allowed_devs.insert(buf.st_dev);
  auto ino = std::make_shared<LinuxIDir>(buf, std::string(path), std::string{},
                                         std::shared_ptr<IDir>{});
  return std::move(ino);
}

// Setup the linuxfs. Must be called before privileges are dropped.
Status<std::shared_ptr<IDir>> InitLinuxRoot() {
  Status<KernelFile> f =
      KernelFile::Open(linux_root, O_RDONLY | O_PATH, S_IRUSR);
  if (unlikely(!f)) return MakeError(f);
  linux_root_fd = std::move(*f);
  int ret = statfs(linux_root.data(), &linux_statfs);
  if (ret) return MakeError(-ret);
  return MountLinux("/");
}

}  // namespace junction::linuxfs
