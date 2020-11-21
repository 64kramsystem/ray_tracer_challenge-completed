mod matrix;
mod tuple;

pub use matrix::Matrix;
pub use tuple::Tuple;

pub const EPSILON: f64 = 1e-4;

#[cfg(test)]
mod matrix_test;

#[cfg(test)]
mod tuple_test;
