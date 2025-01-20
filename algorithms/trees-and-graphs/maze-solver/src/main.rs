use colored::Colorize;
use std::collections::{BinaryHeap, HashSet};

const SMALL: &str = include_str!("../resources/small.txt");
const MEDIUM: &str = include_str!("../resources/medium.txt");
const LARGE: &str = include_str!("../resources/large.txt");

fn main() {
    let mut maze = MazeSolver::new(MEDIUM);
    maze.find_least_expensive_path();
    maze.print_path_on_grid();
}

#[derive(Clone, Copy)]
enum TileType {
    Field,
    Bog,
    Mountain,
}

impl TileType {
    fn cost_to_enter(&self) -> usize {
        match self {
            TileType::Field => 1,
            TileType::Bog => 3,
            TileType::Mountain => 10,
        }
    }
}

#[derive(Clone, Copy)]
struct Tile {
    tile_type: TileType,
    lowest_cost_to_reach: Option<usize>,
    lowest_cost_from: Option<(usize, usize)>,
}

impl Tile {
    fn new(tile_type: TileType) -> Self {
        Tile {
            tile_type,
            lowest_cost_to_reach: None,
            lowest_cost_from: None,
        }
    }
}

#[derive(PartialEq, Eq)]
struct SearchStep {
    visiting: (usize, usize),
    from: Option<(usize, usize)>,
    cost_so_far: usize,
    lowest_possible_cost_to_end: usize,
}

impl PartialOrd for SearchStep {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other)) // use our custom implementation of Ord
    }
}

// We plan to use a BinaryHeap as our PriorityQueue. It will sort itself based on our struct's
// implementation of `Ord` -- so we define our prioritization heuristic here.
impl Ord for SearchStep {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_cost = self.cost_so_far + self.lowest_possible_cost_to_end;
        let other_cost = other.cost_so_far + other.lowest_possible_cost_to_end;
        // sort the heap to prioritize lowest cost -- by it will try to pop greatest item first,
        // so tell it that a lower cost is "greater"
        use std::cmp::Ordering::*;
        match self_cost.cmp(&other_cost) {
            Less => Greater,
            Greater => Less,
            // Tiebreaker: prefer horizontal moves over diagonal, to avoid unnecessary zigging
            // and zagging off-course (visually, not in terms of official cost).
            Equal => match (self.is_straight(), other.is_straight()) {
                (true, false) => Greater,
                (false, true) => Less,
                _ => Equal,
            },
        }
    }
}

impl SearchStep {
    fn new(
        (x, y): (usize, usize),
        from: Option<(usize, usize)>,
        cost_so_far: usize,
        (end_x, end_y): (usize, usize),
    ) -> Self {
        let x_diff = if x >= end_x { x - end_x } else { end_x - x };
        let y_diff = if y >= end_y { y - end_y } else { end_y - y };
        // Since we can travel diagonally, the smaller diff doesn't matter at all. Suppose we
        // need to go 15 steps south and 20 steps east; we can first go 15 steps southeast, then
        // go 5 east (15 + 5 = 20).
        let lowest_possible_cost_to_end = std::cmp::max(x_diff, y_diff);

        SearchStep {
            visiting: (x, y),
            from,
            cost_so_far,
            lowest_possible_cost_to_end,
        }
    }

    fn is_straight(&self) -> bool {
        let (x, y) = self.visiting;
        if let Some((from_x, from_y)) = self.from {
            from_x == x || from_y == y
        } else {
            false
        }
    }
}

struct MazeSolver {
    start: (usize, usize),
    end: (usize, usize),
    height: usize,
    width: usize,
    grid: Vec<Vec<Tile>>,
}

impl MazeSolver {
    fn new(input: &str) -> Self {
        let mut grid: Vec<Vec<Tile>> = Vec::new();
        let mut start: Option<(usize, usize)> = None;
        let mut end: Option<(usize, usize)> = None;

        for (y, line) in input.lines().enumerate() {
            let mut row: Vec<Tile> = Vec::new();
            for (x, tile_char) in line.chars().enumerate() {
                match tile_char {
                    ' ' => row.push(Tile::new(TileType::Field)),
                    '.' => row.push(Tile::new(TileType::Bog)),
                    '#' => row.push(Tile::new(TileType::Mountain)),
                    'O' => {
                        if start.is_none() {
                            start = Some((x, y));
                            row.push(Tile::new(TileType::Field));
                        } else {
                            panic!("Multiple start points found");
                        }
                    }
                    'X' => {
                        if end.is_none() {
                            end = Some((x, y));
                            row.push(Tile::new(TileType::Field));
                        } else {
                            panic!("Multiple end points found");
                        }
                    }
                    _ => panic!("Cannot parse input char '{}'", tile_char),
                }
            }
            grid.push(row);
        }

        MazeSolver {
            start: start.unwrap(),
            end: end.unwrap(),
            height: grid.len(),
            width: grid[0].len(),
            grid,
        }
    }

