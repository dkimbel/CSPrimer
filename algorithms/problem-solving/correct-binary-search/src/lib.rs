use std::cmp;
use std::cmp::Ordering;

/// Search for the given element in the given sorted slice. Return its index if found.
pub fn binary_search(xs: &[i32], x: i32) -> Option<usize> {
    if xs.is_empty() {
        return None;
    }

    let mut start_i: usize = 0;
    let mut end_i: usize = xs.len() - 1;

    while start_i != end_i {
        // NOTE: when picking our midway point between `start` and `end`, we naturally round
        // down -- so if there is an even number of elements, we will always choose a midway
        // point slightly closer to `start`.
        let midway_i = (start_i + end_i) / 2;

        match x.cmp(&xs[midway_i]) {
            Ordering::Equal => return Some(midway_i),
            // Respect the "end_i >= start_i" invariant
            Ordering::Less => end_i = cmp::max(midway_i - 1, start_i),
            // Respect the "start_i <= end_i" invariant
            Ordering::Greater => start_i = cmp::min(midway_i + 1, end_i),
        }
    }

    if xs[start_i] == x {
        return Some(start_i);
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let arr = [];
        let result = binary_search(&arr, 1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_not_present_len_one_too_big() {
        let arr = [17];
        let result = binary_search(&arr, 23);
        assert_eq!(result, None);
    }

    #[test]
    fn test_not_present_len_one_too_small() {
        let arr = [17];
        let result = binary_search(&arr, 1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_not_present_even_len() {
        let arr = [-5, -3, 10, 17];
        let result = binary_search(&arr, 1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_not_present_odd_len() {
        let arr = [-5, -3, 10, 17, 23];
        let result = binary_search(&arr, 1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_present_len_one() {
        let arr = [17];
        let result = binary_search(&arr, 17);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_present_even_len_solution_middle() {
        let arr = [-5, 17, 30, 40];
        let result = binary_search(&arr, 17);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_present_odd_len_solution_middle() {
        let arr = [-5, 17, 30, 40, 50];
        let result = binary_search(&arr, 30);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_present_even_len_solution_first() {
        let arr = [-5, 17, 30, 40];
        let result = binary_search(&arr, -5);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_present_odd_len_solution_first() {
        let arr = [-5, 17, 30, 40, 50];
        let result = binary_search(&arr, -5);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_present_even_len_solution_last() {
        let arr = [-5, 17, 30, 40];
        let result = binary_search(&arr, 40);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_present_odd_len_solution_last() {
        let arr = [-5, 17, 30, 40, 50];
        let result = binary_search(&arr, 50);
        assert_eq!(result, Some(4));
    }
}
