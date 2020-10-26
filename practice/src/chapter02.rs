use library::{Color, Interface, Sdl2Interface};

pub fn practice() {
    let mut interface = Sdl2Interface::init("Yay!");
    let x_base = 0;

    interface.write_pixel(
        x_base + 0,
        0,
        Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        },
    );

    interface.write_pixel(
        x_base + 1,
        1,
        Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        },
    );

    interface.write_pixel(
        x_base + 2,
        2,
        Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        },
    );

    interface.write_pixel(
        x_base + 3,
        3,
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        },
    );

    interface.update_canvas();

    interface.wait_keypress();
}
