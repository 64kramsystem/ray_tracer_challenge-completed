use crate::color::Color;

// Interface for drawing to a canvas, and handling events; intentionally designed to be as simple a
// possible.
//
// Mass-operations can be extremely slow if done in a simplistic way, for example, with the SDL2 library,
// it took 15" to read 1024x768 pixels (in debug mode).
// For this reason, specialized methods/workflows are required (update_canvas(), read_all_pixels()...)
//
pub trait Interface {
    // Initializes the canvas, and maximizes the window.
    //
    fn init(window_title: &str) -> Self;

    // Does not update the canvas; must invoke update_canvas().
    //
    fn write_pixel(&mut self, x: u16, y: u16, color: Color);

    fn update_canvas(&mut self);

    // To be used for testing.
    //
    // WATCH OUT! In some libraries, reading pixel by pixel can be *very* slow: with the SDL2 library,
    // it took 15" to read 1024x768 pixels (in debug mode) (!!!).
    //
    // For mass-reading pixels, use read_all_pixels().
    //
    fn read_pixel(&self, x: u16, y: u16) -> Color;

    fn read_all_pixels(&self) -> Vec<Color>;

    // Wait for keypress; if the key sent if a quit combination (ie. Alt+F4), the program will exit.
    //
    fn wait_keypress(&mut self);
}
