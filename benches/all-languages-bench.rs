use criterion::{criterion_group, criterion_main};
use criterion::Criterion;

use sorting::benchmarking::read_file;
use sorting::string::burst_sort;

fn english(c: &mut Criterion) {
    let text = read_file("data/eng_news_2020_1M/eng_news_2020_1M-sentences.txt", false);
    bench_with_text(c, "english", text);
}

fn arabic(c: &mut Criterion) {
    let text = read_file("data/ara_news_2020_1M/ara_news_2020_1M-sentences.txt", false);
    bench_with_text(c, "arabic", text);
}

fn bench_with_text(c: &mut Criterion, param: &str, text: Vec<String>) {
    let mut group = c.benchmark_group(param);

    group.sample_size(10);

    group.bench_function(
        "burst-v1",
        |b| {
            b.iter(|| burst_sort(&mut text.clone()));
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
