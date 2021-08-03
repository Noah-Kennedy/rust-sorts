#[macro_use]
#[cfg(test)]
extern crate quickcheck_macros;

pub use crate::trie::BurstConfig;
use crate::trie::TrieNode;

mod trie;

#[cfg(feature = "_benchmarking")]
pub mod benching;

#[cfg(test)]
mod tests;

pub fn sort<T, I>(data: &mut Vec<T>, config: &BurstConfig)
    where T: PartialEq + AsRef<[I]> + Clone + Ord,
          I: Into<usize> + Clone
{
    let mut root = TrieNode::root(config);

    for x in data.drain(..) {
        root.insert(x);
    }

    root.merge(data);
}