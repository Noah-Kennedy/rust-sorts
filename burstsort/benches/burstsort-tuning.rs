use criterion::{criterion_group, criterion_main, Throughput, BenchmarkId};
use criterion::Criterion;

use burstsort::{ASCII_CONFIG, BurstConfig};
use burstsort::benching::read_file_alpha;

#[global_allocator]
static GLOBAL: tcmalloc::TCMalloc = tcmalloc::TCMalloc;

const BURST_STR: &str = "burstsort";

fn burst_limit(c: &mut Criterion) {
    let mut group = c.benchmark_group("tune-burst-limit");

    let text = read_file_alpha("data/eng_news_2020_1M/eng_news_2020_1M-sentences.txt", false);

    for burst_limit in [256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536] {
        group.throughput(Throughput::Elements(burst_limit as u64));
        group.bench_function(
            BenchmarkId::new(BURST_STR, burst_limit),
            |b| {
                let config = BurstConfig {
                    burst_limit,
                    ..ASCII_CONFIG
                };
                b.iter(|| burstsort::burstsort(&mut text.clone(), &config));
            },
        );
    }
}

fn initial_capacity(c: &mut Criterion) {
    let mut group = c.benchmark_group("tune-init-cap");

    let capacities = [0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096];

    let text = read_file_alpha("data/eng_news_2020_1M/eng_news_2020_1M-sentences.txt", false);

    for initial_capacity in capacities {
        group.throughput(Throughput::Elements(initial_capacity as u64));
        group.bench_function(
            BenchmarkId::new(BURST_STR, initial_capacity),
            |b| {
                let config = BurstConfig {
                    initial_capacity,
                    ..ASCII_CONFIG
                };
                b.iter(|| burstsort::burstsort(&mut text.clone(), &config));
            },
        );
    }
}

criterion_group!(
    benches,
    burst_limit,
    initial_capacity
);
criterion_main!(benches);
