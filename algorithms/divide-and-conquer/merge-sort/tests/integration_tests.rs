use merge_sort::merge_sort;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_even_list() {
        let unsorted = vec![
            96, 12, -6, 18, 2000, -500, -70, 12, 10000, -1234, -60, 60, 200, 0,
        ];
        assert_eq!(
            merge_sort(&unsorted),
            vec![-1234, -500, -70, -60, -6, 0, 12, 12, 18, 60, 96, 200, 2000, 10000]
        );
    }

    #[test]
    fn sort_odd_list() {
        let unsorted = vec![
            96, 12, -6, 18, 2000, -500, -70, 12, 10000, -1234, -60, 60, 0,
        ];
        assert_eq!(
            merge_sort(&unsorted),
            vec![-1234, -500, -70, -60, -6, 0, 12, 12, 18, 60, 96, 2000, 10000]
        );
    }

    #[test]
    fn sort_empty_list() {
        let unsorted: Vec<i32> = Vec::new();
        assert_eq!(merge_sort(&unsorted), unsorted);
    }
}
