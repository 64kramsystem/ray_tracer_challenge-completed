pub trait HasFloat64Value {
    fn as_f64(self) -> f64;
}

impl HasFloat64Value for u32 {
    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl HasFloat64Value for f64 {
    fn as_f64(self) -> f64 {
        self
    }
}
