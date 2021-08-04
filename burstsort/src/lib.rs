#[macro_use]
#[cfg(test)]
extern crate quickcheck_macros;

use std::borrow::Borrow;

pub use crate::trie::BurstConfig;
use crate::trie::TrieNode;

mod trie;

#[cfg(feature = "_benchmarking")]
pub mod benching;

#[cfg(test)]
mod tests;

pub const ASCII_CONFIG: BurstConfig = BurstConfig {
    burst_limit: 16384,
    initial_capacity: 0,
    classes: 127,
};

pub const UTF8_CONFIG: BurstConfig = BurstConfig {
    classes: 256,
    ..ASCII_CONFIG
};

/// Sorts the provided data using a burstsort algorithm.
///
/// # Arguments
/// * `data`: Vector of inputs to sort.
/// * `config`: Tuning configuration for the burstsort. You should probably use a reference here.
///
/// # Type Parameters
/// * `T`: Type to be sorted. Must be able to be used as a slice of `I`.
/// * `C`: Generic reference to a burstsort config.
/// * `I`: Generic type that `T` is an array of. Must be able to be casted to [usize]. For
/// strings, this would be [u8], for example.
///
/// # Panicking
/// This function *might* panic if inputs contain radixes that are larger than the provided
/// number of radix classes in the config struct.
/// This may become a guaranteed panic in the future.
/// For example, passing non-ASCII characters to a string sort using [ASCII_CONFIG] may panic
/// this way.
///
/// This function *might* also panic if the conversion from `I` to [usize] fails. For example,
/// converting a negative number to a [usize].
///
/// # Examples
/// ```
/// let mut strings = vec!["apple", "strawberry", "pear", "orange", "banana"];
/// let sorted_strings = vec!["apple", "banana", "orange", "pear", "strawberry"];
///
/// burstsort::burstsort(&mut strings, &burstsort::ASCII_CONFIG);
///
/// assert_eq!(sorted_strings, strings);
/// ```
pub fn burstsort<T, C, I>(data: &mut Vec<T>, config: C)
    where T: PartialEq + AsRef<[I]> + Clone + Ord,
          C: Borrow<BurstConfig> + Clone,
          I: Into<usize> + Clone + Ord
{
    let mut root = TrieNode::root(config);

    for x in data.drain(..) {
        root.insert(x);
    }

    root.merge(data);
}