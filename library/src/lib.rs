#[macro_use]
extern crate assert_float_eq;

mod tuple;

pub use tuple::Tuple;

#[cfg(test)]
mod tuple_test;
