use crate::properties::Color;

pub trait Image {
    fn new(width: u16, height: u16) -> Self;

    // Convenience method for creating an image from the pixels provided.
    //
    // The image is updated before return.
    //
    fn from_pixels<T: Image>(pixels: Vec<Vec<Color>>, hsize: u16, vsize: u16) -> T {
        let mut image = T::new(hsize, vsize);

        for (y, row) in pixels.iter().enumerate() {
            for (x, pixel_color) in row.iter().enumerate() {
                image.write_pixel(x as i16, y as i16, *pixel_color);
            }
        }

        image.update();

        image
    }

    // Logical dimensions.
    //
    fn width(&self) -> u16;
    fn height(&self) -> u16;

    // Writes a pixel at (x, y), where (0, 0) is the bottom left of the canvas.
    // The origin is implementation-dependent, as in some cases it doesn't matter.
    // Behavior for coordinates outside the canvas is implementation-dependent.
    //
    fn write_pixel(&mut self, x: i16, y: i16, color: Color);

    // Some implementors may require this after writing the pixels.
    //
    fn update(&mut self);

    // Return a flat vector of pixels, starting at (0, 0).
    //
    // This should not return the internal representation, which can be different (e.g. due to y inversion).
    //
    fn to_pixels(&self) -> Vec<&Color>;

    // If the coordinates are over the borders, None is returned.
    //
    fn pixel_at(&self, x: i16, y: i16) -> Option<&Color>;
}
