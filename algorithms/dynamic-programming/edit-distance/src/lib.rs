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
        let solution = cmp::min(insert, cmp::min(delete, replace));
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

    let from_chars = from.chars().collect::<Vec<char>>();
    let to_chars = to.chars().collect::<Vec<char>>();

    // If from_i is zero, that means we're considering the first 0 chars from the 'from'
    // word (so it's effectively empty). If from_i is one, we're considering the first char,
    // and so on. The solution to the problem is at from_i == from.len() and to_i == to.len().
    for from_i in 0..=from_chars.len() {
        for to_i in 0..=to_chars.len() {
            let cost = if from_i == 0 {
                to_i
            } else if to_i == 0 {
                from_i
            } else {
                let cost_to_replace = if from_chars[from_i - 1] == to_chars[to_i - 1] {
                    0
                } else {
                    1
                };
                let remove = 1 + memo[from_i - 1][to_i];
                let insert = 1 + memo[from_i][to_i - 1];
                let replace = cost_to_replace + memo[from_i - 1][to_i - 1];
                cmp::min(insert, cmp::min(remove, replace))
            };
            memo[from_i][to_i] = cost;
        }
    }
    memo[from.len()][to.len()]
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
    }
}
