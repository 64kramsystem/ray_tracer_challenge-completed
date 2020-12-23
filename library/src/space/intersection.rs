use std::{cmp::Ordering, fmt::Debug};

use crate::space::Shape;

// Setting NaN values for `t` is invalid; it will cause undefined behavior/panic when sorting.
// `uv` is used only by triangles.
// The `object` default is not meaningful, but it's required in order to allow type defaulting.
//
// In the book, `intersection()` and `intersection_with_uv` are instantiations of this type.
//
#[derive(Clone, Debug)]
pub struct Intersection<'a> {
    pub t: f64,
    pub uv: Option<(f64, f64)>,
    pub object: &'a dyn Shape,
}

impl<'a> Eq for Intersection<'a> {}

// Important: this implementation is intended for exact matches; in addition to `object.id`, it compares
// `t` exactly. The use case for it is to match a hit in a collection of intersections, from whom the
// given intersection was extracted.
//
impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.object.id() == other.object.id()
    }
}

impl<'a> Ord for Intersection<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.partial_cmp(&other.t).unwrap()
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}
