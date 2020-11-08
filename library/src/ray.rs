use crate::{has_float64_value::HasFloat64Value, Tuple};

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

    // The sphere is assumed to be located at (0, 0, 0).
    //
    pub fn sphere_intersections(&self) -> Option<(f64, f64)> {
        let sphere_location = Tuple::point(0, 0, 0);
        let sphere_to_ray = self.origin - sphere_location;

        let a = self.direction.dot_product(self.direction);
        let b = 2.0 * self.direction.dot_product(sphere_to_ray);
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
}
