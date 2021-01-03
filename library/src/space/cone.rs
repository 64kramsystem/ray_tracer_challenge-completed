use std::{mem, sync::Weak};

use super::{shape, shape::private::ShapeLocal, BoundedShape, Bounds, Intersection, Ray, Shape};
use crate::{
    lang::{math::sqrt, ApproximateFloat64Ops},
    math::{Matrix, Tuple},
    properties::Material,
};

#[derive(Debug, ShapeAccessors, SmartDefault)]
pub struct Cone {
    #[default(_code = "shape::new_shape_id()")]
    pub id: u32,
    #[default(Weak::<Self>::new())]
    pub parent: Weak<dyn Shape>,
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

impl Cone {
    fn intersect_caps<'a>(&'a self, ray: &Ray, intersections: &mut Vec<Intersection<'a>>) {
        // Caps only matter if the Cone is closed, and might possibly be intersected by the ray.
        //
        if !self.closed || ray.direction.y.approximate() == 0.0 {
            return;
        };

        // Check for an intersection with the caps by intersecting the ray with the plane at y = minimum
        // and maximum.

        let t1 = (self.minimum - ray.origin.y) / ray.direction.y;

        if Self::check_cap(&ray, t1, self.minimum) {
            intersections.push(Intersection {
                t: t1,
                uv: None,
                object: self,
            });
        }

        let t2 = (self.maximum - ray.origin.y) / ray.direction.y;

        if Self::check_cap(&ray, t2, self.maximum) {
            intersections.push(Intersection {
                t: t2,
                uv: None,
                object: self,
            });
        }
    }

    // Check if the intersection at `t` is within a radius of 1 (the Cone radius) from the y axis.
    //
    fn check_cap(ray: &Ray, t: f64, y: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        (x.powi(2) + z.powi(2)) <= y.abs()
    }
}

impl ShapeLocal for Cone {
    // point: In object space.
    //
    fn local_normal(&self, point: &Tuple, _intersection: &Intersection) -> Tuple {
        // Compute the square of the distance from the y axis.
        //
        let dist = point.x.powi(2) + point.z.powi(2);

        if dist < 1.0 && point.y.approximate_greater_or_equal(self.maximum) {
            Tuple::vector(0, 1, 0)
        } else if dist < 1.0 && point.y.approximate_less_or_equal(self.minimum) {
            Tuple::vector(0, -1, 0)
        } else {
            let mut y = sqrt(point.x.powi(2) + point.z.powi(2));

            if point.y > 0.0 {
                y = -y;
            }

            Tuple::vector(point.x, y, point.z)
        }
    }

    // ray: In object space.
    //
    fn local_intersections(&self, ray: &super::Ray) -> Vec<Intersection> {
        let mut intersections = Vec::with_capacity(4);

        let a = ray.direction.x.powi(2) - ray.direction.y.powi(2) + ray.direction.z.powi(2);

        let b = 2.0 * ray.origin.x * ray.direction.x - 2.0 * ray.origin.y * ray.direction.y
            + 2.0 * ray.origin.z * ray.direction.z;

        let c = ray.origin.x.powi(2) - ray.origin.y.powi(2) + ray.origin.z.powi(2);

        // Ray is parallel to one of the cone halve walls.
        //
        if a.approximate_equals(0.0) {
            if b.approximate_equals(0.0) {
                return intersections;
            } else {
                let t = -c / (2.0 * b);

                intersections.push(Intersection {
                    t,
                    uv: None,
                    object: self,
                });
            }
        }

        let disc = b.powi(2) - 4.0 * a * c;

        // Ray intersects the walls.
        //
        if disc >= 0.0 {
            let mut t0 = (-b - sqrt(disc)) / (2.0 * a);
            let mut t1 = (-b + sqrt(disc)) / (2.0 * a);

            if t0 > t1 {
                mem::swap(&mut t0, &mut t1);
            }

            let y0 = ray.origin.y + t0 * ray.direction.y;

            if self.minimum < y0 && y0 < self.maximum {
                intersections.push(Intersection {
                    t: t0,
                    uv: None,
                    object: self,
                });
            }

            let y1 = ray.origin.y + t1 * ray.direction.y;

            if self.minimum < y1 && y1 < self.maximum {
                intersections.push(Intersection {
                    t: t1,
                    uv: None,
                    object: self,
                });
            }
        }

        self.intersect_caps(ray, &mut intersections);

        intersections
    }
}

impl BoundedShape for Cone {
    fn local_bounds(&self) -> Bounds {
        Bounds {
            min: Tuple::point(-1, self.minimum, -1),
            max: Tuple::point(1, self.maximum, 1),
        }
    }
}
