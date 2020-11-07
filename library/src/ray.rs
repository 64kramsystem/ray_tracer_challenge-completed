use crate::has_float64_value::HasFloat64Value;
use crate::Tuple;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn position<T: HasFloat64Value>(&self, t: T) -> Tuple {
        self.origin + self.direction * t.as_f64()
    }
}
