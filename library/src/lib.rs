pub mod interface;
pub mod lang;
pub mod math;
pub mod properties;
pub mod space;

pub enum Axis {
    X,
    Y,
    Z,
}

#[macro_use]
extern crate smart_default;

#[cfg(test)]
#[macro_use]
extern crate assert_float_eq;
