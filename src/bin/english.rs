#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

use sorting::benchmarking::{bench_sort, read_file};
use sorting::string::burst_sort;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let english = read_file("data/eng_news_2020_1M/eng_news_2020_1M-sentences.txt");

    let data = vec![
        english
    ];

    bench_sort(
        "burst",
        |data| burst_sort(data),
        data.clone(),
    );

    bench_sort(
        "std::sort_unstable",
        |data| data.sort_unstable(),
        data.clone(),
    );

    bench_sort(
        "std::sort",
        |data| data.sort(),
        data.clone(),
    );
}