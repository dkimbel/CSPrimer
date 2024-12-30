use std::cmp;

/// Sort given list using the classic merge sort "divide and conquer" algorithm.
/// This is a somewhat advanced implementation, which does NOT use recursion or allocate
/// many vectors on the heap. It only allocates twice, with each of those vectors having
/// the same length as the input (and never needing to be resized).
pub fn merge_sort(ns: &[i32]) -> Vec<i32> {
    let (mut source, mut target) = (ns.to_vec(), vec![0; ns.len()]);
    let mut max_sublist_size: usize = 1;

    // On our final iteration, max_sublist_size will be at LEAST as long as our input
    // list. That's why our loop's end condition is greater than ns.len().
    while max_sublist_size < ns.len() * 2 {
        merge_all_sorted_sublists(&source, &mut target, max_sublist_size);

        max_sublist_size *= 2;
        (source, target) = (target, source); // swap roles every iteration
    }

    target
}

/// Slice up the monolithic 'source' vector into smaller pieces that can be merged, with
/// each merge targeting a mutable slice of the monolithic 'target' vector.
fn merge_all_sorted_sublists(source: &[i32], target: &mut [i32], max_sublist_size: usize) {
    let mut start_i: usize = 0;

    while start_i < source.len() {
        let first_sublist_end_i = cmp::min(start_i + max_sublist_size, source.len());
        let second_sublist_end_i = cmp::min(first_sublist_end_i + max_sublist_size, source.len());
        merge_pair_of_sorted_sublists(
            &source[start_i..first_sublist_end_i],
            &source[first_sublist_end_i..second_sublist_end_i],
            &mut target[start_i..second_sublist_end_i],
        );
        start_i += max_sublist_size * 2;
    }
}

// Merge the pair of pre-sorted sublists, writing the merged collection to the mutable
// 'target' slice.
fn merge_pair_of_sorted_sublists(xs: &[i32], ys: &[i32], target: &mut [i32]) {
    let mut x_i: usize = 0;
    let mut y_i: usize = 0;
    let mut target_i: usize = 0;

    while x_i < xs.len() || y_i < ys.len() {
        let maybe_x = xs.get(x_i);
        let maybe_y = ys.get(y_i);

        match (maybe_x, maybe_y) {
            (Some(x), Some(y)) => {
                if x < y {
                    target[target_i] = *x;
                    x_i += 1;
                } else {
                    target[target_i] = *y;
                    y_i += 1;
                }
            }
            (Some(x), None) => {
                target[target_i] = *x;
                x_i += 1;
            }
            (None, Some(y)) => {
                target[target_i] = *y;
                y_i += 1;
            }
            (None, None) => unreachable!("Exhausted both sublists prematurely during merge"),
        }
        target_i += 1;
    }
}

pub fn slow_merge_sort(ns: &[i32]) -> Vec<i32> {
    if ns.len() <= 1 {
        return ns.to_vec();
    } else {
        let (xs, ys) = slow_split_into_sublists(ns);
        let sorted_xs = slow_merge_sort(&xs);
        let sorted_ys = slow_merge_sort(&ys);
        return slow_merge_sorted_sublists(&sorted_xs, &sorted_ys);
    }
}

fn slow_split_into_sublists(ns: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let first_sublist_len = ns.len() / 2;
    return (
        ns[0..first_sublist_len].to_vec(),
        ns[first_sublist_len..ns.len()].to_vec(),
    );
}

fn slow_merge_sorted_sublists(xs: &[i32], ys: &[i32]) -> Vec<i32> {
    let mut merged: Vec<i32> = Vec::new();

    let mut x_i = 0;
    let mut y_i = 0;
    while x_i < xs.len() || y_i < ys.len() {
        let maybe_x = xs.get(x_i);
        let maybe_y = ys.get(y_i);

        match (maybe_x, maybe_y) {
            (Some(x), Some(y)) => {
                if x < y {
                    merged.push(*x);
                    x_i += 1;
                } else {
                    merged.push(*y);
                    y_i += 1;
                }
            }
            (Some(x), None) => {
                merged.push(*x);
                x_i += 1;
            }
            (None, Some(y)) => {
                merged.push(*y);
                y_i += 1;
            }
            (None, None) => unreachable!("Exhausted both sublists prematurely during merge"),
        }
    }

    merged
}

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
        let unsorted = Vec::new();
        assert_eq!(merge_sort(&unsorted), unsorted);
    }
}
