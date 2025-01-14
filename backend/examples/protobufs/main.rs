#![allow(unused)]

use std::collections::HashMap;

mod generated_rust;
use flatbuffers::FlatBufferBuilder;
use generated_rust::{Event, EventList, PlayerJoined, event::Event as EventType};
use prost::Message;
use serde::Serialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize)]
struct Player {
    id: u32,
    name: String,
    score: u32,
}

#[typeshare]
#[derive(Default, Debug, Serialize)]
struct GameState {
    players: HashMap<u32, Player>,
}

impl GameState {
    fn advance(&mut self, event_list: EventList) {
        for event in event_list.events {
            if let Some(event_type) = event.event {
                match event_type {
                    EventType::PlayerJoined(player_joined) => {
                        self.players.insert(
                            player_joined.id,
                            Player {
                                id: player_joined.id,
                                name: player_joined.name,
                                score: 0,
                            },
                        );
                    },
                    EventType::PlayerLeft(player_left) => todo!(),
                    EventType::PlayerRename(player_rename) => todo!(),
                    EventType::PlayerIncreaseScore(player_increase_score) => todo!(),
                }
            }
        }
    }
}

fn serialize_player_joined() -> Vec<u8> {
    let list = EventList {
        events: vec![
            Event {
                event: Some(
                    EventType::PlayerJoined(
                        PlayerJoined {
                            id: 0,
                            name: "John".into(),
                        }
                    )
                )
            }
        ]
    };
    list.encode_to_vec()
}

fn example_game() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = GameState::default();
    let bytes = serialize_player_joined();
    dbg!(bytes.len(), &bytes);
    let event_list = EventList::decode(&*bytes)?;
    dbg!(&event_list);
    game.advance(event_list);
    dbg!(&game);
    let json_game = serde_json::to_string(&game).unwrap();
    dbg!(json_game.len(), &json_game);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    example_game();
    Ok(())
}