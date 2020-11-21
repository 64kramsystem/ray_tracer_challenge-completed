mod color;
mod has_float64_value;
mod image;
mod material;
pub mod math;
mod sdl2_interface;
pub mod space;
mod virtual_image;

pub use color::Color;
pub use image::Image;
pub use material::Material;
pub use sdl2_interface::Sdl2Interface;
pub use virtual_image::VirtualImage;

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
