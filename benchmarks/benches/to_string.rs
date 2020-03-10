use benchmarks::i32_to_string_java;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_to_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_string");
    for i in [
        std::i32::MIN,
        std::i32::MAX,
        -343658865,
        343658865,
        -1964787,
        1964787,
        -123,
        123,
        -10,
        10,
        -5,
        5,
        0,
    ]
    .iter()
    {
        group.bench_with_input(BenchmarkId::new("java", i), i, |b, i| {
            b.iter(|| i32_to_string_java(*i))
        });
        group.bench_with_input(BenchmarkId::new("default", i), i, |b, i| {
            b.iter(|| i.to_string())
        });
        group.bench_with_input(BenchmarkId::new("trait", i), i, |b, i| {
            b.iter(|| ToString::to_string(i))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_to_string);
criterion_main!(benches);
