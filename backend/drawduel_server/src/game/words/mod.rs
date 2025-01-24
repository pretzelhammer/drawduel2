mod generated;

pub fn random_easy_word() -> &'static str {
    fastrand::choice(generated::EASY).unwrap()
}

pub fn random_hard_word() -> &'static str {
    fastrand::choice(generated::HARD).unwrap()
}
