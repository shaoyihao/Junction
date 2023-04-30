#include "junction/kernel/elf.h"

#include <algorithm>
#include <bit>
#include <cstring>
#include <string>
#include <vector>

#include "junction/base/arch.h"
#include "junction/base/error.h"
#include "junction/base/io.h"
#include "junction/bindings/log.h"
#include "junction/junction.h"
#include "junction/kernel/file.h"
#include "junction/kernel/fs.h"
#include "junction/kernel/ksys.h"
#include "junction/kernel/proc.h"

namespace junction {
namespace {

// JunctionFile provides a wrapper around a Junction FS-provided file.
class JunctionFile {
 public:
  // Open creates a new file descriptor attached to a file path.
  static Status<JunctionFile> Open(std::string_view path, int flags,
                                   mode_t mode) {
    FileSystem *fs = get_fs();
    Status<std::shared_ptr<File>> f = fs->Open(path, mode, flags);
    if (!f) return MakeError(f);
    return JunctionFile(std::move(*f));
  }

  explicit JunctionFile(std::shared_ptr<File> &&f) noexcept
      : f_(std::move(f)) {}
  ~JunctionFile() = default;

  // Read from the file.
  Status<size_t> Read(std::span<std::byte> buf) { return f_->Read(buf, &off_); }

  // Write to the file.
  Status<size_t> Write(std::span<const std::byte> buf) {
    return f_->Write(buf, &off_);
  }

  // Map a portion of the file.
  Status<void *> MMap(size_t length, int prot, int flags, off_t off) {
    assert(!(flags & (MAP_FIXED | MAP_ANONYMOUS)));
    flags |= MAP_PRIVATE;
    return f_->MMap(nullptr, length, prot, flags, off);
  }

  // Map a portion of the file to a fixed address.
  Status<void> MMapFixed(void *addr, size_t length, int prot, int flags,
                         off_t off) {
    assert(!(flags & MAP_ANONYMOUS));
    flags |= MAP_FIXED | MAP_PRIVATE;
    Status<void *> ret = f_->MMap(addr, length, prot, flags, off);
    if (!ret) return MakeError(ret);
    return {};
  }

  // Seek to a different position in the file.
  void Seek(off_t offset) { f_->Seek(offset, SeekFrom::kStart); }

