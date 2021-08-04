use std::time::Duration;

use criterion::{criterion_group, criterion_main, Throughput, BenchmarkId};
use criterion::Criterion;
use tcmalloc::TCMalloc;

use burstsort::ASCII_CONFIG;
use burstsort::benching::{get_random_str, read_file_alpha};

#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;

const LENGTH: usize = 2_000_000;

const BURST_STR: &str = "burstsort";
const STD_STABLE_STR: &str = "std-stable";
const STD_UNSTABLE_STR: &str = "std-unstable";

fn english(c: &mut Criterion) {
    let text = read_file_alpha("data/eng_news_2020_1M/eng_news_2020_1M-sentences.txt", false);
    bench_with_text(c, "tcmalloc-alphabetical-english", text);
}

fn random_count(c: &mut Criterion) {
    const STEP_SIZE: usize = 25_000;

    let mut group = c.benchmark_group("tcmalloc-random-by-count");

    for x in (STEP_SIZE..=2_000_000).step_by(STEP_SIZE) {
        let text = get_random_str(x, 1, 16);

        group.throughput(Throughput::Elements(x as u64));

        group.bench_function(
            BenchmarkId::new(BURST_STR, x),
            |b| {
                b.iter(|| burstsort::burstsort(&mut text.clone(), &ASCII_CONFIG));
            },
        );

        group.bench_function(
            BenchmarkId::new(STD_UNSTABLE_STR, x),
            |b| {
                b.iter(|| text.clone().sort_unstable());
            },
        );

        group.bench_function(
            BenchmarkId::new(STD_STABLE_STR, x),
            |b| {
                b.iter(|| text.clone().sort());
            },
        );
    }
}

fn random_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("tcmalloc-random-by-length");

    for x in [1, 2, 4, 8, 16, 32, 64, 128, 256] {
        let text = get_random_str(LENGTH, 0, x);

        group.throughput(Throughput::Elements(x as u64 / 2));

        group.bench_function(
            BenchmarkId::new(BURST_STR, x),
            |b| {
                b.iter(|| burstsort::burstsort(&mut text.clone(), &ASCII_CONFIG));
            },
        );

        group.bench_function(
            BenchmarkId::new(STD_UNSTABLE_STR, x),
            |b| {
                b.iter(|| text.clone().sort_unstable());
            },
        );

        group.bench_function(
            BenchmarkId::new(STD_STABLE_STR, x),
            |b| {
                b.iter(|| text.clone().sort());
            },
        );
    }
}

fn bench_with_text(c: &mut Criterion, param: &str, text: Vec<String>) {
    println!("{}:\n\t{}", param, text.len());

    let mut group = c.benchmark_group(param);

    group.sample_size(128);
    group.warm_up_time(Duration::from_secs(20));

    group.bench_function(
        BURST_STR,
        |b| {
            b.iter(|| burstsort::burstsort(&mut text.clone(), &ASCII_CONFIG));
        },
    );

    group.bench_function(
        STD_UNSTABLE_STR,
        |b| {
            b.iter(|| text.clone().sort_unstable());
        },
    );

    group.bench_function(
        STD_STABLE_STR,
        |b| {
            b.iter(|| text.clone().sort());
        },
    );
}

criterion_group!(
    benches,
    random_length,
    random_count,
    english,
);
criterion_main!(benches);
