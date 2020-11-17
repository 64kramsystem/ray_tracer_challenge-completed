use crate::{Sphere, Tuple};

#[derive(Debug, PartialEq)]
pub struct IntersectionState<'a> {
    pub t: f64,
    pub object: &'a Sphere,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

impl<'a> IntersectionState<'a> {
    pub fn new(t: f64, object: &'a Sphere, point: Tuple, eyev: Tuple) -> Self {
        let mut normalv = object.normal(&point);
        let inside = if normalv.dot_product(&eyev) >= 0.0 {
            false
        } else {
            normalv = -normalv;
            true
        };

        Self {
            t,
            object,
            point,
            eyev,
            normalv,
            inside,
        }
    }
}
