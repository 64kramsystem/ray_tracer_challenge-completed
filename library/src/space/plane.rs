use std::sync::{Arc, Mutex, Weak};

use super::{
    shape::{self, private::ShapeLocal},
    BoundedShape, Bounds, Intersection, Ray, Shape,
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
    fn local_normal(&self, _object_point: &Tuple, _intersection: &Intersection) -> Tuple {
        Tuple::vector(0, 1, 0)
    }

    fn local_intersections(self: Arc<Self>, transformed_ray: &Ray) -> Vec<Intersection> {
        if transformed_ray.direction.y.within_epsilon() {
            vec![]
        } else {
            let t = -transformed_ray.origin.y / transformed_ray.direction.y;

            vec![Intersection {
                t,
                object: self,
                ..Intersection::default()
            }]
        }
    }
}

impl BoundedShape for Plane {
    fn local_bounds(&self) -> Bounds {
        Bounds {
            min: Tuple::point(f64::NEG_INFINITY, 0, f64::NEG_INFINITY),
            max: Tuple::point(f64::INFINITY, 0, f64::INFINITY),
        }
    }
}
