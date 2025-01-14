#![allow(unused)]

use std::collections::HashMap;

mod generated_rust;
use flatbuffers::FlatBufferBuilder;
use generated_rust::{root_as_event_list, Event, EventArgs, EventList, EventListArgs, EventUnion, PlayerJoined, PlayerJoinedArgs};
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
        if let Some(events) = event_list.events() {
            for event in events {
                match event.event_type() {
                    EventUnion::PlayerJoined => {
                        if let Some(player_joined) = event.event_as_player_joined() {
                            let id = player_joined.id();
                            if let Some(name) = player_joined.name() {
                                self.players.insert(id, Player {
                                    id,
                                    name: name.to_owned(),
                                    score: 0,
                                });
                            }
                        }

                    },
                    _ => unreachable!("unsupported event"),
                }
            }
        }
    }
}

fn serialize_player_joined() -> Vec<u8> {
    let mut builder = FlatBufferBuilder::with_capacity(1024);
    let pj1_id = 1;
    let pj1_name = builder.create_string("John");
    let pj1_args = PlayerJoinedArgs {
        id: pj1_id,
        name: Some(pj1_name),
    };
    let player_joined_1 = PlayerJoined::create(
        &mut builder,
        &pj1_args,
    );
    let player_joined_1_union = player_joined_1.as_union_value();
    let event_args = EventArgs {
        event_type: EventUnion::PlayerJoined,
        event: Some(player_joined_1_union),
    };
    let event = Event::create(
        &mut builder,
        &event_args,
    );
    let events = builder.create_vector(&[event]);
    let event_list_args = EventListArgs {
        events: Some(events),
    };
    let event_list = EventList::create(
        &mut builder,
        &event_list_args,
    );

    builder.finish(event_list, None);
    builder.finished_data().to_owned()
}

fn example_game() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = GameState::default();
    let bytes = serialize_player_joined();
    dbg!(&bytes);
    let event_list = root_as_event_list(&bytes)?;
    dbg!(&event_list);
    game.advance(event_list);
    dbg!(&game);
    dbg!(serde_json::to_string(&game));
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    example_game();
    Ok(())
}