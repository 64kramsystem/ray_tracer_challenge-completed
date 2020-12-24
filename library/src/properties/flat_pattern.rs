use super::{Color, Pattern, COLOR_WHITE};
use crate::math::Matrix;

#[derive(Debug, SmartDefault)]
pub struct FlatPattern {
    #[default(COLOR_WHITE)]
    pub color: Color,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,
    #[default(None)]
    pub previous_pattern: Option<Box<dyn Pattern>>,
}

impl FlatPattern {
    pub fn new<T: Into<f64>, U: Into<f64>, V: Into<f64>>(r: T, g: U, b: V) -> Self {
        Self {
            color: Color {
                r: r.into(),
                g: g.into(),
                b: b.into(),
            },
            ..FlatPattern::default()
        }
    }
}

impl Pattern for FlatPattern {
    fn transform(&self) -> &crate::math::Matrix {
        &self.transform
    }

    fn previous_pattern(&self) -> &Option<Box<dyn Pattern>> {
        &self.previous_pattern
    }

    // point: In pattern space.
    //
    fn current_color_at(&self, _point: &crate::math::Tuple) -> Color {
        self.color
    }
}
