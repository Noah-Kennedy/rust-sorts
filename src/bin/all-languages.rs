use std::thread;

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

use sorting::benchmarking::{bench_sort, read_file};
use sorting::string::burst_sort;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let arabic_join = thread::spawn(
        || read_file("data/ara_news_2020_1M/ara_news_2020_1M-sentences.txt")
    );

    let english_join = thread::spawn(
        || read_file("data/eng_news_2020_1M/eng_news_2020_1M-sentences.txt")
    );

    let arabic = arabic_join.join().unwrap();
    let english = english_join.join().unwrap();

    let data = vec![
        arabic,
        english
    ];

    bench_sort(
        "burst",
        |data| burst_sort(data),
        data.clone(),
    );

    // bench_sort(
    //     "std::sort_unstable",
    //     |data| data.sort_unstable(),
    //     data.clone(),
    // );
    //
    // bench_sort(
    //     "std::sort",
    //     |data| data.sort(),
    //     data.clone(),
    // );
}