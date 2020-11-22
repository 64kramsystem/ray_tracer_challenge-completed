use std::ops::{Add, Div, Mul, Sub};

use crate::math::EPSILON;

use crate::lang::HasFloat64Value;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new<T: HasFloat64Value, U: HasFloat64Value, V: HasFloat64Value>(
        r: T,
        g: U,
        b: V,
    ) -> Self {
        Self {
            r: r.as_f64(),
            g: g.as_f64(),
            b: b.as_f64(),
        }
    }

    pub fn u8_components(&self) -> (u8, u8, u8) {
        fn to_u8(value: f64) -> u8 {
            if (value - 1.0).abs() < EPSILON {
                255
            } else {
                (256.0 * value) as u8
            }
        }

        (to_u8(self.r), to_u8(self.g), to_u8(self.b))
    }
}

impl PartialEq for Color {
    fn eq(&self, rhs: &Self) -> bool {
        ((self.r - rhs.r).abs() < EPSILON)
            && ((self.g - rhs.g).abs() < EPSILON)
            && ((self.b - rhs.b).abs() < EPSILON)
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
