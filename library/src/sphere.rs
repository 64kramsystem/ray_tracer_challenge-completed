use crate::{
    has_float64_value::HasFloat64Value,
    shape::{self, private::ShapeLocal},
    Axis, Material, Matrix, Shape, Tuple,
};

#[derive(Clone, Debug, SmartDefault)]
pub struct Sphere {
    #[default(_code = "shape::new_shape_id()")]
    pub id: u32,
    #[default(Matrix::identity(4))]
    pub transformation: Matrix,
    #[default(Material::default())]
    pub material: Material,
}

impl Sphere {
    pub fn scale<T: HasFloat64Value>(self, x: T, y: T, z: T) -> Self {
        self.transform(&Matrix::scaling(x, y, z))
    }

    pub fn equiscale<T: HasFloat64Value + Copy>(self, s: T) -> Self {
        self.transform(&Matrix::scaling(s, s, s))
    }

    pub fn translate<T: HasFloat64Value>(self, x: T, y: T, z: T) -> Self {
        self.transform(&Matrix::translation(x, y, z))
    }

    pub fn rotate(self, axis: Axis, r: f64) -> Self {
        self.transform(&Matrix::rotation(axis, r))
    }

    // Returns a new Sphere with same id, with new transformation = (transformation * self.transformation).
    //
    pub fn transform(mut self, transformation: &Matrix) -> Self {
        let new_transformation = transformation * &self.transformation;
        self.transformation = new_transformation;
        self
    }
}

impl ShapeLocal for Sphere {
    fn local_normal(&self, object_point: &Tuple) -> Tuple {
        object_point - &Tuple::point(0, 0, 0)
    }
}

impl Shape for Sphere {
    fn id(&self) -> u32 {
        self.id
    }

    fn transformation(&self) -> &Matrix {
        &self.transformation
    }

    fn material(&self) -> &Material {
        &self.material
    }
}
