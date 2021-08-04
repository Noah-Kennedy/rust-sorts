use criterion::{criterion_group, criterion_main};
use criterion::Criterion;

use burstsort::benching::{bench_english, bench_random_count, bench_random_length};

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn english(c: &mut Criterion) {
    bench_english(c, "jemalloc")
}

fn random_count(c: &mut Criterion) {
    bench_random_count(c, "jemalloc")
}

fn random_length(c: &mut Criterion) {
    bench_random_length(c, "jemalloc");
}

criterion_group!(
    benches,
    random_length,
    random_count,
    english,
);
criterion_main!(benches);
