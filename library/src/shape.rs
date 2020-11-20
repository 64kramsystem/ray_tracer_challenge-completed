use std::{fmt, sync::Mutex};

use crate::{Material, Matrix, Tuple};

lazy_static::lazy_static! {
  static ref NEXT_ID: Mutex<u32> = Mutex::new(1);
}

pub(crate) fn new_shape_id() -> u32 {
    let mut next_id_mtx = NEXT_ID.lock().unwrap();

    let next_id = *next_id_mtx;
    *next_id_mtx += 1;

    next_id
}

pub(crate) mod private {
    use crate::Tuple;

    pub trait ShapeLocal {
        fn local_normal(&self, world_point: &Tuple) -> Tuple;
    }
}

pub trait Shape: private::ShapeLocal + fmt::Debug {
    fn id(&self) -> u32;
    fn transformation(&self) -> &Matrix;
    fn material(&self) -> &Material;

    fn normal(&self, world_point: &Tuple) -> Tuple {
        let object_point = if let Some(inverse) = self.transformation().inverse() {
            inverse * world_point
        } else {
            panic!()
        };

        let object_normal = self.local_normal(&object_point);

        let mut world_normal = if let Some(inverse) = self.transformation().inverse() {
            inverse.transpose() * &object_normal
        } else {
            panic!()
        };

        world_normal.w = 0.0;

        world_normal.normalize()
    }
}

impl PartialEq for dyn Shape + '_ {
    fn eq(&self, rhs: &Self) -> bool {
        self.id() == rhs.id()
    }
}
