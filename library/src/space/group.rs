use std::sync::{Arc, Mutex, Weak};

use super::{
    shape::{self, private::ShapeLocal},
    Ray, Shape,
};
use crate::{math::Matrix, math::Tuple, properties::Material};

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
    #[default(Mutex::new(vec![]))]
    pub children: Mutex<Vec<Arc<dyn Shape>>>,
}

impl Group {
    pub fn add_child(group: &Arc<dyn Shape>, child: &Arc<dyn Shape>) {
        group.children().lock().unwrap().push(Arc::clone(child));

        let mut child_parent_ref = child.parent().lock().unwrap();

        *child_parent_ref = Arc::downgrade(group);
    }
}

impl Shape for Group {
    fn id(&self) -> u32 {
        self.id
    }

    fn parent(&self) -> &Mutex<Weak<dyn Shape>> {
        &self.parent
    }

    fn children(&self) -> &Mutex<Vec<Arc<dyn Shape>>> {
        &self.children
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
}

impl ShapeLocal for Group {
    fn local_normal(&self, _object_point: &Tuple) -> Tuple {
        panic!("local normal is not meaningful for Group")
    }

    fn local_intersections(&self, transformed_ray: &Ray) -> Vec<f64> {
        // A more efficient implementation is to pass a BTreeSet, and have the children add elements.
        // As of mid chapter 14, `local_intersections()` doesn't return the objects; in case they will
        // need to be returned, then it will be cleaner to opt for BTreeSet, since Intersection implements
        // floats ordering.
        //
        let mut intersections = self
            .children()
            .lock()
            .unwrap()
            .iter()
            .flat_map(|child| child.intersections(transformed_ray))
            .collect::<Vec<_>>();

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        intersections
    }
}
