use std::collections::{BinaryHeap, HashMap, HashSet};

type Coords = (usize, usize);

pub fn minimal_cost_bottom_up(grid: &[&[u32]]) -> Vec<Coords> {
    // The Coords in our memo's value tell us where we came from when following the
    // optimal (lowest-cost) path. They're adjacent to the coords in the key for
    // that entry. This lets us reconstruct our path at the end, without having lots
    // of heap-allocated path vecs (one for each visited coord).
    let mut memo: Vec<Vec<(u32, Option<Coords>)>> =
        vec![vec![(0, None); grid[0].len()]; grid.len()];

    let max_x = grid[0].len() - 1;
    let max_y = grid.len() - 1;

    // We work through the grid by row, starting with the topmost. The same way you
    // read: top to bottom, left to right.
    for (y, row) in grid.iter().enumerate() {
        for (x, self_cost) in row.iter().enumerate() {
            let maybe_coords_to_left = if x > 0 { Some((x - 1, y)) } else { None };
            let maybe_coords_above = if y > 0 { Some((x, y - 1)) } else { None };

            let (min_prev_cost, maybe_prev_coords) =
                match (maybe_coords_to_left, maybe_coords_above) {
                    (None, None) => (0, None),
                    (Some((right_x, right_y)), Some((down_x, down_y))) => {
                        let (right_cost, _) = memo[right_y][right_x];
                        let (down_cost, _) = memo[down_y][down_x];
                        if right_cost < down_cost {
                            (right_cost, Some((right_x, right_y)))
                        } else {
                            (down_cost, Some((down_x, down_y)))
                        }
                    }
                    (Some((right_x, right_y)), None) => {
                        let (right_cost, _) = memo[right_y][right_x];
                        (right_cost, Some((right_x, right_y)))
                    }
                    (None, Some((down_x, down_y))) => {
                        let (down_cost, _) = memo[down_y][down_x];
                        (down_cost, Some((down_x, down_y)))
                    }
                };
            memo[y][x] = (min_prev_cost + self_cost, maybe_prev_coords);
        }
    }

    // reconstruct path, working backwards from the end
    let mut path = vec![(max_x, max_y)];
    let mut backtracking_coords = (max_x, max_y);

    while let (_, Some(prev_coords)) = memo[backtracking_coords.1][backtracking_coords.0] {
        path.push(prev_coords);
        backtracking_coords = prev_coords;
    }
    path.reverse();
    path
}

pub fn minimal_cost_top_down(grid: &[&[u32]]) -> Vec<Coords> {
    // The Coords in our memo are pointing forward to the next coords along
    // our optimal path. They let us reconstruct the path at the end.
    let mut memo: HashMap<Coords, (u32, Coords)> = HashMap::new();

    // Return the cost from the current coords to the end, along with the current
    // coords we just evaluated.
    fn inner(
        (x, y): Coords,
        grid: &[&[u32]],
        memo: &mut HashMap<Coords, (u32, Coords)>,
    ) -> (u32, Coords) {
        if let Some((cost, _)) = memo.get(&(x, y)) {
            return (*cost, (x, y));
        }

        let coords_cost = grid[y][x];
        let adjacent_coords = get_adjacent_coords((x, y), grid);

        if adjacent_coords.is_empty() {
            // Base case! We've reached the lower right-hand corner.
            return (coords_cost, (x, y));
        }

        let (smaller_solution_cost, smaller_solution_coords) = adjacent_coords
            .iter()
            .map(|adj_coords| inner(*adj_coords, grid, memo))
            .min_by(|(cost1, _), (cost2, _)| cost1.cmp(cost2))
            .unwrap();
        let new_solution_cost = smaller_solution_cost + coords_cost;
        memo.insert((x, y), (new_solution_cost, smaller_solution_coords));
        (new_solution_cost, (x, y))
    }

    inner((0, 0), grid, &mut memo);
    // reconstruct path
    let mut current_path_coords = (0, 0);
    let mut path = vec![current_path_coords];
    while let Some((_, next_path_coords)) = memo.get(&current_path_coords) {
        path.push(*next_path_coords);
        current_path_coords = *next_path_coords;
    }
    path
}

struct SearchParams {
    cost_so_far: u32,
    path: Vec<Coords>,
}

impl PartialEq for SearchParams {
    fn eq(&self, other: &Self) -> bool {
        self.cost_so_far == other.cost_so_far
    }
}

impl Eq for SearchParams {} // reuse PartialEq

impl PartialOrd for SearchParams {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other)) // reuse Ord
    }
}

impl Ord for SearchParams {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Note how order of self and other is flipped! This way, the higher
        // cost is "lower" according to ord... such that higher-cost paths
        // will be lower-priority in our binary heap during search.
        other.cost_so_far.cmp(&self.cost_so_far)
    }
}

// Where "ucs" means "Uniform Cost Search"
pub fn minimal_cost_ucs(grid: &[&[u32]]) -> Vec<Coords> {
    let mut visited: HashSet<Coords> = HashSet::new();
    let max_x = grid[0].len() - 1;
    let max_y = grid.len() - 1;
    let target_coords = (max_x, max_y);

    let mut priority_queue: BinaryHeap<SearchParams> = BinaryHeap::from([SearchParams {
        cost_so_far: grid[0][0],
        path: vec![(0, 0)],
    }]);

    while let Some(SearchParams { cost_so_far, path }) = priority_queue.pop() {
        let visiting = path.last().unwrap();
        let (x, y) = *visiting;

        if visited.contains(&(x, y)) {
            continue;
        } else if (x, y) == target_coords {
            return path;
        }

        // explore space to the right
        if x < max_x {
            let mut path = path.clone();
            path.push((x + 1, y));
            priority_queue.push(SearchParams {
                cost_so_far: cost_so_far + grid[y][x + 1],
                path,
            })
        }

        // explore space down
        if y < max_y {
            let mut path = path.clone();
            path.push((x, y + 1));
            priority_queue.push(SearchParams {
                cost_so_far: cost_so_far + grid[y + 1][x],
                path,
            })
        }

        visited.insert((x, y));
    }

    panic!("Failed to reach target coords!");
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
        assert_eq!(minimal_cost_ucs(test_grid), expected_solution);
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
        assert_eq!(minimal_cost_ucs(test_grid), expected_solution);
    }
}
