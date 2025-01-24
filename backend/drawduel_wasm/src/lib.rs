use drawduel_engine::game::mini::{
    ClientEvent, Game, ServerEvent, ServerEvents,
};
use prost::Message as ProstMessage;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct SerializedNextState {
    pub apply_events: SerializedEvents,
    pub next_game: SerializedGame,
}

struct NextState {
    pub apply_events: ServerEvents,
    pub next_game: Game,
}

type SerializedEvents = Vec<u8>;
type SerializedGame = Vec<u8>;

#[wasm_bindgen]
pub fn client_advance(
    player_id: u32,
    client_msg: &[u8],
    current_game: &[u8],
) -> Result<JsValue, JsError> {
    let client_event = ClientEvent::decode(client_msg)?;
    let server_event = ServerEvent::from_client(player_id, client_event);
    wasm_advance(server_event, current_game)
}

#[wasm_bindgen]
pub fn server_advance(
    server_msg: &[u8],
    current_game: &[u8],
) -> Result<JsValue, JsError> {
    let server_event = ServerEvent::decode(server_msg)?;
    wasm_advance(server_event, current_game)
}

fn wasm_advance(
    server_event: ServerEvent,
    current_game: &[u8],
) -> Result<JsValue, JsError> {
    let game = Game::decode(current_game)?;
    match advance(server_event, game)? {
        Some(next_state) => {
            let mut serialized_events = Vec::new();
            next_state.apply_events.encode(&mut serialized_events)?;
            let mut serialized_game = Vec::new();
            next_state.next_game.encode(&mut serialized_game)?;
            let serialized_next_state = SerializedNextState {
                apply_events: serialized_events,
                next_game: serialized_game,
            };
            Ok(serde_wasm_bindgen::to_value(&serialized_next_state)?)
        }
        None => Ok(JsValue::undefined()),
    }
}

fn advance(
    server_event: ServerEvent,
    mut game: Game,
) -> Result<Option<NextState>, JsError> {
    let mut events = Vec::new();
    game.advance(server_event, &mut events);
    if events.is_empty() {
        return Ok(None);
    }
    let server_events = ServerEvents { events };
    Ok(Some(NextState {
        apply_events: server_events,
        next_game: game,
    }))
}

// Do not put tests here, put them in
// frontend/tests/mini-game/quick/engine.test.ts
