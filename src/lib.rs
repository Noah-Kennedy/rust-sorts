#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod bucket;

pub mod insertion;

pub mod string;

mod burst_trie;

#[cfg(feature = "benchmarking")]
pub mod benchmarking;