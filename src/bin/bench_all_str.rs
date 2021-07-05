#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

use sorting::benchmarking::{bench_sort, get_random_str};
use sorting::bucket::bucket_sort;
use sorting::insertion::insertion_sort;
use sorting::string::burst_sort;

type Value = String;

const LENGTH: usize = 100_000;
const MIN_LENGTH: usize = 1;
const MAX_LENGTH: usize = 24;
const ITERATIONS: usize = 100;

const BUCKETS: usize = 256;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn key(value: &Value) -> usize {
    value.bytes().next().unwrap_or(0) as usize
}

fn main() {
    let data = get_random_str(LENGTH, ITERATIONS, MIN_LENGTH, MAX_LENGTH);

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

    bench_sort(
        "bucket",
        |data| bucket_sort(data, key, BUCKETS).unwrap(),
        data.clone(),
    );

    bench_sort(
        "insertion",
        |data| insertion_sort(data),
        data.clone(),
    );
}