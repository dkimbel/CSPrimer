// Benchmarks courtesy of Claude Sonnet
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use house_robber::{
    almost_best_iterative, best_iterative, find_max_nonadjacent_sum, memoized_recursive,
    naive_recursive,
};

fn bench_house_robber(c: &mut Criterion) {
    // Create test cases of different sizes
    let small_input: Vec<u32> = vec![2, 7, 9, 3, 1];
    let medium_input: Vec<u32> = (0..100).collect();
    let large_input: Vec<u32> = (0..1000).collect();

    let mut group = c.benchmark_group("house_robber");

    // Benchmark small input
    group.bench_function("best_iterative_small", |b| {
        b.iter(|| best_iterative(black_box(&small_input)))
    });
    group.bench_function("almost_best_iterative_small", |b| {
        b.iter(|| almost_best_iterative(black_box(&small_input)))
    });
    group.bench_function("max_nonadjacent_small", |b| {
        b.iter(|| find_max_nonadjacent_sum(black_box(&small_input)))
    });
    group.bench_function("memoized_recursive_small", |b| {
        b.iter(|| memoized_recursive(black_box(&small_input)))
    });
    group.bench_function("naive_recursive_small", |b| {
        b.iter(|| naive_recursive(black_box(&small_input)))
    });

    // Benchmark medium input
    group.bench_function("best_iterative_medium", |b| {
        b.iter(|| best_iterative(black_box(&medium_input)))
    });
    group.bench_function("almost_best_iterative_medium", |b| {
        b.iter(|| almost_best_iterative(black_box(&medium_input)))
    });
    group.bench_function("max_nonadjacent_medium", |b| {
        b.iter(|| find_max_nonadjacent_sum(black_box(&medium_input)))
    });
    group.bench_function("memoized_recursive_medium", |b| {
        b.iter(|| memoized_recursive(black_box(&medium_input)))
    });
    // Skip naive recursive for medium/large as it would be too slow

    // Benchmark large input
    group.bench_function("best_iterative_large", |b| {
        b.iter(|| best_iterative(black_box(&large_input)))
    });
    group.bench_function("almost_best_iterative_large", |b| {
        b.iter(|| best_iterative(black_box(&large_input)))
    });
    group.bench_function("max_nonadjacent_large", |b| {
        b.iter(|| find_max_nonadjacent_sum(black_box(&large_input)))
    });
    group.bench_function("memoized_recursive_large", |b| {
        b.iter(|| memoized_recursive(black_box(&large_input)))
    });

    group.finish();
}

criterion_group!(benches, bench_house_robber);
criterion_main!(benches);
