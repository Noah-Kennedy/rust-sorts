#[inline(never)]
pub fn insertion_sort<T>(data: &mut [T]) where T: PartialOrd {
    for i in 1..data.len() {
        let mut j = i as isize;

        while j > 0 && data[j as usize - 1] > data[j as usize] {
            data.swap(j as usize, j as usize - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn unstable_sort_equivalence(data: Vec<i64>) {
        let mut expected = data.clone();
        let mut actual = data;

        expected.sort_unstable();
        insertion_sort(&mut actual);

        assert_eq!(expected, actual)
    }
}