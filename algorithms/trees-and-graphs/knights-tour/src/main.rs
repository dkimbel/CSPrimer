#[derive(Clone)]
struct BoardState {
    visited: Vec<Vec<bool>>,
    path: Vec<(usize, usize)>,
}

impl BoardState {
    fn new(x_dimension: usize, y_dimension: usize) -> BoardState {
        let visited = vec![vec![false; x_dimension]; y_dimension];
        let path = Vec::new();
        BoardState { visited, path }
    }

    fn clone_with_visited_coords(&self, x: usize, y: usize) -> BoardState {
        let mut new_board_state = self.clone();
        new_board_state.visited[y][x] = true;
        new_board_state.path.push((x, y));
        new_board_state
    }
}

fn main() {
    match find_tour_dfs(8, 8, false) {
        Some(tour_path) => println!("Successfully toured board! Path: {:#?}", &tour_path),
        None => println!("Failed to find a tour path."),
    }
}

fn find_tour_dfs(
    x_dimension: usize,
    y_dimension: usize,
    closed_tours_only: bool,
) -> Option<Vec<(usize, usize)>> {
    let starting_coords = (0, 0);
    let total_num_board_spaces = x_dimension * y_dimension;
    let mut stack = vec![(starting_coords, BoardState::new(x_dimension, y_dimension))];
    let mut next_options = Vec::with_capacity(8);

    while let Some(((x, y), board_state)) = stack.pop() {
        if board_state.path.len() == total_num_board_spaces
            && ((x, y) == starting_coords || !closed_tours_only)
        {
            return Some(board_state.path);
        } else if board_state.visited[y][x] {
            continue;
        }

        let can_move_long_upwards = y < y_dimension - 2;
        let can_move_short_upwards = y < y_dimension - 1;
        let can_move_long_downwards = y >= 2;
        let can_move_short_downwards = y >= 1;
        let can_move_long_right = x < x_dimension - 2;
        let can_move_short_right = x < x_dimension - 1;
        let can_move_long_left = x >= 2;
        let can_move_short_left = x >= 1;

        next_options.clear();
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

        // The last thing added to the stack is popped first; we want to pop the lowest
        // warnsdorf distance first, so we want it added to the stack last. So: sort
        // by warnsdorf DESCENDING.
        next_options.sort_by(|(x1, y1), (x2, y2)| {
            let w1 = warnsdorf_distance(*x1, *y1, x_dimension, y_dimension);
            let w2 = warnsdorf_distance(*x2, *y2, x_dimension, y_dimension);
            w2.cmp(&w1)
        });
        next_options.iter().for_each(|(new_x, new_y)| {
            let new_board_state = board_state.clone_with_visited_coords(x, y);
            stack.push(((*new_x, *new_y), new_board_state));
        });
    }

    None
}

// Calculate the total distance of a point from both edges of the board (distance from
// nearest vertical edge plus distance from nearest horizontal edge).
fn warnsdorf_distance(x: usize, y: usize, x_dimension: usize, y_dimension: usize) -> usize {
    let max_x = x_dimension - 1;
    let max_y = y_dimension - 1;

    std::cmp::min(max_x - x, x) + std::cmp::min(max_y - y, y)
}
