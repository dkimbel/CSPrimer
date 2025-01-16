# Word Ladder

Breadth-first search to find a path from a given 'start word' to a given 'end word', where:

- We only change one letter at a time.
- Every 'word' we use must be an actual English word (per our starting word list file).

Here's an example result for starting at 'food' and ending at 'wine':
```
food -> fond -> find -> wind -> wine
```

At least for now, we're only supporting words of length 3, 4, and 5. The start and end word
must be the same length.

## Usage

Assuming you've pulled down this code and you're in the same directory as this README:
```bash
$ cargo install --path .
$ word-ladder food wine
```

## `split-word-list` helper script

Besides the main program, my Rust crate contains a helper binary that operates on the `words.txt` dictionary
file we started with. That script is `bin/split-word-list.rs`, and it:

- Filters out proper nouns (words with the first letter capitalized).
- Separates the words into different files based on their length.

So, if my program sees that the user's inputs were five letters long, it'll only need to iterate through words
from the five-letter collection. (To be fair, I was only going to iterate one time anyway, so this doesn't save
much work. It's something, though.)
