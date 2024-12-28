use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use varint;

fn bench_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode");
    group.measurement_time(std::time::Duration::from_secs(10));
    group.sample_size(1000);

    // Test cases
    let inputs = vec![127, u64::MAX];

    for input in inputs {
        group.bench_with_input(BenchmarkId::new("current", input), &input, |b, &input| {
            b.iter_with_setup(|| vec![0u8; 10], |mut buf| varint::encode(input, &mut buf))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_encode);
criterion_main!(benches);
