use super::{Color, Pattern, COLOR_BLACK, COLOR_WHITE};
use crate::math::Matrix;

#[derive(Debug, SmartDefault)]
pub struct RingPattern {
    #[default(COLOR_WHITE)]
    pub color_a: Color,
    #[default(COLOR_BLACK)]
    pub color_b: Color,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,
}

impl Pattern for RingPattern {
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn color_at(&self, pattern_point: &crate::math::Tuple) -> Color {
        let sum_powers = pattern_point.x.powi(2) + pattern_point.z.powi(2);

        if sum_powers.sqrt().floor() as u32 % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}
