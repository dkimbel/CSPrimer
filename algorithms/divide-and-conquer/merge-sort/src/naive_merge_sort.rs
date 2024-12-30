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
