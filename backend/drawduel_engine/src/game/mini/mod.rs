use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

mod generated;
pub use generated::client_event::CeType;
pub use generated::draw_op::DoType;
pub use generated::guess::GuessType;
pub use generated::hint::HintType;
pub use generated::server_event::SeType;
pub use generated::*;

mod words;
pub use words::*;

pub type RoundId = u32;
pub type PlayerId = u32;
pub type Score = u32;
pub type EpochMs = u64;
pub type WordIdx = u32;

// scoring
pub const BASE_SCORE: u32 = 120;
pub const EASY_MULTIPLER: u32 = 2;
pub const HARD_MULTIPLER: u32 = 3;

// timing
pub const CHOOSE_WORD_DURATION: Duration = Duration::from_secs(10);
pub const PRE_PLAY_DURATION: Duration = Duration::from_secs(5);
pub const PLAY_EASY_DURATION: Duration = Duration::from_secs(45);
pub const PLAY_HARD_DURATION: Duration = Duration::from_secs(60);
pub const POST_PLAY_DURATION: Duration = Duration::from_secs(5);

pub fn epoch_ms_from_now(duration: Duration) -> EpochMs {
    let now = SystemTime::now();
    let future_time = now + duration;
    future_time
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards!?")
        // truncating u128 to u64 is safe because the sun
        // will explode before u64 overflows
        .as_millis() as EpochMs
}

pub struct TimedEvent {
    // event is only valid in this round
    pub target_round_id: RoundId,
    // event is only valid in this phase
    pub target_phase: Phase,
    // type of event
    pub timed_event_type: TimedEventType,
    // ms since epoch
    pub times_out_at: EpochMs,
}

pub enum TimedEventType {
    PrePlayPhaseOver,
    ChooseWordPhaseOver,
    // drawer doesn't draw anything for
    // 10 secs after play phase start
    InactiveDrawer,
    PlayPhaseOver,
    PostPlayPhaseOver,
    GiveHint,
}

