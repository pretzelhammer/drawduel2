use std::collections::HashMap;

mod generated;
pub use generated::client_event::CeType;
pub use generated::draw_operation::DoType;
pub use generated::hint::HintType;
pub use generated::server_event::SeType;
pub use generated::*;

type PlayerId = u32;

impl Round {
    pub fn new(
        round_id: u32,
        drawer_id: u32,
        easy_word: &'static str,
        hard_word: &'static str,
    ) -> Self {
        Round {
            round_id,
            phase: Phase::ChooseWord.into(),
            drawer: drawer_id,
            drawing: Vec::with_capacity(128),
            easy_word: easy_word.into(),
            hard_word: hard_word.into(),
            word_choice: WordChoice::Easy.into(),
            drawing_score: 0,
            guessing_score: 0,
            guesses: Vec::new(),
            hints: Vec::new(),
        }
    }
    pub fn reset(&mut self, easy_word: &str, hard_word: &str) {
        self.round_id = 0;
        self.phase = Phase::ChooseWord.into();
        self.drawer = 0;
        self.drawing.clear();
        self.easy_word.clear();
        self.easy_word.push_str(easy_word);
        self.hard_word.clear();
        self.hard_word.push_str(hard_word);
        self.word_choice = WordChoice::Easy.into();
        self.drawing_score = 0;
        self.guessing_score = 0;
        self.guesses.clear();
        self.hints.clear();
    }
}

impl Game {
    pub fn new(easy_word: &'static str, hard_word: &'static str) -> Self {
        Self {
            players: HashMap::new(),
            round: Some(Round::new(0, 0, easy_word, hard_word)),
        }
    }
    pub fn reset(&mut self, easy_word: &'static str, hard_word: &'static str) {
        self.players.clear();
        self.round.as_mut().unwrap().reset(easy_word, hard_word);
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
                            drawer_score: 0,
                            guesser_score: 0,
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
            SeType::PlayerIncreaseScore(increase_score) => {
                let player_id = increase_score.id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    player.score += increase_score.increase_by;
                    send_buf.push(event);
                }
            }
            SeType::PlayerIncreaseDrawerScore(increase_score) => {
                let player_id = increase_score.id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    player.drawer_score += increase_score.increase_by;
                    send_buf.push(event);
                }
            }
            SeType::PlayerIncreaseGuesserScore(increase_score) => {
                let player_id = increase_score.id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    player.guesser_score += increase_score.increase_by;
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
            SeType::PlayerDrawOp(player_draw_op) => {
                if let Some(round) = &mut self.round {
                    if round.drawer == player_draw_op.id {
                        if let Some(draw_op) = player_draw_op.draw_op.clone() {
                            round.drawing.push(draw_op);
                            send_buf.push(event);
                        }
                    }
                }
            }
            SeType::PlayerChooseWord(choose_word) => {
                let round = self.round.as_mut().unwrap();
                if round.word_choice != choose_word.choice {
                    round.word_choice = choose_word.choice;
                    send_buf.push(event);
                }
            }
            SeType::PlayerGuessWord(guess_word) => {
                let round = self.round.as_mut().unwrap();
                round.guesses.push(Guess {
                    guesser: guess_word.guesser,
                    guess: guess_word.guess.clone(),
                    after_draw_ops: guess_word.after_draw_ops,
                });
                send_buf.push(event);
            }
            SeType::NewRound(new_round) => {
                // not efficient impl but this
                // only runs during testing,
                // shouldn't ever run on prod
                if let Some(round) = &mut self.round {
                    round.reset(&new_round.easy_word, &new_round.hard_word);
                    round.round_id = new_round.round_id;
                    round.drawer = new_round.drawer;
                    send_buf.push(event);
                }
            }
            SeType::SetGame(set_game) => {
                // not efficient impl but this
                // only runs during testing,
                // shouldn't ever run on prod
                *self = set_game.game.as_ref().unwrap().clone();
                send_buf.push(event);
            }
            SeType::PlayerLikeRound(_like_round) => {
                // no-op, but pass thru
                send_buf.push(event);
            }
            SeType::Error(_error) => {
                // no-op
            }
        }
    }
}

impl ServerEvent {
    pub fn from_client(player_id: PlayerId, client_event: ClientEvent) -> Self {
        match client_event.ce_type.unwrap() {
            CeType::Rename(rename) => ServerEvent {
                se_type: Some(SeType::PlayerRename(SePlayerRename {
                    id: player_id,
                    name: rename.name,
                })),
            },
            CeType::DrawOp(draw_op) => ServerEvent {
                se_type: Some(SeType::PlayerDrawOp(SePlayerDrawOperation {
                    id: player_id,
                    draw_op: draw_op.draw_op,
                })),
            },
            CeType::ChooseWord(choose_word) => ServerEvent {
                se_type: Some(SeType::PlayerChooseWord(SePlayerChooseWord {
                    drawer: player_id,
                    choice: choose_word.choice,
                })),
            },
            CeType::GuessWord(guess_word) => ServerEvent {
                se_type: Some(SeType::PlayerGuessWord(SePlayerGuessWord {
                    guesser: player_id,
                    guess: guess_word.guess,
                    after_draw_ops: guess_word.after_draw_ops,
                })),
            },
            CeType::LikeRound(like_round) => ServerEvent {
                se_type: Some(SeType::PlayerLikeRound(SePlayerLikeRound {
                    liker: player_id,
                    round_id: like_round.round_id,
                })),
            },
        }
    }
}

// Do not put tests here, put them in
// frontend/tests/mini-game/quick/engine.test.ts
