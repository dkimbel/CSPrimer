use std::cmp;
use std::collections::HashMap;

/// My own solution, made from scratch without help. I wrongly concluded that we would have to look
/// at two numbers at a time, so my code was unnecessarily long. The implementation is correct, though.
pub fn find_max_nonadjacent_sum(nums: &[u32]) -> u32 {
    match nums.len() {
        0 => return 0,
        1 => return nums[0],
        _ => (),
    }

    // "2s" means "two spaces", as in "leaving two spaces to the right." So the variable
    // `max_2s` is the maximum possible value we calculated so far, if we completely ignored
    // the two most recently-iterated numbers from our main list.
    let mut max_2s = 0;
    let mut max_1s = nums[0];
    // `max_0s` MUST be leaving no space behind (using the most recent number we iterated
    // over). Our logic depends on it -- otherwise we couldn't validly replace max_2s with
    // max_0s on each iteration. That might be introducing a three-number gap, which we would
    // never ever do in any real solution.
    let mut max_0s = nums[1];

    let is_odd_len = nums.len() % 2 != 0;
    let last_i_exclusive = if is_odd_len {
        nums.len() - 1
    } else {
        nums.len()
    };
    let mut i = 2;

    while i < last_i_exclusive {
        let prev_max_0s = max_0s;
        max_0s = cmp::max(max_0s + nums[i + 1], max_1s + nums[i + 1]);
        max_1s = cmp::max(max_1s + nums[i], max_2s + nums[i]);
        max_2s = prev_max_0s;

        i += 2;
    }

    // take care of last remaining number from input in odd-length case
    if is_odd_len {
        max_2s += nums[i];
        max_1s += nums[i];
    }

    cmp::max(max_0s, cmp::max(max_1s, max_2s))
}

/// A solution I didn't manage to arrive at myself.
pub fn almost_best_iterative(nums: &[u32]) -> u32 {
    let mut max_stealing_curr_num = 0;
    let mut max_skipping_curr_num = 0;

    for curr_num in nums {
        (max_stealing_curr_num, max_skipping_curr_num) = (
            // we can only steal this time if we didn't steal last time
            max_skipping_curr_num + curr_num,
            // we're allowed to skip more than once in a row (that is: reuse skipping value from
            // last iteration, effectively skipping a second time)
            cmp::max(max_stealing_curr_num, max_skipping_curr_num),
        )
    }

    cmp::max(max_stealing_curr_num, max_skipping_curr_num)
}

/// Another solution I didn't manage to arrive at myself.
pub fn best_iterative(nums: &[u32]) -> u32 {
    let mut prev_prev_best = 0;
    let mut prev_best = 0;

    for num in nums {
        (prev_prev_best, prev_best) = (prev_best, cmp::max(prev_prev_best + num, prev_best))
    }

    prev_best
}

/// The recursive solution, which I didn't arrive at myself (I went straight to iterative, not
/// recognizing the recursive approach).
pub fn naive_recursive(nums: &[u32]) -> u32 {
    match nums.len() {
        0 => 0,
        1 => nums[0],
        // max of robbing nums[0] versus skipping it
        _ => cmp::max(
            nums[0] + naive_recursive(&nums[2..]),
            naive_recursive(&nums[1..]),
        ),
    }
}

