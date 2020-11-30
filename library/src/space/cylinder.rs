use super::{shape, shape::private::ShapeLocal, Shape};
use crate::{
    lang::{math::sqrt, ApproximateFloat64Ops},
    math::{Matrix, Tuple},
    properties::Material,
};

#[derive(Debug, ShapeAccessors, SmartDefault)]
pub struct Cylinder {
    #[default(_code = "shape::new_shape_id()")]
    pub id: u32,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,
    #[default(Material::default())]
    pub material: Material,
}

impl ShapeLocal for Cylinder {
    fn local_normal(&self, object_point: &Tuple) -> Tuple {
        Tuple::vector(object_point.x, 0, object_point.z)
    }

    fn local_intersections(&self, transformed_ray: &super::Ray) -> Option<(f64, f64)> {
        let a = transformed_ray.direction.x.powi(2) + transformed_ray.direction.z.powi(2);

        // Ray is parallel to the y axis.
        //
        if a.approximate_equals(0.0) {
            return None;
        }

        let b = 2.0 * transformed_ray.origin.x * transformed_ray.direction.x
            + 2.0 * transformed_ray.origin.z * transformed_ray.direction.z;
        let c = transformed_ray.origin.x.powi(2) + transformed_ray.origin.z.powi(2) - 1.0;

        let disc = b.powi(2) - 4.0 * a * c;

        // Ray does not intersect the cylinder.
        //
        if disc < 0.0 {
            None
        } else {
            let t0 = (-b - sqrt(disc)) / (2.0 * a);
            let t1 = (-b + sqrt(disc)) / (2.0 * a);

            Some((t0, t1))
        }
    }
}
