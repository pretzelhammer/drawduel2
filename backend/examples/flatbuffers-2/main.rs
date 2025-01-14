#![allow(unused)]

use std::collections::HashMap;
use flatbuffers::FlatBufferBuilder;
use serde::Serialize;
use typeshare::typeshare;

mod generated_rs;
use generated_rs::server_event::{root_as_events, Event, EventArgs, EventUnion, Events, EventsArgs, PlayerJoin, PlayerJoinArgs};

#[typeshare]
#[derive(Debug, Serialize)]
struct Player {
    id: u16,
    name: String,
    score: u16,
}

#[typeshare]
#[derive(Default, Debug, Serialize)]
struct GameState {
    players: HashMap<u16, Player>,
}

impl GameState {
    fn advance(&mut self, events: Events<'_>) {
        if let Some(events) = events.events() {
            for event in events {
                match event.event_type() {
                    EventUnion::PlayerJoin => {
                        if let Some(player_join) = event.event_as_player_join() {
                            let id = player_join.id();
                            if let Some(name) = player_join.name() {
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

// just player join for now
fn serialize_events() -> Vec<u8> {
    let mut builder = FlatBufferBuilder::with_capacity(1024);
    let pj1_id = 1;
    let pj1_name = builder.create_string("John");
    let pj1_args = PlayerJoinArgs {
        id: pj1_id,
        name: Some(pj1_name),
    };
    let player_joined_1 = PlayerJoin::create(
        &mut builder,
        &pj1_args,
    );
    let player_joined_1_union = player_joined_1.as_union_value();
    let event_args = EventArgs {
        event_type: EventUnion::PlayerJoin,
        event: Some(player_joined_1_union),
    };
    let event = Event::create(
        &mut builder,
        &event_args,
    );
    let events = builder.create_vector(&[event]);
    let event_list_args = EventsArgs {
        events: Some(events),
    };
    let event_list = Events::create(
        &mut builder,
        &event_list_args,
    );

    builder.finish(event_list, None);
    builder.finished_data().to_owned()
}

fn example_game() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = GameState::default();
    let bytes = serialize_events();
    dbg!(bytes.len(), &bytes);
    let events = root_as_events(&bytes)?;
    dbg!(&events);
    game.advance(events);
    dbg!(&game);
    let json_game = serde_json::to_string(&game).unwrap();
    dbg!(json_game.len(), &json_game);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    example_game();
    Ok(())
}