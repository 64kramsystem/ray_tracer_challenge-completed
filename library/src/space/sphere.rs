use std::sync::{Arc, Mutex, Weak};

use super::{shape, shape::private::ShapeLocal, BoundedShape, Bounds, Intersection, Shape};
use crate::{
    lang::math::sqrt,
    lang::HasFloat64Value,
    math::{Matrix, Tuple},
    properties::Material,
    Axis,
};

#[derive(Debug, ShapeAccessors, SmartDefault)]
pub struct Sphere {
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

impl Sphere {
    pub fn scale<T: HasFloat64Value>(self, x: T, y: T, z: T) -> Self {
        self.transform(&Matrix::scaling(x, y, z))
    }

    pub fn equiscale<T: HasFloat64Value + Copy>(self, s: T) -> Self {
        self.transform(&Matrix::scaling(s, s, s))
    }

    pub fn translate<T: HasFloat64Value>(self, x: T, y: T, z: T) -> Self {
        self.transform(&Matrix::translation(x, y, z))
    }

    pub fn rotate(self, axis: Axis, r: f64) -> Self {
        self.transform(&Matrix::rotation(axis, r))
    }

    // Returns a new Sphere with same id, with new transformation = (transformation * self.transformation).
    //
    pub fn transform(mut self, transformation: &Matrix) -> Self {
        let new_transformation = transformation * &self.transform;
        self.transform = new_transformation;
        self
    }
}

impl ShapeLocal for Sphere {
    fn local_normal(&self, object_point: &Tuple) -> Tuple {
        object_point - &Tuple::point(0, 0, 0)
    }

    fn local_intersections(self: Arc<Self>, transformed_ray: &super::Ray) -> Vec<Intersection> {
        let sphere_location = Tuple::point(0, 0, 0);
        let sphere_to_ray = transformed_ray.origin - &sphere_location;

        let a = transformed_ray
            .direction
            .dot_product(&transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot_product(&sphere_to_ray);
        let c = sphere_to_ray.dot_product(&sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - sqrt(discriminant)) / (2.0 * a);
            let t2 = (-b + sqrt(discriminant)) / (2.0 * a);

            vec![
                Intersection {
                    t: t1,
                    object: Arc::clone(&self) as Arc<dyn Shape>,
                },
                Intersection {
                    t: t2,
                    object: self,
                },
            ]
        }
    }
}

impl BoundedShape for Sphere {
    fn local_bounds(&self) -> Bounds {
        Bounds {
            min: Tuple::point(-1, -1, -1),
            max: Tuple::point(1, 1, 1),
        }
    }
}
