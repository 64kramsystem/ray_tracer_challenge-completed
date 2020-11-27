use super::{
    shape::{self, private::ShapeLocal},
    Shape,
};
use crate::{
    math::Matrix,
    math::{Tuple, EPSILON},
    properties::Material,
};

#[derive(Debug, ShapeAccessors, SmartDefault)]
pub struct Plane {
    #[default(_code = "shape::new_shape_id()")]
    pub id: u32,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,
    #[default(Material::default())]
    pub material: Material,
}

impl ShapeLocal for Plane {
    fn local_normal(&self, _object_point: &Tuple) -> Tuple {
        Tuple::vector(0, 1, 0)
    }

    fn local_intersections(&self, transformed_ray: &super::Ray) -> Option<(f64, f64)> {
        if transformed_ray.direction.y.abs() < EPSILON {
            None
        } else {
            let t = -transformed_ray.origin.y / transformed_ray.direction.y;
            Some((t, t))
        }
    }
}
