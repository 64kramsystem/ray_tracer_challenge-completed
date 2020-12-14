use std::sync::{Arc, Weak};

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
    #[default(Weak::<Self>::new())]
    pub parent: Weak<dyn Shape>,

    // This is tricky. Wrapping the vector with the mutex will cause contention, but wrapping the shape
    // will require all the Shape methods to be converted to functions taking Arc<Mutex<dyn shape>>;
    // this is possible, but ugly.
    //
    #[default(vec![])]
    pub children: Vec<Arc<dyn Shape>>,
}

impl Group {
    // In the book, this is `add_child()`.
    //
    // In order to create a bidirectional tree, interior mutability is required. Since the intent is
    // _not_ to have mutexes, unsafe code is used.
    //
    // Although an Arc can be borrowed as mutable, it's still not possible, because:
    //
    // - if we get the group mutable reference before the children parent set cycle, the children will
    //   require an immutable borrow (in order to clone the Arc)
    // - if we get the mutable reference after the children cycle, there are now multiple Arc clones,
    //   so the mutable reference will fail
    //
    // Group mutability is required in order to add the children.
    //
    // There are no trivial/clean solutions to this problem; see https://users.rust-lang.org/t/is-it-possible-to-safely-build-a-read-only-thread-safe-bidirectional-tree/52759.
    //
    pub fn new(transform: Matrix, mut children: Vec<Arc<dyn Shape>>) -> Arc<Group> {
        let mut group = Arc::new(Group {
            transform,
            ..Group::default()
        });

        for child in children.iter_mut() {
            // Children needs to be unchecked as well, otherwise shapes can't be nested.
            //
            let child_parent_ref = unsafe { Arc::get_mut_unchecked(child) }.parent_mut();
            let parent_ref = Arc::clone(&group) as Arc<dyn Shape>;
            *child_parent_ref = Arc::downgrade(&parent_ref);
        }

        let group_mut = unsafe { Arc::get_mut_unchecked(&mut group) };
        group_mut.children = children;

        group
    }
}

impl Shape for Group {
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

    fn includes(&self, object: &Arc<dyn Shape>) -> bool {
        self.children.iter().any(|child| child.includes(object))
    }

    #[cfg(test)]
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ShapeLocal for Group {
    fn local_normal(&self, _point: &Tuple, _intersection: &Intersection) -> Tuple {
        panic!("local normal is not meaningful for Group")
    }

    // ray: In object space.
    //
    fn local_intersections(self: Arc<Self>, ray: &Ray) -> Vec<Intersection> {
        let local_bounds = self.local_bounds();

        let box_intersections = Cube::generalized_intersections(
            Arc::clone(&self) as Arc<dyn Shape>,
            &local_bounds,
            ray,
        );

        if box_intersections.is_empty() {
            return vec![];
        }

        let mut intersections = self
            .children
            .iter()
            .flat_map(|child| Arc::clone(child).intersections(ray))
            .collect::<Vec<_>>();

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        intersections
    }
}

impl BoundedShape for Group {
    fn local_bounds(&self) -> Bounds {
        Bounds::compute_for_children(&self.children)
    }
}
