mod bounded_shape;
mod bounds;
mod camera;
mod cone;
pub mod csg;
mod cube;
mod cylinder;
mod group;
mod intersection;
mod intersection_state;
mod plane;
mod point_light;
mod ray;
mod shape;
mod sphere;
mod triangle;
mod world;

pub use bounded_shape::BoundedShape;
pub use bounds::Bounds;
pub use camera::Camera;
pub use cone::Cone;
pub use csg::Csg;
pub use cube::Cube;
pub use cylinder::Cylinder;
pub use group::Group;
pub use intersection::Intersection;
pub use intersection_state::IntersectionState;
pub use plane::Plane;
pub use point_light::PointLight;
pub use ray::Ray;
pub use shape::Shape;
pub use sphere::Sphere;
pub use triangle::Triangle;
pub use world::World;

#[cfg(test)]
mod camera_test;

#[cfg(test)]
mod csg_test;

#[cfg(test)]
mod cube_test;

#[cfg(test)]
mod cone_test;

#[cfg(test)]
mod cylinder_test;

#[cfg(test)]
mod group_test;

#[cfg(test)]
mod intersection_state_test;

#[cfg(test)]
mod intersection_test;

#[cfg(test)]
mod plane_test;

#[cfg(test)]
mod ray_test;

#[cfg(test)]
mod shape_test;

#[cfg(test)]
mod sphere_test;

#[cfg(test)]
mod triangle_test;

#[cfg(test)]
mod world_test;
