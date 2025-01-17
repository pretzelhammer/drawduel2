use std::collections::HashMap;

mod generated;
pub use generated::client_event::CeType;
pub use generated::server_event::SeType;
pub use generated::*;

impl Game {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }
    pub fn reset(&mut self) {
        self.players.clear();
    }
    // true if no players, or all players disconnected
    pub fn empty(&self) -> bool {
        self.connected_players() == 0
    }
    pub fn connected_players(&self) -> usize {
        self.players.iter().fold(
            0,
            |acc, (_, p)| {
                if p.connected {
                    acc + 1
                } else {
                    acc
                }
            },
        )
    }
    pub fn advance_all(
        &mut self,
        events: ServerEvents,
        send_buf: &mut Vec<ServerEvent>,
    ) {
        for event in events.events {
            self.advance(event, send_buf);
        }
    }
    pub fn advance(
        &mut self,
        event: ServerEvent,
        send_buf: &mut Vec<ServerEvent>,
    ) {
        match event.se_type.as_ref().unwrap() {
            SeType::PlayerJoin(player_join) => {
                let overwrote_existing_player = self
                    .players
                    .insert(
                        player_join.id,
                        Player {
                            name: player_join.name.clone(),
                            score: 0,
                            connected: true,
                        },
                    )
                    .is_some();
                if overwrote_existing_player {
                    panic!(
                        "overwrote existing player, this should never happen!"
                    );
                }
                send_buf.push(event);
            }
            SeType::PlayerLeave(player_leave) => {
                let player_id = player_leave.id;
                if self.players.remove(&player_id).is_some() {
                    send_buf.push(event);
                }
            }
            SeType::PlayerRename(player_rename) => {
                let player_id = player_rename.id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    player.name = player_rename.name.clone();
                    send_buf.push(event);
                }
            }
            SeType::PlayerIncreaseScore(player_increase_score) => {
                let player_id = player_increase_score.id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    player.score += player_increase_score.score;
                    send_buf.push(event);
                }
            }
            SeType::PlayerConnect(player_connect) => {
                let player_id = player_connect.id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    if !player.connected {
                        player.connected = true;
                        send_buf.push(event);
                    }
                }
            }
            SeType::PlayerDisconnect(player_disconnect) => {
                let player_id = player_disconnect.id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    if player.connected {
                        player.connected = false;
                        send_buf.push(event);
                    }
                }
            }
            _ => {
                // if reached here means we can ignore the event
                // since it doesn't produce a state change
            }
        }
    }
}
