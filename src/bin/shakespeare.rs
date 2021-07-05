use std::time::Instant;

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;
use unicode_segmentation::UnicodeSegmentation;

use sorting::benchmarking::{bench_sort, get_random_str};
use sorting::bucket::bucket_sort;
use sorting::insertion::insertion_sort;
use sorting::string::burst_sort;

type Value = String;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

const BUCKETS: usize = 256;

fn key(value: &Value) -> usize {
    value.bytes().next().unwrap_or(0) as usize
}

fn read_file(file: &str) -> Vec<String> {
    let timer = Instant::now();
    let text = std::fs::read_to_string(file).unwrap();
    let words = text.unicode_words();

    let data = words.map(ToOwned::to_owned).collect();

    let time = timer.elapsed().as_secs_f64();
    println!("{}:\t{:.3} seconds", file, time);

    data
}

fn main() {
    let shakespeare = read_file("data/shakespeare.txt");
    let sherlock = read_file("data/sherlock.txt");

    let mut total = shakespeare.clone();
    total.append(&mut sherlock.clone());

    let data = vec![total];

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