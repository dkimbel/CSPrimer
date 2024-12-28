use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use varint::{decode_v1, decode_v2};

fn bench_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode");
    group.measurement_time(std::time::Duration::from_secs(10));
    group.sample_size(1000);

    // Test cases - example encoded varints
    let inputs = vec![
        vec![127],                                                        // 127
        vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01], // u64::MAX
    ];

    for input in inputs {
        group.bench_with_input(
            BenchmarkId::new("v1", input[0]), // Using first byte as ID
            &input,
            |b, input| b.iter(|| decode_v1(input)),
        );

        group.bench_with_input(
            BenchmarkId::new("v2", input[0]), // Using first byte as ID
            &input,
            |b, input| b.iter(|| decode_v2(input)),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_decode);
criterion_main!(benches);
