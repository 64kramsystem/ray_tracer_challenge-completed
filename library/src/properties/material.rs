use crate::{math::Tuple, properties::Color, space::PointLight};

use super::{FlatPattern, Pattern, COLOR_BLACK};

#[derive(Debug)]
pub struct Material {
    pub pattern: Box<dyn Pattern>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            pattern: Box::new(FlatPattern::default()),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Material {
    pub fn lighting(
        &self,
        light: &PointLight,
        point: &Tuple,
        eyev: &Tuple,
        normalv: &Tuple,
        in_shadow: bool,
    ) -> Color {
        let color = self.pattern.color_at(&point);

        let effective_color = color * &light.intensity;

        let lightv = (light.position - point).normalize();

        let ambient = effective_color * self.ambient;

        let light_dot_normal = lightv.dot_product(&normalv);

        let (diffuse, specular) = if in_shadow || light_dot_normal < 0.0 {
            let diffuse = COLOR_BLACK;
            let specular = COLOR_BLACK;

            (diffuse, specular)
        } else {
            let diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = -lightv.reflect(&normalv);
            let reflect_dot_eye = reflectv.dot_product(&eyev);

            let specular = if reflect_dot_eye <= 0.0 {
                COLOR_BLACK
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);

                light.intensity * self.specular * factor
            };

            (diffuse, specular)
        };

        ambient + &diffuse + &specular
    }
}
