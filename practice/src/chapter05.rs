use std::f64::consts::PI;

use library::{
    interface::{Image, Sdl2Interface},
    math::{Matrix, Tuple},
    properties::Color,
    space::Shape,
    space::{Ray, Sphere},
    Axis,
};

fn hit(ray: &Ray, sphere: &Sphere) -> Option<f64> {
    let intersections = sphere.intersections(ray);

    if let Some(t1) = intersections.0 {
        if t1 >= 0.0 {
            return Some(t1);
        };

        if let Some(t2) = intersections.0 {
            if t2 >= 0.0 {
                return Some(t2);
            };
        }
    }

    return None;
}

pub fn practice() {
    let display_size: u16 = 100;
    let eye_z = -50.0;
    let display_z = 50.0;

    let (center_x, center_y) = ((display_size / 2) as i16, (display_size / 2) as i16);

    let mut interface = Sdl2Interface::init("Chapter 05 exercise", display_size, display_size);
    interface.invert_y = true;
    interface.origin = (center_x, center_y);

    let hit_color = Color::new(1, 0, 0);

    // let mut sphere = Sphere::equiscaled(12.5);

    // let sphere = Sphere::new()
    //     .scale(6.25, 12.5, 12.5)
    //     .rotate(Axis::Z, -PI / 4.0)
    //     .translate(10, 0, 0);

    let mut sphere = Sphere::default();
    let transformation = Matrix::translation(10, 0, 0)
        * &Matrix::rotation(Axis::Z, -PI / 4.0)
        * &Matrix::scaling(6.25, 12.5, 12.5);
    sphere.transform = transformation;

    let ray_origin = Tuple::point(0, 0, eye_z);

    for y in -center_y..center_y {
        println!("Computing y: {}", y);

        for x in -center_x..center_x {
            let ray_direction = Tuple::vector(x as f64, y as f64, display_z - eye_z);

            let ray = Ray {
                origin: ray_origin,
                direction: ray_direction,
            };

            if let Some(_) = hit(&ray, &sphere) {
                interface.write_pixel(x, y, hit_color);
            };
        }
    }

    interface.update();
    interface.wait_keypress();
}
