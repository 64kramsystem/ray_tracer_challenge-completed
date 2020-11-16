use std::f64::consts::PI;

use library::{Axis, Color, Image, Sdl2Interface, Tuple};

pub fn practice() {
    let mut interface = Sdl2Interface::init("Chapter 02 exercise", 1024, 600, (512, 300));
    let point_color = Color::new(1, 0, 0);

    let mut point = Tuple::point(0, 0, 0);

    point = point.translate(0, 150, 0);

    for _ in 0..12 {
        println!("x: {:>4.0}, y: {:>4.0}", point.x, point.y);
        interface.write_pixel(point.x as i16, point.y as i16, point_color);

        // WATCH OUT! With the conventional 2D representation of an x/y diagram, z points towards the
        // horizon. This means that the positive rotation, which is clockwise around the z axis, is
        // counterclockwise for the viewer.
        // Drawing the points ultimately ends with the same picture, however, for the sake of precision,
        // the z rotation is negated, which makes it clockwise for the viewer's perspective.
        //
        point = point.rotate(Axis::Z, -2.0 * PI / 12.0);
    }

    interface.update();
    interface.wait_keypress();
}
