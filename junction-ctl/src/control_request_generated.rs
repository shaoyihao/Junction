// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod junction {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};
#[allow(unused_imports, dead_code)]
pub mod ctl_schema {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_INNER_REQUEST: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_INNER_REQUEST: u8 = 7;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_INNER_REQUEST: [InnerRequest; 8] = [
  InnerRequest::NONE,
  InnerRequest::run,
  InnerRequest::snapshot,
  InnerRequest::restore,
  InnerRequest::startTrace,
  InnerRequest::stopTrace,
  InnerRequest::signal,
  InnerRequest::getStats,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct InnerRequest(pub u8);
#[allow(non_upper_case_globals)]
impl InnerRequest {
  pub const NONE: Self = Self(0);
  pub const run: Self = Self(1);
  pub const snapshot: Self = Self(2);
  pub const restore: Self = Self(3);
  pub const startTrace: Self = Self(4);
  pub const stopTrace: Self = Self(5);
  pub const signal: Self = Self(6);
  pub const getStats: Self = Self(7);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 7;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::run,
    Self::snapshot,
    Self::restore,
    Self::startTrace,
    Self::stopTrace,
    Self::signal,
    Self::getStats,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::run => Some("run"),
      Self::snapshot => Some("snapshot"),
      Self::restore => Some("restore"),
      Self::startTrace => Some("startTrace"),
      Self::stopTrace => Some("stopTrace"),
      Self::signal => Some("signal"),
      Self::getStats => Some("getStats"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for InnerRequest {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for InnerRequest {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for InnerRequest {
    type Output = InnerRequest;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for InnerRequest {
  type Scalar = u8;
  #[inline]
  fn to_little_endian(self) -> u8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u8) -> Self {
    let b = u8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for InnerRequest {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for InnerRequest {}
pub struct InnerRequestUnionTableOffset {}

pub enum RunRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct RunRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for RunRequest<'a> {
  type Inner = RunRequest<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> RunRequest<'a> {
  pub const VT_ARGV: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    RunRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args RunRequestArgs<'args>
  ) -> flatbuffers::WIPOffset<RunRequest<'bldr>> {
    let mut builder = RunRequestBuilder::new(_fbb);
    if let Some(x) = args.argv { builder.add_argv(x); }
    builder.finish()
  }


  #[inline]
  pub fn argv(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>(RunRequest::VT_ARGV, None)}
  }
}

impl flatbuffers::Verifiable for RunRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<&'_ str>>>>("argv", Self::VT_ARGV, false)?
     .finish();
    Ok(())
  }
}
pub struct RunRequestArgs<'a> {
    pub argv: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>,
}
impl<'a> Default for RunRequestArgs<'a> {
  #[inline]
  fn default() -> Self {
    RunRequestArgs {
      argv: None,
    }
  }
}

pub struct RunRequestBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> RunRequestBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_argv(&mut self, argv: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RunRequest::VT_ARGV, argv);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> RunRequestBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    RunRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<RunRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for RunRequest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("RunRequest");
      ds.field("argv", &self.argv());
      ds.finish()
  }
}
pub enum SnapshotRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct SnapshotRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for SnapshotRequest<'a> {
  type Inner = SnapshotRequest<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> SnapshotRequest<'a> {
  pub const VT_PID: flatbuffers::VOffsetT = 4;
  pub const VT_SNAPSHOT_PATH: flatbuffers::VOffsetT = 6;
  pub const VT_ELF_PATH: flatbuffers::VOffsetT = 8;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    SnapshotRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args SnapshotRequestArgs<'args>
  ) -> flatbuffers::WIPOffset<SnapshotRequest<'bldr>> {
    let mut builder = SnapshotRequestBuilder::new(_fbb);
    builder.add_pid(args.pid);
    if let Some(x) = args.elf_path { builder.add_elf_path(x); }
    if let Some(x) = args.snapshot_path { builder.add_snapshot_path(x); }
    builder.finish()
  }


  #[inline]
  pub fn pid(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(SnapshotRequest::VT_PID, Some(0)).unwrap()}
  }
  #[inline]
  pub fn snapshot_path(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(SnapshotRequest::VT_SNAPSHOT_PATH, None)}
  }
  #[inline]
  pub fn elf_path(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(SnapshotRequest::VT_ELF_PATH, None)}
  }
}

