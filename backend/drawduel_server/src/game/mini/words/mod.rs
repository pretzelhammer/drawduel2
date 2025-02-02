use drawduel_engine::game::mini::{EASY_WORDS, HARD_WORDS};

pub fn random_easy_word() -> &'static str {
    fastrand::choice(EASY_WORDS).unwrap()
}

pub fn random_hard_word() -> &'static str {
    fastrand::choice(HARD_WORDS).unwrap()
}
