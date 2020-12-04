use std::{
    fmt,
    sync::{Arc, Mutex, MutexGuard, Weak},
};

use super::{BoundedShape, Bounds, PointLight, Ray};
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
        // In the book, this is local_normal_at().
        //
        fn local_normal(&self, world_point: &Tuple) -> Tuple;
        // In the book, this is local_intersect(), and returns also the shapes.
        //
        fn local_intersections(&self, transformed_ray: &Ray) -> Vec<f64>;
    }
}

pub trait Shape: private::ShapeLocal + BoundedShape + fmt::Debug + Sync + Send {
    fn id(&self) -> u32;
    fn parent(&self) -> Option<Arc<dyn Shape>>;
    fn parent_mut(&self) -> MutexGuard<Weak<dyn Shape>>;
    fn children(&self) -> MutexGuard<Vec<Arc<dyn Shape>>>;
    fn transform(&self) -> &Matrix;
    fn transform_mut(&mut self) -> &mut Matrix;
    fn material(&self) -> &Material;
    fn material_mut(&mut self) -> &mut Material;

    // In the book, this is normal_at().
    //
    fn normal(&self, world_point: &Tuple) -> Tuple {
        let local_point = self.world_to_object(world_point);
        let local_normal = self.local_normal(&local_point);
        self.normal_to_world(&local_normal)
    }

    fn world_to_object(&self, world_point: &Tuple) -> Tuple {
        let transform_inverse = self.transform().inverse();

        if let Some(parent) = self.parent() {
            transform_inverse * &parent.world_to_object(world_point)
        } else {
            transform_inverse * world_point
        }
    }

    fn normal_to_world(&self, object_normal: &Tuple) -> Tuple {
        let mut object_normal = self.transform().inverse().transpose() * object_normal;
        object_normal.w = 0.0;
        object_normal = object_normal.normalize();

        if let Some(parent) = self.parent() {
            parent.normal_to_world(&object_normal)
        } else {
            object_normal
        }
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

    // Local (object-level) bounds, with the shape transformation applied.
    //
    fn bounds(&self) -> Bounds {
        let local_bounds = self.local_bounds();

        // This ugly processing (disassemble/transform/reassemble) is required because after the transformation,
        // the bounding box can change completely.
        // Some negligible processing is spared by flattening the transformations (see comment on the
        // Group type).

        let local_corners = [
            Tuple::point(local_bounds.min.x, local_bounds.min.y, local_bounds.min.z),
            Tuple::point(local_bounds.min.x, local_bounds.min.y, local_bounds.max.z),
            Tuple::point(local_bounds.min.x, local_bounds.max.y, local_bounds.min.z),
            Tuple::point(local_bounds.min.x, local_bounds.max.y, local_bounds.max.z),
            Tuple::point(local_bounds.max.x, local_bounds.min.y, local_bounds.min.z),
            Tuple::point(local_bounds.max.x, local_bounds.min.y, local_bounds.max.z),
            Tuple::point(local_bounds.max.x, local_bounds.max.y, local_bounds.min.z),
            Tuple::point(local_bounds.max.x, local_bounds.max.y, local_bounds.max.z),
        ];

        let transform = self.transform();
        let mut bounds = Bounds::default();

        for corner in local_corners.iter() {
            let transformed_corner = transform * corner;

            if transformed_corner.x < bounds.min.x {
                bounds.min.x = transformed_corner.x
            } else if transformed_corner.x > bounds.max.x {
                bounds.max.x = transformed_corner.x
            }

            if transformed_corner.y < bounds.min.y {
                bounds.min.y = transformed_corner.y
            } else if transformed_corner.y > bounds.max.y {
                bounds.max.y = transformed_corner.y
            }

            if transformed_corner.z < bounds.min.z {
                bounds.min.z = transformed_corner.z
            } else if transformed_corner.z > bounds.max.z {
                bounds.max.z = transformed_corner.z
            }
        }

        bounds
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
        let object_point = self.world_to_object(&world_point);

        self.material()
            .lighting(light, &object_point, world_point, eyev, normalv, in_shadow)
    }
}

impl PartialEq for dyn Shape + '_ {
    fn eq(&self, rhs: &Self) -> bool {
        self.id() == rhs.id()
    }
}
