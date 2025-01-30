use minimal_grid_path::minimal_cost_ucs;

fn main() {
    let grid: &[&[u32]] = &[
        &[1, 1, 1, 1, 1, 1, 1],
        &[5, 5, 5, 5, 5, 5, 1],
        &[5, 5, 5, 5, 5, 5, 1],
        &[5, 5, 5, 5, 5, 5, 1],
        &[5, 5, 5, 5, 5, 5, 1],
        &[5, 5, 5, 5, 5, 5, 1],
        &[5, 5, 5, 5, 5, 5, 1],
    ];

    let solution = minimal_cost_ucs(grid);
    println!("{solution:?}");
}
