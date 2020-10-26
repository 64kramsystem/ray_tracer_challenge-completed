use crate::color::Color;

// Interface for drawing to a canvas, and handling events; intentionally designed to be as simple a
// possible.
//
// Previously, read methods were applied, but testing proved to be a complete trainwreck (see history).
//
pub trait Interface {
    // Initializes the canvas, and maximizes the window.
    //
    fn init(window_title: &str) -> Self;

    // Does not update the canvas; must invoke update_canvas().
    //
    fn write_pixel(&mut self, x: u16, y: u16, color: Color);

    fn update_canvas(&mut self);

    // Wait for keypress; if the key sent if a quit combination (ie. Alt+F4), the program will exit.
    //
    fn wait_keypress(&mut self);
}