impl Round {
    pub fn new(
        round_id: RoundId,
        drawer_id: PlayerId,
        easy_word: WordIdx,
        hard_word: WordIdx,
    ) -> Self {
        Round {
            round_id,
            phase: Phase::ChooseWord.into(),
            drawer_id: drawer_id,
            draw_ops: Vec::with_capacity(128),
            easy_word: easy_word,
            hard_word: hard_word,
            word_choice: WordChoice::Easy.into(),
            draw_score: 0,
            guess_score: 0,
            guesses: Vec::new(),
            hints: Vec::new(),
            phase_ends_at: 0,
        }
    }
    pub fn next(
        &mut self,
        round_id: RoundId,
        drawer_id: PlayerId,
        easy_word: WordIdx,
        hard_word: WordIdx,
    ) {
        self.round_id = round_id;
        self.phase = Phase::ChooseWord.into();
        self.drawer_id = drawer_id;
        self.draw_ops.clear();
        self.easy_word = easy_word;
        self.hard_word = hard_word;
        self.word_choice = WordChoice::Easy.into();
        self.draw_score = 0;
        self.guess_score = 0;
        self.guesses.clear();
        self.hints.clear();
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            round: None,
        }
    }
    pub fn reset(&mut self) {
        self.players.clear();
        self.round = None;
    }
    // true if no players, or all players disconnected
    pub fn is_empty(&self) -> bool {
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
    // true if all connected players have scored this round
    pub fn all_connected_players_scored(&self) -> bool {
        self.players
            .iter()
            .all(|(_, p)| !p.connected || p.round_score > 0)
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
                        player_join.player_id,
                        Player {
                            name: player_join.name.clone(),
                            round_score: 0,
                            draw_score: 0,
                            guess_score: 0,
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
                let player_id = player_leave.player_id;
                if self.players.remove(&player_id).is_some() {
                    send_buf.push(event);
                }
            }
            SeType::PlayerRename(player_rename) => {
                let player_id = player_rename.player_id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    player.name = player_rename.name.clone();
                    send_buf.push(event);
                }
            }
            SeType::PlayerIncRoundScore(inc_score) => {
                let player_id = inc_score.player_id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    player.round_score += inc_score.inc_by;
                    send_buf.push(event);
                }
            }
            SeType::PlayerIncDrawScore(inc_score) => {
                let drawer_id = inc_score.drawer_id;
                if let Some(player) = self.players.get_mut(&drawer_id) {
                    player.draw_score += inc_score.inc_by;
                    send_buf.push(event);
                }
            }
            SeType::PlayerIncGuessScore(inc_score) => {
                let guesser_id = inc_score.guesser_id;
                if let Some(player) = self.players.get_mut(&guesser_id) {
                    player.guess_score += inc_score.inc_by;
                    send_buf.push(event);
                }
            }
            SeType::PlayerConnect(player_connect) => {
                let player_id = player_connect.player_id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    if !player.connected {
                        player.connected = true;
                        send_buf.push(event);
                    }
                }
            }
            SeType::PlayerDisconnect(player_disconnect) => {
                let player_id = player_disconnect.player_id;
                if let Some(player) = self.players.get_mut(&player_id) {
                    if player.connected {
                        player.connected = false;
                        send_buf.push(event);
                    }
                }
            }
            SeType::PlayerDrawOp(player_draw_op) => {
                if let Some(round) = &mut self.round {
                    if round.phase() == Phase::Play {
                        if round.drawer_id == player_draw_op.drawer_id {
                            if let Some(draw_op) =
                                player_draw_op.draw_op.clone()
                            {
                                round.draw_ops.push(draw_op);
                                send_buf.push(event);
                            }
                        }
                    }
                }
            }
            SeType::PlayerChooseWord(choose_word) => {
                if let Some(round) = &mut self.round {
                    if round.phase() == Phase::ChooseWord {
                        if round.drawer_id == choose_word.drawer_id {
                            if round.word_choice != choose_word.choice {
                                round.word_choice = choose_word.choice;
                                send_buf.push(event);
                            }
                            let phase_duration =
                                if round.word_choice() == WordChoice::Easy {
                                    PLAY_EASY_DURATION
                                } else {
                                    PLAY_HARD_DURATION
                                };
                            let ends_at_ms = epoch_ms_from_now(phase_duration);
                            send_buf.push(ServerEvent {
                                se_type: Some(SeType::RoundChangePhase(
                                    SeRoundChangePhase {
                                        phase: Phase::PrePlay.into(),
                                        phase_ends_at: ends_at_ms,
                                    },
                                )),
                            });
                        }
                    }
                }
            }
            SeType::PlayerGuessWord(guess_word) => {
                if let Some(round) = &mut self.round {
                    if round.phase() != Phase::Play {
                        return;
                    }
                    // don't need to check if guesser is in
                    // players map because clients cannot
                    // effect this field, it's only set on
                    // the server
                    if round.drawer_id != guess_word.guesser_id {
                        // check if it's correct
                        let (is_correct, multipler, phase_duration) =
                            if round.word_choice == WordChoice::Hard.into() {
                                (
                                    HARD_WORDS[round.hard_word as usize]
                                        == guess_word.guess,
                                    HARD_MULTIPLER,
                                    PLAY_HARD_DURATION,
                                )
                            } else {
                                (
                                    EASY_WORDS[round.easy_word as usize]
                                        == guess_word.guess,
                                    EASY_MULTIPLER,
                                    PLAY_EASY_DURATION,
                                )
                            };

                        let guess_type = if is_correct {
                            GuessType::CorrectGuess(CorrectGuess {})
                        } else {
                            GuessType::IncorrectGuess(IncorrectGuess {
                                guess: guess_word.guess.clone(),
                            })
                        };
                        // add guess to guesses
                        let guesser_id = guess_word.guesser_id;
                        round.guesses.push(Guess {
                            guesser_id: guess_word.guesser_id,
                            guess_type: Some(guess_type),
                            after_draw_ops: guess_word.after_draw_ops,
                        });
                        send_buf.push(event);
                        // let guess = &round.guesses.last().unwrap().guess;

                        // if not then return
                        if !is_correct {
                            return;
                        }

                        // if correct then calculate score
                        let first_correct_bonus =
                            if round.guess_score == 0 { 10 } else { 0 };

                        let hints_gotten = round.hints.len() as u32;

                        let now = SystemTime::now();
                        let phase_started_at = UNIX_EPOCH
                            + Duration::from_millis(round.phase_ends_at)
                            - phase_duration;
                        let secs_elapsed: u32 = if let Ok(duration) =
                            now.duration_since(phase_started_at)
                        {
                            // truncates u64
                            duration.as_secs() as u32
                        } else {
                            0
                        };

                        let inc_score = ((first_correct_bonus + BASE_SCORE)
                            - (secs_elapsed + hints_gotten))
                            * multipler;

                        // score correct guess
                        if let Some(player) = self.players.get_mut(&guesser_id)
                        {
                            player.guess_score += inc_score;
                            send_buf.push(ServerEvent {
                                se_type: Some(SeType::PlayerIncRoundScore(
                                    SePlayerIncRoundScore {
                                        player_id: guesser_id,
                                        inc_by: inc_score,
                                    },
                                )),
                            });
                        }

                        round.guess_score += inc_score;
                        send_buf.push(ServerEvent {
                            se_type: Some(SeType::RoundIncGuessScore(
                                SeRoundIncGuessScore { inc_by: inc_score },
                            )),
                        });

                        // score drawing for 1st correct guess
                        if first_correct_bonus > 0 {
                            if let Some(player) =
                                self.players.get_mut(&round.drawer_id)
                            {
                                player.draw_score += inc_score;
                                send_buf.push(ServerEvent {
                                    se_type: Some(SeType::PlayerIncRoundScore(
                                        SePlayerIncRoundScore {
                                            player_id: round.drawer_id,
                                            inc_by: inc_score,
                                        },
                                    )),
                                });
                            }

                            round.draw_score += inc_score;
                            send_buf.push(ServerEvent {
                                se_type: Some(SeType::RoundIncDrawScore(
                                    SeRoundIncDrawScore { inc_by: inc_score },
                                )),
                            });
                        }

                        if self.all_connected_players_scored() {
                            send_buf.push(ServerEvent {
                                se_type: Some(SeType::RoundChangePhase(
                                    SeRoundChangePhase {
                                        phase: Phase::PostPlay.into(),
                                        phase_ends_at: epoch_ms_from_now(
                                            POST_PLAY_DURATION,
                                        ),
                                    },
                                )),
                            });
                        }

                        // TODO: as currently implemented, this
                        // assumes every guess is incorrect, however
                        // if we get a correct guess we must:
                        // - inc guesser's score & guess_score
                        // - inc round's guess_score
                        // - if 1st correct guess also:
                        //   - inc drawer's score & draw_score
                        //   - inc round's draw_score
                        // - check if all connected guessers have correct guesses:
                        //   - if so, switch to post-play phase
                        // how to calc score:
                        // base = 120
                        // guesser: (firstCorrectBonus = 10 for 1st correct guesser)
                        // ((firstCorrectBonus + base) - (floor(roundTimeElapsedSecs) + len(hints) * 10)) * difficulty multipler
                        // drawer (only for first guess):
                        // ((10 + base) - (floor(roundTimeElapsedSecs) + len(hints) * 10)) * difficulty multipler
                    }
                }
            }
            // only server can generate this event
            SeType::NewRound(new_round) => {
                // not efficient impl but this
                // only runs during testing,
                // shouldn't ever run on prod
                if let Some(round) = &mut self.round {
                    if new_round.round_id != round.round_id {
                        round.next(
                            new_round.round_id,
                            new_round.drawer_id,
                            new_round.easy_word,
                            new_round.hard_word,
                        );
                        send_buf.push(event);
                    }
                }
            }
            // only server can generate this event
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
            // only server can create this event
            SeType::RoundIncDrawScore(inc_score) => {
                if let Some(round) = &mut self.round {
                    round.draw_score += inc_score.inc_by;
                    send_buf.push(event);
                }
            }
            // only server can create this event
            SeType::RoundIncGuessScore(inc_score) => {
                if let Some(round) = &mut self.round {
                    round.guess_score += inc_score.inc_by;
                    send_buf.push(event);
                }
            }
            // only server can create this event
            SeType::RoundChangePhase(round_change_phase) => {
                if let Some(round) = &mut self.round {
                    round.phase = round_change_phase.phase;
                    send_buf.push(event);
                }
            }
        }
    }
    pub fn timed_advance(
        &mut self,
        event: TimedEvent,
        send_buf: &mut Vec<ServerEvent>,
        timer_buf: &mut Vec<TimedEvent>,
    ) {
        if self.round.is_none() {
            return;
        }
        let round = self.round.as_mut().unwrap();
        if round.round_id != event.target_round_id {
            return;
        }
        if round.phase != event.target_phase.into() {
            return;
        }
        match event.timed_event_type {
            TimedEventType::PrePlayPhaseOver => {
                let play_duration = if round.word_choice() == WordChoice::Hard {
                    PLAY_HARD_DURATION
                } else {
                    PLAY_EASY_DURATION
                };
                let play_phase_over_at =
                    event.times_out_at + play_duration.as_millis() as u64;
                round.phase = Phase::Play.into();
                round.phase_ends_at = play_phase_over_at;
                send_buf.push(ServerEvent {
                    se_type: Some(SeType::RoundChangePhase(
                        SeRoundChangePhase {
                            phase: Phase::Play.into(),
                            phase_ends_at: play_phase_over_at,
                        },
                    )),
                });
                timer_buf.push(TimedEvent {
                    target_round_id: round.round_id,
                    target_phase: Phase::Play,
                    timed_event_type: TimedEventType::PlayPhaseOver,
                    times_out_at: play_phase_over_at,
                });
            }
            TimedEventType::ChooseWordPhaseOver => {
                let pre_play_phase_over_at =
                    event.times_out_at + PRE_PLAY_DURATION.as_millis() as u64;
                round.phase = Phase::PrePlay.into();
                round.phase_ends_at = pre_play_phase_over_at;
                send_buf.push(ServerEvent {
                    se_type: Some(SeType::RoundChangePhase(
                        SeRoundChangePhase {
                            phase: Phase::PrePlay.into(),
                            phase_ends_at: pre_play_phase_over_at,
                        },
                    )),
                });
                timer_buf.push(TimedEvent {
                    target_round_id: round.round_id,
                    target_phase: Phase::PrePlay,
                    timed_event_type: TimedEventType::PrePlayPhaseOver,
                    times_out_at: pre_play_phase_over_at,
                });
            }
            TimedEventType::InactiveDrawer => todo!(),
            TimedEventType::PlayPhaseOver => {
                let post_play_phase_over_at =
                    event.times_out_at + POST_PLAY_DURATION.as_millis() as u64;
                round.phase = Phase::PostPlay.into();
                round.phase_ends_at = post_play_phase_over_at;
                send_buf.push(ServerEvent {
                    se_type: Some(SeType::RoundChangePhase(
                        SeRoundChangePhase {
                            phase: Phase::PostPlay.into(),
                            phase_ends_at: post_play_phase_over_at,
                        },
                    )),
                });
                timer_buf.push(TimedEvent {
                    target_round_id: round.round_id,
                    target_phase: Phase::PostPlay,
                    timed_event_type: TimedEventType::PostPlayPhaseOver,
                    times_out_at: post_play_phase_over_at,
                });
            }
            TimedEventType::PostPlayPhaseOver => {
                let post_play_phase_over_at =
                    event.times_out_at + POST_PLAY_DURATION.as_millis() as u64;
                round.phase = Phase::PostPlay.into();
                round.phase_ends_at = post_play_phase_over_at;
                send_buf.push(ServerEvent {
                    se_type: Some(SeType::RoundChangePhase(
                        SeRoundChangePhase {
                            phase: Phase::PostPlay.into(),
                            phase_ends_at: post_play_phase_over_at,
                        },
                    )),
                });
                timer_buf.push(TimedEvent {
                    target_round_id: round.round_id,
                    target_phase: Phase::PostPlay,
                    timed_event_type: TimedEventType::PostPlayPhaseOver,
                    times_out_at: post_play_phase_over_at,
                });
            }
            TimedEventType::GiveHint => todo!(),
        }
    }
}

