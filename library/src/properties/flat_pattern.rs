use super::{Color, Pattern, COLOR_WHITE};
use crate::{lang::HasFloat64Value, math::Matrix};

#[derive(Debug, SmartDefault)]
pub struct FlatPattern {
    #[default(COLOR_WHITE)]
    pub color: Color,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,
}

impl FlatPattern {
    pub fn new<T: HasFloat64Value, U: HasFloat64Value, V: HasFloat64Value>(
        r: T,
        g: U,
        b: V,
    ) -> Self {
        Self {
            color: Color {
                r: r.as_f64(),
                g: g.as_f64(),
                b: b.as_f64(),
            },
            ..FlatPattern::default()
        }
    }
}

impl Pattern for FlatPattern {
    fn transform(&self) -> &crate::math::Matrix {
        &self.transform
    }

    fn color_at(&self, _pattern_point: &crate::math::Tuple) -> Color {
        self.color
    }
}
