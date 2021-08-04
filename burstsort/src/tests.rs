use super::*;

#[quickcheck]
fn check_sort_string(mut data: Vec<String>) {
    let config = BurstConfig {
        burst_limit: 8,
        initial_capacity: 4,
        classes: 256,
    };

    let mut expected = data.clone();

    burstsort(&mut data, &config);

    expected.sort();

    assert_eq!(expected, data);
}
