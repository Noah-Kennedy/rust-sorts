#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub use sorts::*;

mod sorts;

mod dynamic;
mod tabular;
mod alphanumeric;

#[cfg(test)]
mod tests;