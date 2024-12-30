/// Sort given list using the classic merge sort "divide and conquer" algorithm. First
/// split the list into sublists recursively until every sublist has a length of 1 and
/// is de facto sorted, then use a linear-time function to combine each pair of sorted
/// sublists.
pub fn merge_sort(ns: &[i32]) -> Vec<i32> {
    if ns.len() <= 1 {
        return ns.to_vec();
    } else {
        let (xs, ys) = split_into_sublists(ns);
        let sorted_xs = merge_sort(&xs);
        let sorted_ys = merge_sort(&ys);
        return merge_sorted_sublists(&sorted_xs, &sorted_ys);
    }
}

fn split_into_sublists(ns: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let first_sublist_len = ns.len() / 2;
    return (
        ns[0..first_sublist_len].to_vec(),
        ns[first_sublist_len..ns.len()].to_vec(),
    );
}

fn merge_sorted_sublists(xs: &[i32], ys: &[i32]) -> Vec<i32> {
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
