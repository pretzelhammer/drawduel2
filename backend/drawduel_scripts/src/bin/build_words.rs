use std::{
    fmt::Write as fmtWrite,
    fs::{self, OpenOptions},
    io::Write as IoWrite,
};

#[derive(PartialEq)]
enum Category {
    EASY,
    HARD,
    SKIP,
    NONE,
}

const WORDS_PATH: &'static str = "../../agnostic/words.txt";
const GENERATED_RS_PATH: &'static str =
    "../drawduel_engine/src/game/mini/words/generated.rs";
const GENERATED_TS_PATH: &'static str =
    "../../frontend/src/game/mini/words/index.ts";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("cwd: {:?}", env::current_dir()?);
    let input = fs::read_to_string(WORDS_PATH)?;
    // println!("input");
    // print!("{input}");
    let mut easy = Vec::new();
    let mut hard = Vec::new();
    let mut category = Category::NONE;
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "EASY" {
            category = Category::EASY;
            continue;
        } else if trimmed == "HARD" {
            category = Category::HARD;
            continue;
        } else if trimmed == "SKIP" {
            category = Category::SKIP;
            continue;
        }
        if category == Category::NONE {
            continue;
        } else if category == Category::SKIP {
            continue;
        }
        match category {
            Category::EASY => easy.push(trimmed),
            Category::HARD => hard.push(trimmed),
            _ => unreachable!("unreachable"),
        };
    }
    fastrand::shuffle(&mut easy);
    fastrand::shuffle(&mut hard);

    let easy_len = easy.len();
    let hard_len = hard.len();

    let mut output_rs = String::with_capacity(8192);
    let mut output_ts = String::with_capacity(8192);

    // write easy words
    write!(
        &mut output_rs,
        "pub const EASY_WORDS: [&'static str; {easy_len}] = [\n"
    )?;
    write!(&mut output_ts, "export const easyWords = [\n")?;
    for word in easy {
        write!(&mut output_rs, "    \"{word}\",\n")?;
        write!(&mut output_ts, "    '{word}',\n")?;
    }
    write!(&mut output_rs, "];\n\n")?;
    write!(&mut output_ts, "];\n\n")?;

    // write hard words
    write!(
        &mut output_rs,
        "pub const HARD_WORDS: [&'static str; {hard_len}] = [\n"
    )?;
    write!(&mut output_ts, "export const hardWords = [\n")?;
    for word in hard {
        write!(&mut output_rs, "    \"{word}\",\n")?;
        write!(&mut output_ts, "    '{word}',\n")?;
    }
    write!(&mut output_rs, "];\n")?;
    write!(&mut output_ts, "];\n")?;

    // println!("normalized");
    // print!("{output_rs}");

    let mut rs_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(GENERATED_RS_PATH)?;
    rs_file.write_all(output_rs.as_bytes())?;

    let mut ts_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(GENERATED_TS_PATH)?;
    ts_file.write_all(output_ts.as_bytes())?;

    Ok(())
}
