use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

fn filling_with_initial_capacity(capacity: usize) {
    let mut v = Vec::with_capacity(capacity);
    for i in 0..capacity {
        v.push(i);
    }

    black_box(v);
}

fn filling_without_initial_capacity(size: usize) {
    let mut v = Vec::new();
    for i in 0..size {
        v.push(i);
    }

    black_box(v);
}

pub fn vec_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec initialization");

    for size in [
        10,
        100,
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
    ]
    .iter()
    {
        group.bench_with_input(BenchmarkId::new("with_capacity", size), size, |b, &size| {
            b.iter(|| filling_with_initial_capacity(black_box(size)));
        });

        group.bench_with_input(
            BenchmarkId::new("without_capacity", size),
            size,
            |b, &size| {
                b.iter(|| filling_without_initial_capacity(black_box(size)));
            },
        );
    }

    group.finish();
}

criterion_group!(benches, vec_benchmarks);
criterion_main!(benches);
