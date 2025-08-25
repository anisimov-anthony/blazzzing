use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::prelude::*;
use std::hint::black_box;

fn generate_random_array(size: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (0..size as i32).collect();
    nums.shuffle(&mut rng);
    nums
}

// Based on 1470. Shuffle the Array Leetcode's problem
// Given the array nums consisting of 2n elements in the form [x1,x2,...,xn,y1,y2,...,yn].
// Return the array in the form [x1,y1,x2,y2,...,xn,yn].

fn with_initial_capacity(nums: Vec<i32>, n: usize) -> Vec<i32> {
    let mut res = Vec::with_capacity(n * 2);
    for i in 0..n {
        res.push(nums[i as usize]);
        res.push(nums[(n + i) as usize]);
    }
    res
}

fn without_initial_capacity(nums: Vec<i32>, n: usize) -> Vec<i32> {
    let mut res = Vec::new();
    for i in 0..n {
        res.push(nums[i as usize]);
        res.push(nums[(n + i) as usize]);
    }
    res
}

pub fn vec_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("with_without_capacity_shuffling");

    for size in [10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000].iter() {
        let container = generate_random_array(*size);
        group.bench_with_input(
            BenchmarkId::new("with_cap", size),
            &container,
            |b, container| {
                b.iter(|| black_box(with_initial_capacity(container.clone(), *size / 2)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("withot_cap", size),
            &container,
            |b, container| {
                b.iter(|| black_box(without_initial_capacity(container.clone(), *size / 2)));
            },
        );
    }

    group.finish();
}

criterion_group!(benches, vec_benchmarks);
criterion_main!(benches);
