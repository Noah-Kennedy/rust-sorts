#[macro_use]
#[cfg(test)]
extern crate quickcheck_macros;

pub use crate::trie::BurstConfig;
use crate::trie::TrieNode;
use std::borrow::Borrow;

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

/// Performs an element-by-element burstsort on the provided input.
///
/// # Arguments
///
/// * `data`: Vector of inputs to sort.
/// * `config`: Tuning configuration for the burstsort.
///
/// # Examples
///
/// ```
///
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