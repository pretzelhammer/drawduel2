// @generated
// This file is @generated by prost-build.
// GAME STATE

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Game {
    #[prost(map="uint32, message", tag="1")]
    pub players: ::std::collections::HashMap<u32, Player>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Player {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub score: u32,
    #[prost(bool, tag="3")]
    pub connected: bool,
}
// SERVER EVENTS
// Se = \[S\]erver \[e\]vent
// mostly represent game state transitions
// but can also be other things, like
// communicating server errors to client

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeSetGame {
    #[prost(uint32, tag="1")]
    pub player_id: u32,
    #[prost(message, optional, tag="2")]
    pub game: ::core::option::Option<Game>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeError {
    #[prost(enumeration="SeErrorType", tag="1")]
    pub se_error_type: i32,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SePlayerJoin {
    #[prost(uint32, tag="1")]
    pub id: u32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SePlayerLeave {
    #[prost(uint32, tag="1")]
    pub id: u32,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SePlayerConnect {
    #[prost(uint32, tag="1")]
    pub id: u32,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SePlayerDisconnect {
    #[prost(uint32, tag="1")]
    pub id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SePlayerRename {
    #[prost(uint32, tag="1")]
    pub id: u32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SePlayerIncreaseScore {
    #[prost(uint32, tag="1")]
    pub id: u32,
    #[prost(uint32, tag="2")]
    pub score: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerEvent {
    #[prost(oneof="server_event::SeType", tags="1, 2, 3, 4, 5, 6, 7, 8")]
    pub se_type: ::core::option::Option<server_event::SeType>,
}
/// Nested message and enum types in `ServerEvent`.
pub mod server_event {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SeType {
        #[prost(message, tag="1")]
        PlayerJoin(super::SePlayerJoin),
        #[prost(message, tag="2")]
        PlayerLeave(super::SePlayerLeave),
        #[prost(message, tag="3")]
        PlayerRename(super::SePlayerRename),
        #[prost(message, tag="4")]
        PlayerIncreaseScore(super::SePlayerIncreaseScore),
        #[prost(message, tag="5")]
        SetGame(super::SeSetGame),
        #[prost(message, tag="6")]
        Error(super::SeError),
        #[prost(message, tag="7")]
        PlayerConnect(super::SePlayerConnect),
        #[prost(message, tag="8")]
        PlayerDisconnect(super::SePlayerDisconnect),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerEvents {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<ServerEvent>,
}
// CLIENT EVENTS
// Ce = \[C\]lient \[e\]vent
// mostly represent player actions

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CeRename {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CeIncreaseScore {
    #[prost(uint32, tag="1")]
    pub score: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientEvent {
    #[prost(oneof="client_event::CeType", tags="1, 2")]
    pub ce_type: ::core::option::Option<client_event::CeType>,
}
/// Nested message and enum types in `ClientEvent`.
pub mod client_event {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CeType {
        #[prost(message, tag="1")]
        Rename(super::CeRename),
        #[prost(message, tag="2")]
        IncreaseScore(super::CeIncreaseScore),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SeErrorType {
    Unknown = 0,
    AlreadyConnected = 1,
    FullGame = 2,
}
impl SeErrorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "UNKNOWN",
            Self::AlreadyConnected => "ALREADY_CONNECTED",
            Self::FullGame => "FULL_GAME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "ALREADY_CONNECTED" => Some(Self::AlreadyConnected),
            "FULL_GAME" => Some(Self::FullGame),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
