use std::cmp;

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

    println!("### Before iteration: max_2s = {max_2s}, max_1s = {max_1s}, max_0s = {max_0s}, i = {i}, last_i_exclusive = {last_i_exclusive}");
    while i < last_i_exclusive {
        max_2s = cmp::max(max_0s, max_2s + nums[i]);
        max_0s = cmp::max(max_0s + nums[i + 1], max_1s + nums[i + 1]);
        max_1s += nums[i];

        i += 2;
        println!("### End of an iteration: max_2s = {max_2s}, max_1s = {max_1s}, max_0s = {max_0s}, i = {i}");
    }

    // take care of any last remaining number
    if is_odd_len {
        max_2s += nums[i];
        max_1s += nums[i];
    }

    println!("### End: max_2s = {max_2s}, max_1s = {max_1s}, max_0s = {max_0s}, i = {i}");
    // unsure if max_2s can actually be the highest number at this point
    cmp::max(max_0s, cmp::max(max_1s, max_2s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(find_max_nonadjacent_sum(&[]), 0);
    }

    #[test]
    fn single_item() {
        assert_eq!(find_max_nonadjacent_sum(&[5]), 5);
    }

    #[test]
    fn two_items() {
        assert_eq!(find_max_nonadjacent_sum(&[5, 2]), 5);
    }

    #[test]
    fn three_mid() {
        assert_eq!(find_max_nonadjacent_sum(&[1, 7, 2]), 7);
    }

    #[test]
    fn three_normal() {
        assert_eq!(find_max_nonadjacent_sum(&[6, 7, 4]), 10);
    }

    #[test]
    fn four_split() {
        assert_eq!(find_max_nonadjacent_sum(&[7, 2, 1, 8]), 15);
    }

    #[test]
    fn fifth_huge() {
        // this test case involves us skipping two from the second spot to the fifth
        assert_eq!(find_max_nonadjacent_sum(&[2, 20, 4, 7, 100, 12, 14]), 134);
    }

    #[test]
    fn full_size() {
        assert_eq!(
            find_max_nonadjacent_sum(&[5, 1, 2, 7, 13, 44, 100, 99, 2, 200]),
            355
        );
    }

    #[test]
    fn full_splits() {
        assert_eq!(
            find_max_nonadjacent_sum(&[1, 10, 2, 3, 20, 4, 5, 30, 1]),
            60
        );
    }

    #[test]
    fn ascending() {
        assert_eq!(
            find_max_nonadjacent_sum(&[2, 4, 6, 8, 12, 14, 16, 18, 20, 22, 24, 26]),
            92
        );
    }

    #[test]
    fn descending() {
        assert_eq!(
            find_max_nonadjacent_sum(&[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]),
            64
        );
    }
}
