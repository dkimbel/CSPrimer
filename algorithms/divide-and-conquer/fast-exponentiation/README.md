# Fast exponentiation

This library crate contains several different custom implementations of exponentiation.
It has tests (`cargo test`) and benchmarks (`cargo bench`).

Awkwardly, my original 'sublinear' version is the slowest by far, at least when calculating 2 ^ 64:
- Sublinear: ~100ns
- Linear iterative: ~15ns
- Linear recursive: ~9.6ns
- 'Fast' sublinear  iterative: ~3ns
- 'Fast' sublinear recursive: ~3ns
- Standard library: ~2.5ns to ~2.7ns
