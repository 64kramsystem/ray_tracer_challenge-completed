use std::fmt;

use super::Color;
use crate::math::{Matrix, Tuple};

pub trait Pattern: fmt::Debug + Sync {
    fn transform(&self) -> &Matrix;

    // Watch out! The point is in pattern coordinates! Use Shape#color_at when dealing with world coordinates.
    //
    fn color_at(&self, pattern_point: &Tuple) -> Color;
}
