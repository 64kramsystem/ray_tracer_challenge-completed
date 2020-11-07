use library::{Color, Sdl2Interface, Tuple};

use crate::chapter01::{Environment, Projectile};

pub fn practice() {
    let start = Tuple::point(0, 1, 0);
    let velocity = Tuple::vector(1, 1.8, 0).normalize() * 11.25;
    let projectile = Projectile::with_values(start, velocity);

    let projectile_color = Color::new(1, 0, 0);
    let mut interface = Sdl2Interface::init("Chapter 02 exercise", 900, 550, (0, 0));

    let mut environment = Environment::new(projectile);

    while environment.projectile.position.y > 0.0 {
        // Watch out the y coordinate conversion!
        let x_position = environment.projectile.position.x as i16;
        let y_position = environment.projectile.position.y as i16;

        interface.write_pixel(x_position, y_position, projectile_color);

        println!("Position: x:{}, y:{}", x_position, y_position);

        environment.tick();
    }

    interface.update_canvas();

    interface.wait_keypress();
}
