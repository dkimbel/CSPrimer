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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive() {
        assert_eq!(edit_distance_recursive("horse", "ros"), 3);
        assert_eq!(edit_distance_recursive("food", "good"), 1);
        assert_eq!(edit_distance_recursive("a", "zaadz"), 4);
        assert_eq!(edit_distance_recursive("", ""), 0);
        assert_eq!(edit_distance_recursive("be", ""), 2);
        assert_eq!(edit_distance_recursive("intention", "execution"), 5);
    }
}
