use std::f64::consts::PI;

use super::{Color, Pattern, COLOR_BLACK, COLOR_WHITE};
use crate::math::Matrix;

#[derive(Debug, SmartDefault)]
pub struct GradientPattern {
    #[default(COLOR_WHITE)]
    pub color_a: Color,
    #[default(COLOR_BLACK)]
    pub color_b: Color,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,
    #[default(None)]
    pub previous_pattern: Option<Box<dyn Pattern>>,
}

impl Pattern for GradientPattern {
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn previous_pattern(&self) -> &Option<Box<dyn Pattern>> {
        &self.previous_pattern
    }

    fn current_color_at(&self, pattern_point: &crate::math::Tuple) -> Color {
        // This shouldn't need float denoise, as it doesn't rely on exact transformations/operations.
        //
        // Original formula:
        //
        //   let distance = self.color_b - &self.color_a;
        //   let fraction = pattern_point.x - pattern_point.x.floor();
        //   self.color_a + &(distance * fraction)

        // This formula starts at (color_a + half_distance), for simplicity (see test suite).
        // In order to start from color_a, just shift the sin() by subtracting π/4 from x.
        //
        let distance = self.color_b - &self.color_a;
        let half_distance = distance * 0.5;

        self.color_a + &half_distance - &(half_distance * (2.0 * PI * pattern_point.x).sin())
    }
}
