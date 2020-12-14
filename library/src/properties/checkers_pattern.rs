use super::{Color, Pattern, COLOR_BLACK, COLOR_WHITE};
use crate::{lang::ApproximateFloat64Ops, math::Matrix};

#[derive(Debug, SmartDefault)]
pub struct CheckersPattern {
    #[default(COLOR_WHITE)]
    pub color_a: Color,
    #[default(COLOR_BLACK)]
    pub color_b: Color,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,
    #[default(None)]
    pub previous_pattern: Option<Box<dyn Pattern>>,
}

impl Pattern for CheckersPattern {
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn previous_pattern(&self) -> &Option<Box<dyn Pattern>> {
        &self.previous_pattern
    }

    // point: In pattern space.
    //
    fn current_color_at(&self, point: &crate::math::Tuple) -> Color {
        let approximated_floors_sum = point.x.approximate().floor()
            + point.y.approximate().floor()
            + point.z.approximate().floor();

        if approximated_floors_sum as i32 % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}
