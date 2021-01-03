use super::{Intersection, IntersectionState};
use crate::{
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
    pub fn new<T: Into<f64>, U: Into<f64>>(origin: (T, T, T), direction: (U, U, U)) -> Self {
        Ray {
            origin: Tuple::point(origin.0, origin.1, origin.2),
            direction: Tuple::vector(direction.0, direction.1, direction.2),
        }
    }

    pub fn position<T: Into<f64>>(&self, t: T) -> Tuple {
        self.origin + &(self.direction * t.into())
    }

    pub fn translate<T: Into<f64>>(&self, x: T, y: T, z: T) -> Self {
        Self {
            origin: self.origin.translate(x, y, z),
            direction: self.direction,
        }
    }

    pub fn scale<T: Into<f64> + Copy>(&self, x: T, y: T, z: T) -> Self {
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
    ) -> IntersectionState<'a> {
        let point = self.position(intersection.t);
        let eyev = -self.direction;
        let mut normalv = intersection.object.normal(&point, intersection);
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
            object: intersection.object,
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
        let mut containers = Vec::<&dyn Shape>::new();
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
                containers.push(intersection.object);
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
        if intersections.is_empty() {
            (REFRACTIVE_INDEX_VACUUM, REFRACTIVE_INDEX_VACUUM)
        } else {
            (comps.0.unwrap(), comps.1.unwrap())
        }
    }
}
