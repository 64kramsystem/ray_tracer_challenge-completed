use std::fmt;

use super::Color;
use crate::math::{Matrix, Tuple};

pub trait Pattern: fmt::Debug + Sync + Send {
    fn transform(&self) -> &Matrix;
    fn previous_pattern(&self) -> &Option<Box<dyn Pattern>>;

    fn color_at(&self, pattern_point: &crate::math::Tuple) -> Color {
        let mut summed_colors = self.current_color_at(pattern_point);
        let mut colors_count = 1;

        let mut current_pattern_opt = self.previous_pattern();

        while let Some(current_pattern) = current_pattern_opt {
            summed_colors = summed_colors + &current_pattern.current_color_at(pattern_point);
            colors_count += 1;
            current_pattern_opt = current_pattern.previous_pattern();
        }

        summed_colors / colors_count as f64
    }

    // Watch out! The point is in pattern coordinates! Use Shape#color_at when dealing with world coordinates.
    //
    fn current_color_at(&self, pattern_point: &Tuple) -> Color;
}
