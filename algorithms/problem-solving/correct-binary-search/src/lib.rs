use std::cmp::Ordering;

/// Search for the given element in the given sorted slice. Return its index if found.
pub fn binary_search(xs: &[i32], x: i32) -> Option<usize> {
    let mut start_i_inclusive: usize = 0;
    let mut end_i_exclusive: usize = xs.len();

    while start_i_inclusive < end_i_exclusive {
        let midway_i = (start_i_inclusive + end_i_exclusive) / 2;

        match x.cmp(&xs[midway_i]) {
            Ordering::Equal => return Some(midway_i),
            Ordering::Less => end_i_exclusive = midway_i,
            Ordering::Greater => start_i_inclusive = midway_i + 1,
        }
    }

    return None;
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
