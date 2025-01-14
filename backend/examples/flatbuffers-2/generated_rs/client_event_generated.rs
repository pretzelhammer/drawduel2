// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod client_event {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_EVENT_UNION: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_EVENT_UNION: u8 = 1;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_EVENT_UNION: [EventUnion; 2] = [
  EventUnion::NONE,
  EventUnion::PlayerRename,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct EventUnion(pub u8);
#[allow(non_upper_case_globals)]
impl EventUnion {
  pub const NONE: Self = Self(0);
  pub const PlayerRename: Self = Self(1);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 1;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::PlayerRename,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::PlayerRename => Some("PlayerRename"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for EventUnion {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for EventUnion {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for EventUnion {
    type Output = EventUnion;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for EventUnion {
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

impl<'a> flatbuffers::Verifiable for EventUnion {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for EventUnion {}
pub struct EventUnionUnionTableOffset {}

pub enum PlayerRenameOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct PlayerRename<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for PlayerRename<'a> {
  type Inner = PlayerRename<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> PlayerRename<'a> {
  pub const VT_NAME: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    PlayerRename { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args PlayerRenameArgs<'args>
  ) -> flatbuffers::WIPOffset<PlayerRename<'bldr>> {
    let mut builder = PlayerRenameBuilder::new(_fbb);
    if let Some(x) = args.name { builder.add_name(x); }
    builder.finish()
  }


  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(PlayerRename::VT_NAME, None)}
  }
}

impl flatbuffers::Verifiable for PlayerRename<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .finish();
    Ok(())
  }
}
pub struct PlayerRenameArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for PlayerRenameArgs<'a> {
  #[inline]
  fn default() -> Self {
    PlayerRenameArgs {
      name: None,
    }
  }
}

pub struct PlayerRenameBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> PlayerRenameBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(PlayerRename::VT_NAME, name);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> PlayerRenameBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    PlayerRenameBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<PlayerRename<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for PlayerRename<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("PlayerRename");
      ds.field("name", &self.name());
      ds.finish()
  }
}
pub enum EventOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Event<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Event<'a> {
  type Inner = Event<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Event<'a> {
  pub const VT_EVENT_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_EVENT: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Event { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args EventArgs
  ) -> flatbuffers::WIPOffset<Event<'bldr>> {
    let mut builder = EventBuilder::new(_fbb);
    if let Some(x) = args.event { builder.add_event(x); }
    builder.add_event_type(args.event_type);
    builder.finish()
  }


  #[inline]
  pub fn event_type(&self) -> EventUnion {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<EventUnion>(Event::VT_EVENT_TYPE, Some(EventUnion::NONE)).unwrap()}
  }
  #[inline]
  pub fn event(&self) -> Option<flatbuffers::Table<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Event::VT_EVENT, None)}
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn event_as_player_rename(&self) -> Option<PlayerRename<'a>> {
    if self.event_type() == EventUnion::PlayerRename {
      self.event().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { PlayerRename::init_from_table(t) }
     })
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for Event<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_union::<EventUnion, _>("event_type", Self::VT_EVENT_TYPE, "event", Self::VT_EVENT, false, |key, v, pos| {
        match key {
          EventUnion::PlayerRename => v.verify_union_variant::<flatbuffers::ForwardsUOffset<PlayerRename>>("EventUnion::PlayerRename", pos),
          _ => Ok(()),
        }
     })?
     .finish();
    Ok(())
  }
}
pub struct EventArgs {
    pub event_type: EventUnion,
    pub event: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for EventArgs {
  #[inline]
  fn default() -> Self {
    EventArgs {
      event_type: EventUnion::NONE,
      event: None,
    }
  }
}

pub struct EventBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> EventBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_event_type(&mut self, event_type: EventUnion) {
    self.fbb_.push_slot::<EventUnion>(Event::VT_EVENT_TYPE, event_type, EventUnion::NONE);
  }
  #[inline]
  pub fn add_event(&mut self, event: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Event::VT_EVENT, event);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> EventBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    EventBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Event<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Event<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Event");
      ds.field("event_type", &self.event_type());
      match self.event_type() {
        EventUnion::PlayerRename => {
          if let Some(x) = self.event_as_player_rename() {
            ds.field("event", &x)
          } else {
            ds.field("event", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("event", &x)
        },
      };
      ds.finish()
  }
}
pub enum EventsOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Events<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Events<'a> {
  type Inner = Events<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Events<'a> {
  pub const VT_EVENTS: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Events { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args EventsArgs<'args>
  ) -> flatbuffers::WIPOffset<Events<'bldr>> {
    let mut builder = EventsBuilder::new(_fbb);
    if let Some(x) = args.events { builder.add_events(x); }
    builder.finish()
  }


  #[inline]
  pub fn events(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Event<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Event>>>>(Events::VT_EVENTS, None)}
  }
}

impl flatbuffers::Verifiable for Events<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Event>>>>("events", Self::VT_EVENTS, false)?
     .finish();
    Ok(())
  }
}
pub struct EventsArgs<'a> {
    pub events: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Event<'a>>>>>,
}
impl<'a> Default for EventsArgs<'a> {
  #[inline]
  fn default() -> Self {
    EventsArgs {
      events: None,
    }
  }
}

pub struct EventsBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> EventsBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_events(&mut self, events: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Event<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Events::VT_EVENTS, events);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> EventsBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    EventsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Events<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Events<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Events");
      ds.field("events", &self.events());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `Events`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_events_unchecked`.
pub fn root_as_events(buf: &[u8]) -> Result<Events, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Events>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Events` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_events_unchecked`.
pub fn size_prefixed_root_as_events(buf: &[u8]) -> Result<Events, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Events>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Events` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_events_unchecked`.
pub fn root_as_events_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Events<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Events<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Events` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_events_unchecked`.
pub fn size_prefixed_root_as_events_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Events<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Events<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Events and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Events`.
pub unsafe fn root_as_events_unchecked(buf: &[u8]) -> Events {
  flatbuffers::root_unchecked::<Events>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Events and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Events`.
pub unsafe fn size_prefixed_root_as_events_unchecked(buf: &[u8]) -> Events {
  flatbuffers::size_prefixed_root_unchecked::<Events>(buf)
}
#[inline]
pub fn finish_events_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    root: flatbuffers::WIPOffset<Events<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_events_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<Events<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod ClientEvent

