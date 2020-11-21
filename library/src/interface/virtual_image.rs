use super::Image;
use crate::properties::Color;

// An image that has no phisical representation, usable for testing rendering.
//
pub struct VirtualImage {
    pub pixels_buffer: Vec<Color>,
    pub width: u16,
    pub height: u16,
}

impl VirtualImage {
    // The behavior of a the pixel outside the canvas is undefined.
    //
    fn pixel_buffer_index(&self, x: i16, y: i16) -> usize {
        y as usize * self.width as usize + x as usize
    }
}

impl Image for VirtualImage {
    fn new(width: u16, height: u16) -> Self {
        let pixels_buffer = vec![
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            };
            width as usize * height as usize
        ];

        Self {
            pixels_buffer,
            width,
            height,
        }
    }

    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn write_pixel(&mut self, x: i16, y: i16, color: Color) {
        let pixel_buffer_index = self.pixel_buffer_index(x, y);
        self.pixels_buffer[pixel_buffer_index] = color;
    }

    fn update(&mut self) {
        // nothing to do for this type
    }

    fn to_pixels(&self) -> Vec<&Color> {
        self.pixels_buffer.iter().collect::<Vec<_>>()
    }

    fn pixel_at(&self, x: i16, y: i16) -> Option<&Color> {
        let pixel_buffer_index = self.pixel_buffer_index(x, y);
        self.pixels_buffer.get(pixel_buffer_index)
    }
}
