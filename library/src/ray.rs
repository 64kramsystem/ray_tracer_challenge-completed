use crate::{has_float64_value::HasFloat64Value, Matrix, Sphere, Tuple};

#[derive(PartialEq, Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn position<T: HasFloat64Value>(&self, t: T) -> Tuple {
        self.origin + self.direction * t.as_f64()
    }

    pub fn translate<T: HasFloat64Value>(&self, x: T, y: T, z: T) -> Self {
        Self {
            origin: self.origin.translate(x, y, z),
            direction: self.direction,
        }
    }

    pub fn scale<T: HasFloat64Value + Copy>(&self, x: T, y: T, z: T) -> Self {
        Self {
            origin: self.origin.scale(x, y, z),
            direction: self.direction.scale(x, y, z),
        }
    }

    pub fn inverse_transform(&self, transform: Matrix) -> Self {
        let inverse_transform = transform.inverse();

        if let Some(inverse_transform) = inverse_transform {
            let inverse_transform_clone = inverse_transform.clone();

            Self {
                origin: inverse_transform * self.origin,
                direction: inverse_transform_clone * self.direction,
            }
        } else {
            panic!("Non-invertible transform matrix!")
        }
    }

    // The sphere is assumed to be located at (0, 0, 0).
    //
    pub fn intersections(&self, sphere: Sphere) -> Option<(f64, f64)> {
        let transformed_ray = self.inverse_transform(sphere.transformation);

        let sphere_location = Tuple::point(0, 0, 0);
        let sphere_to_ray = transformed_ray.origin - sphere_location;

        let a = transformed_ray
            .direction
            .dot_product(transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot_product(sphere_to_ray);
        let c = sphere_to_ray.dot_product(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            Some((t1, t2))
        }
    }

    pub fn hit(&self, sphere: Sphere) -> Option<f64> {
        if let Some((t1, t2)) = self.intersections(sphere) {
            if t1 >= 0.0 {
                if t2 >= 0.0 {
                    Some(f64::min(t1, t2))
                } else {
                    Some(t1)
                }
            } else {
                if t2 >= 0.0 {
                    Some(t2)
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}
