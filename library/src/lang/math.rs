use super::HasFloat64Value;

// Makes math formulas look better, since having sqrt() at the right of the expression is confusing.
//
pub fn sqrt<T: HasFloat64Value>(value: T) -> f64 {
    value.as_f64().sqrt()
}
