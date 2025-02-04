use std::cmp;

pub fn edit_distance_recursive(from: &str, to: &str) -> usize {
    if from.is_empty() {
        return to.len(); // all insertions
    } else if to.is_empty() {
        return from.len(); // all deletions
    }

    if from.chars().next() == to.chars().next() {
        return edit_distance_recursive(&from[1..], &to[1..]);
    }

    let insert = 1 + edit_distance_recursive(from, &to[1..]);
    let delete = 1 + edit_distance_recursive(&from[1..], to);
    let replace = 1 + edit_distance_recursive(&from[1..], &to[1..]);
    cmp::min(insert, cmp::min(delete, replace))
}

pub fn edit_distance_top_down(from: &str, to: &str) -> usize {
    // Our memo is a 2d vec where the "coordinates" are the length remaining for
    // `from` (y) and `to` (x). Since we're always solving from left to right, we
    // only need to know their remaining length.
    let mut memo: Vec<Vec<Option<usize>>> = vec![vec![None; to.len() + 1]; from.len() + 1];

    fn inner(from: &str, to: &str, memo: &mut Vec<Vec<Option<usize>>>) -> usize {
        if let Some(memoized) = memo[from.len()][to.len()] {
            return memoized;
        }

        let solution = if from.is_empty() {
            to.len() // all insertions
        } else if to.is_empty() {
            from.len() // all deletions
        } else if from.chars().next() == to.chars().next() {
            inner(&from[1..], &to[1..], memo)
        } else {
            let insert = inner(from, &to[1..], memo);
            let delete = inner(&from[1..], to, memo);
            let replace = inner(&from[1..], &to[1..], memo);
            1 + cmp::min(insert, cmp::min(delete, replace))
        };

        memo[from.len()][to.len()] = Some(solution);
        solution
    }

    inner(from, to, &mut memo)
}

pub fn edit_distance_bottom_up(from: &str, to: &str) -> usize {
    // Our memo is a 2d vec where the "coordinates" are the one-based indexes we're
    // currently checking in the 'from' and 'to' words. The 'from' index acts as the
    // y coordinate, with the 'to' index being x.
    let mut memo: Vec<Vec<usize>> = vec![vec![0; to.len() + 1]; from.len() + 1];

    // If from_i is zero, that means we're considering the first 0 chars from the 'from'
    // word (so it's effectively empty). If from_i is one, we're considering the first char,
    // and so on. The solution to the problem is at from_i == from.len() and to_i == to.len().
    // We're basically filling out a grid, where we determine the value of each cell (the edit
    // distance) by comparing the values to the left, above, and above-and-left. Typically we
    // add one to each of those values; the only exception is when the 'to' and 'from' char
    // are identical, where there is no added cost.
    for from_i in 0..=from.len() {
        for to_i in 0..=to.len() {
            let cost = if from_i == 0 {
                to_i
            } else if to_i == 0 {
                from_i
            } else if from[(from_i - 1)..from_i] == to[(to_i - 1)..to_i] {
                // This is a slight variation from the classic Wagner-Fischer algorithm -- they
                // don't treat this as a separate case, but rather set the 'replacement' cost to
                // zero in the `min` comparisons below. Possibly there are some wild edge cases
                // where my change is problematic, but I couldn't think of any reason we would
                // ever NOT want to make both substrings smaller for free.
                memo[from_i - 1][to_i - 1]
            } else {
                let remove = memo[from_i - 1][to_i];
                let insert = memo[from_i][to_i - 1];
                let replace = memo[from_i - 1][to_i - 1];
                1 + cmp::min(insert, cmp::min(remove, replace))
            };
            memo[from_i][to_i] = cost;
        }
    }
    memo[from.len()][to.len()]
}

pub fn edit_distance_bottom_up_linear_space(from: &str, to: &str) -> usize {
    let mut prev_row_memo: Vec<usize> = vec![0; to.len() + 1];
    let mut curr_row_memo: Vec<usize> = prev_row_memo.clone();

    for from_i in 0..=from.len() {
        (prev_row_memo, curr_row_memo) = (curr_row_memo, prev_row_memo);
        for to_i in 0..=to.len() {
            let cost = if from_i == 0 {
                to_i
            } else if to_i == 0 {
                from_i
            } else if from[(from_i - 1)..from_i] == to[(to_i - 1)..to_i] {
                prev_row_memo[to_i - 1]
            } else {
                let remove = prev_row_memo[to_i];
                let insert = curr_row_memo[to_i - 1];
                let replace = prev_row_memo[to_i - 1];
                1 + cmp::min(insert, cmp::min(remove, replace))
            };
            curr_row_memo[to_i] = cost;
        }
    }
    curr_row_memo[to.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive() {
        assert_eq!(edit_distance_recursive("horse", "ros"), 3);
        assert_eq!(edit_distance_recursive("food", "good"), 1);
        assert_eq!(edit_distance_recursive("a", "zaadz"), 4);
        assert_eq!(edit_distance_recursive("hi", "hi"), 0);
        assert_eq!(edit_distance_recursive("", ""), 0);
        assert_eq!(edit_distance_recursive("be", ""), 2);
        assert_eq!(edit_distance_recursive("intention", "execution"), 5);
        assert_eq!(edit_distance_recursive("ready", "tread"), 2);
    }

    #[test]
    fn test_top_down() {
        assert_eq!(edit_distance_top_down("horse", "ros"), 3);
        assert_eq!(edit_distance_top_down("food", "good"), 1);
        assert_eq!(edit_distance_top_down("a", "zaadz"), 4);
        assert_eq!(edit_distance_top_down("hi", "hi"), 0);
        assert_eq!(edit_distance_top_down("", ""), 0);
        assert_eq!(edit_distance_top_down("be", ""), 2);
        assert_eq!(edit_distance_top_down("intention", "execution"), 5);
        assert_eq!(edit_distance_top_down("ready", "tread"), 2);
    }

    #[test]
    fn test_bottom_up() {
        assert_eq!(edit_distance_bottom_up("horse", "ros"), 3);
        assert_eq!(edit_distance_bottom_up("food", "good"), 1);
        assert_eq!(edit_distance_bottom_up("a", "zaadz"), 4);
        assert_eq!(edit_distance_bottom_up("hi", "hi"), 0);
        assert_eq!(edit_distance_bottom_up("", ""), 0);
        assert_eq!(edit_distance_bottom_up("be", ""), 2);
        assert_eq!(edit_distance_bottom_up("intention", "execution"), 5);
        assert_eq!(edit_distance_bottom_up("ready", "tread"), 2);
    }

    #[test]
    fn test_bottom_up_linear_space() {
        assert_eq!(edit_distance_bottom_up_linear_space("horse", "ros"), 3);
        assert_eq!(edit_distance_bottom_up_linear_space("food", "good"), 1);
        assert_eq!(edit_distance_bottom_up_linear_space("a", "zaadz"), 4);
        assert_eq!(edit_distance_bottom_up_linear_space("hi", "hi"), 0);
        assert_eq!(edit_distance_bottom_up_linear_space("", ""), 0);
        assert_eq!(edit_distance_bottom_up_linear_space("be", ""), 2);
        assert_eq!(
            edit_distance_bottom_up_linear_space("intention", "execution"),
            5
        );
        assert_eq!(edit_distance_bottom_up_linear_space("ready", "tread"), 2);
    }
}
