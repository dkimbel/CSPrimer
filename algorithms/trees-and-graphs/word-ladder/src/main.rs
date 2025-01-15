use std::collections::{HashMap, HashSet, VecDeque};

const THREE_LETTER_WORDS: &str = include_str!("../resources/three_letter_words.txt");
const FOUR_LETTER_WORDS: &str = include_str!("../resources/four_letter_words.txt");
const FIVE_LETTER_WORDS: &str = include_str!("../resources/five_letter_words.txt");

fn main() {
    let start_word = "wheat";
    let end_word = "bread";
    let word_len = 5;

    let mut word_wildcard_lookups: HashMap<String, Vec<&str>> = HashMap::new();

    let unsplit_words = match word_len {
        3 => THREE_LETTER_WORDS,
        4 => FOUR_LETTER_WORDS,
        5 => FIVE_LETTER_WORDS,
        _ => panic!(
            "Words with {word_len} letters are not supported! Please use between 3 and 5 letters."
        ),
    };
    for word in unsplit_words.lines() {
        for wildcard_word in make_wildcard_words(word) {
            word_wildcard_lookups
                .entry(wildcard_word)
                .or_insert_with(Vec::new)
                .push(word);
        }
    }

    let shortest_path_if_any = find_shortest_path(start_word, end_word, word_wildcard_lookups);

    dbg!(&shortest_path_if_any);
}

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

// TODO actually take inputs from command line and validate them
//   validate that they're the same length
//   validate that they're between 3 and 5 chars long
//   that the start and target word are not the same
//   lowercase them
//   validate that they're both in word list?
// TODO print results, incl number of steps it took (path.len() - 2)
// TODO any really fancy way to replace any of my Vecs with tuples or arrays? At least, in the
//   'wildcard words' case where we know how many items we're dealing with, but only at runtime?
// TODO make sure I didn't mess up lifetimes on find_shortest_path... and understand what they really mean
// TODO split up code, at least into fns and maybe mods
// TODO add comments / docstrings
