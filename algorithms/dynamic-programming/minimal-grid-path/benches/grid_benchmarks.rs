// As usual, credit to Claude Sonnet for writing the benchmarks
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use minimal_grid_path::{minimal_cost_bottom_up, minimal_cost_top_down, minimal_cost_ucs};

fn create_test_grid(size: usize) -> Vec<Vec<u32>> {
    let mut grid = vec![vec![0; size]; size];

    // Fill with some semi-random but deterministic values
    for i in 0..size {
        for j in 0..size {
            // Using a simple formula to generate numbers between 1 and 100
            grid[i][j] = ((i * 7 + j * 13) % 100 + 1) as u32;
        }
    }
    grid
}

fn benchmark_grid_algorithms(c: &mut Criterion) {
    let size = 100; // Reduced from 500 to 100
    let grid = create_test_grid(size);

    // Convert grid to slice of slices format required by functions
    let grid_refs: Vec<&[u32]> = grid.iter().map(|row| row.as_slice()).collect();
    let grid_slice = grid_refs.as_slice();

    let mut group = c.benchmark_group("grid_path_100x100");

    // Configure for fewer samples
    group.sample_size(10); // Reduced from default 100 to 10

    group.bench_function("bottom_up", |b| {
        b.iter(|| minimal_cost_bottom_up(black_box(grid_slice)))
    });

    group.bench_function("top_down", |b| {
        b.iter(|| minimal_cost_top_down(black_box(grid_slice)))
    });

    group.bench_function("uniform_cost_search", |b| {
        b.iter(|| minimal_cost_ucs(black_box(grid_slice)))
    });

    group.finish();
}

criterion_group!(benches, benchmark_grid_algorithms);
criterion_main!(benches);
