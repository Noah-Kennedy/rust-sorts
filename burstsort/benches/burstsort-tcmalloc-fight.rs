use criterion::{criterion_group, criterion_main};
use criterion::Criterion;
use tcmalloc::TCMalloc;

use burstsort::benching::{bench_english, bench_random_count, bench_random_length};

#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;

fn english(c: &mut Criterion) {
    bench_english(c, "tcmalloc")
}

fn random_count(c: &mut Criterion) {
    bench_random_count(c, "tcmalloc")
}

fn random_length(c: &mut Criterion) {
    bench_random_length(c, "tcmalloc");
}

criterion_group!(
    benches,
    random_length,
    random_count,
    english,
);
criterion_main!(benches);
