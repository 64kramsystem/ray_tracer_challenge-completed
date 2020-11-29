use super::{
    shape::{self, private::ShapeLocal},
    Ray, Shape,
};
use crate::{math::Matrix, math::Tuple, properties::Material};

#[derive(Debug, ShapeAccessors, SmartDefault)]
pub struct Cube {
    #[default(_code = "shape::new_shape_id()")]
    pub id: u32,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,
    #[default(Material::default())]
    pub material: Material,
}

impl ShapeLocal for Cube {
    fn local_normal(&self, object_point: &Tuple) -> Tuple {
        let x_abs = object_point.x.abs();
        let y_abs = object_point.y.abs();
        let z_abs = object_point.z.abs();

        // Algorithm with less comparisons. An extreme version, possibly without any measurable improvement,
        // is to duplicate the second if/else inside the branches of the first.

        let (max_dimension_abs, current_normal) = if x_abs > y_abs {
            (x_abs, (object_point.x, 0.0, 0.0))
        } else {
            (y_abs, (0.0, object_point.y, 0.0))
        };

        return if max_dimension_abs > z_abs {
            Tuple::vector(current_normal.0, current_normal.1, current_normal.2)
        } else {
            Tuple::vector(0.0, 0.0, object_point.z)
        };

        // Original algorithm
        //
        // let max_dimension_abs = x_abs.max(y_abs).max(z_abs);

        // return if max_dimension_abs == x_abs {
        //     Tuple::vector(object_point.x, 0.0, 0.0)
        // } else if max_dimension_abs == y_abs {
        //     Tuple::vector(0.0, object_point.y, 0.0)
        // } else {
        //     Tuple::vector(0.0, 0.0, object_point.z)
        // };
    }

    fn local_intersections(&self, transformed_ray: &Ray) -> Option<(f64, f64)> {
        let (xtmin, xtmax) =
            Self::check_axis(transformed_ray.origin.x, transformed_ray.direction.x);
        let (ytmin, ytmax) =
            Self::check_axis(transformed_ray.origin.y, transformed_ray.direction.y);

        let mut tmin = xtmin.max(ytmin);
        let mut tmax = xtmax.min(ytmax);

        // Optimized version, as suggested in the practice section.
        //
        if tmin > tmax {
            return None;
        }

        let (ztmin, ztmax) =
            Self::check_axis(transformed_ray.origin.z, transformed_ray.direction.z);

        tmin = tmin.max(ztmin);
        tmax = tmax.min(ztmax);

        if tmin > tmax {
            None
        } else {
            Some((tmin, tmax))
        }
    }
}

impl Cube {
    fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let tmin = tmin_numerator / direction;
        let tmax = tmax_numerator / direction;

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }
}
