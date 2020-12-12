use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::{
    shape::{self, private::ShapeLocal},
    BoundedShape, Bounds, Cube, Intersection, Ray, Shape,
};
use crate::{math::Matrix, math::Tuple, properties::Material};

#[cfg(test)]
use std::any::Any;

// For nested groups, an optimization is to reduce the transformations by pushing them to the children,
// although it requires some modification. Since in this project there is a maximum of one level, the
// optimization doesn't apply.
//
// Creating a struct with a single Mutex doesn't simplify things, since parent and children are not
// accessed together (at least, currently, directly and in the same context).
//
#[derive(Debug, SmartDefault)]
pub struct Group {
    #[default(_code = "shape::new_shape_id()")]
    pub id: u32,
    #[default(Matrix::identity(4))]
    pub transform: Matrix,
    #[default(Mutex::new(Weak::<Self>::new()))]
    pub parent: Mutex<Weak<dyn Shape>>,
    // This is tricky. Wrapping the vector with the mutex will cause contention, but wrapping the shape
    // will require all the Shape methods to be converted to functions taking Arc<Mutex<dyn shape>>;
    // this is possible, but ugly.
    //
    #[default(Mutex::new(vec![]))]
    pub children: Mutex<Vec<Arc<dyn Shape>>>,

    // Simple optimization. Needs to be behind a Mutex since it's used in an Arc<Self> context (damn);
    // otherwise, it should be manipulated from outside.
    // It saves relatively little (10% on the astronaut1 test), however, it's very simple and fits nicely.
    //
    pub local_bounds: Mutex<Bounds>,
}

impl Group {
    pub fn add_child(self: &Arc<Self>, child: &Arc<dyn Shape>) {
        self.children().push(Arc::clone(child));

        let mut child_parent_ref = child.parent_mut();

        *child_parent_ref = Arc::downgrade(&(Arc::clone(self) as Arc<dyn Shape>));

        let mut local_bounds_mtx = self.local_bounds.lock().unwrap();

        Bounds::update_from_bound(&mut *local_bounds_mtx, &child.local_bounds());
    }

    // Convenience method.
    //
    pub fn children(&self) -> MutexGuard<Vec<Arc<dyn Shape>>> {
        self.children.lock().unwrap()
    }
}

impl Shape for Group {
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
        self.children().iter().any(|child| child.includes(object))
    }

    #[cfg(test)]
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ShapeLocal for Group {
    fn local_normal(&self, _object_point: &Tuple, _intersection: &Intersection) -> Tuple {
        panic!("local normal is not meaningful for Group")
    }

    fn local_intersections(
        &self,
        self_arc: &Arc<dyn Shape>,
        transformed_ray: &Ray,
    ) -> Vec<Intersection> {
        let local_bounds = self.local_bounds();

        let box_intersections =
            Cube::generalized_intersections(self_arc, &local_bounds, transformed_ray);

        if box_intersections.is_empty() {
            return vec![];
        }

        let mut intersections = self
            .children()
            .iter()
            .flat_map(|child| child.intersections(child, transformed_ray))
            .collect::<Vec<_>>();

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        intersections
    }
}

impl BoundedShape for Group {
    fn local_bounds(&self) -> Bounds {
        *self.local_bounds.lock().unwrap()
    }
}
