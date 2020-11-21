use super::Color;
use crate::math::Tuple;

pub trait Pattern {
    fn color_at(&self, point: &Tuple) -> Color;
}
