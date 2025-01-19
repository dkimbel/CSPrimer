use indexmap::IndexSet;

fn main() {
    let tour_path = find_tour_dfs(8, 8);
    println!("Successfully toured board! Path: {:#?}", &tour_path);
}

pub fn find_tour_dfs(x_dimension: usize, y_dimension: usize) -> IndexSet<(usize, usize)> {
    let total_num_board_spaces = x_dimension * y_dimension;
    // IndexSet maintains insertion order, so it can serve as both our 'visited' list
    // AND our currently-searched path. We only need an IndexSet to represent each
    // current search within our stack. Inspired by Oz's use of a Python dict, which
    // preserves order, to accomplish the same thing.
    let mut stack: Vec<IndexSet<(usize, usize)>> = vec![IndexSet::from([(0, 0)])];
    let mut next_options = Vec::with_capacity(8);

    while let Some(path) = stack.pop() {
        if path.len() == total_num_board_spaces {
            return path;
        }

        let (x, y) = *(path.last().unwrap());

        next_options.clear();
        get_next_options(x, y, x_dimension, y_dimension, &mut next_options);

        // The last thing added to the stack is popped first; we want to pop the lowest
        // warnsdorf distance first, so we want it added to the stack last. So: sort
        // by warnsdorf DESCENDING.
        next_options.sort_by(|(x1, y1), (x2, y2)| {
            let w1 = warnsdorf_distance(*x1, *y1, x_dimension, y_dimension);
            let w2 = warnsdorf_distance(*x2, *y2, x_dimension, y_dimension);
            w2.cmp(&w1)
        });
        next_options
            .iter()
            // remove already-visited coords
            .filter(|coords| !path.contains(*coords))
            .for_each(|coords| {
                let mut new_path = path.clone();
                new_path.insert(*coords);
                stack.push(new_path);
            });
    }

    panic!("Failed to find a tour path.")
}

fn get_next_options(
    x: usize,
    y: usize,
    x_dimension: usize,
    y_dimension: usize,
    next_options: &mut Vec<(usize, usize)>,
) {
    let can_move_long_upwards = y < y_dimension - 2;
    let can_move_short_upwards = y < y_dimension - 1;
    let can_move_long_downwards = y >= 2;
    let can_move_short_downwards = y >= 1;
    let can_move_long_right = x < x_dimension - 2;
    let can_move_short_right = x < x_dimension - 1;
    let can_move_long_left = x >= 2;
    let can_move_short_left = x >= 1;

    if can_move_long_upwards && can_move_short_right {
        next_options.push((x + 1, y + 2));
    }
    if can_move_long_upwards && can_move_short_left {
        next_options.push((x - 1, y + 2));
    }
    if can_move_long_downwards && can_move_short_right {
        next_options.push((x + 1, y - 2));
    }
    if can_move_long_downwards && can_move_short_left {
        next_options.push((x - 1, y - 2));
    }
    if can_move_short_upwards && can_move_long_right {
        next_options.push((x + 2, y + 1));
    }
    if can_move_short_upwards && can_move_long_left {
        next_options.push((x - 2, y + 1));
    }
    if can_move_short_downwards && can_move_long_right {
        next_options.push((x + 2, y - 1));
    }
    if can_move_short_downwards && can_move_long_left {
        next_options.push((x - 2, y - 1));
    }
}

// Calculate the total distance of a point from both edges of the board (distance from
// nearest vertical edge plus distance from nearest horizontal edge).
fn warnsdorf_distance(x: usize, y: usize, x_dimension: usize, y_dimension: usize) -> usize {
    let max_x = x_dimension - 1;
    let max_y = y_dimension - 1;

    std::cmp::min(max_x - x, x) + std::cmp::min(max_y - y, y)
}
