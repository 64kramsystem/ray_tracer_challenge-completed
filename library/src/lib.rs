mod color;
mod export_to_pixels;
mod has_float64_value;
mod matrix;
mod ppm_encoder;
mod sdl2_interface;
mod tuple;

pub use color::Color;
pub use export_to_pixels::ExportToPixels;
pub use matrix::Matrix2;
pub use matrix::Matrix3;
pub use matrix::Matrix4;
pub use ppm_encoder::PpmEncoder;
pub use sdl2_interface::Sdl2Interface;
pub use tuple::Tuple;

#[cfg(test)]
#[macro_use]
extern crate assert_float_eq;

#[cfg(test)]
mod tuple_test;

#[cfg(test)]
mod color_test;

#[cfg(test)]
mod ppm_encoder_test;

#[cfg(test)]
mod matrix_test;
