mod camera;
mod color;
mod has_float64_value;
mod image;
mod intersection;
mod intersection_state;
mod material;
mod matrix;
mod point_light;
mod ppm_encoder;
mod ray;
mod sdl2_interface;
mod shape;
mod sphere;
mod tuple;
mod virtual_image;
mod world;

pub use camera::Camera;
pub use color::Color;
pub use image::Image;
pub use intersection_state::IntersectionState;
pub use material::Material;
pub use matrix::Matrix;
pub use point_light::PointLight;
pub use ppm_encoder::PpmEncoder;
pub use ray::Ray;
pub use sdl2_interface::Sdl2Interface;
pub use shape::Shape;
pub use sphere::Sphere;
pub use tuple::Tuple;
pub use virtual_image::VirtualImage;
pub use world::World;

pub const EPSILON: f64 = 1e-4;

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
mod tuple_test;

#[cfg(test)]
mod color_test;

#[cfg(test)]
mod ppm_encoder_test;

#[cfg(test)]
mod matrix_test;

#[cfg(test)]
mod ray_test;

#[cfg(test)]
mod sphere_test;

#[cfg(test)]
mod material_test;

#[cfg(test)]
mod world_test;

#[cfg(test)]
mod camera_test;

#[cfg(test)]
mod shape_test;
