use super::*;
use unicode_segmentation::UnicodeSegmentation;

fn read_file(file: &str) -> Vec<String> {
    let text = std::fs::read_to_string(file).unwrap();
    let words = text.unicode_words();

    let data = words.map(ToOwned::to_owned).collect();

    data
}

fn read_file_alpha(file: &str) -> Vec<String> {
    let text = std::fs::read_to_string(file).unwrap();
    let words = text.unicode_words();

    let data = words
        .filter(|s| s.as_bytes().iter().all(|c| c.is_ascii_alphabetic()))
        .map(ToOwned::to_owned)
        .collect();

    data
}

#[quickcheck]
fn check_byte_sort(data: Vec<String>) {
    let mut expected = data.clone();
    let mut actual = data;

    expected.sort();
    byte_sort(&mut actual);

    pretty_assertions::assert_eq!(expected, actual)
}

#[quickcheck]
fn check_unicode_sort(data: Vec<String>) {
    let mut expected = data.clone();
    let mut actual = data;

    expected.sort();
    unicode_sort_unstable(&mut actual);

    pretty_assertions::assert_eq!(expected, actual)
}

#[test]
fn byte_file_english() {
    let mut data = read_file("../data/eng_news_2020_1M/eng_news_2020_1M-sentences.txt");

    drop(data.drain(8192*4..));

    let mut expected = data.clone();
    let mut actual = data;

    expected.sort();
    byte_sort(&mut actual);

    pretty_assertions::assert_eq!(expected, actual)
}

#[test]
fn unicode_file_english() {
    let mut data = read_file("../data/eng_news_2020_1M/eng_news_2020_1M-sentences.txt");

    drop(data.drain(8192*4..));

    let mut expected = data.clone();
    let mut actual = data;

    expected.sort();
    unicode_sort_unstable(&mut actual);

    pretty_assertions::assert_eq!(expected, actual)
}

#[test]
fn byte_file_arabic() {
    let mut data = read_file("../data/ara_news_2020_1M/ara_news_2020_1M-sentences.txt");

    drop(data.drain(8192*4..));

    let mut expected = data.clone();
    let mut actual = data;

    expected.sort();
    byte_sort(&mut actual);

    pretty_assertions::assert_eq!(expected, actual)
}

#[test]
fn unicode_file_arabic() {
    let mut data = read_file("../data/ara_news_2020_1M/ara_news_2020_1M-sentences.txt");

    drop(data.drain(8192*4..));

    let mut expected = data.clone();
    let mut actual = data;

    expected.sort();
    unicode_sort_unstable(&mut actual);

    pretty_assertions::assert_eq!(expected, actual)
}

#[test]
fn alphanumeric_file() {
    let mut data = read_file_alpha("../data/eng_news_2020_1M/eng_news_2020_1M-sentences.txt");

    drop(data.drain(8192*4..));

    let mut expected = data.clone();
    let mut actual = data;

    expected.sort();
    alphanumeric_sort(&mut actual).unwrap();

    pretty_assertions::assert_eq!(expected, actual)
}

#[test]
fn unchecked_alphanumeric_file() {
    let mut data = read_file_alpha("../data/eng_news_2020_1M/eng_news_2020_1M-sentences.txt");

    drop(data.drain(8192*4..));

    let mut expected = data.clone();
    let mut actual = data;

    expected.sort();
    alphanumeric_unchecked_sort(&mut actual);

    pretty_assertions::assert_eq!(expected, actual)
}