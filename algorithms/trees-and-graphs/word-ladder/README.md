# Word Ladder

## `split-word-list` helper script

Besides the main program, my Rust crate contains a helper binary that operates on the `words.txt` dictionary
file we started with. That script is `bin/split-word-list.rs`, and it:

- Filters out proper nouns (words with the first letter capitalized).
- Separates the words into different files based on their length.

So, if my program sees that the user's inputs were five letters long, it'll only need to iterate through words
from the five-letter collection. (To be fair, I was only going to iterate one time anyway, so this doesn't save
much work. It's something, though.)
