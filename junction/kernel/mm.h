// mm.h - memory mapping support

#pragma once

#include <map>
#include <memory>
#include <vector>

#include "junction/base/arch.h"
#include "junction/base/error.h"
#include "junction/bindings/sync.h"
#include "junction/kernel/file.h"
#include "junction/kernel/ksys.h"

namespace junction {

constexpr bool AddressValid(void *addr, size_t len) {
  // TODO(amb): maybe check if address is not in the Linux Kernel (negative)?
  return len > 0 && IsPageAligned(reinterpret_cast<uintptr_t>(addr));
}

enum class VMType : int {
  kNormal,  // mapping contains regular anonymous memory
  kHeap,    // mapping is part of the heap (allocated with brk())
  kStack,   // mapping is used as a stack
  kFile,    // mapping is backed by a file
};

// VMArea describes one mapping
struct VMArea {
  VMArea() = default;
  VMArea(void *addr, size_t len, int prot, VMType type)
      : start(reinterpret_cast<uintptr_t>(addr)),
        end(start + len),
        prot(prot),
        type(type) {}
  VMArea(void *addr, size_t len, int prot, std::shared_ptr<File> file,
         off_t offset)
      : start(reinterpret_cast<uintptr_t>(addr)),
        end(start + len),
        prot(prot),
        type(VMType::kFile),
        file(std::move(file)),
        offset(offset) {}

  // Addr returns a pointer to the base address of the VMA.
  void *Addr() const { return reinterpret_cast<void *>(start); }
  // Length returns the length of the VMA.
  size_t Length() const { return end - start; }

  uintptr_t start;
  uintptr_t end;
  int prot;
  VMType type;
  std::shared_ptr<File> file;
  off_t offset;
};

// MemoryMap manages memory for a process
class alignas(kCacheLineSize) MemoryMap {
 public:
  MemoryMap(void *base, size_t len)
      : brk_start_(reinterpret_cast<uintptr_t>(base)),
        brk_end_(brk_start_ + len),
        brk_addr_(brk_start_) {}
  ~MemoryMap();

  [[nodiscard]] std::vector<VMArea> get_vmas();

  // SetBreak sets the break address (for the heap). It returns the new address
  // on success, the old address on failure, or EINTR if interrupted.
  Status<uintptr_t> SetBreak(uintptr_t brk_addr);

  // MMap inserts a memory mapping.
  Status<void *> MMap(void *addr, size_t len, int prot, int flags,
                      std::shared_ptr<File> f, off_t off);

  // MMapAnonymous inserts an anonymous memory mapping.
  Status<void *> MMapAnonymous(void *addr, size_t len, int prot, int flags) {
    return MMap(addr, len, prot, flags | MAP_PRIVATE | MAP_ANONYMOUS, {}, 0);
  }

  // MProtect changes the access protections of a range of mappings.
  Status<void> MProtect(void *addr, size_t len, int prot);

  // MUnmap removes a range of mappings.
  Status<void> MUnmap(void *addr, size_t len);

  // MAdvise gives the kernel a hint about how a range of mappings will be used.
  Status<void> MAdvise(void *addr, size_t len, int hint);

  // VirtualUsage returns the size (in bytes) of allocated virtual memory.
  [[nodiscard]] size_t VirtualUsage();

  // HeapUsage returns the size (in bytes) of the heap.
  [[nodiscard]] size_t HeapUsage() const { return brk_addr_ - brk_start_; }

  // LogMappings prints all the mappings to the log.
  void LogMappings();

  // Snapshot functions
  // TODO(amb): Use Cereal here
  MemoryMap(const ProcessMetadata &pm);
  void Snapshot(ProcessMetadata &s);
  void Restore(ProcessMetadata const &pm, FileTable &ftbl);

 private:
  // Clear removes existing VMAreas that overlap with the range [start, end)
  // Ex: ClearMappings(2, 6) when vmareas_ = [1, 3), [5, 7) results in vmareas_
  // = [1, 2), [6, 7). Returns an iterator to the first mapping after the
  // region that was cleared.
  std::map<uintptr_t, VMArea>::iterator Clear(uintptr_t start, uintptr_t end);

  // Modify changes the access protections for memory in the range [start,
  // end).
  void Modify(uintptr_t start, uintptr_t end, int prot);

  // Insert inserts a VMA, removing any overlapping mappings.
  void Insert(VMArea &&vma);

  rt::SharedMutex mu_;
  const uintptr_t brk_start_;
  const uintptr_t brk_end_;
  uintptr_t brk_addr_;
  std::map<uintptr_t, VMArea> vmareas_;
};

// Reserve a region of virtual memory for a MemoryMap.
inline Status<std::shared_ptr<MemoryMap>> CreateMemoryMap(size_t len) {
  Status<void *> ret = KernelMMap(nullptr, len, PROT_NONE, 0);
  if (!ret) return MakeError(ret);
  return std::make_shared<MemoryMap>(*ret, len);
}

}  // namespace junction