impl flatbuffers::Verifiable for SnapshotRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u64>("pid", Self::VT_PID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("snapshot_path", Self::VT_SNAPSHOT_PATH, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("elf_path", Self::VT_ELF_PATH, false)?
     .finish();
    Ok(())
  }
}
pub struct SnapshotRequestArgs<'a> {
    pub pid: u64,
    pub snapshot_path: Option<flatbuffers::WIPOffset<&'a str>>,
    pub elf_path: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for SnapshotRequestArgs<'a> {
  #[inline]
  fn default() -> Self {
    SnapshotRequestArgs {
      pid: 0,
      snapshot_path: None,
      elf_path: None,
    }
  }
}

pub struct SnapshotRequestBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> SnapshotRequestBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_pid(&mut self, pid: u64) {
    self.fbb_.push_slot::<u64>(SnapshotRequest::VT_PID, pid, 0);
  }
  #[inline]
  pub fn add_snapshot_path(&mut self, snapshot_path: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SnapshotRequest::VT_SNAPSHOT_PATH, snapshot_path);
  }
  #[inline]
  pub fn add_elf_path(&mut self, elf_path: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SnapshotRequest::VT_ELF_PATH, elf_path);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> SnapshotRequestBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    SnapshotRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<SnapshotRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for SnapshotRequest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("SnapshotRequest");
      ds.field("pid", &self.pid());
      ds.field("snapshot_path", &self.snapshot_path());
      ds.field("elf_path", &self.elf_path());
      ds.finish()
  }
}
pub enum RestoreRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct RestoreRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for RestoreRequest<'a> {
  type Inner = RestoreRequest<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> RestoreRequest<'a> {
  pub const VT_SNAPSHOT_PATH: flatbuffers::VOffsetT = 4;
  pub const VT_ELF_PATH: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    RestoreRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args RestoreRequestArgs<'args>
  ) -> flatbuffers::WIPOffset<RestoreRequest<'bldr>> {
    let mut builder = RestoreRequestBuilder::new(_fbb);
    if let Some(x) = args.elf_path { builder.add_elf_path(x); }
    if let Some(x) = args.snapshot_path { builder.add_snapshot_path(x); }
    builder.finish()
  }


  #[inline]
  pub fn snapshot_path(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(RestoreRequest::VT_SNAPSHOT_PATH, None)}
  }
  #[inline]
  pub fn elf_path(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(RestoreRequest::VT_ELF_PATH, None)}
  }
}

impl flatbuffers::Verifiable for RestoreRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("snapshot_path", Self::VT_SNAPSHOT_PATH, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("elf_path", Self::VT_ELF_PATH, false)?
     .finish();
    Ok(())
  }
}
pub struct RestoreRequestArgs<'a> {
    pub snapshot_path: Option<flatbuffers::WIPOffset<&'a str>>,
    pub elf_path: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for RestoreRequestArgs<'a> {
  #[inline]
  fn default() -> Self {
    RestoreRequestArgs {
      snapshot_path: None,
      elf_path: None,
    }
  }
}

