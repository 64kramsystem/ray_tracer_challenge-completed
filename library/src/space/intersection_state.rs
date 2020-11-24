use crate::{math::Tuple, space::Shape};

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
