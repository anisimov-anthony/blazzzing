use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

#[allow(dead_code)]
struct SomeData {
    id: u32,
    data: String,
}

fn filling_with_initial_capacity(capacity: usize) {
    let mut v = Vec::with_capacity(capacity);
    for i in 0..=capacity {
        v.push(i);
    }

    black_box(v);
}

fn filling_without_initial_capacity(size: usize) {
    let mut v = Vec::new();
    for i in 0..=size {
        v.push(i);
    }

    black_box(v);
}

fn criterion_benchmark_1k(c: &mut Criterion) {
    let sz_cp = 1_000;

    c.bench_function("filling_with_initial_capacity_1k", |b| {
        b.iter(|| filling_with_initial_capacity(sz_cp))
    });

    c.bench_function("filling_without_initial_capacity_1k", |b| {
        b.iter(|| filling_without_initial_capacity(sz_cp))
    });
}

fn criterion_benchmark_10k(c: &mut Criterion) {
    let sz_cp = 10_000;

    c.bench_function("filling_with_initial_capacity_10k", |b| {
        b.iter(|| filling_with_initial_capacity(sz_cp))
    });

    c.bench_function("filling_without_initial_capacity_10k", |b| {
        b.iter(|| filling_without_initial_capacity(sz_cp))
    });
}

fn criterion_benchmark_100k(c: &mut Criterion) {
    let sz_cp = 100_000;

    c.bench_function("filling_with_initial_capacity_100k", |b| {
        b.iter(|| filling_with_initial_capacity(sz_cp))
    });

    c.bench_function("filling_without_initial_capacity_100k", |b| {
        b.iter(|| filling_without_initial_capacity(sz_cp))
    });
}

fn criterion_benchmark_1m(c: &mut Criterion) {
    let sz_cp = 1_000_000;

    c.bench_function("filling_with_initial_capacity_1M", |b| {
        b.iter(|| filling_with_initial_capacity(sz_cp))
    });

    c.bench_function("filling_without_initial_capacity_1M", |b| {
        b.iter(|| filling_without_initial_capacity(sz_cp))
    });
}

criterion_group!(benches_1k, criterion_benchmark_1k);
criterion_group!(benches_10k, criterion_benchmark_10k);
criterion_group!(benches_100k, criterion_benchmark_100k);
criterion_group!(benches_1m, criterion_benchmark_1m);

criterion_main!(benches_1k, benches_10k, benches_100k, benches_1m);
