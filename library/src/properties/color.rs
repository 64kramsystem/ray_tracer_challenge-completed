use std::ops::{Add, Div, Mul, Sub};

use crate::lang::ApproximateFloat64Ops;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new<T: Into<f64>, U: Into<f64>, V: Into<f64>>(r: T, g: U, b: V) -> Self {
        Self {
            r: r.into(),
            g: g.into(),
            b: b.into(),
        }
    }

    pub fn u8_components(&self) -> (u8, u8, u8) {
        fn to_u8(value: f64) -> u8 {
            let unbounded_result = 256.0 * value;
            unbounded_result.min(255.0) as u8
        }

        (to_u8(self.r), to_u8(self.g), to_u8(self.b))
    }
}

impl PartialEq for Color {
    // Values are considered as equal if within ε.
    //
    fn eq(&self, rhs: &Self) -> bool {
        self.r.approximate_equals(rhs.r)
            && self.g.approximate_equals(rhs.g)
            && self.b.approximate_equals(rhs.b)
    }
}

impl Add<&Self> for Color {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub<&Self> for Color {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

// The text doesn't specify the range and signedness of the scalar, so a conservative choice is made.
//
impl Mul<i32> for Color {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            r: self.r * rhs as f64,
            g: self.g * rhs as f64,
            b: self.b * rhs as f64,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

// "Hadamard product".
//
impl Mul<&Self> for Color {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}
