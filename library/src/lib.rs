mod color;
mod has_float64_value;
pub mod interface;
mod material;
pub mod math;
pub mod space;

pub use color::Color;
pub use material::Material;

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

#[cfg(test)]
mod color_test;

#[cfg(test)]
mod material_test;
