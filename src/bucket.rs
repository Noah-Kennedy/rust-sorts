use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use crate::insertion::insertion_sort;

#[derive(Debug)]
pub struct BucketError<T> {
    pub kind: BucketErrorKind<T>,
}

#[derive(Debug)]
pub enum BucketErrorKind<T> {
    OutOfRange(T)
}

impl<T> Display for BucketError<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            BucketErrorKind::OutOfRange(e) => {
                writeln!(f, "Bucketing error: {} is out of range!", e)
            }
        }
    }
}

impl<T> Error for BucketError<T> where T: Debug + Display {}

#[inline(never)]
pub fn bucket_sort<T, F>(data: &mut [T], key: F, num_buckets: usize) -> Result<(), BucketError<T>>
    where T: PartialOrd + Clone,
          F: Fn(&T) -> usize
{
    let buckets = make_buckets(data, key, num_buckets)?;

    dump_buckets(data, buckets);

    insertion_sort(data);

    Ok(())
}

#[inline(never)]
fn make_buckets<T, F>(data: &mut [T], key: F, num_buckets: usize)
                      -> Result<Vec<Vec<T>>, BucketError<T>>
    where T: Clone,
          F: Fn(&T) -> usize
{
    let mut buckets = vec![Vec::new(); num_buckets];

    for x in data {
        let k = key(x);

        let b = buckets.get_mut(k)
            .ok_or_else(|| BucketError { kind: BucketErrorKind::OutOfRange(x.clone()) })?;

        b.push(x.clone());
    }

    Ok(buckets)
}

#[inline(never)]
fn dump_buckets<T>(data: &mut [T], buckets: Vec<Vec<T>>)
    where T: Clone
{
    let mut start = 0;

    for b in buckets {
        let end = start + b.len();
        data[start..end].clone_from_slice(&b);
        start = end;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BUCKETS: usize = 256;

    // this is a bad key
    fn key(value: &String) -> usize {
        value.bytes().next().unwrap_or(0) as usize
    }


    #[quickcheck]
    fn unstable_sort_equivalence(data: Vec<String>) {
        let mut expected = data.clone();
        let mut actual = data;

        expected.sort_unstable();
        bucket_sort(&mut actual, key, BUCKETS)
            .unwrap();

        assert_eq!(expected, actual)
    }
}