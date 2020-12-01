use std::sync::{Arc, Mutex, Weak};

use super::{
    shape::{self, private::ShapeLocal},
    Ray, Shape,
};
use crate::{lang::ApproximateFloat64Ops, math::Matrix, math::Tuple, properties::Material};

#[derive(Debug, ShapeAccessors, SmartDefault)]
pub struct Plane {
    #[default(_code = "shape::new_shape_id()")]
    pub id: u32,
    #[default(Mutex::new(Weak::<Self>::new()))]
    pub parent: Mutex<Weak<dyn Shape>>,
    #[default(Mutex::new(vec![]))]
    pub children: Mutex<Vec<Arc<dyn Shape>>>,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,
    #[default(Material::default())]
    pub material: Material,
}

impl ShapeLocal for Plane {
    fn local_normal(&self, _object_point: &Tuple) -> Tuple {
        Tuple::vector(0, 1, 0)
    }

    fn local_intersections(&self, transformed_ray: &Ray) -> Vec<f64> {
        if transformed_ray.direction.y.within_epsilon() {
            vec![]
        } else {
            let t = -transformed_ray.origin.y / transformed_ray.direction.y;

            vec![t]
        }
    }
}
