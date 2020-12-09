use std::sync::{Arc, Mutex, Weak};

use super::{
    shape::{self, private::ShapeLocal},
    BoundedShape, Bounds, Intersection, Ray, Shape,
};
use crate::{lang::ApproximateFloat64Ops, math::Matrix, math::Tuple, properties::Material};

const UNSUPPORTED_FEATURE_MESSAGE: &str = "Group/Box logic not implemented";

#[derive(Debug, ShapeAccessors, SmartDefault)]
pub struct Triangle {
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
    // The following defaults are not meaningful, but are required in order to allow type defaulting.
    //
    #[default(Tuple::point(0, 0, 0))]
    pub p1: Tuple,
    #[default(Tuple::point(0, 0, 0))]
    pub p2: Tuple,
    #[default(Tuple::point(0, 0, 0))]
    pub p3: Tuple,
    #[default(Tuple::point(0, 0, 0))]
    pub e1: Tuple,
    #[default(Tuple::point(0, 0, 0))]
    pub e2: Tuple,
    #[default(Tuple::point(0, 0, 0))]
    pub normal: Tuple,
}

impl Triangle {
    // Also computes the normal.
    //
    pub fn new(p1: Tuple, p2: Tuple, p3: Tuple) -> Self {
        let e1 = p2 - &p1;
        let e2 = p3 - &p1;

        let normal = e2.cross_product(e1).normalize();

        Triangle {
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
            ..Triangle::default()
        }
    }

    // Convenience for the test suite.
    //
    pub fn from_ints(p1: (i32, i32, i32), p2: (i32, i32, i32), p3: (i32, i32, i32)) -> Self {
        let p1 = Tuple::point(p1.0, p1.1, p1.2);
        let p2 = Tuple::point(p2.0, p2.1, p2.2);
        let p3 = Tuple::point(p3.0, p3.1, p3.2);

        Self::new(p1, p2, p3)
    }
}

impl ShapeLocal for Triangle {
    fn local_normal(&self, _object_point: &Tuple) -> Tuple {
        self.normal
    }

    fn local_intersections(self: Arc<Self>, transformed_ray: &Ray) -> Vec<Intersection> {
        let dir_cross_e2 = transformed_ray.direction.cross_product(self.e2);
        let determinant = self.e1.dot_product(&dir_cross_e2);

        if determinant.within_epsilon() {
            return vec![];
        }

        let f = 1.0 / determinant;
        let p1_to_origin = transformed_ray.origin - &self.p1;
        let u = f * p1_to_origin.dot_product(&dir_cross_e2);

        if u < 0.0 || u > 1.0 {
            return vec![];
        }

        let origin_cross_e1 = p1_to_origin.cross_product(self.e1);
        let v = f * transformed_ray.direction.dot_product(&origin_cross_e1);

        if v < 0.0 || (u + v) > 1.0 {
            return vec![];
        }

        let t = f * self.e2.dot_product(&origin_cross_e1);

        vec![Intersection {
            t,
            object: self,
            ..Intersection::default()
        }]
    }
}

impl BoundedShape for Triangle {
    fn local_bounds(&self) -> Bounds {
        panic!(UNSUPPORTED_FEATURE_MESSAGE)
    }
}
