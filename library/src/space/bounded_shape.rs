use super::Bounds;

// Quite ugly.
//
// We can't define multiple implementation blocks for the same trait, and since the Shape trait is implemented
// via macro, we can't just add this method on the shape implementors.
// We also can't create a proxy Shape#bound() method, with the function prototype on ShapeLocal, because
// Group needs its own implementation (and can't override).
//
pub trait BoundedShape {
    fn local_bounds(&self) -> Bounds;
}
