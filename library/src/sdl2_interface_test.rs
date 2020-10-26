use demonstrate::demonstrate;

use crate::color::Color;
use crate::interface::Interface;
use crate::sdl2_interface::*;

// Uh-oh 🤯
//
// It takes some time before SDL releases the resources after drop, so we need to either put waits,
// or do this.
// Additionally, the `let mut interface` can't be put in a `before` block, otherwise, even with waits,
// the issue happens.
//
use std::sync::Mutex;

unsafe impl Send for Sdl2Interface {}

lazy_static! {
    static ref INTERFACE: Mutex<Sdl2Interface> = Mutex::new(Sdl2Interface::init("test"));
}

demonstrate! {
    // It's not possible to reliably the SDL interface. Beside the issue mentioned above, the writes
    // are seemingly asynchronous, so that it's not guaranteed that even after an update is invoked,
    // reads will return the expected value without a wait.
    //
    describe "Sdl2Interface" {
        use super::*;

        before {
            #[allow(unused_mut)]
            let mut interface = INTERFACE.lock().unwrap();
        }

        it "should initialize with a black canvas" {
            let expected_color = Color { r: 0.0, g: 0.0, b: 0.0};

            let pixel_colors = interface.read_all_pixels();

            assert_eq!(pixel_colors.len(), CANVAS_WIDTH as usize * CANVAS_HEIGHT as usize);

            for pixel in pixel_colors {
                assert_eq!(pixel, expected_color);
            }
        }

        it "should write a pixel" {
            let expected_color = Color { r: 1.0, g: 1.0, b: 0.0};
            let (x, y) = (2, 1);

            interface.write_pixel(x, y, expected_color);
            interface.update_canvas();

            let pixel_color = interface.read_pixel(x, y);

            assert_eq!(pixel_color, expected_color);
        }
    }
}
