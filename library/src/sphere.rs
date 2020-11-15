use std::sync::Mutex;

use crate::{has_float64_value::HasFloat64Value, Axis, Material, Matrix, Tuple};

lazy_static::lazy_static! {
  static ref NEXT_ID: Mutex<u32> = Mutex::new(1);
}

#[derive(Clone, Debug, PartialEq, SmartDefault)]
pub struct Sphere {
    #[default(_code = "Self::new_id()")]
    pub id: u32,
    #[default(Matrix::identity(4))]
    pub transformation: Matrix,
    #[default(Material::default())]
    pub material: Material,
}

impl Sphere {
    fn new_id() -> u32 {
        let mut next_id_mtx = NEXT_ID.lock().unwrap();

        let next_id = *next_id_mtx;
        *next_id_mtx += 1;

        next_id
    }

    pub fn scale<T: HasFloat64Value>(self, x: T, y: T, z: T) -> Self {
        self.transform(&Matrix::scaling(x, y, z))
    }

    pub fn equiscale<T: HasFloat64Value + Copy>(self, s: T) -> Self {
        self.transform(&Matrix::scaling(s, s, s))
    }

    pub fn translate<T: HasFloat64Value>(self, x: T, y: T, z: T) -> Self {
        self.transform(&Matrix::translation(x, y, z))
    }

    pub fn rotate(self, axis: Axis, r: f64) -> Self {
        self.transform(&Matrix::rotation(axis, r))
    }

    // Returns a new Sphere with same id, with new transformation = (transformation * self.transformation).
    //
    pub fn transform(mut self, transformation: &Matrix) -> Self {
        let new_transformation = transformation * &self.transformation;
        self.transformation = new_transformation;
        self
    }

    // Not clear if this is actually useful (it's used once in world normal()).
    //
    // pub fn normal<T: HasFloat64Value>(point_x: T, point_y: T, point_z: T) -> Tuple {
    //     Tuple::point(point_x, point_y, point_z) - Tuple::point(0, 0, 0)
    // }

    pub fn normal(&self, world_point: &Tuple) -> Tuple {
        let object_point = if let Some(inverse) = self.transformation.inverse() {
            inverse * world_point
        } else {
            panic!()
        };

        let object_normal = object_point - &Tuple::point(0, 0, 0);

        let mut world_normal = if let Some(inverse) = self.transformation.inverse() {
            inverse.transpose() * &object_normal
        } else {
            panic!()
        };

        world_normal.w = 0.0;

        world_normal.normalize()
    }
}
