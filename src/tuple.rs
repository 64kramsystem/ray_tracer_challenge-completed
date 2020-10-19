pub const POINT_TYPE: f64 = 1.0;
pub const VECTOR_TYPE: f64 = 0.0;

pub const EPSILON: f64 = 1e-6;

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

impl PartialEq for Tuple {
    // Values are considered as equals if within EPSILON.
    //
    fn eq(&self, rhs: &Self) -> bool {
        ((self.0 - rhs.0).abs() < EPSILON)
            && ((self.1 - rhs.1).abs() < EPSILON)
            && ((self.2 - rhs.2).abs() < EPSILON)
            && ((self.3 - rhs.3).abs() < EPSILON)
    }
}
