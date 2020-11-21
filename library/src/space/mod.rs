mod camera;
mod intersection;
mod intersection_state;
mod point_light;
mod ray;
mod shape;
mod sphere;
mod world;

pub use camera::Camera;
pub use intersection_state::IntersectionState;
pub use point_light::PointLight;
pub use ray::Ray;
pub use shape::Shape;
pub use sphere::Sphere;
pub use world::World;

#[cfg(test)]
mod camera_test;

#[cfg(test)]
mod ray_test;

#[cfg(test)]
mod shape_test;

#[cfg(test)]
mod sphere_test;

#[cfg(test)]
mod world_test;
