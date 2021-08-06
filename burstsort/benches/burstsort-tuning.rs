use criterion::{criterion_group, criterion_main, Throughput, BenchmarkId};
use criterion::Criterion;

use burstsort::{ASCII_CONFIG, BurstConfig};
use burstsort::benching::read_file_alpha;

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

const BURST_STR: &str = "par-burstsort";

fn burst_limit(c: &mut Criterion) {
    let mut group = c.benchmark_group("tune-burst-limit");

    let text = read_file_alpha("data/eng_news_2020_1M/eng_news_2020_1M-sentences.txt", false);

    let limits = [256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072];

    for burst_limit in limits {
        group.throughput(Throughput::Elements(burst_limit as u64));
        group.bench_function(
            BenchmarkId::new(BURST_STR, burst_limit),
            |b| {
                let config = BurstConfig {
                    burst_limit,
                    ..ASCII_CONFIG
                };
                b.iter(|| burstsort::par_burstsort(&mut text.clone(), &config));
            },
        );
    }
}

fn initial_capacity(c: &mut Criterion) {
    let mut group = c.benchmark_group("tune-init-cap");

    let capacities = [0, 32, 64, 128, 256, 512, 1024, 2048, 4096];

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
                b.iter(|| burstsort::par_burstsort(&mut text.clone(), &config));
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
