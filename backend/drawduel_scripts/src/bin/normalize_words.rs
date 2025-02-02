use std::{
    collections::HashSet,
    env,
    fs::{self, OpenOptions},
    io::Write,
};

#[derive(PartialEq)]
enum Category {
    EASY,
    HARD,
    SKIP,
    NONE,
}

const WORDS_PATH: &'static str = "../../agnostic/words.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("cwd: {:?}", env::current_dir()?);
    let input = fs::read_to_string(WORDS_PATH)?;
    // println!("input");
    // print!("{input}");
    let mut easy = HashSet::new();
    let mut hard = HashSet::new();
    let mut skip = HashSet::new();
    let mut category = Category::NONE;
    for line in input.lines() {
        // trim leading and trailing whitespace
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
        }
        // trim inner whitespace and lowercase words
        let normalized = trimmed
            .split_ascii_whitespace()
            .map(|word| word.to_lowercase())
            .collect::<Vec<_>>()
            .join(" ");

        // de-dup words
        match category {
            Category::EASY => easy.insert(normalized),
            Category::HARD => hard.insert(normalized),
            Category::SKIP => skip.insert(normalized),
            Category::NONE => unreachable!("unreachable"),
        };
    }
    let mut sorted_easy: Vec<_> = easy.into_iter().collect();
    sorted_easy.sort();
    let mut sorted_hard: Vec<_> = hard.into_iter().collect();
    sorted_hard.sort();
    let mut sorted_skip: Vec<_> = skip.into_iter().collect();
    sorted_skip.sort();
    let mut output = String::with_capacity(4096);
    output.push_str("EASY\n");
    for easy in sorted_easy {
        output.push_str(&easy);
        output.push('\n');
    }
    output.push_str("\nHARD\n");
    for hard in sorted_hard {
        output.push_str(&hard);
        output.push('\n');
    }
    output.push_str("\nSKIP\n");
    for skip in sorted_skip {
        output.push_str(&skip);
        output.push('\n');
    }
    // println!("normalized");
    // print!("{output}");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(WORDS_PATH)?;
    file.write_all(output.as_bytes())?;
    Ok(())
}
