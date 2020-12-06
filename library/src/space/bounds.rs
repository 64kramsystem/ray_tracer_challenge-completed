use crate::math::Tuple;

#[derive(SmartDefault)]
pub struct Bounds {
    #[default(Tuple::point(f64::INFINITY, f64::INFINITY, f64::INFINITY))]
    pub min: Tuple,
    #[default(Tuple::point(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY))]
    pub max: Tuple,
}
