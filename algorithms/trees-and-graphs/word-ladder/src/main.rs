use colored::Colorize;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::process;

const THREE_LETTER_WORDS: &str = include_str!("../resources/three_letter_words.txt");
const FOUR_LETTER_WORDS: &str = include_str!("../resources/four_letter_words.txt");
const FIVE_LETTER_WORDS: &str = include_str!("../resources/five_letter_words.txt");

const USAGE_INSTRUCTIONS: &str = "Please provide two words as arguments. The words should have the same length, between 3 and 5 characters each.";

fn main() {
    let (start_word, end_word) = validate_args(env::args());
    let wildcard_word_lookup = make_wildcard_word_lookup(start_word.len());

    if let Some(shortest_path) = find_shortest_path(&start_word, &end_word, wildcard_word_lookup) {
        report_success(&start_word, &end_word, &shortest_path);
    } else {
        report_failure(&start_word, &end_word);
    }
}

/// Use breadth-first search to find the shortest path between the start and end word. If no
/// path can be found, return None. Otherwise return Some(path), where path is a list of all
/// visited words (including start and end).
/// This search is powered by our "wildcard words". If our currently-visited word is "tea",
/// we'll transform that into ["*ea", "t*a", "te*"], then look up all words that matched
/// those wildcards. We'll push all those words onto our queue, to be visited later.
/// This function also keeps track of words that have been visited already, and refuses to
/// visit the same word twice -- not only within a single path, but globally for the entire
/// search. Since we're going breadth-first, if ANY of our searches have already included a
/// word, it cannot be worth revisiting.
fn find_shortest_path<'a, 'b>(
    start_word: &'a str,
    end_word: &'a str,
    word_wildcard_lookups: HashMap<String, Vec<&'b str>>,
) -> Option<Vec<&'b str>>
where
    'a: 'b,
{
    let mut queue: VecDeque<Vec<&str>> = VecDeque::from([vec![start_word]]);
    let mut visited: HashSet<&str> = HashSet::from([start_word]);

    while let Some(path) = queue.pop_front() {
        let word = path.last().unwrap();

        for wildcard_word in make_wildcard_words(word) {
            for adjacent_word in word_wildcard_lookups.get(&wildcard_word).unwrap() {
                if !visited.contains(adjacent_word) {
                    let mut next_path = path.clone();
                    next_path.push(adjacent_word);
                    if *adjacent_word == end_word {
                        return Some(next_path);
                    }
                    queue.push_back(next_path);
                    visited.insert(*adjacent_word);
                }
            }
        }
    }

    None
}

/// Construct a list of "wildcard words" that would match the given word.
/// Example input: "piano"
/// Example output: ["*iano", "p*ano", "pi*no", "pia*o", "pian*"]
fn make_wildcard_words(word: &str) -> Vec<String> {
    (0..word.len())
        .map(|i| {
            word.chars()
                .enumerate()
                .map(|(char_i, c)| if i == char_i { '*' } else { c })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}

/// Build a map of "wildcard words" to collections of words from our built-in word list.
/// Here's how one key-value pair might look, where the key is the "wildcard word":
/// "*ish" => ["fish", "wish"]
fn make_wildcard_word_lookup(word_len: usize) -> HashMap<String, Vec<&'static str>> {
    let unsplit_words = match word_len {
        3 => THREE_LETTER_WORDS,
        4 => FOUR_LETTER_WORDS,
        5 => FIVE_LETTER_WORDS,
        _ => unreachable!("Please provide words between between 3 and 5 letters long."),
    };
    let mut word_wildcard_lookups: HashMap<String, Vec<&str>> = HashMap::new();
    for word in unsplit_words.lines() {
        for wildcard_word in make_wildcard_words(word) {
            word_wildcard_lookups
                .entry(wildcard_word)
                .or_insert_with(Vec::new)
                .push(word);
        }
    }
    word_wildcard_lookups
}

/// Parse our 'start' and 'end' words out of the provided command line arguments, returning
/// them in a (start, end) tuple. Print to stderr and exit if the args are not valid.
fn validate_args(args: env::Args) -> (String, String) {
    let mut args = args.skip(1);
    let start_word = args.next().map(|w| w.to_lowercase()).unwrap_or_else(|| {
        eprint!("{}", USAGE_INSTRUCTIONS.red());
        process::exit(1)
    });
    let end_word = args.next().map(|w| w.to_lowercase()).unwrap_or_else(|| {
        eprint!("{}", USAGE_INSTRUCTIONS.red());
        process::exit(1)
    });
    let word_len = start_word.len(); // really assuming ASCII here: one byte per char
    if word_len > 5 || word_len < 3 || word_len != end_word.len() {
        eprint!("{}", USAGE_INSTRUCTIONS.red());
        process::exit(1)
    }
    if start_word == end_word {
        eprint!(
            "{}",
            "The starting and ending word must be different from each other.".red()
        );
        process::exit(1)
    }
    (start_word, end_word)
}

fn report_success(start_word: &str, end_word: &str, shortest_path: &[&str]) -> () {
    let num_steps = shortest_path.len() - 1;
    let plural = if num_steps > 1 { "s" } else { "" };
    let success_announcement =
        format!("Found path from '{start_word}' to '{end_word}' in {num_steps} step{plural}!");
    let formatted_steps = shortest_path.join(" -> ");
    println!("{}", success_announcement.green());
    println!("{}", formatted_steps.green());
}

fn report_failure(start_word: &str, end_word: &str) {
    let failure_announcement = format!("No path found between '{start_word}' and '{end_word}'.");
    println!("{}", failure_announcement.red());
}
