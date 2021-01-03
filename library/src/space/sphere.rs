use std::sync::Weak;

use super::{shape, shape::private::ShapeLocal, BoundedShape, Bounds, Intersection, Shape};
use crate::{
    lang::math::sqrt,
    math::{Matrix, Tuple},
    properties::Material,
};

#[derive(Debug, ShapeAccessors, SmartDefault)]
pub struct Sphere {
    #[default(_code = "shape::new_shape_id()")]
    pub id: u32,
    #[default(Weak::<Self>::new())]
    pub parent: Weak<dyn Shape>,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,
    #[default(Material::default())]
    pub material: Material,
}

impl ShapeLocal for Sphere {
    // point: In object space.
    //
    fn local_normal(&self, point: Tuple, _intersection: &Intersection) -> Tuple {
        point - &Tuple::point(0, 0, 0)
    }

    // ray: In object space.
    //
    fn local_intersections(&self, ray: &super::Ray) -> Vec<Intersection> {
        let sphere_location = Tuple::point(0, 0, 0);
        let sphere_to_ray = ray.origin - &sphere_location;

        let a = ray.direction.dot_product(&ray.direction);
        let b = 2.0 * ray.direction.dot_product(&sphere_to_ray);
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
                    uv: None,
                    object: self,
                },
                Intersection {
                    t: t2,
                    uv: None,
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