pub struct RestoreRequestBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> RestoreRequestBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_snapshot_path(&mut self, snapshot_path: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RestoreRequest::VT_SNAPSHOT_PATH, snapshot_path);
  }
  #[inline]
  pub fn add_elf_path(&mut self, elf_path: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RestoreRequest::VT_ELF_PATH, elf_path);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> RestoreRequestBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    RestoreRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<RestoreRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for RestoreRequest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("RestoreRequest");
      ds.field("snapshot_path", &self.snapshot_path());
      ds.field("elf_path", &self.elf_path());
      ds.finish()
  }
}
pub enum StartTraceRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct StartTraceRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for StartTraceRequest<'a> {
  type Inner = StartTraceRequest<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> StartTraceRequest<'a> {
  pub const VT_PID: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    StartTraceRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args StartTraceRequestArgs
  ) -> flatbuffers::WIPOffset<StartTraceRequest<'bldr>> {
    let mut builder = StartTraceRequestBuilder::new(_fbb);
    builder.add_pid(args.pid);
    builder.finish()
  }


  #[inline]
  pub fn pid(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(StartTraceRequest::VT_PID, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for StartTraceRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u64>("pid", Self::VT_PID, false)?
     .finish();
    Ok(())
  }
}
pub struct StartTraceRequestArgs {
    pub pid: u64,
}
impl<'a> Default for StartTraceRequestArgs {
  #[inline]
  fn default() -> Self {
    StartTraceRequestArgs {
      pid: 0,
    }
  }
}

pub struct StartTraceRequestBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> StartTraceRequestBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_pid(&mut self, pid: u64) {
    self.fbb_.push_slot::<u64>(StartTraceRequest::VT_PID, pid, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> StartTraceRequestBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    StartTraceRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<StartTraceRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for StartTraceRequest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("StartTraceRequest");
      ds.field("pid", &self.pid());
      ds.finish()
  }
}
pub enum StopTraceRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct StopTraceRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for StopTraceRequest<'a> {
  type Inner = StopTraceRequest<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> StopTraceRequest<'a> {
  pub const VT_PID: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    StopTraceRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args StopTraceRequestArgs
  ) -> flatbuffers::WIPOffset<StopTraceRequest<'bldr>> {
    let mut builder = StopTraceRequestBuilder::new(_fbb);
    builder.add_pid(args.pid);
    builder.finish()
  }


  #[inline]
  pub fn pid(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(StopTraceRequest::VT_PID, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for StopTraceRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u64>("pid", Self::VT_PID, false)?
     .finish();
    Ok(())
  }
}
pub struct StopTraceRequestArgs {
    pub pid: u64,
}
impl<'a> Default for StopTraceRequestArgs {
  #[inline]
  fn default() -> Self {
    StopTraceRequestArgs {
      pid: 0,
    }
  }
}

pub struct StopTraceRequestBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> StopTraceRequestBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_pid(&mut self, pid: u64) {
    self.fbb_.push_slot::<u64>(StopTraceRequest::VT_PID, pid, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> StopTraceRequestBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    StopTraceRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<StopTraceRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for StopTraceRequest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("StopTraceRequest");
      ds.field("pid", &self.pid());
      ds.finish()
  }
}
pub enum SignalRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct SignalRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for SignalRequest<'a> {
  type Inner = SignalRequest<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> SignalRequest<'a> {
  pub const VT_PID: flatbuffers::VOffsetT = 4;
  pub const VT_SIGNO: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    SignalRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args SignalRequestArgs
  ) -> flatbuffers::WIPOffset<SignalRequest<'bldr>> {
    let mut builder = SignalRequestBuilder::new(_fbb);
    builder.add_signo(args.signo);
    builder.add_pid(args.pid);
    builder.finish()
  }


  #[inline]
  pub fn pid(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(SignalRequest::VT_PID, Some(0)).unwrap()}
  }
  #[inline]
  pub fn signo(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(SignalRequest::VT_SIGNO, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for SignalRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u64>("pid", Self::VT_PID, false)?
     .visit_field::<u64>("signo", Self::VT_SIGNO, false)?
     .finish();
    Ok(())
  }
}
pub struct SignalRequestArgs {
    pub pid: u64,
    pub signo: u64,
}
impl<'a> Default for SignalRequestArgs {
  #[inline]
  fn default() -> Self {
    SignalRequestArgs {
      pid: 0,
      signo: 0,
    }
  }
}

