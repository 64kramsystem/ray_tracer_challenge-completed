use super::{Color, Pattern, COLOR_BLACK, COLOR_WHITE};

#[derive(Clone, Debug, SmartDefault)]
pub struct StripePattern {
    #[default(COLOR_WHITE)]
    pub color_a: Color,
    #[default(COLOR_BLACK)]
    pub color_b: Color,
}

impl Pattern for StripePattern {
    fn color_at(&self, point: &crate::math::Tuple) -> Color {
        if point.x.floor() as i32 % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}
