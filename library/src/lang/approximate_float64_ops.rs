use crate::math::EPSILON;

// Approximate f64 operations, very useful to keep the code clean, and to avoid easy ε-related errors.
// Naming is quite difficult, in particular, for the `approximate()` method.
//
pub trait ApproximateFloat64Ops {
    fn approximate(self) -> f64;
}

// Returns 0 if the absolute value is smaller than EPSILON, and the value otherwise.
// In some contexts, values very close to 0 can cause havoc, for example when running floor().
//
impl ApproximateFloat64Ops for f64 {
    fn approximate(self) -> Self {
        if self.abs() < EPSILON {
            0.0
        } else {
            self
        }
    }
}
