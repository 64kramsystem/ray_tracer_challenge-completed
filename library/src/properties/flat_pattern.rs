use super::{Color, Pattern, COLOR_WHITE};
use crate::lang::HasFloat64Value;

#[derive(Debug, SmartDefault)]
pub struct FlatPattern {
    #[default(COLOR_WHITE)]
    pub color: Color,
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
        }
    }
}

impl Pattern for FlatPattern {
    fn color_at(&self, _point: &crate::math::Tuple) -> Color {
        self.color
    }
}
