use std::f64::consts::PI;

use library::{Axis, Color, Matrix, PointLight, Ray, Sdl2Interface, Sphere, Tuple};

pub fn practice() {
    // The wall matches the display; for simplicity, the "wall" concept represents both.
    //
    let wall_size: u16 = 100;
    let eye_z = -50.0;
    let wall_z = 50.0;
    let light_position = Tuple::point(-20, 30, -50);

    let (center_x, center_y) = ((wall_size / 2) as i16, (wall_size / 2) as i16);

    let mut interface = Sdl2Interface::init(
        "Chapter 05 exercise",
        wall_size,
        wall_size,
        (center_x, center_y),
    );

    let mut sphere = Sphere::new();
    sphere.material.color = Color::new(1, 0.2, 1);
    sphere.transformation = Matrix::translation(10, 0, 0)
        * &Matrix::rotation(Axis::Z, -PI / 4.0)
        * &Matrix::scaling(6.25, 12.5, 12.5);

    let light = PointLight::new(light_position, Color::new(1, 1, 1));

    let eye_position = Tuple::point(0, 0, eye_z);

    for y in -center_y..center_y {
        println!("Computing y: {}", y);

        for x in -center_x..center_x {
            let eye_ray_direction = Tuple::vector(x as f64, y as f64, wall_z - eye_z).normalize();

            let eye_ray = Ray {
                origin: eye_position,
                direction: eye_ray_direction,
            };

            if let Some(hit) = eye_ray.hit(&sphere) {
                let hit_point = eye_ray.position(hit);
                let hit_normal = sphere.normal(&hit_point);

                let light_color =
                    sphere
                        .material
                        .lighting(&light, &hit_point, &-eye_ray.direction, &hit_normal);

                interface.write_pixel(x, y, light_color);
            };
        }
    }

    interface.update_canvas();
    interface.wait_keypress();
}
