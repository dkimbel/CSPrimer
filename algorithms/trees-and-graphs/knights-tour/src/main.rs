use indexmap::IndexSet;

const KNIGHT_MOVE_DELTAS: [(i32, i32); 8] = [
    (2, 1),
    (2, -1),
    (-2, 1),
    (-2, -1),
    (1, 2),
    (1, -2),
    (-1, 2),
    (-1, -2),
];

fn main() {
    let tour_path = find_tour_dfs(8, 8);
    println!("Successfully toured board! Path: {:#?}", &tour_path);
}

pub fn find_tour_dfs(x_dimension: i32, y_dimension: i32) -> IndexSet<(i32, i32)> {
    let total_num_board_spaces = (x_dimension * y_dimension) as usize;
    // IndexSet maintains insertion order, so it can serve as both our 'visited' list
    // AND our currently-searched path. We only need an IndexSet to represent each
    // current search within our stack. Inspired by Oz's use of a Python dict, which
    // preserves order, to accomplish the same thing.
    let mut stack: Vec<IndexSet<(i32, i32)>> = vec![IndexSet::from([(0, 0)])];
    let mut next_options = Vec::with_capacity(8);

    while let Some(path) = stack.pop() {
        if path.len() == total_num_board_spaces {
            return path;
        }

        let (x, y) = *(path.last().unwrap());

        next_options.clear();
        for (x_diff, y_diff) in KNIGHT_MOVE_DELTAS {
            let (new_x, new_y) = (x + x_diff, y + y_diff);
            if new_x >= 0
                && new_x < x_dimension
                && new_y >= 0
                && new_y < y_dimension
                && !path.contains(&(new_x, new_y))
            {
                next_options.push((new_x, new_y));
            }
        }

        // The last thing added to the stack is popped first; we want to pop the lowest
        // warnsdorf distance first, so we want it added to the stack last. So: sort
        // by warnsdorf DESCENDING.
        next_options.sort_by(|(x1, y1), (x2, y2)| {
            let w1 = warnsdorf_distance(*x1, *y1, x_dimension, y_dimension);
            let w2 = warnsdorf_distance(*x2, *y2, x_dimension, y_dimension);
            w2.cmp(&w1)
        });
        next_options.iter().for_each(|coords| {
            let mut new_path = path.clone();
            new_path.insert(*coords);
            stack.push(new_path);
        });
    }

    panic!("Failed to find a tour path.")
}

// Calculate the total distance of a point from both edges of the board (distance from
// nearest vertical edge plus distance from nearest horizontal edge).
fn warnsdorf_distance(x: i32, y: i32, x_dimension: i32, y_dimension: i32) -> i32 {
    let max_x = x_dimension - 1;
    let max_y = y_dimension - 1;

    std::cmp::min(max_x - x, x) + std::cmp::min(max_y - y, y)
}
