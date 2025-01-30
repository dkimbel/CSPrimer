use std::cmp;
use std::collections::HashMap;

type Coords = (usize, usize);

pub fn minimal_cost_bottom_up(grid: &[&[u32]]) -> Vec<Coords> {
    let mut memo: HashMap<Coords, (u32, Vec<Coords>)> = HashMap::new();

    let max_x = grid[0].len() - 1;
    let max_y = grid.len() - 1;
    // We plan to evaluate diagonal strips of the grid, one "/"-shaped slice at a time.
    // Starting at the lower right corner, it will take max_offset iterations to evaluate
    // the upper left corner. We call the following variables "lower left" because they
    // form the lower left point of the "/" shape.
    let mut lower_left_x = max_x;
    let mut lower_left_y = max_y;

    loop {
        // When combined with lower_left_x and lower_left_y, 'offset' will give us the
        // next coords in the "/"-shaped slice of the grid we're currently evaluating.
        for offset in 0..=cmp::min(max_x - lower_left_x, lower_left_y) {
            let (x, y) = (lower_left_x + offset, lower_left_y - offset);
            let self_cost = grid[y][x];

            let maybe_coords_to_right = if x < max_x { Some((x + 1, y)) } else { None };
            let maybe_coords_below = if y < max_y { Some((x, y + 1)) } else { None };

            let (min_next_cost, min_next_path) = match (maybe_coords_to_right, maybe_coords_below) {
                (None, None) => (0, Vec::new()),
                (Some((right_x, right_y)), Some((down_x, down_y))) => {
                    let (right_cost, right_path) = memo.get(&(right_x, right_y)).unwrap();
                    let (down_cost, down_path) = memo.get(&(down_x, down_y)).unwrap();
                    if right_cost < down_cost {
                        (*right_cost, right_path.clone())
                    } else {
                        (*down_cost, down_path.clone())
                    }
                }
                (Some((right_x, right_y)), None) => {
                    let (right_cost, right_path) = memo.get(&(right_x, right_y)).unwrap();
                    (*right_cost, right_path.clone())
                }
                (None, Some((down_x, down_y))) => {
                    let (down_cost, down_path) = memo.get(&(down_x, down_y)).unwrap();
                    (*down_cost, down_path.clone())
                }
            };
            let mut self_path = min_next_path;
            self_path.push((x, y));
            if (x, y) == (0, 0) {
                self_path.reverse(); // path was built back-to-front, must be reversed
                return self_path;
            }
            memo.insert((x, y), (self_cost + min_next_cost, self_path));
        }
        // By reducing the value of lower_left_x or lower_left_y, we're moving on to
        // the next "/"-shaped slice (one spot to the left our most recent slice).
        match lower_left_x.checked_sub(1) {
            Some(new_x) => lower_left_x = new_x,
            None => lower_left_y -= 1,
        }
    }
}

pub fn minimal_cost_top_down(grid: &[&[u32]]) -> Vec<Coords> {
    let mut memo: HashMap<Coords, (u32, Vec<Coords>)> = HashMap::new();

    fn inner(
        (x, y): Coords,
        grid: &[&[u32]],
        memo: &mut HashMap<Coords, (u32, Vec<Coords>)>,
    ) -> (u32, Vec<Coords>) {
        if let Some((cost, path)) = memo.get(&(x, y)) {
            return (*cost, path.clone());
        }

        let coords_cost = grid[y][x];
        let adjacent_coords = get_adjacent_coords((x, y), grid);

        if adjacent_coords.is_empty() {
            // Base case! We've reached the lower right-hand corner.
            return (coords_cost, vec![(x, y)]);
        }

        let (smaller_solution_cost, smaller_solution_path) = adjacent_coords
            .iter()
            .map(|adj_coords| inner(*adj_coords, grid, memo))
            .min_by(|(cost1, _), (cost2, _)| cost1.cmp(cost2))
            .unwrap();
        let new_solution_cost = smaller_solution_cost + coords_cost;
        let mut new_solution_path = smaller_solution_path;
        new_solution_path.push((x, y));
        memo.insert((x, y), (new_solution_cost, new_solution_path.clone()));
        (new_solution_cost, new_solution_path)
    }

    let (_cost, mut path) = inner((0, 0), grid, &mut memo);
    path.reverse(); // path started from end, then had earlier coords pushed to it
    path
}

fn get_adjacent_coords((from_x, from_y): Coords, grid: &[&[u32]]) -> Vec<Coords> {
    let mut adjacent_coords = Vec::new();
    let max_y_exclusive = grid.len();
    let max_x_exclusive = grid[0].len();

    if from_x + 1 < max_x_exclusive {
        adjacent_coords.push((from_x + 1, from_y));
    }
    if from_y + 1 < max_y_exclusive {
        adjacent_coords.push((from_x, from_y + 1));
    }
    adjacent_coords
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_by_five() {
        let test_grid: &[&[u32]] = &[
            &[131, 673, 234, 103, 18],
            &[201, 96, 342, 965, 150],
            &[630, 803, 746, 422, 111],
            &[537, 699, 497, 121, 956],
            &[805, 732, 524, 37, 331],
        ];

        let expected_solution = vec![
            (0, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (2, 2),
            (3, 2),
            (3, 3),
            (3, 4),
            (4, 4),
        ];
        assert_eq!(minimal_cost_top_down(test_grid), expected_solution);
        assert_eq!(minimal_cost_bottom_up(test_grid), expected_solution);
    }

    #[test]
    fn seven_by_seven() {
        let test_grid: &[&[u32]] = &[
            &[1, 1, 1, 1, 1, 1, 1],
            &[5, 5, 5, 5, 5, 5, 1],
            &[5, 5, 5, 5, 5, 5, 1],
            &[5, 5, 5, 5, 5, 5, 1],
            &[5, 5, 5, 5, 5, 5, 1],
            &[5, 5, 5, 5, 5, 5, 1],
            &[5, 5, 5, 5, 5, 5, 1],
        ];

        let expected_solution = vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (6, 1),
            (6, 2),
            (6, 3),
            (6, 4),
            (6, 5),
            (6, 6),
        ];
        assert_eq!(minimal_cost_top_down(test_grid), expected_solution);
        assert_eq!(minimal_cost_bottom_up(test_grid), expected_solution);
    }
}
