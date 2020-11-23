use super::{Color, Pattern, COLOR_BLACK, COLOR_WHITE};
use crate::{lang::NoisyFloat64, math::Matrix};

#[derive(Debug, SmartDefault)]
pub struct RingPattern {
    #[default(COLOR_WHITE)]
    pub color_a: Color,
    #[default(COLOR_BLACK)]
    pub color_b: Color,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,
    #[default(None)]
    pub previous_pattern: Option<Box<dyn Pattern>>,
}

impl Pattern for RingPattern {
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn previous_pattern(&self) -> &Option<Box<dyn Pattern>> {
        &self.previous_pattern
    }

    fn current_color_at(&self, pattern_point: &crate::math::Tuple) -> Color {
        let denoised_root_floor = (pattern_point.x.powi(2) + pattern_point.z.powi(2))
            .sqrt()
            .denoise()
            .floor();

        if denoised_root_floor as u32 % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}
