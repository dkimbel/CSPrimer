use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Result, Write};

/// One-off script that:
/// - Filters out proper nouns from our starter word list
/// - Writes each word to a specific file based on its length (three, four, or five chars)
fn main() -> Result<()> {
    let all_words_file = File::open("resources/words.txt")?;

    let mut three_letter_words_file = create_file_if_not_exists("resources/three_letter_words.txt");
    let mut four_letter_words_file = create_file_if_not_exists("resources/four_letter_words.txt");
    let mut five_letter_words_file = create_file_if_not_exists("resources/five_letter_words.txt");

    let lines = BufReader::new(all_words_file).lines();
    for line in lines {
        let word = line.unwrap();
        if word.chars().next().unwrap().is_uppercase() {
            // no proper nouns allowed
            continue;
        }

        let num_chars = word.chars().count();
        let target_file = match num_chars {
            3 => &mut three_letter_words_file,
            4 => &mut four_letter_words_file,
            5 => &mut five_letter_words_file,
            _ => panic!("Found word with unexpected number of chars: {word}"),
        };

        writeln!(target_file, "{word}")?;
    }

    Ok(())
}

fn create_file_if_not_exists(file_path: &str) -> File {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_path)
        .expect(&format!("File already exists at {file_path}!"))
}