    /// Find all possible moves to adjacent characters (up to eight). Takes a mutable reference to a
    /// Vec, which will be _cleared_ and replaced with a new batch of moves.
    fn next_legal_moves(&self, coords: (usize, usize), next_coords: &mut Vec<(usize, usize)>) {
        next_coords.clear();
        let (x, y) = coords;
        let max_x = self.width - 1;
        let max_y = self.height - 1;

        let maybe_north_y = y.checked_sub(1);
        let maybe_south_y = if y >= max_y { None } else { Some(y + 1) };
        let maybe_east_x = if x >= max_x { None } else { Some(x + 1) };
        let maybe_west_x = x.checked_sub(1);

        if let Some(north_y) = maybe_north_y {
            next_coords.push((x, north_y));
            if let Some(east_x) = maybe_east_x {
                next_coords.push((east_x, north_y));
            }
            if let Some(west_x) = maybe_west_x {
                next_coords.push((west_x, north_y));
            }
        }

        if let Some(east_x) = maybe_east_x {
            next_coords.push((east_x, y));
        }
        if let Some(west_x) = maybe_west_x {
            next_coords.push((west_x, y));
        }

        if let Some(south_y) = maybe_south_y {
            next_coords.push((x, south_y));
            if let Some(east_x) = maybe_east_x {
                next_coords.push((east_x, south_y));
            }
            if let Some(west_x) = maybe_west_x {
                next_coords.push((west_x, south_y));
            }
        }
    }

    fn find_least_expensive_path(&mut self) -> Vec<(usize, usize)> {
        let mut priority_queue = BinaryHeap::from([SearchStep::new(self.start, None, 0, self.end)]);
        let mut next_moves: Vec<(usize, usize)> = Vec::new();

        while let Some(SearchStep {
            visiting,
            from,
            cost_so_far,
            ..
        }) = priority_queue.pop()
        {
            let (x, y) = visiting;
            let tile = &mut self.grid[y][x];

            // If we already reached these coordinates by paying an equal or lower cost, the path
            // we're currently exploring cannot possibly be an improvement.
            if let Some(lowest_cost_so_far) = tile.lowest_cost_to_reach {
                if lowest_cost_so_far <= cost_so_far {
                    continue;
                }
            }

            tile.lowest_cost_to_reach = Some(cost_so_far);
            tile.lowest_cost_from = from;
            if visiting == self.end {
                return self.generate_path();
            }

            self.next_legal_moves(visiting, &mut next_moves);
            for (x, y) in next_moves.iter() {
                let to_tile = &self.grid[*y][*x];
                priority_queue.push(SearchStep::new(
                    (*x, *y),
                    Some(visiting),
                    cost_so_far + to_tile.tile_type.cost_to_enter(),
                    self.end,
                ));
            }
        }

        panic!("Failed to find a path");
    }

    fn generate_path(&self) -> Vec<(usize, usize)> {
        let mut path: Vec<(usize, usize)> = Vec::from([self.end]);

        // Build the path backwards, starting from the end, since each tile (including our
        // end tile) knows which coordinates led to it.
        let (end_x, end_y) = self.end;
        let mut tile = self.grid[end_y][end_x];

        while let Some((prev_x, prev_y)) = tile.lowest_cost_from {
            path.push((prev_x, prev_y));
            tile = self.grid[prev_y][prev_x];
        }

        // Since we built our path backwards, we need to reverse it before returning.
        path.reverse();
        path
    }

    fn print_path_on_grid(&self) {
        let path: HashSet<(usize, usize)> =
            self.generate_path().into_iter().collect::<HashSet<_>>();

        for (y, row) in self.grid.iter().enumerate() {
            let mut s = String::from("");
            for (x, tile) in row.iter().enumerate() {
                if path.contains(&(x, y)) {
                    let glyph = if (x, y) == self.start {
                        "O"
                    } else if (x, y) == self.end {
                        "X"
                    } else {
                        "o"
                    };
                    let colored_glyph = match tile.tile_type {
                        TileType::Field => glyph.green(),
                        TileType::Bog => glyph.yellow(),
                        TileType::Mountain => glyph.red(),
                    };
                    s.push_str(&format!("{}", colored_glyph));
                } else {
                    let tile_char = match tile.tile_type {
                        TileType::Field => ' ',
                        TileType::Bog => '.',
                        TileType::Mountain => '#',
                    };
                    s.push(tile_char);
                }
            }
            println!("{}", s);
        }
    }
}
