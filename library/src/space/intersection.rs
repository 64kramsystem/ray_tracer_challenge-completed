use std::{cmp::Ordering, fmt::Debug};

use crate::space::Shape;

// Setting NaN values for t is invalid; it will cause undefined behavior when sorting, likely panic.
//
#[derive(Debug)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
}

// This only informs the compiler that the type supports (full) equivalence.
//
impl<'a> Eq for Intersection<'a> {}

// Important: this implementation is intended for exact matches; in addition to `object.id`, it compares
// `t` exactly. The use case for it is to match a hit in a collection of intersections, from whom the
// given intersection was extracted.
//
impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.t.is_nan() {
            panic!()
        } else {
            self.t == other.t && self.object.id() == other.object.id()
        }
    }
}

impl<'a> Ord for Intersection<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let (lhs, rhs) = (self.t, other.t);

        if let Some(result) = lhs.partial_cmp(&rhs) {
            result
        } else {
            if lhs.is_nan() {
                panic!()
            } else {
                Ordering::Less
            }
        }
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
