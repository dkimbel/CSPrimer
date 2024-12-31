/// Sort the given collection in place using the classic 'quicksort' algorithm. For
/// simplicity, we use Nico Lomuto's partitioning scheme and always choose the first
/// index as our pivot.
pub fn quick_sort<T: Ord>(input: &mut [T]) {
    if input.len() <= 1 {
        return;
    }

    // by convention, our pivot index is called t
    let t: usize = 0;
    // m is the maximum index whose value is less than or equal to our pivot value
    let mut m: usize = 0;

    // no need to check the pivot value against itself, so start at i=1
    for i in 1..input.len() {
        if &input[i] <= &input[t] {
            input.swap(i, m + 1);
            m += 1;
        }
    }

    // swap pivot element into its new position at the end of all the elements that
    // are less than or equal to it
    input.swap(t, m);

    // recursively sort everything to the left of where we put our pivot element
    quick_sort(&mut input[0..m]);
    // recursively sort everything to the RIGHT of where we put our pivot element
    let input_len = input.len();
    let safe_min_right_i = std::cmp::min(m + 1, input_len - 1);
    quick_sort(&mut input[safe_min_right_i..input_len]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_even_list() {
        let mut input = vec![
            96, 12, -6, 18, 2000, -500, -70, 12, 10000, -1234, -60, 60, 200, 0,
        ];
        quick_sort(&mut input);
        assert_eq!(
            input,
            vec![-1234, -500, -70, -60, -6, 0, 12, 12, 18, 60, 96, 200, 2000, 10000]
        );
    }

    #[test]
    fn sort_odd_list() {
        let mut input = vec![
            96, 12, -6, 18, 2000, -500, -70, 12, 10000, -1234, -60, 60, 0,
        ];
        quick_sort(&mut input);
        assert_eq!(
            input,
            vec![-1234, -500, -70, -60, -6, 0, 12, 12, 18, 60, 96, 2000, 10000]
        );
    }

    #[test]
    fn sort_empty_list() {
        let mut input: Vec<i32> = Vec::new();
        quick_sort(&mut input);
        assert_eq!(input, Vec::new());
    }
}
