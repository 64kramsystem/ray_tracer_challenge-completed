use std::{
    fmt,
    sync::{Arc, Mutex, Weak},
};

use super::{PointLight, Ray};
use crate::{
    math::{Matrix, Tuple},
    properties::{Color, Material},
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
        // In the book, this is local_intersect(), and returns also the shapes.
        //
        fn local_intersections(&self, transformed_ray: &Ray) -> Vec<f64>;
    }
}

pub trait Shape: private::ShapeLocal + fmt::Debug + Sync + Send {
    fn id(&self) -> u32;
    fn parent(&self) -> &Mutex<Weak<dyn Shape>>;
    fn children(&self) -> &Mutex<Vec<Arc<dyn Shape>>>;
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

    // Return value properties:
    //
    // - they're not guaranteed to be ordered;
    // - negative values are allowed.
    //
    // An possible optimization is to receive an ordered collection, and have the intersections added
    // to it; this avoids allocating an array for each shape.
    //
    fn intersections(&self, ray: &Ray) -> Vec<f64> {
        let transformed_ray = ray.inverse_transform(self.transform());
        self.local_intersections(&transformed_ray)
    }

    // Divergence from the book design. Having the lighting method here avoids going back and forth
    // between Shape and Material, and makes World#shade_hit cleaner.
    //
    fn lighting(
        &self,
        light: &PointLight,
        world_point: &Tuple,
        eyev: &Tuple,
        normalv: &Tuple,
        in_shadow: bool,
    ) -> Color {
        let object_point = self.transform().inverse() * world_point;

        self.material()
            .lighting(light, &object_point, world_point, eyev, normalv, in_shadow)
    }
}

impl PartialEq for dyn Shape + '_ {
    fn eq(&self, rhs: &Self) -> bool {
        self.id() == rhs.id()
    }
}
