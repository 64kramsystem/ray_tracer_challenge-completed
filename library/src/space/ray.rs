use std::sync::Arc;

use super::{Intersection, IntersectionState};
use crate::{
    lang::HasFloat64Value,
    math::{Matrix, Tuple, EPSILON},
    properties::REFRACTIVE_INDEX_VACUUM,
    space::Shape,
};

#[derive(PartialEq, Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new<T: HasFloat64Value, U: HasFloat64Value>(
        origin: (T, T, T),
        direction: (U, U, U),
    ) -> Self {
        Ray {
            origin: Tuple::point(origin.0, origin.1, origin.2),
            direction: Tuple::vector(direction.0, direction.1, direction.2),
        }
    }

    pub fn position<T: HasFloat64Value>(&self, t: T) -> Tuple {
        self.origin + &(self.direction * t.as_f64())
    }

    pub fn translate<T: HasFloat64Value>(&self, x: T, y: T, z: T) -> Self {
        Self {
            origin: self.origin.translate(x, y, z),
            direction: self.direction,
        }
    }

    pub fn scale<T: HasFloat64Value + Copy>(&self, x: T, y: T, z: T) -> Self {
        Self {
            origin: self.origin.scale(x, y, z),
            direction: self.direction.scale(x, y, z),
        }
    }

    pub fn inverse_transform(&self, transform: &Matrix) -> Self {
        let inverse_transform = transform.inverse();

        Self {
            origin: &inverse_transform * &self.origin,
            direction: &inverse_transform * &self.direction,
        }
    }

    // In the book, this is split between `intersection(t, s)` and `prepare_computations(i, r)`.
    //
    pub fn intersection_state<'a>(
        &self,
        intersection: &'a Intersection,
        intersections: &[Intersection],
    ) -> IntersectionState {
        let point = self.position(intersection.t);
        let eyev = -self.direction;
        let mut normalv = intersection.object.normal(&point);
        let inside = if normalv.dot_product(&eyev) >= 0.0 {
            false
        } else {
            normalv = -normalv;
            true
        };
        let over_point = point + &(normalv * EPSILON);
        let under_point = point - &(normalv * EPSILON);
        let reflectv = self.direction.reflect(&normalv);
        let (n1, n2) = Ray::refraction_indexes(intersection, intersections);

        IntersectionState {
            t: intersection.t,
            object: Arc::clone(&intersection.object),
            point,
            over_point,
            under_point,
            eyev,
            normalv,
            reflectv,
            n1,
            n2,
            inside,
        }
    }

    // In the book, this is part of `prepare_computations(i, r)`.
    //
    pub fn refraction_indexes(hit: &Intersection, intersections: &[Intersection]) -> (f64, f64) {
        let mut containers = Vec::<Arc<dyn Shape>>::new();
        let mut comps = (None, None);

        for intersection in intersections.iter() {
            if intersection == hit {
                let container_last = containers.last();

                if let Some(object) = container_last {
                    comps.0 = Some(object.material().refractive_index);
                } else {
                    comps.0 = Some(REFRACTIVE_INDEX_VACUUM);
                }
            }

            if let Some(pos) = containers
                .iter()
                .position(|object| object.id() == intersection.object.id())
            {
                containers.remove(pos);
            } else {
                containers.push(Arc::clone(&intersection.object));
            }

            if intersection == hit {
                let container_last = containers.last();

                if let Some(object) = container_last {
                    comps.1 = Some(object.material().refractive_index);
                } else {
                    comps.1 = Some(REFRACTIVE_INDEX_VACUUM);
                }

                break;
            }
        }

        // In many tests, we don't care about refraction, so we pass no intersections and get bogus
        // values.
        //
        if intersections.len() == 0 {
            (REFRACTIVE_INDEX_VACUUM, REFRACTIVE_INDEX_VACUUM)
        } else {
            (comps.0.unwrap(), comps.1.unwrap())
        }
    }
}
