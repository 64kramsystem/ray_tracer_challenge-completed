use crate::{Color, Tuple};

pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}
