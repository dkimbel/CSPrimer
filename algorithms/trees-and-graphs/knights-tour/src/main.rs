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

    fn clone_with_added_coords(&self, x: usize, y: usize) -> BoardState {
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

        if can_move_long_upwards && can_move_short_right {
            stack.push(((x + 1, y + 2), board_state.clone_with_added_coords(x, y)));
        }
        if can_move_long_upwards && can_move_short_left {
            stack.push(((x - 1, y + 2), board_state.clone_with_added_coords(x, y)));
        }
        if can_move_long_downwards && can_move_short_right {
            stack.push(((x + 1, y - 2), board_state.clone_with_added_coords(x, y)));
        }
        if can_move_long_downwards && can_move_short_left {
            stack.push(((x - 1, y - 2), board_state.clone_with_added_coords(x, y)));
        }
        if can_move_short_upwards && can_move_long_right {
            stack.push(((x + 2, y + 1), board_state.clone_with_added_coords(x, y)));
        }
        if can_move_short_upwards && can_move_long_left {
            stack.push(((x - 2, y + 1), board_state.clone_with_added_coords(x, y)));
        }
        if can_move_short_downwards && can_move_long_right {
            stack.push(((x + 2, y - 1), board_state.clone_with_added_coords(x, y)));
        }
        if can_move_short_downwards && can_move_long_left {
            stack.push(((x - 2, y - 1), board_state.clone_with_added_coords(x, y)));
        }
    }

    None
}
