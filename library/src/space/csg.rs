use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::{
    shape::{self, private::ShapeLocal},
    BoundedShape, Bounds, Intersection, Plane, Ray, Shape,
};
use crate::{math::Matrix, math::Tuple, properties::Material};

#[cfg(test)]
use std::any::Any;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Difference,
    Intersection,
    Union,
}

#[derive(Clone, Copy)]
pub enum ChildHit {
    Left,
    Right,
}

#[derive(Debug, SmartDefault)]
pub struct Csg {
    #[default(_code = "shape::new_shape_id()")]
    pub id: u32,
    #[default(Mutex::new(Weak::<Self>::new()))]
    pub parent: Mutex<Weak<dyn Shape>>,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,

    // The defaults below are phony, for default purposes.
    //
    #[default(Operation::Union)]
    pub operation: Operation,
    // For ease, we follow the Group#children pattern, but this prevents modifications to the children.
    // Structure: (left, right).
    #[default(Mutex::new((Arc::new(Plane::default()), Arc::new(Plane::default()))))]
    pub children: Mutex<(Arc<dyn Shape>, Arc<dyn Shape>)>,
}

impl Csg {
    // Sets the children's parent to the new Csg instance.
    // This can't be translated to a convenient constructor with defaults, because of the cross-references.
    // Besides being arguably more convenient, it follows the book's pattern. The tradeoff is `transform`
    // not having a default.
    //
    pub fn new(
        operation: Operation,
        left: Arc<dyn Shape>,
        right: Arc<dyn Shape>,
        transform: Matrix,
    ) -> Arc<Csg> {
        let csg = Arc::new(Csg {
            operation,
            children: Mutex::new((left, right)),
            transform,
            ..Csg::default()
        });

        // Must release the mutex (which borrows) in order to return the owned instance.
        {
            let (left, right) = &(*csg.children());

            let mut left_parent_ref = left.parent_mut();
            *left_parent_ref = Arc::downgrade(&(Arc::clone(&csg) as Arc<dyn Shape>));

            let mut right_parent_ref = right.parent_mut();
            *right_parent_ref = Arc::downgrade(&(Arc::clone(&csg) as Arc<dyn Shape>));
        }

        csg
    }

    // Convenience method.
    //
    pub fn children(&self) -> MutexGuard<(Arc<dyn Shape>, Arc<dyn Shape>)> {
        self.children.lock().unwrap()
    }

    pub(crate) fn intersection_allowed(
        &self,
        child_hit: ChildHit,
        inside_left: bool,
        inside_right: bool,
    ) -> bool {
        match self.operation {
            Operation::Difference => {
                return match child_hit {
                    ChildHit::Left => !inside_right,
                    ChildHit::Right => inside_left,
                }
            }
            Operation::Intersection => {
                return match child_hit {
                    ChildHit::Left => inside_right,
                    ChildHit::Right => inside_left,
                }
            }
            Operation::Union => {
                return match child_hit {
                    ChildHit::Left => !inside_right,
                    ChildHit::Right => !inside_left,
                }
            }
        }
    }

    pub(crate) fn filter_intersections(
        &self,
        intersections: Vec<Intersection>,
    ) -> Vec<Intersection> {
        // begin outside of both children
        //
        let mut inside_left = false;
        let mut inside_right = false;

        let mut result = Vec::with_capacity(intersections.len());

        let left_child = &self.children().0;

        for intersection in intersections {
            let child_hit = if left_child.includes(&intersection.object) {
                ChildHit::Left
            } else {
                ChildHit::Right
            };

            if self.intersection_allowed(child_hit, inside_left, inside_right) {
                result.push(intersection);
            }

            // depending on which object was hit, toggle either inside_left or inside_right;

            match child_hit {
                ChildHit::Left => inside_left = !inside_left,
                ChildHit::Right => inside_right = !inside_right,
            };
        }

        result
    }
}

impl Shape for Csg {
    fn id(&self) -> u32 {
        self.id
    }

    fn parent(&self) -> Option<Arc<dyn Shape>> {
        Weak::upgrade(&*self.parent.lock().unwrap())
    }

    fn parent_mut(&self) -> MutexGuard<Weak<dyn Shape>> {
        self.parent.lock().unwrap()
    }

    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Matrix {
        &mut self.transform
    }

    fn material(&self) -> &Material {
        panic!()
    }

    fn material_mut(&mut self) -> &mut Material {
        panic!()
    }

    fn includes(&self, object: &Arc<dyn Shape>) -> bool {
        let children = self.children();

        children.0.includes(object) || children.1.includes(object)
    }

    #[cfg(test)]
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ShapeLocal for Csg {
    fn local_normal(&self, _object_point: &Tuple, _intersection: &Intersection) -> Tuple {
        todo!()
    }

    fn local_intersections(self: Arc<Self>, transformed_ray: &Ray) -> Vec<Intersection> {
        // filter_intersections() locks the children, so we need to drop the mutex before then.
        //
        let mut all_intersections = {
            let (left_child, right_child) = &(*self.children());

            let mut left_intersections = Arc::clone(left_child).intersections(&transformed_ray);
            let right_intersections = Arc::clone(right_child).intersections(&transformed_ray);

            left_intersections.extend(right_intersections);

            left_intersections
        };

        all_intersections.sort();
        all_intersections.dedup();

        return self.filter_intersections(all_intersections);
    }
}

impl BoundedShape for Csg {
    fn local_bounds(&self) -> Bounds {
        todo!()
    }
}
