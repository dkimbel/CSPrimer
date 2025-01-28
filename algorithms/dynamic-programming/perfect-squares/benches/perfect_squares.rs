use criterion::{black_box, criterion_group, criterion_main, Criterion};
use perfect_squares::{
    fewest_perfect_squares_bfs, fewest_perfect_squares_bottom_up, lowest_num_perfect_squares,
};

fn benchmark_perfect_squares(c: &mut Criterion) {
    let mut group = c.benchmark_group("perfect_squares");

    // Small input that all implementations can handle
    group.bench_function("bottom_up_23", |b| {
        b.iter(|| fewest_perfect_squares_bottom_up(black_box(23)))
    });
    group.bench_function("bfs_23", |b| {
        b.iter(|| fewest_perfect_squares_bfs(black_box(23)))
    });
    group.bench_function("recursive_23", |b| {
        b.iter(|| lowest_num_perfect_squares(black_box(23)))
    });

    // Medium input
    group.bench_function("bottom_up_120", |b| {
        b.iter(|| fewest_perfect_squares_bottom_up(black_box(120)))
    });
    group.bench_function("bfs_120", |b| {
        b.iter(|| fewest_perfect_squares_bfs(black_box(120)))
    });
    group.bench_function("recursive_120", |b| {
        b.iter(|| lowest_num_perfect_squares(black_box(120)))
    });

    group.bench_function("bottom_up_1111", |b| {
        b.iter(|| fewest_perfect_squares_bottom_up(black_box(1111)))
    });
    group.bench_function("bfs_1111", |b| {
        b.iter(|| fewest_perfect_squares_bfs(black_box(1111)))
    });
    group.bench_function("recursive_1111", |b| {
        b.iter(|| lowest_num_perfect_squares(black_box(1111)))
    });

    // Very large input (skip recursive version)
    group.bench_function("bottom_up_5000", |b| {
        b.iter(|| fewest_perfect_squares_bottom_up(black_box(5000)))
    });
    group.bench_function("bfs_5000", |b| {
        b.iter(|| fewest_perfect_squares_bfs(black_box(5000)))
    });

    group.finish();
}

criterion_group!(benches, benchmark_perfect_squares);
criterion_main!(benches);
