// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod game {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

pub enum StateOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct State<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for State<'a> {
  type Inner = State<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> State<'a> {
  pub const VT_PLAYERS: flatbuffers::VOffsetT = 4;
  pub const VT_ADMIN: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    State { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args StateArgs<'args>
  ) -> flatbuffers::WIPOffset<State<'bldr>> {
    let mut builder = StateBuilder::new(_fbb);
    builder.add_admin(args.admin);
    if let Some(x) = args.players { builder.add_players(x); }
    builder.finish()
  }


  #[inline]
  pub fn players(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Player<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Player>>>>(State::VT_PLAYERS, None)}
  }
  #[inline]
  pub fn admin(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(State::VT_ADMIN, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for State<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Player>>>>("players", Self::VT_PLAYERS, false)?
     .visit_field::<u32>("admin", Self::VT_ADMIN, false)?
     .finish();
    Ok(())
  }
}
pub struct StateArgs<'a> {
    pub players: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Player<'a>>>>>,
    pub admin: u32,
}
impl<'a> Default for StateArgs<'a> {
  #[inline]
  fn default() -> Self {
    StateArgs {
      players: None,
      admin: 0,
    }
  }
}

pub struct StateBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> StateBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_players(&mut self, players: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Player<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(State::VT_PLAYERS, players);
  }
  #[inline]
  pub fn add_admin(&mut self, admin: u32) {
    self.fbb_.push_slot::<u32>(State::VT_ADMIN, admin, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> StateBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    StateBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<State<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for State<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("State");
      ds.field("players", &self.players());
      ds.field("admin", &self.admin());
      ds.finish()
  }
}
pub enum PlayerOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Player<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Player<'a> {
  type Inner = Player<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Player<'a> {
  pub const VT_ID: flatbuffers::VOffsetT = 4;
  pub const VT_NAME: flatbuffers::VOffsetT = 6;
  pub const VT_SCORE: flatbuffers::VOffsetT = 8;
  pub const VT_CONNECTED: flatbuffers::VOffsetT = 10;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Player { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args PlayerArgs<'args>
  ) -> flatbuffers::WIPOffset<Player<'bldr>> {
    let mut builder = PlayerBuilder::new(_fbb);
    builder.add_score(args.score);
    if let Some(x) = args.name { builder.add_name(x); }
    builder.add_id(args.id);
    builder.add_connected(args.connected);
    builder.finish()
  }


  #[inline]
  pub fn id(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(Player::VT_ID, Some(0)).unwrap()}
  }
  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Player::VT_NAME, None)}
  }
  #[inline]
  pub fn score(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(Player::VT_SCORE, Some(0)).unwrap()}
  }
  #[inline]
  pub fn connected(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(Player::VT_CONNECTED, Some(false)).unwrap()}
  }
}

impl flatbuffers::Verifiable for Player<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u32>("id", Self::VT_ID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .visit_field::<u32>("score", Self::VT_SCORE, false)?
     .visit_field::<bool>("connected", Self::VT_CONNECTED, false)?
     .finish();
    Ok(())
  }
}
pub struct PlayerArgs<'a> {
    pub id: u32,
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub score: u32,
    pub connected: bool,
}
impl<'a> Default for PlayerArgs<'a> {
  #[inline]
  fn default() -> Self {
    PlayerArgs {
      id: 0,
      name: None,
      score: 0,
      connected: false,
    }
  }
}

pub struct PlayerBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> PlayerBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_id(&mut self, id: u32) {
    self.fbb_.push_slot::<u32>(Player::VT_ID, id, 0);
  }
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Player::VT_NAME, name);
  }
  #[inline]
  pub fn add_score(&mut self, score: u32) {
    self.fbb_.push_slot::<u32>(Player::VT_SCORE, score, 0);
  }
  #[inline]
  pub fn add_connected(&mut self, connected: bool) {
    self.fbb_.push_slot::<bool>(Player::VT_CONNECTED, connected, false);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> PlayerBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    PlayerBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Player<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Player<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Player");
      ds.field("id", &self.id());
      ds.field("name", &self.name());
      ds.field("score", &self.score());
      ds.field("connected", &self.connected());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `State`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_state_unchecked`.
pub fn root_as_state(buf: &[u8]) -> Result<State, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<State>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `State` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_state_unchecked`.
pub fn size_prefixed_root_as_state(buf: &[u8]) -> Result<State, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<State>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `State` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_state_unchecked`.
pub fn root_as_state_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<State<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<State<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `State` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_state_unchecked`.
pub fn size_prefixed_root_as_state_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<State<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<State<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a State and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `State`.
pub unsafe fn root_as_state_unchecked(buf: &[u8]) -> State {
  flatbuffers::root_unchecked::<State>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed State and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `State`.
pub unsafe fn size_prefixed_root_as_state_unchecked(buf: &[u8]) -> State {
  flatbuffers::size_prefixed_root_unchecked::<State>(buf)
}
#[inline]
pub fn finish_state_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    root: flatbuffers::WIPOffset<State<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_state_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<State<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod Game

