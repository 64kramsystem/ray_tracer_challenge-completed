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
    #[default(Weak::<Mutex<Self>>::new())]
    pub parent: Weak<Mutex<dyn Shape>>,
    // This is tricky. Wrapping the vector with the mutex will cause contention, but wrapping the shape
    // will require all the Shape methods to be converted to functions taking Arc<Mutex<dyn shape>>;
    // this is possible, but ugly.
    //
    #[default(vec![])]
    pub children: Vec<Arc<Mutex<dyn Shape>>>,

    // Simple optimization.
    // It saves relatively little (10% on the astronaut1 test), however, it's very simple and fits nicely.
    //
    pub local_bounds: Bounds,
}

impl Group {
    pub fn add_child(group: &Arc<Mutex<Self>>, child: &Arc<Mutex<dyn Shape>>) {
        let mut group_mtx = group.lock().unwrap();
        group_mtx.children.push(Arc::clone(child));

        let mut child_mtx = child.lock().unwrap();

        let child_parent_ref = child_mtx.parent_mut();

        *child_parent_ref = Arc::downgrade(&(Arc::clone(group) as Arc<Mutex<dyn Shape>>));

        Bounds::update_from_bound(&mut group_mtx.local_bounds, &child_mtx.local_bounds());
    }
}

impl Shape for Group {
    fn id(&self) -> u32 {
        self.id
    }

    fn parent(&self) -> Option<Arc<Mutex<dyn Shape>>> {
        Weak::upgrade(&self.parent)
    }

    fn parent_mut(&mut self) -> &mut Weak<Mutex<dyn Shape>> {
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

    fn includes(&self, object: &Arc<Mutex<dyn Shape>>) -> bool {
        self.children
            .iter()
            .any(|child| child.lock().unwrap().includes(object))
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
        self_arc: &Arc<Mutex<dyn Shape>>,
        transformed_ray: &Ray,
    ) -> Vec<Intersection> {
        let local_bounds = self.local_bounds();

        let box_intersections =
            Cube::generalized_intersections(self_arc, &local_bounds, transformed_ray);

        if box_intersections.is_empty() {
            return vec![];
        }

        let mut intersections = self
            .children
            .iter()
            .flat_map(|child| child.lock().unwrap().intersections(child, transformed_ray))
            .collect::<Vec<_>>();

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        intersections
    }
}

impl BoundedShape for Group {
    fn local_bounds(&self) -> Bounds {
        self.local_bounds
    }
}
