use std::collections::{HashSet, VecDeque};

pub fn fewest_perfect_squares_bottom_up(n: usize) -> Vec<usize> {
    // 'Key' to memo is just index into vec. At index 0, we place
    // an empty vec (there are no perfect squares that sum to 0).
    let mut memo: Vec<Vec<usize>> = vec![vec![]];
    let mut x: usize = 1;

    while x <= n {
        let lesser_or_equal_squares = get_perfect_squares_smaller_or_eq(x);
        let (new_square, prev_solution) = lesser_or_equal_squares
            .iter()
            .map(|new_square| (new_square, &memo[x - *new_square]))
            // take shortest list (list containing fewest perfect squares)
            .min_by(|(_, l1), (_, l2)| l1.len().cmp(&l2.len()))
            .unwrap();
        let mut new_solution = prev_solution.clone();
        new_solution.push(*new_square);
        memo.insert(x, new_solution);
        x += 1;
    }

    memo[n].clone()
}

pub fn fewest_perfect_squares_top_down(n: usize) -> Vec<usize> {
    // 'Key' to memo is just index into vec. At index 0, we place
    // an empty vec (there are no perfect squares that sum to 0).
    let mut memo: Vec<Vec<usize>> = vec![vec![]];

    fn inner(n: usize, memo: &mut Vec<Vec<usize>>) -> Vec<usize> {
        if let Some(memoized) = memo.get(n) {
            return memoized.clone();
        }

        let lesser_or_equal_squares = get_perfect_squares_smaller_or_eq(n);
        let (new_square, smaller_solution) = lesser_or_equal_squares
            .iter()
            .map(|new_square| {
                let smaller_solution = inner(n - *new_square, memo);
                (new_square, smaller_solution)
            })
            .min_by(|(_, l1), (_, l2)| l1.len().cmp(&l2.len()))
            .unwrap();

        let mut new_solution = smaller_solution.clone();
        new_solution.push(*new_square);
        memo.insert(n, new_solution.clone());
        new_solution
    }

    inner(n, &mut memo)
}

pub fn fewest_perfect_squares_bfs(n: usize) -> Vec<usize> {
    if n == 0 {
        return vec![];
    }

    let mut queue: VecDeque<(usize, Vec<usize>)> = VecDeque::from([(n, Vec::new())]);
    let mut visited: HashSet<usize> = HashSet::from([n]);

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

fn get_perfect_squares_smaller_or_eq(n: usize) -> Vec<usize> {
    let mut perfect_square_base: usize = 1;
    let mut perfect_squares: Vec<usize> = Vec::new();

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
        assert_eq!(fewest_perfect_squares_top_down(0), vec![]);
        assert_eq!(fewest_perfect_squares_bfs(0), vec![]);
    }

    #[test]
    fn test_one() {
        assert_eq!(fewest_perfect_squares_bottom_up(1), vec![1]);
        assert_eq!(fewest_perfect_squares_top_down(1), vec![1]);
        assert_eq!(fewest_perfect_squares_bfs(1), vec![1]);
    }

    #[test]
    fn test_three() {
        assert_eq!(fewest_perfect_squares_bottom_up(3), vec![1, 1, 1]);
        assert_eq!(fewest_perfect_squares_top_down(3), vec![1, 1, 1]);
        assert_eq!(fewest_perfect_squares_bfs(3), vec![1, 1, 1]);
    }

    #[test]
    fn test_four() {
        assert_eq!(fewest_perfect_squares_bottom_up(4), vec![4]);
        assert_eq!(fewest_perfect_squares_top_down(4), vec![4]);
        assert_eq!(fewest_perfect_squares_bfs(4), vec![4]);
    }

    #[test]
    fn test_eight() {
        assert_eq!(fewest_perfect_squares_bottom_up(8), vec![4, 4]);
        assert_eq!(fewest_perfect_squares_top_down(8), vec![4, 4]);
        assert_eq!(fewest_perfect_squares_bfs(8), vec![4, 4]);
    }

    #[test]
    fn test_larger() {
        assert_eq!(fewest_perfect_squares_bottom_up(23), vec![9, 9, 4, 1]);
        assert_eq!(fewest_perfect_squares_top_down(23), vec![9, 9, 4, 1]);
        assert_eq!(fewest_perfect_squares_bfs(23), vec![1, 4, 9, 9]);
    }
}
