// Makes math formulas look better, since having sqrt() at the right of the expression is confusing.
//
pub fn sqrt<T: Into<f64>>(value: T) -> f64 {
    value.into().sqrt()
}
