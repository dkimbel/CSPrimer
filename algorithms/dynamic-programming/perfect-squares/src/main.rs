use perfect_squares::fewest_perfect_squares_bfs;

fn main() {
    let target = 7777;
    println!(
        "Fewest perfect squares for {target}: {:?}",
        fewest_perfect_squares_bfs(target)
    );
}
