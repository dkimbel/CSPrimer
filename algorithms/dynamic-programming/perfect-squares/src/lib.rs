use std::collections::{HashMap, HashSet, VecDeque};

pub fn fewest_perfect_squares_bottom_up(n: u32) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }

    let mut memo: HashMap<u32, Vec<u32>> = HashMap::from([(0, vec![])]);
    let mut x = 1;

    while x <= n {
        let solution = get_perfect_squares_smaller_or_eq(x)
            .iter()
            .map(|m| {
                let mut solution = vec![*m];
                solution.extend(memo.get(&(x - *m)).unwrap());
                solution
            })
            // take shortest list (list containing fewest perfect squares)
            .min_by(|l1, l2| l1.len().cmp(&l2.len()))
            .unwrap();
        memo.insert(x, solution);
        x += 1;
    }

    memo.get(&n).unwrap().clone()
}

// This works, but it easily fails with a stack overflow.
pub fn lowest_num_perfect_squares(n: u32) -> u32 {
    let mut memo: HashMap<u32, u32> = HashMap::new();

    fn inner(n: u32, memo: &mut HashMap<u32, u32>) -> u32 {
        if n == 0 {
            return 0;
        } else if let Some(memoized) = memo.get(&n) {
            return *memoized;
        }

        let solution = 1 + get_perfect_squares_smaller_or_eq(n)
            .iter()
            .map(|m| inner(n - *m, memo))
            .min()
            .unwrap();

        memo.insert(n, solution);
        solution
    }

    inner(n, &mut memo)
}

pub fn fewest_perfect_squares_bfs(n: u32) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }

    let mut queue: VecDeque<(u32, Vec<u32>)> = VecDeque::from([(n, Vec::new())]);
    let mut visited: HashSet<u32> = HashSet::from([n]);

    while let Some((n, path)) = queue.pop_front() {
        for square in get_perfect_squares_smaller_or_eq(n) {
            let new_target = n - square;
            if visited.contains(&new_target) {
                continue;
            }

            let mut path = path.clone();
            path.push(square);

            if new_target == 0 {
                return path;
            }

            queue.push_back((new_target, path));
            visited.insert(new_target);
        }
    }

    panic!("Failed to find perfect squares for {n}");
}

fn get_perfect_squares_smaller_or_eq(n: u32) -> Vec<u32> {
    let mut perfect_square_base: u32 = 1;
    let mut perfect_squares: Vec<u32> = Vec::new();

    loop {
        let perfect_square = perfect_square_base.pow(2);
        if perfect_square > n {
            break;
        }

        perfect_squares.push(perfect_square);
        perfect_square_base += 1;
    }

    perfect_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(fewest_perfect_squares_bottom_up(0), vec![]);
        assert_eq!(lowest_num_perfect_squares(0), 0);
        assert_eq!(fewest_perfect_squares_bfs(0), vec![]);
    }

    #[test]
    fn test_one() {
        assert_eq!(fewest_perfect_squares_bottom_up(1), vec![1]);
        assert_eq!(lowest_num_perfect_squares(1), 1);
        assert_eq!(fewest_perfect_squares_bfs(1), vec![1]);
    }

    #[test]
    fn test_three() {
        assert_eq!(fewest_perfect_squares_bottom_up(3), vec![1, 1, 1]);
        assert_eq!(lowest_num_perfect_squares(3), 3);
        assert_eq!(fewest_perfect_squares_bfs(3), vec![1, 1, 1]);
    }

    #[test]
    fn test_four() {
        assert_eq!(fewest_perfect_squares_bottom_up(4), vec![4]);
        assert_eq!(lowest_num_perfect_squares(4), 1);
        assert_eq!(fewest_perfect_squares_bfs(4), vec![4]);
    }

    #[test]
    fn test_eight() {
        assert_eq!(fewest_perfect_squares_bottom_up(8), vec![4, 4]);
        assert_eq!(lowest_num_perfect_squares(8), 2);
        assert_eq!(fewest_perfect_squares_bfs(8), vec![4, 4]);
    }

    #[test]
    fn test_larger() {
        assert_eq!(fewest_perfect_squares_bottom_up(23), vec![1, 4, 9, 9]);
        assert_eq!(lowest_num_perfect_squares(23), 4);
        assert_eq!(fewest_perfect_squares_bfs(23), vec![1, 4, 9, 9]);
    }
}
