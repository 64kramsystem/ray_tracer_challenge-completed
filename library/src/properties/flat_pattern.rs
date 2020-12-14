use super::{Color, Pattern, COLOR_WHITE};
use crate::{lang::HasFloat64Value, math::Matrix};

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

    fn previous_pattern(&self) -> &Option<Box<dyn Pattern>> {
        &self.previous_pattern
    }

    // point: In pattern space.
    //
    fn current_color_at(&self, _point: &crate::math::Tuple) -> Color {
        self.color
    }
}
