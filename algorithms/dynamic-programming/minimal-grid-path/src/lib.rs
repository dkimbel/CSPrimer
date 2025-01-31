use std::collections::BinaryHeap;

type Coords = (usize, usize);

pub fn minimal_cost_bottom_up(grid: &[&[u32]]) -> Vec<Coords> {
    if grid.is_empty() || grid[0].is_empty() {
        return vec![];
    }

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
                    (Some((left_x, left_y)), Some((up_x, up_y))) => {
                        let (left_cost, _) = memo[left_y][left_x];
                        let (up_cost, _) = memo[up_y][up_x];
                        if left_cost < up_cost {
                            (left_cost, Some((left_x, left_y)))
                        } else {
                            (up_cost, Some((up_x, up_y)))
                        }
                    }
                    (Some((left_x, left_y)), None) => {
                        let (left_cost, _) = memo[left_y][left_x];
                        (left_cost, Some((left_x, left_y)))
                    }
                    (None, Some((up_x, up_y))) => {
                        let (up_cost, _) = memo[up_y][up_x];
                        (up_cost, Some((up_x, up_y)))
                    }
                };
            memo[y][x] = (min_prev_cost + self_cost, maybe_prev_coords);
        }
    }

    // reconstruct path, working backwards from the end
    let mut backtracking_coords = (max_x, max_y);
    let mut path = vec![backtracking_coords];

    while let (_, Some(prev_coords)) = memo[backtracking_coords.1][backtracking_coords.0] {
        path.push(prev_coords);
        backtracking_coords = prev_coords;
    }
    path.reverse();
    path
}

pub fn minimal_cost_top_down(grid: &[&[u32]]) -> Vec<Coords> {
    if grid.is_empty() || grid[0].is_empty() {
        return vec![];
    }

    // The Coords in our memo are pointing forward to the next coords along
    // our optimal path. They let us reconstruct the path at the end.
    let mut memo: Vec<Vec<Option<(u32, Coords)>>> = vec![vec![None; grid[0].len()]; grid.len()];

    // Return the cost from the current coords to the end, along with the current
    // coords we just evaluated.
    fn inner(
        (x, y): Coords,
        grid: &[&[u32]],
        memo: &mut Vec<Vec<Option<(u32, Coords)>>>,
    ) -> (u32, Coords) {
        if let Some((cost, _)) = memo[y][x] {
            return (cost, (x, y));
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
        memo[y][x] = Some((new_solution_cost, smaller_solution_coords));
        (new_solution_cost, (x, y))
    }

    inner((0, 0), grid, &mut memo);
    // reconstruct path
    let mut current_path_coords = (0, 0);
    let mut path = vec![current_path_coords];
    while let Some((_, next_path_coords)) = memo[current_path_coords.1][current_path_coords.0] {
        path.push(next_path_coords);
        current_path_coords = next_path_coords;
    }
    path
}

struct SearchParams {
    cost_so_far: u32,
    visiting_from: Option<Coords>,
    visiting: Coords,
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
    if grid.is_empty() || grid[0].is_empty() {
        return vec![];
    }

    // Used to reconstruct path at end; if we want to know where we visited (2, 1) from, we'd
    // call visited_from[1][2] (y indexed before x) and find something like Some((1, 1)).
    // Based on our logic, we'll never add visited_from[0][0]; that's fine, though. We can only
    // move down and to the right anyway; we'll never try to revisit (0, 0). If we really needed
    // to fix that, we could have the memo store (cost, Option<Coords>) instead; we'd at least
    // fill in a cost for (0, 0).
    let mut visited_from: Vec<Vec<Option<Coords>>> = vec![vec![None; grid[0].len()]; grid.len()];
    let max_x = grid[0].len() - 1;
    let max_y = grid.len() - 1;
    let target_coords = (max_x, max_y);

    let mut priority_queue: BinaryHeap<SearchParams> = BinaryHeap::from([SearchParams {
        cost_so_far: grid[0][0],
        visiting_from: None,
        visiting: (0, 0),
    }]);

    while let Some(SearchParams {
        cost_so_far,
        visiting_from,
        visiting,
    }) = priority_queue.pop()
    {
        let (x, y) = visiting;

        if visited_from[y][x].is_some() {
            // we've already visited these coords
            continue;
        }
        visited_from[y][x] = visiting_from;
        if (x, y) == target_coords {
            break;
        }

        // explore space to the right
        if x < max_x {
            priority_queue.push(SearchParams {
                cost_so_far: cost_so_far + grid[y][x + 1],
                visiting_from: Some(visiting),
                visiting: (x + 1, y),
            })
        }

        // explore space down
        if y < max_y {
            priority_queue.push(SearchParams {
                cost_so_far: cost_so_far + grid[y + 1][x],
                visiting_from: Some(visiting),
                visiting: (x, y + 1),
            })
        }
    }

    // reconstruct path (starting from end)
    let mut current_path_coords = target_coords;
    let mut path = vec![current_path_coords];
    while let Some(visited_from_coords) = visited_from[current_path_coords.1][current_path_coords.0]
    {
        path.push(visited_from_coords);
        current_path_coords = visited_from_coords;
    }
    path.reverse();
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
    fn empty_y() {
        let test_grid: &[&[u32]] = &[];
        let expected_solution = vec![];
        assert_eq!(minimal_cost_top_down(test_grid), expected_solution);
        assert_eq!(minimal_cost_bottom_up(test_grid), expected_solution);
        assert_eq!(minimal_cost_ucs(test_grid), expected_solution);
    }

    #[test]
    fn empty_x() {
        let test_grid: &[&[u32]] = &[&[]];
        let expected_solution = vec![];
        assert_eq!(minimal_cost_top_down(test_grid), expected_solution);
        assert_eq!(minimal_cost_bottom_up(test_grid), expected_solution);
        assert_eq!(minimal_cost_ucs(test_grid), expected_solution);
    }

    #[test]
    fn one_by_one() {
        let test_grid: &[&[u32]] = &[&[5]];
        let expected_solution = vec![(0, 0)];
        assert_eq!(minimal_cost_top_down(test_grid), expected_solution);
        assert_eq!(minimal_cost_bottom_up(test_grid), expected_solution);
        assert_eq!(minimal_cost_ucs(test_grid), expected_solution);
    }

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
