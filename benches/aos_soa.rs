use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;
use std::hint::black_box;

#[allow(dead_code)]
struct SomeBankAccount {
    full_name: String,
    balance: u128,
}

impl SomeBankAccount {
    fn new(full_name: String, balance: u128) -> Self {
        SomeBankAccount { full_name, balance }
    }
}

struct LotsOfBankAccounts {
    full_names: Vec<String>,
    balances: Vec<u128>,
}

#[allow(dead_code)]
impl LotsOfBankAccounts {
    fn new() -> Self {
        Self {
            full_names: Vec::new(),
            balances: Vec::new(),
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            full_names: Vec::with_capacity(capacity),
            balances: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, full_name: String, balance: u128) {
        self.full_names.push(full_name);
        self.balances.push(balance);
    }
}

fn generate_test_data_aos_soa(size: usize) -> (Vec<SomeBankAccount>, LotsOfBankAccounts) {
    let mut aos = Vec::with_capacity(size);
    let mut soa = LotsOfBankAccounts::with_capacity(size);
    let mut rng = rand::rng();
    let mut nums: Vec<u128> = (1..1000).collect();
    nums.shuffle(&mut rng);

    for _ in 0..size {
        let name = "mock_user".to_string() + &rng.random::<char>().to_string();
        let balance = nums.choose(&mut rng).unwrap();
        aos.push(SomeBankAccount::new(name.clone(), *balance));
        soa.push(name, *balance);
    }

    (aos, soa)
}

fn sum_balances_aos(accounts: &[SomeBankAccount]) -> u64 {
    accounts.iter().map(|x| x.balance as u64).sum()
}

fn sum_balances_soa(accounts: &LotsOfBankAccounts) -> u64 {
    accounts.balances.iter().map(|&x| x as u64).sum()
}

pub fn aos_soa_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("AoS/SoA sum");

    for size in [10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000].iter() {
        let (aos, soa) = generate_test_data_aos_soa(*size);
        group.bench_with_input(BenchmarkId::new("AoS", size), &aos, |b, aos| {
            b.iter(|| sum_balances_aos(black_box(&aos)));
        });

        group.bench_with_input(BenchmarkId::new("SoA", size), &soa, |b, soa| {
            b.iter(|| sum_balances_soa(black_box(&soa)));
        });
    }

    group.finish();
}

criterion_group!(benches, aos_soa_benchmarks);
criterion_main!(benches);
