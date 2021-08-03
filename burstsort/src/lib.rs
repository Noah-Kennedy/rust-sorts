#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub use sorts::*;

mod sorts;

mod unicode;
mod byte;
#[cfg(feature = "unstable")]
mod alphanumeric;

#[cfg(test)]
mod tests;