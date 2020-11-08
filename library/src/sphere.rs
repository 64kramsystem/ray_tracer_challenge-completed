use std::sync::Mutex;

use crate::Matrix;

lazy_static::lazy_static! {
  static ref NEXT_ID: Mutex<u32> = Mutex::new(1);
}

pub struct Sphere {
    pub id: u32,
    // Defaults to an identity matrix of order 4.
    pub transformation: Matrix,
}

impl Sphere {
    pub fn new(transformation: Option<Matrix>) -> Self {
        let mut next_id_mtx = NEXT_ID.lock().unwrap();

        let next_id = *next_id_mtx;
        *next_id_mtx += 1;

        let transformation = if let Some(transformation) = transformation {
            transformation
        } else {
            Matrix::identity(4)
        };

        Self {
            id: next_id,
            transformation,
        }
    }
}
