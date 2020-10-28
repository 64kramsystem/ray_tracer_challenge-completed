use std::ops::{Add, Mul, Sub};

use crate::has_float64_value::HasFloat64Value;

pub const EPSILON: f64 = 1e-6;

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
}

impl PartialEq for Color {
    fn eq(&self, rhs: &Self) -> bool {
        ((self.r - rhs.r).abs() < EPSILON)
            && ((self.g - rhs.g).abs() < EPSILON)
            && ((self.b - rhs.b).abs() < EPSILON)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

// The text doesn't specify the range and signedness of the scalar, so a conservative choice is made.
//
impl Mul<i32> for Color {
    type Output = Color;

    fn mul(self, rhs: i32) -> Self::Output {
        Color {
            r: self.r * rhs as f64,
            g: self.g * rhs as f64,
            b: self.b * rhs as f64,
        }
    }
}

// "Hadamard product".
//
impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
