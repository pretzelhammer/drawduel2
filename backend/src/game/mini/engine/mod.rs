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
    // true if no players, or all players disconnected
    pub fn empty(&self) -> bool {
        let connected_players = self.players.iter().fold(0, |acc, (_, p)| {
            if p.connected {
                acc + 1
            } else {
                acc
            }
        });
        connected_players == 0
    }
    pub fn advance(&mut self, event: ServerEvent) -> Option<ServerEvent> {
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
                Some(event)
            }
            SeType::PlayerLeave(player_leave) => {
                let player_id = player_leave.id;
                self.players.remove(&player_id).map(|_| event)
            }
            SeType::PlayerRename(player_rename) => {
                let player_id = player_rename.id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    player.name = player_rename.name.clone();
                    Some(event)
                } else {
                    None
                }
            }
            SeType::PlayerIncreaseScore(player_increase_score) => {
                let player_id = player_increase_score.id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    player.score += player_increase_score.score;
                    Some(event)
                } else {
                    None
                }
            }
            SeType::PlayerConnect(player_connect) => {
                let player_id = player_connect.id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    if !player.connected {
                        player.connected = true;
                        Some(event)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            SeType::PlayerDisconnect(player_disconnect) => {
                let player_id = player_disconnect.id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    if player.connected {
                        player.connected = false;
                        Some(event)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => {
                // if reached here means we can ignore the event
                // since it doesn't produce a state change
                None
            }
        }
    }
}
