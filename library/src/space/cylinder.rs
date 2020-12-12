use std::{
    mem,
    sync::{Arc, Mutex, Weak},
};

use super::{shape, shape::private::ShapeLocal, BoundedShape, Bounds, Intersection, Ray, Shape};
use crate::{
    lang::{math::sqrt, ApproximateFloat64Ops},
    math::{Matrix, Tuple},
    properties::Material,
};

#[derive(Debug, ShapeAccessors, SmartDefault)]
pub struct Cylinder {
    #[default(_code = "shape::new_shape_id()")]
    pub id: u32,
    #[default(Mutex::new(Weak::<Self>::new()))]
    pub parent: Mutex<Weak<dyn Shape>>,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,
    #[default(Material::default())]
    pub material: Material,

    #[default(f64::NEG_INFINITY)]
    pub minimum: f64,
    #[default(f64::INFINITY)]
    pub maximum: f64,
    pub closed: bool,
}

impl Cylinder {
    fn intersect_caps(
        &self,
        self_arc: &Arc<dyn Shape>,
        ray: &Ray,
        intersections: &mut Vec<Intersection>,
    ) {
        // Caps only matter if the cylinder is closed, and might possibly be intersected by the ray.
        //
        if !self.closed || ray.direction.y.approximate() == 0.0 {
            return;
        };

        // Check for an intersection with the caps by intersecting the ray with the plane at y = minimum
        // and maximum.

        let t1 = (self.minimum - ray.origin.y) / ray.direction.y;

        if Self::check_cap(&ray, t1) {
            intersections.push(Intersection {
                t: t1,
                object: Arc::clone(self_arc),
                ..Intersection::default()
            });
        }

        let t2 = (self.maximum - ray.origin.y) / ray.direction.y;

        if Self::check_cap(&ray, t2) {
            intersections.push(Intersection {
                t: t2,
                object: Arc::clone(self_arc),
                ..Intersection::default()
            });
        }
    }

    // Check if the intersection at `t` is within a radius of 1 (the cylinder radius) from the y axis.
    //
    fn check_cap(ray: &Ray, t: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        (x.powi(2) + z.powi(2)) <= 1.0
    }
}

impl ShapeLocal for Cylinder {
    fn local_normal(&self, object_point: &Tuple, _intersection: &Intersection) -> Tuple {
        // Compute the square of the distance from the y axis.
        //
        let dist = object_point.x.powi(2) + object_point.z.powi(2);

        if dist < 1.0 && object_point.y.approximate_greater_or_equal(self.maximum) {
            Tuple::vector(0, 1, 0)
        } else if dist < 1.0 && object_point.y.approximate_less_or_equal(self.minimum) {
            Tuple::vector(0, -1, 0)
        } else {
            Tuple::vector(object_point.x, 0, object_point.z)
        }
    }

    fn local_intersections(
        &self,
        self_arc: &Arc<dyn Shape>,
        transformed_ray: &super::Ray,
    ) -> Vec<Intersection> {
        let mut intersections = Vec::with_capacity(2);

        let a = transformed_ray.direction.x.powi(2) + transformed_ray.direction.z.powi(2);

        // Ray is parallel to the y axis.
        //
        if a.approximate_equals(0.0) {
            self.intersect_caps(self_arc, transformed_ray, &mut intersections);

            return intersections;
        }

        let b = 2.0 * transformed_ray.origin.x * transformed_ray.direction.x
            + 2.0 * transformed_ray.origin.z * transformed_ray.direction.z;
        let c = transformed_ray.origin.x.powi(2) + transformed_ray.origin.z.powi(2) - 1.0;

        let disc = b.powi(2) - 4.0 * a * c;

        // Ray intersects the walls.
        // Note: the book says "intersect the cylinder", but that's confusing.
        //
        if disc >= 0.0 {
            let mut t0 = (-b - sqrt(disc)) / (2.0 * a);
            let mut t1 = (-b + sqrt(disc)) / (2.0 * a);

            if t0 > t1 {
                mem::swap(&mut t0, &mut t1);
            }

            let y0 = transformed_ray.origin.y + t0 * transformed_ray.direction.y;

            if self.minimum < y0 && y0 < self.maximum {
                intersections.push(Intersection {
                    t: t0,
                    object: Arc::clone(self_arc),
                    ..Intersection::default()
                });
            }

            let y1 = transformed_ray.origin.y + t1 * transformed_ray.direction.y;

            if self.minimum < y1 && y1 < self.maximum {
                intersections.push(Intersection {
                    t: t1,
                    object: Arc::clone(self_arc),
                    ..Intersection::default()
                });
            }
        }

        self.intersect_caps(self_arc, transformed_ray, &mut intersections);

        intersections
    }
}

impl BoundedShape for Cylinder {
    fn local_bounds(&self) -> Bounds {
        Bounds {
            min: Tuple::point(-1, self.minimum, -1),
            max: Tuple::point(1, self.maximum, 1),
        }
    }
}
