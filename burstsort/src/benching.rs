use std::time::{Duration, Instant};

use criterion::{BenchmarkId, black_box, Criterion, Throughput};
use rand::distributions::{Alphanumeric, Distribution, Uniform};
use rand::distributions::uniform::SampleUniform;
use unicode_segmentation::UnicodeSegmentation;

use crate::{ASCII_CONFIG, burstsort, LONG_ASCII_CONFIG, par_burstsort};
use rayon::prelude::ParallelSliceMut;

const LENGTH: usize = 2_000_000;

const PAR_BURST_STR: &str = "par-burstsort";
const PAR_LONG_BURST_STR: &str = "par-burstsort-long";
const BURST_STR: &str = "burstsort";
const LONG_BURST_STR: &str = "burstsort-long";
const STD_STABLE_STR: &str = "std-stable";
const STD_UNSTABLE_STR: &str = "std-unstable";
const RAYON_STABLE_STR: &str = "rayon-par-stable";
const RAYON_UNSTABLE_STR: &str = "rayon-par-unstable";


pub fn bench_english(c: &mut Criterion, allocator: &str) {
    let text = read_file_alpha("data/eng_news_2020_1M/eng_news_2020_1M-sentences.txt", false);
    bench_with_text(c, "compare-encoding-ascii-alpha", allocator, text);
}

pub fn bench_random_count(c: &mut Criterion, allocator: &str) {
    const STEP_SIZE: usize = 25_000;

    let name = format!("{}-compare-random-by-count", allocator);

    let mut group = c.benchmark_group(name);

    for x in (STEP_SIZE..=2_000_000).step_by(STEP_SIZE) {
        let text = get_random_str(x, 1, 16);

        group.throughput(Throughput::Elements(x as u64));

        group.bench_function(
            BenchmarkId::new(PAR_BURST_STR, x),
            |b| {
                b.iter(|| par_burstsort(&mut text.clone(), &ASCII_CONFIG));
            },
        );

        group.bench_function(
            BenchmarkId::new(BURST_STR, x),
            |b| {
                b.iter(|| burstsort(&mut text.clone(), &ASCII_CONFIG));
            },
        );

        group.bench_function(
            BenchmarkId::new(STD_UNSTABLE_STR, x),
            |b| {
                b.iter(|| text.clone().sort_unstable());
            },
        );

        group.bench_function(
            BenchmarkId::new(STD_STABLE_STR, x),
            |b| {
                b.iter(|| text.clone().sort());
            },
        );

        group.bench_function(
            BenchmarkId::new(RAYON_STABLE_STR, x),
            |b| {
                b.iter(|| text.clone().par_sort());
            },
        );

        group.bench_function(
            BenchmarkId::new(RAYON_UNSTABLE_STR, x),
            |b| {
                b.iter(|| text.clone().par_sort_unstable());
            },
        );
    }
}

pub fn bench_random_length(c: &mut Criterion, allocator: &str) {
    let name = format!("{}-compare-random-by-length", allocator);
    let mut group = c.benchmark_group(name);

    for x in [1, 2, 4, 8, 16, 32, 64, 128, 256] {
        let text = get_random_str(LENGTH, 0, x);

        group.throughput(Throughput::Elements(x as u64 / 2));

        group.bench_function(
            BenchmarkId::new(PAR_BURST_STR, x),
            |b| {
                b.iter(|| par_burstsort(&mut text.clone(), &ASCII_CONFIG));
            },
        );

        group.bench_function(
            BenchmarkId::new(PAR_LONG_BURST_STR, x),
            |b| {
                b.iter(|| par_burstsort(&mut text.clone(), &LONG_ASCII_CONFIG));
            },
        );

        group.bench_function(
            BenchmarkId::new(BURST_STR, x),
            |b| {
                b.iter(|| burstsort(&mut text.clone(), &ASCII_CONFIG));
            },
        );

        group.bench_function(
            BenchmarkId::new(LONG_BURST_STR, x),
            |b| {
                b.iter(|| burstsort(&mut text.clone(), &LONG_ASCII_CONFIG));
            },
        );

        group.bench_function(
            BenchmarkId::new(STD_UNSTABLE_STR, x),
            |b| {
                b.iter(|| text.clone().sort_unstable());
            },
        );

        group.bench_function(
            BenchmarkId::new(STD_STABLE_STR, x),
            |b| {
                b.iter(|| text.clone().sort());
            },
        );

        group.bench_function(
            BenchmarkId::new(RAYON_STABLE_STR, x),
            |b| {
                b.iter(|| text.clone().par_sort());
            },
        );

        group.bench_function(
            BenchmarkId::new(RAYON_UNSTABLE_STR, x),
            |b| {
                b.iter(|| text.clone().par_sort_unstable());
            },
        );
    }
}

