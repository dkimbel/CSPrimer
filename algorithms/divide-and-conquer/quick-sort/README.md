# Quick sort

As usual, this is a library crate that supports `cargo test`.

There are also benchmarks, but they're in `../merge-sort`. Run `cargo bench` from there to
compare the speed of this quick sort versus that merge sort. On my own laptop:

- The non-naive merge sort needed ~53ms to sort a list of a million shuffled/random integers.
  - Remember, this is NOT sorting in place; it's returning a new vec.
- Quick sort needed ~50ms to sort a list of a million shuffled/random integers, in place.

So, quick sort narrowly won (as expected). But, maybe I could speed my merge sort up ever so
slightly if I refactored it to work in place? I'm not sure.
