use std::time::Instant;

use criterion::black_box;
use rand::distributions::{Alphanumeric, Distribution, Uniform};
use rand::distributions::uniform::SampleUniform;
use unicode_segmentation::UnicodeSegmentation;

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
        .filter(|s| s.as_bytes().iter().all(|c| c.is_ascii_alphanumeric()))
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