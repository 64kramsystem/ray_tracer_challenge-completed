use std::fmt;

use super::Color;
use crate::math::Tuple;

pub trait Pattern: fmt::Debug + Sync {
    fn color_at(&self, point: &Tuple) -> Color;
}
