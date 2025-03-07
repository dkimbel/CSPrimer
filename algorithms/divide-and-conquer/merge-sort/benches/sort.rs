use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use merge_sort::{merge_sort, naive_merge_sort};
use quick_sort::quick_sort;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn bench_random_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_sorting");
    group.measurement_time(std::time::Duration::from_secs(10));
    group.sample_size(100); // Reduced from 1000 since these are slower benchmarks

    // Create test data
    let mut data: Vec<i32> = (0..1_000_000).collect();
    let mut rng = thread_rng();
    data.shuffle(&mut rng);

    // Convert to slice once for all tests
    let data_slice = data.as_slice();

    group.bench_with_input(
        BenchmarkId::new("merge_sort", "1M random"),
        &data_slice,
        |b, data| b.iter(|| merge_sort(data)),
    );

    group.bench_with_input(
        BenchmarkId::new("slow_merge_sort", "1M random"),
        &data_slice,
        |b, data| b.iter(|| naive_merge_sort(data)),
    );

    group.bench_with_input(
        BenchmarkId::new("quick_sort", "1M random"),
        &data_slice,
        |b, data| {
            b.iter_with_setup(
                || data.to_vec(), // Create fresh clone for each iteration
                |mut arr| quick_sort(&mut arr),
            )
        },
    );

    group.finish();
}

fn bench_pre_sorted_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("pre_sorted_sorting");
    group.measurement_time(std::time::Duration::from_secs(10));
    group.sample_size(100);

    let data: Vec<i32> = (0..1_000_000).collect(); // Already sorted
    let data_slice = data.as_slice();

    group.bench_with_input(
        BenchmarkId::new("merge_sort", "1M pre-sorted"),
        &data_slice,
        |b, data| b.iter(|| merge_sort(data)),
    );

    group.bench_with_input(
        BenchmarkId::new("quick_sort", "1M pre-sorted"),
        &data_slice,
        |b, data| b.iter_with_setup(|| data.to_vec(), |mut arr| quick_sort(&mut arr)),
    );

    group.finish();
}

criterion_group!(random_benches, bench_random_sorts);
criterion_group!(pre_sorted_benches, bench_pre_sorted_sorts);
criterion_main!(random_benches, pre_sorted_benches);
