use std::sync::{Arc, Weak};

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
    #[default(Weak::<Self>::new())]
    pub parent: Weak<dyn Shape>,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,

    // The defaults below are phony, for default purposes.
    //
    #[default(Operation::Union)]
    pub operation: Operation,
    // For ease, we follow the Group#children pattern, but this prevents modifications to the children.
    // Structure: (left, right).
    #[default((Arc::new(Plane::default()), Arc::new(Plane::default())))]
    pub children: (Arc<dyn Shape>, Arc<dyn Shape>),
}

impl Csg {
    // Sets the children's parent to the new Csg instance.
    //
    pub fn new(
        operation: Operation,
        mut left: Arc<dyn Shape>,
        mut right: Arc<dyn Shape>,
        transform: Matrix,
    ) -> Arc<Csg> {
        let mut csg = Arc::new(Csg {
            operation,
            transform,
            ..Csg::default()
        });

        // Children needs to be unchecked as well, otherwise shapes can't be nested.

        let left_mut = unsafe { Arc::get_mut_unchecked(&mut left) };
        let left_parent_ref = left_mut.parent_mut();
        *left_parent_ref = Arc::downgrade(&(Arc::clone(&csg) as Arc<dyn Shape>));

        let right_mut = unsafe { Arc::get_mut_unchecked(&mut right) };
        let right_parent_ref = right_mut.parent_mut();
        *right_parent_ref = Arc::downgrade(&(Arc::clone(&csg) as Arc<dyn Shape>));

        let csg_mut = unsafe { Arc::get_mut_unchecked(&mut csg) };

        csg_mut.children = (left, right);

        csg
    }

    pub(crate) fn intersection_allowed(
        &self,
        child_hit: ChildHit,
        inside_left: bool,
        inside_right: bool,
    ) -> bool {
        match self.operation {
            Operation::Difference => match child_hit {
                ChildHit::Left => !inside_right,
                ChildHit::Right => inside_left,
            },
            Operation::Intersection => match child_hit {
                ChildHit::Left => inside_right,
                ChildHit::Right => inside_left,
            },
            Operation::Union => match child_hit {
                ChildHit::Left => !inside_right,
                ChildHit::Right => !inside_left,
            },
        }
    }

    pub(crate) fn filter_intersections<'a>(
        &self,
        intersections: Vec<Intersection<'a>>,
    ) -> Vec<Intersection<'a>> {
        // begin outside of both children
        //
        let mut inside_left = false;
        let mut inside_right = false;

        let mut result = Vec::with_capacity(intersections.len());

        let left_child = &self.children.0;

        for intersection in intersections {
            let child_hit = if left_child.includes(intersection.object) {
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
        Weak::upgrade(&self.parent)
    }

    fn parent_mut(&mut self) -> &mut Weak<dyn Shape> {
        &mut self.parent
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

    fn includes(&self, object: &dyn Shape) -> bool {
        self.children.0.includes(object) || self.children.1.includes(object)
    }

    #[cfg(test)]
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ShapeLocal for Csg {
    fn local_normal(&self, _point: Tuple, _intersection: &Intersection) -> Tuple {
        panic!("local normal is not meaningful for Group")
    }

    // ray: In object space.
    //
    fn local_intersections<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>> {
        // filter_intersections() locks the children, so we need to drop the mutex before then.
        //
        let mut all_intersections = {
            let (left_child, right_child) = &self.children;

            let mut left_intersections = left_child.intersections(ray);
            let right_intersections = right_child.intersections(ray);

            left_intersections.extend(right_intersections);

            left_intersections
        };

        all_intersections.sort();
        all_intersections.dedup();

        self.filter_intersections(all_intersections)
    }
}

impl BoundedShape for Csg {
    fn local_bounds(&self) -> Bounds {
        let children = vec![Arc::clone(&self.children.0), Arc::clone(&self.children.1)];
        Bounds::compute_for_children(&children)
    }
}
