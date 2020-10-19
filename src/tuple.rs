pub const POINT_TYPE: f64 = 1.0;
pub const VECTOR_TYPE: f64 = 0.0;

// At this stage, is not clear is direct operation will be carried also on the type field. If so, using
// an enum may make things more complicated, so conservatively, a float is used.
//
#[derive(Debug)]
pub struct Tuple(pub f64, pub f64, pub f64, pub f64);

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple(x, y, z, POINT_TYPE)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple(x, y, z, VECTOR_TYPE)
    }
}
