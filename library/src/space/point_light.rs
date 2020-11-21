use crate::{math::Tuple, Color};

pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: (i32, i32, i32), intensity: (i32, i32, i32)) -> Self {
        Self {
            position: Tuple::point(position.0, position.1, position.2),
            intensity: Color::new(intensity.0, intensity.1, intensity.2),
        }
    }
}
