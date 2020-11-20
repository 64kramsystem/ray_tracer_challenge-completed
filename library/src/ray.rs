use crate::{has_float64_value::HasFloat64Value, IntersectionState, Matrix, Shape, Tuple};

#[derive(PartialEq, Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new<T: HasFloat64Value, U: HasFloat64Value>(
        origin: (T, T, T),
        direction: (U, U, U),
    ) -> Self {
        Ray {
            origin: Tuple::point(origin.0, origin.1, origin.2),
            direction: Tuple::vector(direction.0, direction.1, direction.2),
        }
    }

    pub fn position<T: HasFloat64Value>(&self, t: T) -> Tuple {
        self.origin + &(self.direction * t.as_f64())
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

    pub fn inverse_transform(&self, transform: &Matrix) -> Self {
        let inverse_transform = transform.inverse();

        if let Some(inverse_transform) = inverse_transform {
            Self {
                origin: &inverse_transform * &self.origin,
                direction: &inverse_transform * &self.direction,
            }
        } else {
            panic!("Non-invertible transform matrix!")
        }
    }

    pub fn intersection_state<'a>(&self, t: f64, object: &'a dyn Shape) -> IntersectionState<'a> {
        let point = self.position(t);
        let eyev = -self.direction;

        IntersectionState::new(t, object, point, eyev)
    }

    pub fn hit(&self, sphere: &dyn Shape) -> Option<f64> {
        if let Some((t1, t2)) = sphere.intersections(self) {
            if t1 >= 0.0 {
                Some(t1)
            } else if t2 >= 0.0 {
                Some(t2)
            } else {
                None
            }
        } else {
            None
        }
    }
}
