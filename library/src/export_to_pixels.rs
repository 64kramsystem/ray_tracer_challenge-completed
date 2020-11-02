use crate::Color;

pub trait ExportToPixels {
    // Returns (vector of Color tuples, image width)
    //
    fn to_pixels(&self) -> (&Vec<Color>, u16);
}
