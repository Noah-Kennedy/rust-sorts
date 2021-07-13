use std::time::Duration;

use criterion::{criterion_group, criterion_main};
use criterion::Criterion;
use tcmalloc::TCMalloc;

use benchmarking::{get_random_str, read_file};
use burstsort::string::{dynamic_burst_sort, tabular_burst_sort};

#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;

fn english(c: &mut Criterion) {
    let text = read_file("data/eng_news_2020_1M/eng_news_2020_1M-sentences.txt", false);
    bench_with_text(c, "english", text);
}

fn arabic(c: &mut Criterion) {
    let text = read_file("data/ara_news_2020_1M/ara_news_2020_1M-sentences.txt", false);
    bench_with_text(c, "arabic", text);
}

fn random_short(c: &mut Criterion) {
    let text = get_random_str(10_000_000, 1, 8);
    bench_with_text(c, "random-short", text);
}

fn random_medium(c: &mut Criterion) {
    let text = get_random_str(10_000_000, 16, 32);
    bench_with_text(c, "random-medium", text);
}

fn random_long(c: &mut Criterion) {
    let text = get_random_str(10_000_000, 64, 256);
    bench_with_text(c, "random-long", text);
}

fn bench_with_text(c: &mut Criterion, param: &str, text: Vec<String>) {
    println!("{}:\n\t{}", param, text.len());

    let mut group = c.benchmark_group(param);

    group.sample_size(32);
    group.warm_up_time(Duration::from_secs(20));

    group.bench_function(
        "burst-dynamic",
        |b| {
            b.iter(|| dynamic_burst_sort(&mut text.clone()));
        },
    );

    group.bench_function(
        "burst-v1",
        |b| {
            b.iter(|| tabular_burst_sort(&mut text.clone()));
        },
    );

    group.bench_function(
        "std::unstable",
        |b| {
            b.iter(|| text.clone().sort_unstable());
        },
    );

    group.bench_function(
        "std::stable",
        |b| {
            b.iter(|| text.clone().sort());
        },
    );
}

criterion_group!(benches, english, arabic, random_short, random_medium, random_long);
criterion_main!(benches);
