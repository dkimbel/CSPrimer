# Merge sort

I made two implementations of merge sort: a 'naive' one that uses recursion and makes
a lot of unnecessary allocations, and a 'fast' one that avoids many allocations. Based
on my local benchmarks:

- The fast version needs about 58ms to sort a list of a million integers.
- The naive version needs about 138ms to do the same.

As always, this is a library crate whose tests can be run locally with `cargo test`.
This crate also has benchmarks runnable with `cargo bench`.
