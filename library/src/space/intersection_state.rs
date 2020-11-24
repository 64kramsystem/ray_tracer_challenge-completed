use crate::{
    math::{Tuple, EPSILON},
    space::Shape,
};

#[derive(Debug, PartialEq)]
pub struct IntersectionState<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

impl<'a> IntersectionState<'a> {
    pub fn new(t: f64, object: &'a dyn Shape, point: Tuple, eyev: Tuple) -> Self {
        let mut normalv = object.normal(&point);
        let inside = if normalv.dot_product(&eyev) >= 0.0 {
            false
        } else {
            normalv = -normalv;
            true
        };
        let over_point = point + &(normalv * EPSILON);

        Self {
            t,
            object,
            point,
            over_point,
            eyev,
            normalv,
            inside,
        }
    }
}