fn bench_with_text(c: &mut Criterion, param: &str, allocator: &str, text: Vec<String>) {
    let name = format!("{}-{}", allocator, param);

    println!("{}:\n\t{}", &name, text.len());

    let mut group = c.benchmark_group(name);

    group.sample_size(128);
    group.warm_up_time(Duration::from_secs(20));

    group.bench_function(
        PAR_BURST_STR,
        |b| {
            b.iter(|| par_burstsort(&mut text.clone(), &ASCII_CONFIG));
        },
    );

    group.bench_function(
        PAR_LONG_BURST_STR,
        |b| {
            b.iter(|| par_burstsort(&mut text.clone(), &LONG_ASCII_CONFIG));
        },
    );

    group.bench_function(
        BURST_STR,
        |b| {
            b.iter(|| burstsort(&mut text.clone(), &ASCII_CONFIG));
        },
    );

    group.bench_function(
        LONG_BURST_STR,
        |b| {
            b.iter(|| burstsort(&mut text.clone(), &LONG_ASCII_CONFIG));
        },
    );

    group.bench_function(
        STD_UNSTABLE_STR,
        |b| {
            b.iter(|| text.clone().sort_unstable());
        },
    );

    group.bench_function(
        STD_STABLE_STR,
        |b| {
            b.iter(|| text.clone().sort());
        },
    );

    group.bench_function(
        RAYON_STABLE_STR,
        |b| {
            b.iter(|| text.clone().par_sort());
        },
    );

    group.bench_function(
        RAYON_UNSTABLE_STR,
        |b| {
            b.iter(|| text.clone().par_sort_unstable());
        },
    );
}

pub fn read_file(file: &str, printing: bool) -> Vec<String> {
    let timer = Instant::now();
    let text = std::fs::read_to_string(file).unwrap();
    let words = text.unicode_words();

    let data = words.map(ToOwned::to_owned).collect();

    let time = timer.elapsed().as_secs_f64();

    if printing {
        println!("{}:\t{:.3} seconds", file, time);
    }

    data
}

pub fn read_file_alpha(file: &str, printing: bool) -> Vec<String> {
    let timer = Instant::now();
    let text = std::fs::read_to_string(file).unwrap();
    let words = text.unicode_words();

    let data = words
        .filter(|s| s.as_bytes().iter().all(|c| c.is_ascii_alphabetic()))
        .map(ToOwned::to_owned)
        .collect();

    let time = timer.elapsed().as_secs_f64();

    if printing {
        println!("{}:\t{:.3} seconds", file, time);
    }

    data
}

pub fn get_random_ranges<T>(length: usize, samples: usize, low: T, high: T) -> Vec<Vec<T>>
    where T: SampleUniform + Clone
{
    let mut sample_set = Vec::with_capacity(samples);

    for _ in 0..samples {
        let uniform = Uniform::new_inclusive(low.clone(), high.clone());
        let rng = rand::thread_rng();

        let sample_iterator = uniform.sample_iter(rng);

        let sample = sample_iterator.take(length).collect();
        sample_set.push(sample)
    }

    sample_set
}

pub fn get_random_str(length: usize, min_length: usize, max_length: usize) -> Vec<String> {
    let mut elements = Vec::with_capacity(length);

    let length_dist = Uniform::new_inclusive(min_length, max_length);

    let mut lengths = length_dist.sample_iter(rand::thread_rng());

    for _ in 0..length {
        let str_length = lengths.next().unwrap();

        let rng = rand::thread_rng();
        let sample_iterator = Alphanumeric.sample_iter(rng);
        let item = sample_iterator.take(str_length).map(char::from).collect();
        elements.push(item)
    }

    elements
}


pub fn bench_sort<T>(name: &str, sort: fn(&mut Vec<T>), mut samples: Vec<Vec<T>>) {
    let timer = Instant::now();

    for s in &mut samples {
        sort(s);
    }

    black_box(&samples);

    let time = timer.elapsed().as_secs_f64() / samples.len() as f64;

    println!("{}:\t{:.3} seconds", name, time);
}