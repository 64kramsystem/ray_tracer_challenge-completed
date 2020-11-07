use std::{
    mem::MaybeUninit,
    ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub},
};

use crate::{has_float64_value::HasFloat64Value, Matrix};

use crate::EPSILON;

pub const POINT_TYPE: f64 = 1.0;
pub const VECTOR_TYPE: f64 = 0.0;

// At this stage, is not clear is direct operation will be carried also on the type field. If so, using
// an enum may make things more complicated, so conservatively, a float is used.
//
// This struct could be divided in Vector and Point. This has the advantage of enforcing type safety,
// e.g., disallowing adding a Point to a Point, at the cost of either duplication of the methods, or
// of creating a trait exposing x/y/z/w, which would make attribute access uneven in cases where the
// trait is referenced. At this stage, it's better to wait to see how the vector/point logic mix, although
// if there isn't much mixing, splitting could be advantageous.
//
#[derive(Clone, Copy, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    // Too many lulz will kill me.
    //
    pub fn uninitialized() -> Self {
        unsafe {
            Self {
                x: MaybeUninit::uninit().assume_init(),
                y: MaybeUninit::uninit().assume_init(),
                z: MaybeUninit::uninit().assume_init(),
                w: MaybeUninit::uninit().assume_init(),
            }
        }
    }

    pub fn new<T: HasFloat64Value>(x: T, y: T, z: T, w: T) -> Self {
        Self {
            x: x.as_f64(),
            y: y.as_f64(),
            z: z.as_f64(),
            w: w.as_f64(),
        }
    }

    pub fn point<T: HasFloat64Value, U: HasFloat64Value, V: HasFloat64Value>(
        x: T,
        y: U,
        z: V,
    ) -> Self {
        Self {
            x: x.as_f64(),
            y: y.as_f64(),
            z: z.as_f64(),
            w: POINT_TYPE,
        }
    }

    pub fn vector<T: HasFloat64Value, U: HasFloat64Value, V: HasFloat64Value>(
        x: T,
        y: U,
        z: V,
    ) -> Self {
        Self {
            x: x.as_f64(),
            y: y.as_f64(),
            z: z.as_f64(),
            w: VECTOR_TYPE,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();

        Self::vector(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }

    pub fn dot_product(&self, rhs: Tuple) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn cross_product(&self, rhs: Tuple) -> Tuple {
        Tuple::vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn translate<T: HasFloat64Value>(self, x: T, y: T, z: T) -> Tuple {
        Matrix::translation(x, y, z) * self
    }

    pub fn scale<T: HasFloat64Value>(self, x: T, y: T, z: T) -> Tuple {
        Matrix::scaling(x, y, z) * self
    }
}

// Index[Mut] implementations are for the lulz, although they're actually convenient for matrix operations.
//
impl Index<usize> for Tuple {
    type Output = f64;

    fn index(&self, y: usize) -> &Self::Output {
        match y {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index too high!"),
        }
    }
}

impl IndexMut<usize> for Tuple {
    fn index_mut(&mut self, y: usize) -> &mut f64 {
        match y {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index too high!"),
        }
    }
}

impl PartialEq for Tuple {
    // Values are considered as equals if within EPSILON.
    //
    fn eq(&self, rhs: &Self) -> bool {
        ((self.x - rhs.x).abs() < EPSILON)
            && ((self.y - rhs.y).abs() < EPSILON)
            && ((self.z - rhs.z).abs() < EPSILON)
            && ((self.w - rhs.w).abs() < EPSILON)
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}
