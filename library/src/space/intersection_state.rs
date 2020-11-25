use crate::{math::Tuple, space::Shape};

#[derive(Debug, PartialEq)]
pub struct IntersectionState<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
    pub point: Tuple,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub reflectv: Tuple,
    pub n1: f64,
    pub n2: f64,
    pub inside: bool,
}

impl<'a> IntersectionState<'a> {
    pub fn schlick(&self) -> f64 {
        let mut cos = self.eyev.dot_product(&self.normalv);

        if self.n1 > self.n2 {
            let n_ratio = self.n1 / self.n2;
            let sin2_t = n_ratio.powi(2) * (1.0 - cos.powi(2));

            if sin2_t > 1.0 {
                return 1.0;
            }

            cos = (1.0 - sin2_t).sqrt();
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}
