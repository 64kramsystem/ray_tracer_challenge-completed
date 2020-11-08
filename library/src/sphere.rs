use std::sync::Mutex;

use crate::{has_float64_value::HasFloat64Value, Axis, Matrix};

lazy_static::lazy_static! {
  static ref NEXT_ID: Mutex<u32> = Mutex::new(1);
}

#[derive(Clone)]
pub struct Sphere {
    pub id: u32,
    // Defaults to an identity matrix of order 4.
    pub transformation: Matrix,
}

impl Sphere {
    pub fn new() -> Self {
        let mut next_id_mtx = NEXT_ID.lock().unwrap();

        let next_id = *next_id_mtx;
        *next_id_mtx += 1;

        let transformation = Matrix::identity(4);

        Self {
            id: next_id,
            transformation,
        }
    }

    pub fn scale<T: HasFloat64Value>(self, x: T, y: T, z: T) -> Self {
        self.transform(Matrix::scaling(x, y, z))
    }

    pub fn equiscale<T: HasFloat64Value + Copy>(self, s: T) -> Self {
        self.transform(Matrix::scaling(s, s, s))
    }

    pub fn translate<T: HasFloat64Value>(self, x: T, y: T, z: T) -> Self {
        self.transform(Matrix::translation(x, y, z))
    }

    pub fn rotate(self, axis: Axis, r: f64) -> Self {
        self.transform(Matrix::rotation(axis, r))
    }

    // Returns a new Sphere with same id, with new transformation = (transformation * self.transformation).
    //
    pub fn transform(mut self, transformation: Matrix) -> Self {
        let new_transformation = transformation * self.transformation;
        self.transformation = new_transformation;
        self
    }
}
