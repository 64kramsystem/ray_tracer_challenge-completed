use library::{Color, Sdl2Interface};

pub fn practice() {
    let (width, height) = (3, 3);

    let mut interface = Sdl2Interface::init("Yay!", width, height);

    for y in 0..height {
        for x in 0..width {
            let base_color = match x % 3 {
                0 => (0.25, 0.0, 0.0),
                1 => (0.0, 0.25, 0.0),
                2 => (0.0, 0.0, 0.25),
                _ => unreachable!(),
            };

            // Don't start with (0, 0, 0) - black, which is ugly.
            //
            let multiplied_color = Color {
                r: (base_color.0 * (y + 1) as f64) % 1.0, // won't be 1.0
                g: (base_color.1 * (y + 1) as f64) % 1.0, // won't be 1.0
                b: (base_color.2 * (y + 1) as f64) % 1.0, // won't be 1.0
            };

            interface.write_pixel(x as i16, y as i16, multiplied_color);
        }
    }

    interface.update_canvas();

    interface.wait_keypress();
}
