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
    let wildcard_word_lookup = make_wildcard_word_lookup(&start_word, &end_word);

    if let Some(shortest_path) = find_shortest_path(&start_word, &end_word, wildcard_word_lookup) {
        report_search_success(&start_word, &end_word, &shortest_path);
    } else {
        report_search_failure(&start_word, &end_word);
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
/// At the same time, while making its single pass through our word list, this function
/// confirms that both the starting and ending word were found. If either is missing,
/// we exit the program with an error message.
fn make_wildcard_word_lookup(
    start_word: &str,
    end_word: &str,
) -> HashMap<String, Vec<&'static str>> {
    let unsplit_words = match start_word.len() {
        3 => THREE_LETTER_WORDS,
        4 => FOUR_LETTER_WORDS,
        5 => FIVE_LETTER_WORDS,
        _ => unreachable!("Please provide words between between 3 and 5 letters long."),
    };
    let mut word_wildcard_lookups: HashMap<String, Vec<&str>> = HashMap::new();
    let mut start_word_found = false;
    let mut end_word_found = false;

    for word in unsplit_words.lines() {
        if word == start_word {
            start_word_found = true;
        }
        if word == end_word {
            end_word_found = true;
        }
        for wildcard_word in make_wildcard_words(word) {
            word_wildcard_lookups
                .entry(wildcard_word)
                .or_insert_with(Vec::new)
                .push(word);
        }
    }

    if !start_word_found {
        report_failure_to_find_word_and_exit(start_word);
    } else if !end_word_found {
        report_failure_to_find_word_and_exit(end_word);
    }
    word_wildcard_lookups
}

/// Parse our 'start' and 'end' words out of the provided command line arguments, returning
/// them in a (start, end) tuple. Print to stderr and exit if the args are not valid.
fn validate_args(args: env::Args) -> (String, String) {
    let mut args = args.skip(1);
    let start_word = args.next().map(|w| w.to_lowercase()).unwrap_or_else(|| {
        eprintln!("{}", USAGE_INSTRUCTIONS.red());
        process::exit(1)
    });
    let end_word = args.next().map(|w| w.to_lowercase()).unwrap_or_else(|| {
        eprintln!("{}", USAGE_INSTRUCTIONS.red());
        process::exit(1)
    });
    if args.next().is_some() {
        eprintln!(
            "{}",
            "Please only provide two words as inputs: the starting word and ending word.".red()
        );
        process::exit(1)
    }
    let word_len = start_word.len(); // really assuming ASCII here: one byte per char
    if word_len > 5 || word_len < 3 || word_len != end_word.len() {
        eprintln!("{}", USAGE_INSTRUCTIONS.red());
        process::exit(1)
    }
    if start_word == end_word {
        eprintln!(
            "{}",
            "The starting and ending word must be different from each other.".red()
        );
        process::exit(1)
    }
    (start_word, end_word)
}

fn report_search_success(start_word: &str, end_word: &str, shortest_path: &[&str]) -> () {
    let num_steps = shortest_path.len() - 1;
    let plural = if num_steps > 1 { "s" } else { "" };
    let success_announcement =
        format!("Found path from '{start_word}' to '{end_word}' in {num_steps} step{plural}!");
    let formatted_steps = shortest_path.join(" -> ");
    println!("{}", success_announcement.green());
    println!("{}", formatted_steps.green());
}

fn report_search_failure(start_word: &str, end_word: &str) {
    let failure_announcement = format!("No path found between '{start_word}' and '{end_word}'.");
    println!("{}", failure_announcement.red());
}

fn report_failure_to_find_word_and_exit(word: &str) {
    let failure_message = format!("Failed to find '{word}' in word list; please use a valid English word that is not a proper noun.");
    eprintln!("{}", failure_message.red());
    process::exit(1);
}
