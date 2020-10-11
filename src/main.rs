use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const WORDS_PATH: &'static str = "/usr/share/dict/words";

fn load_words(path: &Path) -> std::io::Result<Vec<String>> {
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents
        .lines()
        .map(|line| line.to_ascii_lowercase())
        .collect::<Vec<String>>())
}

fn is_candidate_constructible(
    prime_letter: char,
    available_letters: &Vec<char>,
    candidate: &String,
) -> bool {
    return candidate.len() >= 4
        && candidate.contains(prime_letter)
        && candidate.chars().all(|c| available_letters.contains(&c));
}

fn print_usage(path: &String) -> std::io::Result<()> {
    println!("Correct usage:");
    println!("  {} <8 letters>", path);
    println!("");
    println!("  Where the 1st letter is the prime letter");

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return print_usage(&args[0]);
    }

    for arg in &args[1..] {
        if arg.len() != 1 {
            return print_usage(&args[0]);
        }
    }

    let available_letters: Vec<char> = (&mut args[1..])
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>()[0])
        .collect();
    let prime_letter = available_letters[0];

    let words = load_words(Path::new(WORDS_PATH))?;
    for word in words {
        if is_candidate_constructible(prime_letter, &available_letters, &word) {
            println!("{}", &word)
        }
    }

    Ok(())
}
