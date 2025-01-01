// benches/exponent.rs
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fast_exponentiation::{
    raise_to_power, raise_to_power_fast_iterative, raise_to_power_fast_recursive,
    raise_to_power_linear_iterative, raise_to_power_linear_recursive,
};

fn bench_exponents(c: &mut Criterion) {
    let mut group = c.benchmark_group("exponents");

    // Test 2^64
    let base: u64 = 2;
    let power: u32 = 64;

    group.bench_with_input(
        BenchmarkId::new("sublinear", "2^64"),
        &(base, power),
        |b, &(base, power)| b.iter(|| raise_to_power(base, power)),
    );

    group.bench_with_input(
        BenchmarkId::new("linear recursive", "2^64"),
        &(base, power),
        |b, &(base, power)| b.iter(|| raise_to_power_linear_recursive(base, power)),
    );

    group.bench_with_input(
        BenchmarkId::new("linear iterative", "2^64"),
        &(base, power),
        |b, &(base, power)| b.iter(|| raise_to_power_linear_iterative(base, power)),
    );

    group.bench_with_input(
        BenchmarkId::new("fast iterative", "2^64"),
        &(base, power),
        |b, &(base, power)| b.iter(|| raise_to_power_fast_iterative(base, power)),
    );

    group.bench_with_input(
        BenchmarkId::new("fast recursive", "2^64"),
        &(base, power),
        |b, &(base, power)| b.iter(|| raise_to_power_fast_recursive(base, power)),
    );

    group.bench_with_input(
        BenchmarkId::new("stdlib", "2^64"),
        &(base, power),
        |b, &(base, power)| b.iter(|| base.pow(power)),
    );

    group.finish();
}

criterion_group!(benches, bench_exponents);
criterion_main!(benches);
