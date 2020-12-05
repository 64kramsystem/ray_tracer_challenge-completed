use std::{cmp::Ordering, fmt::Debug, sync::Arc};

use crate::space::Shape;

// Setting NaN values for t is invalid; it will cause undefined behavior/panic when sorting.
//
#[derive(Clone, Debug)]
pub struct Intersection {
    pub t: f64,
    pub object: Arc<dyn Shape>,
}

impl Eq for Intersection {}

// Important: this implementation is intended for exact matches; in addition to `object.id`, it compares
// `t` exactly. The use case for it is to match a hit in a collection of intersections, from whom the
// given intersection was extracted.
//
impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.object.id() == other.object.id()
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.partial_cmp(&other.t).unwrap()
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}
