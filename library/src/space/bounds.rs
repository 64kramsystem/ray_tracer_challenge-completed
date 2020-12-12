use crate::math::Tuple;

#[derive(Copy, Clone, Debug, SmartDefault)]
pub struct Bounds {
    #[default(Tuple::point(f64::INFINITY, f64::INFINITY, f64::INFINITY))]
    pub min: Tuple,
    #[default(Tuple::point(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY))]
    pub max: Tuple,
}

impl Bounds {
    // Updates the reference.
    //
    // Simple logic, but convenient to have it ready.
    //
    pub fn update_from_bound(reference: &mut Bounds, other: &Bounds) {
        reference.min.x = reference.min.x.min(other.min.x);
        reference.min.y = reference.min.y.min(other.min.y);
        reference.min.z = reference.min.z.min(other.min.z);

        reference.max.x = reference.max.x.max(other.max.x);
        reference.max.y = reference.max.y.max(other.max.y);
        reference.max.z = reference.max.z.max(other.max.z);
    }

    // Same as above.
    //
    // It's a PITA not to have overloading, and it's not worth to use the trait workaround.
    //
    pub fn update_from_tuple(reference: &mut Bounds, other: &Tuple) {
        // Interestingly, using seemingly optimized logic like the following, is slower:
        //
        // if other.x < reference.min.x {
        //     reference.min.x = other.x
        // } else if other.x > reference.max.x {
        //     reference.max.x = other.x
        // }

        reference.min.x = reference.min.x.min(other.x);
        reference.min.y = reference.min.y.min(other.y);
        reference.min.z = reference.min.z.min(other.z);

        reference.max.x = reference.max.x.max(other.x);
        reference.max.y = reference.max.y.max(other.y);
        reference.max.z = reference.max.z.max(other.z);
    }
}
