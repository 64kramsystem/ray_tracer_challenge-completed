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
}

impl Pattern for GradientPattern {
    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn color_at(&self, pattern_point: &crate::math::Tuple) -> Color {
        let distance = self.color_b - &self.color_a;
        let fraction = pattern_point.x - pattern_point.x.floor();
        self.color_a + &(distance * fraction)
    }
}
