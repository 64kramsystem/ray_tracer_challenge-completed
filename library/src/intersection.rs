use std::{cmp::Ordering, fmt::Debug};

use crate::Sphere;

// Setting NaN values for t is invalid; it will cause undefined behavior when sorting, likely panic.
//
#[derive(Debug)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a Sphere,
}

// This only informs the compiler that the type supports (full) equivalence.
//
impl<'a> Eq for Intersection<'a> {}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.t.is_nan() {
            panic!()
        } else {
            self.t == other.t
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
