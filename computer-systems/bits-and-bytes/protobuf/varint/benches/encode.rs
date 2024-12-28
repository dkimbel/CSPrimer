use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use varint::{encode_v1, encode_v2};

fn bench_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode");
    group.measurement_time(std::time::Duration::from_secs(10));
    group.sample_size(1000);

    // Test cases
    let inputs = vec![127, u64::MAX];

    for input in inputs {
        group.bench_with_input(BenchmarkId::new("v1", input), &input, |b, &input| {
            b.iter_with_setup(|| vec![0u8; 10], |mut buf| encode_v1(input, &mut buf))
        });

        group.bench_with_input(BenchmarkId::new("v2", input), &input, |b, &input| {
            b.iter_with_setup(|| vec![0u8; 10], |mut buf| encode_v2(input, &mut buf))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_encode);
criterion_main!(benches);
