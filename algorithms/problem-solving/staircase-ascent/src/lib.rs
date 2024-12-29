/// Calculates how many different combinations of steps of different 'stride lengths'
/// could be taken to ascend a staircase of the given length. For example, if we
/// have a staircase of length 3 with a max stride length of 2, there are 3 total
/// ways we could ascend that staircase:
/// - Two steps then one step
/// - One step then two steps
/// - One step, repeated three times
/// So if this function is called with total_num_steps=3 and max_steps_per_stride=2,
/// it will return 3.
pub fn num_stride_combinations(total_num_steps: u64, max_steps_per_stride: u64) -> u64 {
    if total_num_steps == 0 || max_steps_per_stride == 0 {
        return 0;
    }

    let mut num_combinations: u64 = 0;
    // We use a stack to keep track of every possible number of remaining steps. If our
    // function was called with 12 total steps and a max stride length of 3, then after
    // one iteration, this vector would equal [11, 10, 9]. That's because there's one
    // iteration per stride, and our first stride could have been for 1, 2, or 3 steps.
    let mut all_possible_steps_left: Vec<u64> = vec![total_num_steps];

    while let Some(steps_left) = all_possible_steps_left.pop() {
        if steps_left == 0 {
            num_combinations += 1;
            continue;
        }
        for steps_in_stride in 1..max_steps_per_stride + 1 {
            if steps_left >= steps_in_stride {
                all_possible_steps_left.push(steps_left - steps_in_stride);
            } else {
                // Steps_in_stride will only get larger as we iterate; as soon as we've
                // seen this case once, we know we'll see it for the rest of the loop.
                break;
            }
        }
    }

    num_combinations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_strides() {
        for (num_steps, expected_num_combinations) in
            [(0, 0), (1, 1), (2, 2), (3, 4), (4, 7), (5, 13)]
        {
            let result = num_stride_combinations(num_steps, 3);
            assert_eq!(result, expected_num_combinations);
        }
    }

    #[test]
    fn test_two_strides() {
        for (num_steps, expected_num_combinations) in
            [(0, 0), (1, 1), (2, 2), (3, 3), (4, 5), (5, 8)]
        {
            let result = num_stride_combinations(num_steps, 2);
            assert_eq!(result, expected_num_combinations);
        }
    }

    #[test]
    fn test_zero_strides() {
        let result = num_stride_combinations(2, 0);
        assert_eq!(result, 0);
    }
}