 private:
  std::shared_ptr<File> f_;
  off_t off_{0};
};

constexpr size_t kMagicLen = 16;

struct elf_header {
  uint8_t magic[kMagicLen];  // used to detect the file type
  uint16_t type;             // the type of ELF file
  uint16_t machine;          // the machine's architecture
  uint32_t version;          // the object file version
  uint64_t entry;            // the entry point (a virtual address)
  uint64_t phoff;            // program header table offset (start location)
  uint64_t shoff;            // section header table offset (start location)
  uint32_t flags;            // processor-specific flags (ignored)
  uint16_t ehsize;           // ELF header size in bytes
  uint16_t phsize;           // size of a program header entry in bytes
  uint16_t phnum;            // number of program header entries
  uint16_t shsize;           // size of a section header entry in bytes
  uint16_t shnum;            // number of section header entries
  uint16_t shstrndx;         // section header string table index
};

// Magic values for just the file type we can support (X86_64 CPUs).
constexpr uint8_t kMagicClass64 = 2;    // 64-bit object file
constexpr uint8_t kMagicData2LSB = 1;   // 2's complement, little endian
constexpr uint8_t kMagicVersion = 1;    // the current ELF format version
constexpr uint16_t kMachineAMD64 = 62;  // X86_64 processor (Intel and AMD)

enum {
  kETypeExec = 2,     // Executable type
  kETypeDynamic = 3,  // Dynamically loaded type
  // other types are not supported
};

// program header format
struct elf_phdr {
  uint32_t type;    // the type of PHDR header
  uint32_t flags;   // permission flags
  uint64_t offset;  // the offset in the file that contains the data
  uint64_t vaddr;   // the target virtual address
  uint64_t paddr;   // can be ignored
  uint64_t filesz;  // size in bytes stored in the backing file
  uint64_t memsz;   // size in bytes in memory (can be larger than filesz)
  uint64_t align;   // the alignment; must be power of 2. offset and vaddr must
                    // be the same value modulo the alignment.
};

enum {
  kPTypeNull = 0,       // entry is unused
  kPTypeLoad = 1,       // segment that should be loaded
  kPTypeDynamic = 2,    // dynamic linker information
  kPTypeInterp = 3,     // contains a path to the interpreter to load
  kPTypeNote = 4,       // auxiliary information
  kPTypeSharedLib = 5,  // not used
  kPTypeSelf = 6,       // entry for the PHDR header table itself
  kPTypeTLS = 7,        // thread local storage segment
  // several more architecture-specific types are omitted for now
};

enum {
  kFlagExec = 1,   // Executable permission
  kFlagWrite = 2,  // Write permission
  kFlagRead = 4,   // Read permission
};

constexpr bool HeaderIsValid(const elf_header &hdr) {
  if (hdr.magic[0] != '\177' || hdr.magic[1] != 'E' || hdr.magic[2] != 'L' ||
      hdr.magic[3] != 'F') {
    return false;
  }
  if (hdr.magic[4] != kMagicClass64) return false;
  if (hdr.magic[5] != kMagicData2LSB) return false;
  if (hdr.magic[6] != kMagicVersion) return false;
  if (hdr.version != static_cast<uint32_t>(kMagicVersion)) return false;
  if (hdr.machine != kMachineAMD64) return false;
  if (hdr.phsize != sizeof(elf_phdr)) return false;
  if (hdr.ehsize != sizeof(elf_header)) return false;
  return true;
}

// ReadHeader reads and validates the header of the ELF file
Status<elf_header> ReadHeader(JunctionFile &f) {
  elf_header hdr;
  Status<void> ret = ReadFull(f, writable_byte_view(hdr));
  if (!ret) return MakeError(ret);
  if (!HeaderIsValid(hdr)) {
    LOG(ERR) << "elf: invalid/unsupported ELF file.";
    return MakeError(EINVAL);
  }
  return hdr;
}

// ReadPHDRs reads a vector of PHDRs from the ELF file
Status<std::vector<elf_phdr>> ReadPHDRs(JunctionFile &f,
                                        const elf_header &hdr) {
  std::vector<elf_phdr> phdrs(hdr.phnum);

  // Read the PHDRs into the vector.
  f.Seek(hdr.phoff);
  Status<void> ret = ReadFull(f, std::as_writable_bytes(std::span(phdrs)));
  if (!ret) return MakeError(ret);

  // Confirm that the PHDRs contain valid state.
  for (const elf_phdr &phdr : phdrs) {
    if (!std::has_single_bit(phdr.align) || phdr.filesz > phdr.memsz ||
        (phdr.vaddr & (phdr.align - 1)) != (phdr.offset & (phdr.align - 1))) {
      LOG(ERR) << "elf: encountered an invalid PHDR.";
      return MakeError(EINVAL);
    }
  }
  return std::move(phdrs);
}

// CountTotalSize returns the size of all the segments together
size_t CountTotalLength(const std::vector<elf_phdr> &phdrs) {
  size_t len = 0;
  for (const elf_phdr &phdr : phdrs) {
    if (phdr.type != kPTypeLoad) continue;
    len = std::max(len, phdr.vaddr + phdr.memsz);
  }
  return len;
}

// ReadInterp loads the interpretor section and returns a path
Status<std::string> ReadInterp(JunctionFile &f, const elf_phdr &phdr) {
  std::string interp_path(phdr.filesz - 1,
                          '\0');  // Don't read the null terminator
  f.Seek(phdr.offset);
  Status<void> ret =
      ReadFull(f, std::as_writable_bytes(std::span(interp_path)));
  if (!ret) return MakeError(ret);
  return std::move(interp_path);
}

// LoadOneSegment loads one loadable PHDR into memory
Status<void> LoadOneSegment(JunctionFile &f, off_t map_off,
                            const elf_phdr &phdr) {
  // Determine the mapping permissions.
  unsigned int prot = 0;
  if (phdr.flags & kFlagExec) prot |= PROT_EXEC;
  if (phdr.flags & kFlagWrite) prot |= PROT_WRITE;
  if (phdr.flags & kFlagRead) prot |= PROT_READ;

  // Determine the layout.
  uintptr_t start = PageAlignDown(phdr.vaddr + map_off);
  uintptr_t file_end = phdr.vaddr + map_off + phdr.filesz;
  uintptr_t gap_end = PageAlign(file_end);
  uintptr_t mem_end = phdr.vaddr + map_off + phdr.memsz;

  // Map the file part of the segment.
  if (file_end > start) {
    Status<void> ret =
        f.MMapFixed(reinterpret_cast<void *>(start), file_end - start, prot,
                    MAP_DENYWRITE, PageAlignDown(phdr.offset));
    if (unlikely(!ret)) return MakeError(ret);
  }

  // Zero the gap
  if (gap_end > file_end) {
    if ((prot & PROT_WRITE) == 0) {
      Status<void> ret =
          KernelMProtect(reinterpret_cast<void *>(PageAlignDown(file_end)),
                         kPageSize, prot | PROT_WRITE);
      if (unlikely(!ret)) return MakeError(ret);
    }
    std::memset(reinterpret_cast<void *>(file_end), 0, gap_end - file_end);
    if ((prot & PROT_WRITE) == 0) {
      Status<void> ret = KernelMProtect(
          reinterpret_cast<void *>(PageAlignDown(file_end)), kPageSize, prot);
      if (unlikely(!ret)) return MakeError(ret);
    }
  }

  // Map the remaining anonymous part of the segment.
  if (mem_end > gap_end) {
    Status<void> ret = KernelMMapFixed(reinterpret_cast<void *>(gap_end),
                                       mem_end - gap_end, prot, 0);
    if (unlikely(!ret)) return MakeError(ret);
  }

  return {};
}

// LoadSegments loads all loadable PHDRs
Status<std::tuple<uintptr_t, size_t>> LoadSegments(
    MemoryMap &mm, JunctionFile &f, const std::vector<elf_phdr> &phdrs,
    bool reloc) {
  // Determine the base address.
  off_t map_off = 0;
  size_t map_len = CountTotalLength(phdrs);
  if (reloc) {
    void *ret = mm.ReserveForMapping(map_len);
    if (!ret) return MakeError(ENOMEM);
    map_off = reinterpret_cast<off_t>(ret);
  }

  // Load the segments.
  for (const elf_phdr &phdr : phdrs) {
    if (phdr.type != kPTypeLoad) continue;
    Status<void> ret = LoadOneSegment(f, map_off, phdr);
    if (!ret) return MakeError(ret);
  }

  return std::make_tuple(map_off, map_len);
}

// LoadInterp loads an interpreter binary (usually ld.so).
Status<elf_data::interp_data> LoadInterp(MemoryMap &mm, std::string_view path) {
  if (junction::GetCfg().get_interp_path().size())
    path = junction::GetCfg().get_interp_path();

  DLOG(INFO) << "elf: loading interpreter ELF object file '" << path << "'";

  // Open the file.
  Status<JunctionFile> file = JunctionFile::Open(path, 0, S_IRUSR | S_IXUSR);
  if (!file) return MakeError(file);

  // Load the ELF header.
  Status<elf_header> hdr = ReadHeader(*file);
  if (!hdr) return MakeError(hdr);

  // Check if the ELF type is supported.
  if (hdr->type != kETypeDynamic) return MakeError(EINVAL);

  // Load the PHDR table.
  Status<std::vector<elf_phdr>> phdrs = ReadPHDRs(*file, *hdr);
  if (!phdrs) return MakeError(phdrs);

  // Load the PHDR segments.
  Status<std::tuple<uintptr_t, size_t>> ret =
      LoadSegments(mm, *file, *phdrs, true);
  if (!ret) return MakeError(ret);

  DLOG(DEBUG) << "gdb: add-symbol-file " << path << " -o " << std::get<0>(*ret);

  // Success, return metadata.
  return elf_data::interp_data{.map_base{std::get<0>(*ret)},
                               .map_len{std::get<1>(*ret)},
                               .entry_addr{hdr->entry + std::get<0>(*ret)}};
}

// FindPHDRByType returns the first PHDR of a type if one exists.
std::optional<elf_phdr> FindPHDRByType(const std::vector<elf_phdr> &v,
                                       uint32_t type) {
  auto it = std::find_if(v.begin(), v.end(), [type](const elf_phdr &phdr) {
    return phdr.type == type;
  });
  if (it == v.end()) return {};
  return *it;
}

}  // namespace

Status<elf_data> LoadELF(MemoryMap &mm, std::string_view path) {
  DLOG(INFO) << "elf: loading ELF object file '" << path << "'";

  // Open the file.
  Status<JunctionFile> file = JunctionFile::Open(path, 0, S_IRUSR | S_IXUSR);
  if (!file) return MakeError(file);

  // Load the ELF header.
  Status<elf_header> hdr = ReadHeader(*file);
  if (!hdr) return MakeError(hdr);

  // Check if the ELF type is supported.
  if (hdr->type != kETypeExec && hdr->type != kETypeDynamic)
    return MakeError(EINVAL);

  // Load the PHDR table.
  Status<std::vector<elf_phdr>> phdrs = ReadPHDRs(*file, *hdr);
  if (!phdrs) return MakeError(phdrs);

  // Load the interpreter (if present).
  std::optional<elf_data::interp_data> interp_data;
  std::optional<elf_phdr> phdr = FindPHDRByType(*phdrs, kPTypeInterp);
  if (phdr) {
    Status<std::string> path = ReadInterp(*file, *phdr);
    if (!path) return MakeError(path);
    Status<elf_data::interp_data> data = LoadInterp(mm, *path);
    if (!data) return MakeError(data);
    interp_data = *data;
  }

  // Load the PHDR segments.
  Status<std::tuple<uintptr_t, size_t>> ret =
      LoadSegments(mm, *file, *phdrs, hdr->type == kETypeDynamic);
  if (!ret) return MakeError(ret);

  // Look for a PHDR table segment
  uintptr_t phdr_va = 0;
  phdr = FindPHDRByType(*phdrs, kPTypeSelf);
  if (phdr) phdr_va = phdr->vaddr + std::get<0>(*ret);

  DLOG(DEBUG) << "gdb: add-symbol-file " << path << " -o " << std::get<0>(*ret);

  // Success, return metadata.
  return elf_data{.map_base{std::get<0>(*ret)},
                  .map_len{std::get<1>(*ret)},
                  .entry_addr{hdr->entry + std::get<0>(*ret)},
                  .phdr_addr{phdr_va},
                  .phdr_num{hdr->phnum},
                  .phdr_entsz{hdr->phsize},
                  .interp{std::move(interp_data)}};
}

}  // namespace junction