impl ServerEvent {
    pub fn from_client(player_id: PlayerId, client_event: ClientEvent) -> Self {
        match client_event.ce_type.unwrap() {
            CeType::Rename(rename) => ServerEvent {
                se_type: Some(SeType::PlayerRename(SePlayerRename {
                    player_id: player_id,
                    name: rename.name,
                })),
            },
            CeType::DrawOp(draw_op) => ServerEvent {
                se_type: Some(SeType::PlayerDrawOp(SePlayerDrawOp {
                    drawer_id: player_id,
                    draw_op: draw_op.draw_op,
                })),
            },
            CeType::ChooseWord(choose_word) => ServerEvent {
                se_type: Some(SeType::PlayerChooseWord(SePlayerChooseWord {
                    drawer_id: player_id,
                    choice: choose_word.choice,
                })),
            },
            CeType::GuessWord(guess_word) => ServerEvent {
                se_type: Some(SeType::PlayerGuessWord(SePlayerGuessWord {
                    guesser_id: player_id,
                    guess: guess_word.guess,
                    after_draw_ops: guess_word.after_draw_ops,
                })),
            },
            CeType::LikeRound(like_round) => ServerEvent {
                se_type: Some(SeType::PlayerLikeRound(SePlayerLikeRound {
                    player_id: player_id,
                    round_id: like_round.round_id,
                })),
            },
        }
    }
}

// Do not put tests here, put them in
// frontend/tests/mini-game/quick/engine.test.ts
