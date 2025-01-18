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
}

fn main() {
    match solve_dfs(8, 8) {
        Some(tour_path) => println!("Successfully toured board! Path: {:#?}", &tour_path),
        None => println!("Failed to find a tour path."),
    }
}

fn solve_dfs(x_dimension: usize, y_dimension: usize) -> Option<Vec<(usize, usize)>> {
    let total_num_board_spaces = x_dimension * y_dimension;
    let mut stack = vec![((0, 0), BoardState::new(x_dimension, y_dimension))];

    while let Some(((x, y), board_state)) = stack.pop() {
        if board_state.visited[y][x] {
            continue;
        }

        let mut board_state = board_state.clone();
        board_state.visited[y][x] = true;

        board_state.path.push((x, y));
        if board_state.path.len() == total_num_board_spaces {
            return Some(board_state.path);
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
            stack.push(((x + 1, y + 2), board_state.clone()));
        }
        if can_move_long_upwards && can_move_short_left {
            stack.push(((x - 1, y + 2), board_state.clone()));
        }
        if can_move_long_downwards && can_move_short_right {
            stack.push(((x + 1, y - 2), board_state.clone()));
        }
        if can_move_long_downwards && can_move_short_left {
            stack.push(((x - 1, y - 2), board_state.clone()));
        }
        if can_move_short_upwards && can_move_long_right {
            stack.push(((x + 2, y + 1), board_state.clone()));
        }
        if can_move_short_upwards && can_move_long_left {
            stack.push(((x - 2, y + 1), board_state.clone()));
        }
        if can_move_short_downwards && can_move_long_right {
            stack.push(((x + 2, y - 1), board_state.clone()));
        }
        if can_move_short_downwards && can_move_long_left {
            stack.push(((x - 2, y - 1), board_state.clone()));
        }
    }

    None
}
