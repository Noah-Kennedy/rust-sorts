use std::time::Duration;

use criterion::{criterion_group, criterion_main};
use criterion::Criterion;
use tcmalloc::TCMalloc;

use internal_benchmarking::{read_file};
use burstsort::{byte_sort, unicode_sort_unstable, byte_sort_unstable};

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

fn bench_with_text(c: &mut Criterion, param: &str, text: Vec<String>) {
    println!("{}:\n\t{}", param, text.len());

    let mut group = c.benchmark_group(param);

    group.sample_size(128);
    group.warm_up_time(Duration::from_secs(20));

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

criterion_group!(benches, english, arabic);
criterion_main!(benches);