pub fn memoized_recursive(nums: &[u32]) -> u32 {
    let mut memo: HashMap<usize, u32> = HashMap::new();

    fn inner(nums: &[u32], memo: &mut HashMap<usize, u32>) -> u32 {
        if let Some(memoized) = memo.get(&nums.len()) {
            return *memoized;
        }

        let result = match nums.len() {
            0 => 0,
            1 => nums[0],
            // max of robbing nums[0] versus skipping it
            _ => cmp::max(nums[0] + inner(&nums[2..], memo), inner(&nums[1..], memo)),
        };

        memo.insert(nums.len(), result);

        result
    }

    inner(nums, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(find_max_nonadjacent_sum(&[]), 0);
        assert_eq!(almost_best_iterative(&[]), 0);
        assert_eq!(best_iterative(&[]), 0);
        assert_eq!(naive_recursive(&[]), 0);
        assert_eq!(memoized_recursive(&[]), 0);
    }

    #[test]
    fn single_item() {
        assert_eq!(find_max_nonadjacent_sum(&[5]), 5);
        assert_eq!(almost_best_iterative(&[5]), 5);
        assert_eq!(best_iterative(&[5]), 5);
        assert_eq!(naive_recursive(&[5]), 5);
        assert_eq!(memoized_recursive(&[5]), 5);
    }

    #[test]
    fn two_items() {
        assert_eq!(find_max_nonadjacent_sum(&[5, 2]), 5);
        assert_eq!(almost_best_iterative(&[5, 2]), 5);
        assert_eq!(best_iterative(&[5, 2]), 5);
        assert_eq!(naive_recursive(&[5, 2]), 5);
        assert_eq!(memoized_recursive(&[5, 2]), 5);
    }

    #[test]
    fn three_mid() {
        assert_eq!(find_max_nonadjacent_sum(&[1, 7, 2]), 7);
        assert_eq!(almost_best_iterative(&[1, 7, 2]), 7);
        assert_eq!(best_iterative(&[1, 7, 2]), 7);
        assert_eq!(naive_recursive(&[1, 7, 2]), 7);
        assert_eq!(memoized_recursive(&[1, 7, 2]), 7);
    }

    #[test]
    fn three_normal() {
        assert_eq!(find_max_nonadjacent_sum(&[6, 7, 4]), 10);
        assert_eq!(almost_best_iterative(&[6, 7, 4]), 10);
        assert_eq!(best_iterative(&[6, 7, 4]), 10);
        assert_eq!(naive_recursive(&[6, 7, 4]), 10);
        assert_eq!(memoized_recursive(&[6, 7, 4]), 10);
    }

    #[test]
    fn four_split() {
        assert_eq!(find_max_nonadjacent_sum(&[7, 2, 1, 8]), 15);
        assert_eq!(almost_best_iterative(&[7, 2, 1, 8]), 15);
        assert_eq!(best_iterative(&[7, 2, 1, 8]), 15);
        assert_eq!(naive_recursive(&[7, 2, 1, 8]), 15);
        assert_eq!(memoized_recursive(&[7, 2, 1, 8]), 15);
    }

    #[test]
    fn fifth_huge() {
        // this test case involves us skipping two from the second spot to the fifth
        assert_eq!(find_max_nonadjacent_sum(&[2, 20, 4, 7, 100, 12, 14]), 134);
        assert_eq!(almost_best_iterative(&[2, 20, 4, 7, 100, 12, 14]), 134);
        assert_eq!(best_iterative(&[2, 20, 4, 7, 100, 12, 14]), 134);
        assert_eq!(naive_recursive(&[2, 20, 4, 7, 100, 12, 14]), 134);
        assert_eq!(memoized_recursive(&[2, 20, 4, 7, 100, 12, 14]), 134);
    }

    #[test]
    fn full_size() {
        assert_eq!(
            find_max_nonadjacent_sum(&[5, 1, 2, 7, 13, 44, 100, 99, 2, 200]),
            355
        );
        assert_eq!(
            almost_best_iterative(&[5, 1, 2, 7, 13, 44, 100, 99, 2, 200]),
            355
        );
        assert_eq!(best_iterative(&[5, 1, 2, 7, 13, 44, 100, 99, 2, 200]), 355);
        assert_eq!(naive_recursive(&[5, 1, 2, 7, 13, 44, 100, 99, 2, 200]), 355);
        assert_eq!(
            memoized_recursive(&[5, 1, 2, 7, 13, 44, 100, 99, 2, 200]),
            355
        );
    }

    #[test]
    fn full_splits() {
        assert_eq!(
            find_max_nonadjacent_sum(&[1, 10, 2, 3, 20, 4, 5, 30, 1]),
            60
        );
        assert_eq!(almost_best_iterative(&[1, 10, 2, 3, 20, 4, 5, 30, 1]), 60);
        assert_eq!(best_iterative(&[1, 10, 2, 3, 20, 4, 5, 30, 1]), 60);
        assert_eq!(naive_recursive(&[1, 10, 2, 3, 20, 4, 5, 30, 1]), 60);
        assert_eq!(memoized_recursive(&[1, 10, 2, 3, 20, 4, 5, 30, 1]), 60);
    }

    #[test]
    fn ascending() {
        assert_eq!(
            find_max_nonadjacent_sum(&[2, 4, 6, 8, 12, 14, 16, 18, 20, 22, 24, 26]),
            92
        );
        assert_eq!(
            almost_best_iterative(&[2, 4, 6, 8, 12, 14, 16, 18, 20, 22, 24, 26]),
            92
        );
        assert_eq!(
            best_iterative(&[2, 4, 6, 8, 12, 14, 16, 18, 20, 22, 24, 26]),
            92
        );
        assert_eq!(
            naive_recursive(&[2, 4, 6, 8, 12, 14, 16, 18, 20, 22, 24, 26]),
            92
        );
        assert_eq!(
            memoized_recursive(&[2, 4, 6, 8, 12, 14, 16, 18, 20, 22, 24, 26]),
            92
        );
    }

    #[test]
    fn descending() {
        assert_eq!(
            find_max_nonadjacent_sum(&[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]),
            64
        );
        assert_eq!(
            almost_best_iterative(&[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]),
            64
        );
        assert_eq!(
            best_iterative(&[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]),
            64
        );
        assert_eq!(
            naive_recursive(&[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]),
            64
        );
        assert_eq!(
            memoized_recursive(&[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]),
            64
        );
    }
}
