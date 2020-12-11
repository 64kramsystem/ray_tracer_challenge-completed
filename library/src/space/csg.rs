#[cfg(test)]
use std::any::Any;

use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::{
    shape::{self, private::ShapeLocal},
    BoundedShape, Bounds, Intersection, Plane, Ray, Shape,
};
use crate::{math::Matrix, math::Tuple, properties::Material};

#[derive(Debug, PartialEq)]
pub enum Operation {
    Union,
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
    // Can't define a new() function, without returning Arc<Self>, since the children's parent needs
    // to be so. It could be ok to return Arc, but first the whole architecture needs to settle.
    //
    // In the book, this is `csg()`.
    //
    pub fn set_children(self: &Arc<Self>, left: Arc<dyn Shape>, right: Arc<dyn Shape>) {
        *self.children.lock().unwrap() = (Arc::clone(&left), Arc::clone(&right));

        let mut left_parent_ref = left.parent_mut();
        *left_parent_ref = Arc::downgrade(&(Arc::clone(self) as Arc<dyn Shape>));

        let mut right_parent_ref = right.parent_mut();
        *right_parent_ref = Arc::downgrade(&(Arc::clone(self) as Arc<dyn Shape>));
    }

    // Convenience method.
    //
    pub fn children(&self) -> MutexGuard<(Arc<dyn Shape>, Arc<dyn Shape>)> {
        self.children.lock().unwrap()
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

    #[cfg(test)]
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ShapeLocal for Csg {
    fn local_normal(&self, _object_point: &Tuple, _intersection: &Intersection) -> Tuple {
        todo!()
    }

    fn local_intersections(self: Arc<Self>, _transformed_ray: &Ray) -> Vec<Intersection> {
        todo!()
    }
}

impl BoundedShape for Csg {
    fn local_bounds(&self) -> Bounds {
        todo!()
    }
}
