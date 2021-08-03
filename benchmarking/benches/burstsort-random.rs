use std::time::Duration;

use criterion::{criterion_group, criterion_main, Throughput};
use criterion::Criterion;
use tcmalloc::TCMalloc;

use burstsort::*;
use internal_benchmarking::{get_random_str, read_file_alpha};

#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;

const LENGTH: usize = 2_000_000;

fn english(c: &mut Criterion) {
    let text = read_file_alpha("data/eng_news_2020_1M/eng_news_2020_1M-sentences.txt", false);
    bench_with_text(c, "alphabetical-english", text);
}

fn random_count(c: &mut Criterion) {
    const STEP_SIZE: usize = 25_000;

    let mut group = c.benchmark_group("random-by-count");

    for x in (STEP_SIZE..=2_000_000).step_by(STEP_SIZE) {
        let text = get_random_str(x, 1, 16);

        group.throughput(Throughput::Elements(x as u64));

        // group.bench_function(
        //     "alphanumeric-checked",
        //     |b| {
        //         b.iter(|| alphanumeric_sort(&mut text.clone()).unwrap());
        //     },
        // );
        //
        // group.bench_function(
        //     "alphanumeric-unchecked",
        //     |b| {
        //         b.iter(|| alphanumeric_unchecked_sort(&mut text.clone()));
        //     },
        // );

        group.bench_function(
            "unicode-sort-unstable",
            |b| {
                b.iter(|| unicode_sort_unstable(&mut text.clone()));
            },
        );

        group.bench_function(
            "byte-sort",
            |b| {
                b.iter(|| byte_sort(&mut text.clone()));
            },
        );

        group.bench_function(
            "byte-sort-unstable",
            |b| {
                b.iter(|| byte_sort_unstable(&mut text.clone()));
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
}

fn random_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("random-by-length");

    for x in [1, 2, 4, 8, 16, 32, 64, 128, 256] {
        let text = get_random_str(LENGTH, 0, x);

        group.throughput(Throughput::Elements(x as u64 / 2));

        // group.bench_function(
        //     "alphanumeric-checked",
        //     |b| {
        //         b.iter(|| alphanumeric_sort(&mut text.clone()).unwrap());
        //     },
        // );
        //
        // group.bench_function(
        //     "alphanumeric-unchecked",
        //     |b| {
        //         b.iter(|| alphanumeric_unchecked_sort(&mut text.clone()));
        //     },
        // );

        group.bench_function(
            "unicode-sort-unstable",
            |b| {
                b.iter(|| unicode_sort_unstable(&mut text.clone()));
            },
        );

        group.bench_function(
            "byte-sort",
            |b| {
                b.iter(|| byte_sort(&mut text.clone()));
            },
        );

        group.bench_function(
            "byte-sort-unstable",
            |b| {
                b.iter(|| byte_sort_unstable(&mut text.clone()));
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
}

fn bench_with_text(c: &mut Criterion, param: &str, text: Vec<String>) {
    println!("{}:\n\t{}", param, text.len());

    let mut group = c.benchmark_group(param);

    group.sample_size(128);
    group.warm_up_time(Duration::from_secs(20));

    // group.bench_function(
    //     "alphanumeric-checked",
    //     |b| {
    //         b.iter(|| alphanumeric_sort(&mut text.clone()).unwrap());
    //     },
    // );
    //
    // group.bench_function(
    //     "alphanumeric-unchecked",
    //     |b| {
    //         b.iter(|| alphanumeric_unchecked_sort(&mut text.clone()));
    //     },
    // );

    group.bench_function(
        "unicode-sort-unstable",
        |b| {
            b.iter(|| unicode_sort_unstable(&mut text.clone()));
        },
    );

    group.bench_function(
        "byte-sort",
        |b| {
            b.iter(|| byte_sort(&mut text.clone()));
        },
    );

    group.bench_function(
        "byte-sort-unstable",
        |b| {
            b.iter(|| byte_sort_unstable(&mut text.clone()));
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

criterion_group!(
    benches,
    random_length, random_count,
    english,
);
criterion_main!(benches);
