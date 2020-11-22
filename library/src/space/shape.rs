use std::{fmt, sync::Mutex};

use super::Ray;
use crate::{
    math::{Matrix, Tuple},
    properties::Material,
};

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
    use super::Ray;
    use crate::math::Tuple;

    pub trait ShapeLocal {
        fn local_normal(&self, world_point: &Tuple) -> Tuple;
        fn local_intersections(&self, transformed_ray: &Ray) -> Option<(f64, f64)>;
    }
}

pub trait Shape: private::ShapeLocal + fmt::Debug + Sync {
    fn id(&self) -> u32;
    fn transform(&self) -> &Matrix;
    fn transform_mut(&mut self) -> &mut Matrix;
    fn material(&self) -> &Material;
    fn material_mut(&mut self) -> &mut Material;

    fn normal(&self, world_point: &Tuple) -> Tuple {
        let object_point = self.transform().inverse() * world_point;

        let object_normal = self.local_normal(&object_point);

        let mut world_normal = self.transform().inverse().transpose() * &object_normal;

        world_normal.w = 0.0;

        world_normal.normalize()
    }

    // Intersections are returned in order.
    //
    fn intersections(&self, ray: &Ray) -> Option<(f64, f64)> {
        let transformed_ray = ray.inverse_transform(self.transform());
        self.local_intersections(&transformed_ray)
    }
}

impl PartialEq for dyn Shape + '_ {
    fn eq(&self, rhs: &Self) -> bool {
        self.id() == rhs.id()
    }
}
