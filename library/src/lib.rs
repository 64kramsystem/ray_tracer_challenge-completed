pub mod interface;
pub mod lang;
pub mod math;
pub mod properties;
pub mod space;
pub mod utils;

pub enum Axis {
    X,
    Y,
    Z,
}

#[macro_use]
extern crate smart_default;

#[macro_use]
extern crate macros;

#[cfg(test)]
#[macro_use]
extern crate assert_float_eq;
