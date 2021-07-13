#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub use sorts::{
    dynamic_burst_sort,
    tabular_burst_sort,
};

mod sorts;

mod dynamic;
mod tabular;
