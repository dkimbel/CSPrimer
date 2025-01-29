# Minimal Grid Path

Given a 2D array 'grid' of numbers, find the path that goes from the upper left to the lower right
while incurring the lowest total cost (where 'cost' is the value of each integer in the grid). We're
only allowed to move down and to the right.

Returns the path taken -- a vector of coordinates like `[(0, 0), (0, 1), (1, 1), (2, 1), (2, 2)]`.

I'm solving using uniform cost search, top-down dynamic programming, and bottom-up dynamic programming.

## Usage

Plug whatever input you'd like into the code in `src/main.rs`, then use `cargo run`.
