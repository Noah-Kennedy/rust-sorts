use std::time::Duration;

use criterion::{criterion_group, criterion_main};
use criterion::Criterion;
use tcmalloc::TCMalloc;

use internal_benchmarking::{get_random_str, read_file_alpha};
use burstsort::{unicode_sort, byte_sort, alphanumeric_sort, alphanumeric_unchecked_sort};

#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;

const LENGTH: usize = 10_000_000;

fn english(c: &mut Criterion) {
    let text = read_file_alpha("data/eng_news_2020_1M/eng_news_2020_1M-sentences.txt", false);
    bench_with_text(c, "alphabetical-english", text);
}

fn random_short(c: &mut Criterion) {
    let text = get_random_str(LENGTH, 1, 8);
    bench_with_text(c, "random-short", text);
}

fn random_medium(c: &mut Criterion) {
    let text = get_random_str(LENGTH, 16, 32);
    bench_with_text(c, "random-medium", text);
}

fn random_long(c: &mut Criterion) {
    let text = get_random_str(LENGTH, 64, 256);
    bench_with_text(c, "random-long", text);
}

fn bench_with_text(c: &mut Criterion, param: &str, text: Vec<String>) {
    println!("{}:\n\t{}", param, text.len());

    let mut group = c.benchmark_group(param);

    group.sample_size(128);
    group.warm_up_time(Duration::from_secs(20));

    group.bench_function(
        "alphanumeric-checked",
        |b| {
            b.iter(|| alphanumeric_sort(&mut text.clone()).unwrap());
        },
    );

    group.bench_function(
        "alphanumeric-unchecked",
        |b| {
            b.iter(|| alphanumeric_unchecked_sort(&mut text.clone()));
        },
    );

    group.bench_function(
        "unicode-sort",
        |b| {
            b.iter(|| unicode_sort(&mut text.clone()));
        },
    );

    group.bench_function(
        "byte-sort",
        |b| {
            b.iter(|| byte_sort(&mut text.clone()));
        },
    );

    group.bench_function(
        "std-unstable",
        |b| {
            b.iter(|| text.clone().sort_unstable());
        },
    );

    group.bench_function(
        "std-stable",
        |b| {
            b.iter(|| text.clone().sort());
        },
    );
}

criterion_group!(benches, english, random_short, random_medium, random_long);
criterion_main!(benches);