pub struct SignalRequestBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> SignalRequestBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_pid(&mut self, pid: u64) {
    self.fbb_.push_slot::<u64>(SignalRequest::VT_PID, pid, 0);
  }
  #[inline]
  pub fn add_signo(&mut self, signo: u64) {
    self.fbb_.push_slot::<u64>(SignalRequest::VT_SIGNO, signo, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> SignalRequestBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    SignalRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<SignalRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for SignalRequest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("SignalRequest");
      ds.field("pid", &self.pid());
      ds.field("signo", &self.signo());
      ds.finish()
  }
}
pub enum GetStatsRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct GetStatsRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for GetStatsRequest<'a> {
  type Inner = GetStatsRequest<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> GetStatsRequest<'a> {

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    GetStatsRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    _args: &'args GetStatsRequestArgs
  ) -> flatbuffers::WIPOffset<GetStatsRequest<'bldr>> {
    let mut builder = GetStatsRequestBuilder::new(_fbb);
    builder.finish()
  }

}

impl flatbuffers::Verifiable for GetStatsRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .finish();
    Ok(())
  }
}
pub struct GetStatsRequestArgs {
}
impl<'a> Default for GetStatsRequestArgs {
  #[inline]
  fn default() -> Self {
    GetStatsRequestArgs {
    }
  }
}

