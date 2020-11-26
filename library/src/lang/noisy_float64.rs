use crate::math::EPSILON;

// In some context, values very close to 0 can cause havoc, for example when running floor().
// This trait add the denoise() method, that returns 0 if the absolute value is smaller than EPSILON,
// and the value otherwise. This is very useful to keep the patterns code clean.
//
pub trait NoisyFloat64 {
    fn denoise(self) -> f64;
    fn denoised_less_or_equal(self, rhs: f64) -> bool;
}

impl NoisyFloat64 for f64 {
    fn denoise(self) -> Self {
        if self.abs() < EPSILON {
            0.0
        } else {
            self
        }
    }

    fn denoised_less_or_equal(self, rhs: f64) -> bool {
        self <= rhs + EPSILON
    }
}
