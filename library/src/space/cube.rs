use super::{
    shape::{self, private::ShapeLocal},
    BoundedShape, Bounds, Intersection, Ray, Shape,
};
use crate::{math::Matrix, math::Tuple, properties::Material};

#[derive(Debug, ShapeAccessors, SmartDefault)]
pub struct Cube {
    #[default(_code = "shape::new_shape_id()")]
    pub id: u32,
    pub parent: Option<usize>,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,
    #[default(Material::default())]
    pub material: Material,
}

impl Cube {
    // Passing the object as parameter rather than modeling this as associated method, it allows to
    // use this logic on any Shape.
    //
    pub fn generalized_intersections<'a>(
        object: &'a dyn Shape,
        bounds: &Bounds,
        transformed_ray: &Ray,
    ) -> Vec<Intersection<'a>> {
        let (xtmin, xtmax) = Self::check_axis(
            transformed_ray.origin.x,
            transformed_ray.direction.x,
            bounds.min.x,
            bounds.max.x,
        );
        let (ytmin, ytmax) = Self::check_axis(
            transformed_ray.origin.y,
            transformed_ray.direction.y,
            bounds.min.y,
            bounds.max.y,
        );

        let mut tmin = xtmin.max(ytmin);
        let mut tmax = xtmax.min(ytmax);

        // Optimized version, as suggested in the practice section.
        //
        if tmin > tmax {
            return vec![];
        }

        let (ztmin, ztmax) = Self::check_axis(
            transformed_ray.origin.z,
            transformed_ray.direction.z,
            bounds.min.z,
            bounds.max.z,
        );

        tmin = tmin.max(ztmin);
        tmax = tmax.min(ztmax);

        if tmin > tmax {
            vec![]
        } else {
            vec![
                Intersection {
                    t: tmin,
                    uv: None,
                    object,
                },
                Intersection {
                    t: tmax,
                    uv: None,
                    object,
                },
            ]
        }
    }
}

impl ShapeLocal for Cube {
    // point: In object space.
    //
    fn local_normal(&self, point: Tuple, _intersection: &Intersection) -> Tuple {
        let x_abs = point.x.abs();
        let y_abs = point.y.abs();
        let z_abs = point.z.abs();

        // Algorithm with less comparisons. An extreme version, possibly without any measurable improvement,
        // is to duplicate the second if/else inside the branches of the first.

        let (max_dimension_abs, current_normal) = if x_abs > y_abs {
            (x_abs, (point.x, 0.0, 0.0))
        } else {
            (y_abs, (0.0, point.y, 0.0))
        };

        if max_dimension_abs > z_abs {
            Tuple::vector(current_normal.0, current_normal.1, current_normal.2)
        } else {
            Tuple::vector(0.0, 0.0, point.z)
        }

        // Original algorithm
        //
        // let max_dimension_abs = x_abs.max(y_abs).max(z_abs);

        // return if max_dimension_abs == x_abs {
        //     Tuple::vector(point.x, 0.0, 0.0)
        // } else if max_dimension_abs == y_abs {
        //     Tuple::vector(0.0, point.y, 0.0)
        // } else {
        //     Tuple::vector(0.0, 0.0, point.z)
        // };
    }

    // ray: In object space.
    //
    fn local_intersections(&self, ray: &Ray) -> Vec<Intersection> {
        let bounds = Bounds {
            min: Tuple::point(-1, -1, -1),
            max: Tuple::point(1, 1, 1),
        };

        Self::generalized_intersections(self, &bounds, ray)
    }
}

impl Cube {
    fn check_axis(origin: f64, direction: f64, minimum: f64, maximum: f64) -> (f64, f64) {
        let tmin_numerator = minimum - origin;
        let tmax_numerator = maximum - origin;

        let tmin = tmin_numerator / direction;
        let tmax = tmax_numerator / direction;

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }
}

impl BoundedShape for Cube {
    fn local_bounds(&self) -> Bounds {
        Bounds {
            min: Tuple::point(-1, -1, -1),
            max: Tuple::point(1, 1, 1),
        }
    }
}
