use std::{f64::consts::PI, sync::Mutex};

use library::{Axis, Color, Matrix, PointLight, Ray, Sdl2Interface, Sphere, Tuple};
use rayon::prelude::*;

pub fn practice() {
    // The wall matches the display; for simplicity, the "wall" concept represents both.
    //
    const WALL_SIZE: u16 = 100;
    let eye_z = -50.0;
    let wall_z = 50.0;
    let light_position = Tuple::point(-20, 30, -50);

    let (center_x, center_y) = ((WALL_SIZE / 2) as i16, (WALL_SIZE / 2) as i16);

    let mut interface = Sdl2Interface::init(
        "Chapter 05 exercise",
        WALL_SIZE,
        WALL_SIZE,
        (center_x, center_y),
    );

    let mut sphere = Sphere::new();
    sphere.material.color = Color::new(1, 0.2, 1);
    sphere.transformation = Matrix::translation(10, 0, 0)
        * &Matrix::rotation(Axis::Z, -PI / 4.0)
        * &Matrix::scaling(6.25, 12.5, 12.5);

    let light = PointLight::new(light_position, Color::new(1, 1, 1));

    let eye_position = Tuple::point(0, 0, eye_z);

    let mut pixels_buffer = vec![[Color::new(0, 0, 0); WALL_SIZE as usize]; WALL_SIZE as usize];
    let pixels_buffer_mtx = Mutex::new(&mut pixels_buffer);

    // buffer_y/x are just for convenience.
    //
    (-center_y..center_y)
        .into_par_iter()
        .enumerate()
        .for_each(|(buffer_y, interface_y)| {
            println!("Computing y: {}", interface_y);

            let mut row_buffer = [Color::new(0, 0, 0); WALL_SIZE as usize];

            for (buffer_x, interface_x) in (-center_x..center_x).enumerate() {
                let eye_ray_direction =
                    Tuple::vector(interface_x as f64, interface_y as f64, wall_z - eye_z)
                        .normalize();

                let eye_ray = Ray {
                    origin: eye_position,
                    direction: eye_ray_direction,
                };

                if let Some(hit) = eye_ray.hit(&sphere) {
                    let hit_point = eye_ray.position(hit);
                    let hit_normal = sphere.normal(&hit_point);

                    let light_color = sphere.material.lighting(
                        &light,
                        &hit_point,
                        &-eye_ray.direction,
                        &hit_normal,
                    );

                    row_buffer[buffer_x] = light_color;
                };
            }

            let mut pixels_buffer = pixels_buffer_mtx.lock().unwrap();
            pixels_buffer[buffer_y] = row_buffer;
        });

    interface.set_center(0, 0);

    for (y, row) in pixels_buffer.iter().enumerate() {
        for (x, pixel_color) in row.iter().enumerate() {
            interface.write_pixel(x as i16, y as i16, *pixel_color);
        }
    }

    interface.update_canvas();
    interface.wait_keypress();
}
