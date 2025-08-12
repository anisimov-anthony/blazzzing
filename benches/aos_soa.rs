use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;

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

fn criterion_benchmark_1k(c: &mut Criterion) {
    let size = 1_000;
    let (aos, soa) = generate_test_data_aos_soa(size);

    c.bench_function("sum_balances_aos_1k", |b| b.iter(|| sum_balances_aos(&aos)));

    c.bench_function("sum_balances_soa_1k", |b| b.iter(|| sum_balances_soa(&soa)));
}

fn criterion_benchmark_10k(c: &mut Criterion) {
    let size = 10_000;
    let (aos, soa) = generate_test_data_aos_soa(size);

    c.bench_function("sum_balances_aos_10k", |b| {
        b.iter(|| sum_balances_aos(&aos))
    });

    c.bench_function("sum_balances_soa_10k", |b| {
        b.iter(|| sum_balances_soa(&soa))
    });
}

fn criterion_benchmark_100k(c: &mut Criterion) {
    let size = 100_000;
    let (aos, soa) = generate_test_data_aos_soa(size);

    c.bench_function("sum_balances_aos_100k", |b| {
        b.iter(|| sum_balances_aos(&aos))
    });

    c.bench_function("sum_balances_soa_100k", |b| {
        b.iter(|| sum_balances_soa(&soa))
    });
}

fn criterion_benchmark_1m(c: &mut Criterion) {
    let size = 1_000_000;
    let (aos, soa) = generate_test_data_aos_soa(size);

    c.bench_function("sum_balances_aos_1M", |b| b.iter(|| sum_balances_aos(&aos)));

    c.bench_function("sum_balances_soa_1M", |b| b.iter(|| sum_balances_soa(&soa)));
}

criterion_group!(benches_1k, criterion_benchmark_1k);
criterion_group!(benches_10k, criterion_benchmark_10k);
criterion_group!(benches_100k, criterion_benchmark_100k);
criterion_group!(benches_1m, criterion_benchmark_1m);

criterion_main!(benches_1k, benches_10k, benches_100k, benches_1m);