pub struct GetStatsRequestBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> GetStatsRequestBuilder<'a, 'b, A> {
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> GetStatsRequestBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    GetStatsRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<GetStatsRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for GetStatsRequest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("GetStatsRequest");
      ds.finish()
  }
}
pub enum RequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Request<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Request<'a> {
  type Inner = Request<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Request<'a> {
  pub const VT_INNER_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_INNER: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Request { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args RequestArgs
  ) -> flatbuffers::WIPOffset<Request<'bldr>> {
    let mut builder = RequestBuilder::new(_fbb);
    if let Some(x) = args.inner { builder.add_inner(x); }
    builder.add_inner_type(args.inner_type);
    builder.finish()
  }


  #[inline]
  pub fn inner_type(&self) -> InnerRequest {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<InnerRequest>(Request::VT_INNER_TYPE, Some(InnerRequest::NONE)).unwrap()}
  }
  #[inline]
  pub fn inner(&self) -> Option<flatbuffers::Table<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Request::VT_INNER, None)}
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn inner_as_run(&self) -> Option<RunRequest<'a>> {
    if self.inner_type() == InnerRequest::run {
      self.inner().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { RunRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn inner_as_snapshot(&self) -> Option<SnapshotRequest<'a>> {
    if self.inner_type() == InnerRequest::snapshot {
      self.inner().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { SnapshotRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn inner_as_restore(&self) -> Option<RestoreRequest<'a>> {
    if self.inner_type() == InnerRequest::restore {
      self.inner().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { RestoreRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn inner_as_start_trace(&self) -> Option<StartTraceRequest<'a>> {
    if self.inner_type() == InnerRequest::startTrace {
      self.inner().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { StartTraceRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn inner_as_stop_trace(&self) -> Option<StopTraceRequest<'a>> {
    if self.inner_type() == InnerRequest::stopTrace {
      self.inner().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { StopTraceRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn inner_as_signal(&self) -> Option<SignalRequest<'a>> {
    if self.inner_type() == InnerRequest::signal {
      self.inner().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { SignalRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn inner_as_get_stats(&self) -> Option<GetStatsRequest<'a>> {
    if self.inner_type() == InnerRequest::getStats {
      self.inner().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { GetStatsRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for Request<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_union::<InnerRequest, _>("inner_type", Self::VT_INNER_TYPE, "inner", Self::VT_INNER, false, |key, v, pos| {
        match key {
          InnerRequest::run => v.verify_union_variant::<flatbuffers::ForwardsUOffset<RunRequest>>("InnerRequest::run", pos),
          InnerRequest::snapshot => v.verify_union_variant::<flatbuffers::ForwardsUOffset<SnapshotRequest>>("InnerRequest::snapshot", pos),
          InnerRequest::restore => v.verify_union_variant::<flatbuffers::ForwardsUOffset<RestoreRequest>>("InnerRequest::restore", pos),
          InnerRequest::startTrace => v.verify_union_variant::<flatbuffers::ForwardsUOffset<StartTraceRequest>>("InnerRequest::startTrace", pos),
          InnerRequest::stopTrace => v.verify_union_variant::<flatbuffers::ForwardsUOffset<StopTraceRequest>>("InnerRequest::stopTrace", pos),
          InnerRequest::signal => v.verify_union_variant::<flatbuffers::ForwardsUOffset<SignalRequest>>("InnerRequest::signal", pos),
          InnerRequest::getStats => v.verify_union_variant::<flatbuffers::ForwardsUOffset<GetStatsRequest>>("InnerRequest::getStats", pos),
          _ => Ok(()),
        }
     })?
     .finish();
    Ok(())
  }
}
pub struct RequestArgs {
    pub inner_type: InnerRequest,
    pub inner: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for RequestArgs {
  #[inline]
  fn default() -> Self {
    RequestArgs {
      inner_type: InnerRequest::NONE,
      inner: None,
    }
  }
}

pub struct RequestBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> RequestBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_inner_type(&mut self, inner_type: InnerRequest) {
    self.fbb_.push_slot::<InnerRequest>(Request::VT_INNER_TYPE, inner_type, InnerRequest::NONE);
  }
  #[inline]
  pub fn add_inner(&mut self, inner: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Request::VT_INNER, inner);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> RequestBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    RequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Request<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Request<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Request");
      ds.field("inner_type", &self.inner_type());
      match self.inner_type() {
        InnerRequest::run => {
          if let Some(x) = self.inner_as_run() {
            ds.field("inner", &x)
          } else {
            ds.field("inner", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        InnerRequest::snapshot => {
          if let Some(x) = self.inner_as_snapshot() {
            ds.field("inner", &x)
          } else {
            ds.field("inner", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        InnerRequest::restore => {
          if let Some(x) = self.inner_as_restore() {
            ds.field("inner", &x)
          } else {
            ds.field("inner", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        InnerRequest::startTrace => {
          if let Some(x) = self.inner_as_start_trace() {
            ds.field("inner", &x)
          } else {
            ds.field("inner", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        InnerRequest::stopTrace => {
          if let Some(x) = self.inner_as_stop_trace() {
            ds.field("inner", &x)
          } else {
            ds.field("inner", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        InnerRequest::signal => {
          if let Some(x) = self.inner_as_signal() {
            ds.field("inner", &x)
          } else {
            ds.field("inner", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        InnerRequest::getStats => {
          if let Some(x) = self.inner_as_get_stats() {
            ds.field("inner", &x)
          } else {
            ds.field("inner", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("inner", &x)
        },
      };
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `Request`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_request_unchecked`.
pub fn root_as_request(buf: &[u8]) -> Result<Request, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Request>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Request` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_request_unchecked`.
pub fn size_prefixed_root_as_request(buf: &[u8]) -> Result<Request, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Request>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Request` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_request_unchecked`.
pub fn root_as_request_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Request<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Request<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Request` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_request_unchecked`.
pub fn size_prefixed_root_as_request_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Request<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Request<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Request and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Request`.
pub unsafe fn root_as_request_unchecked(buf: &[u8]) -> Request {
  flatbuffers::root_unchecked::<Request>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Request and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Request`.
pub unsafe fn size_prefixed_root_as_request_unchecked(buf: &[u8]) -> Request {
  flatbuffers::size_prefixed_root_unchecked::<Request>(buf)
}
#[inline]
pub fn finish_request_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    root: flatbuffers::WIPOffset<Request<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_request_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<Request<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod ctl_schema
}  // pub mod junction
