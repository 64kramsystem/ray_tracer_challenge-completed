use crate::{math::Tuple, space::Shape};

#[derive(Debug, PartialEq)]
pub struct IntersectionState<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
    pub point: Tuple,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub reflectv: Tuple,
    pub n1: f64,
    pub n2: f64,
    pub inside: bool,
}
