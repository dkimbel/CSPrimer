use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use merge_sort::{merge_sort, naive_merge_sort};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorting");
    group.measurement_time(std::time::Duration::from_secs(10));
    group.sample_size(100); // Reduced from 1000 since these are slower benchmarks

    // Create test data
    let mut data: Vec<i32> = (0..1_000_000).collect();
    let mut rng = thread_rng();
    data.shuffle(&mut rng);

    // Convert to slice once for both tests
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

    group.finish();
}

criterion_group!(benches, bench_sorts);
criterion_main!(benches);
