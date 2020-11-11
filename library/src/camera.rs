use crate::Matrix;

pub struct Camera {
    pub hsize: u16,
    pub vsize: u16,
    pub field_of_view: f64,
    pub transform: Matrix,
    pub pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: u16, vsize: u16, field_of_view: f64) -> Self {
        let view_units = (field_of_view / 2.0).tan() * 2.0;
        let min_dimension = hsize.max(vsize) as f64;
        let pixel_size = view_units / min_dimension;

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity(4),
            pixel_size,
        }
    }
}
