#[macro_use]
extern crate assert_float_eq;

mod color;
mod tuple;

pub use color::Color;
pub use tuple::Tuple;

#[cfg(test)]
mod tuple_test;

#[cfg(test)]
mod color_test;
