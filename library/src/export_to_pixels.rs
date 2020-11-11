use crate::Color;

pub trait ExportToPixels {
    // Returns (vector of Color tuples, image width)
    //
    fn to_pixels(&self) -> (&Vec<Color>, u16);

    // If the coordinates are over the borders, None is returned.
    //
    fn pixel_at(&self, x: i16, y: i16) -> Option<Color>;
}
