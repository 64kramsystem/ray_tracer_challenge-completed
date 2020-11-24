use std::f64::consts::PI;

use library::{
    interface::Sdl2Interface,
    math::{Matrix, Tuple},
    properties::*,
    space::*,
};

const SCREEN_WIDTH: u16 = 300; // height is half

fn prepare_world() -> World {
    let mut world = World::default();

    let plane = Plane {
        material: Material {
            reflective: 1.0,
            ..Material::default()
        },
        transform: Matrix::translation(0, -1, 0),
        ..Plane::default()
    };

    world.light_source = PointLight {
        position: Tuple::point(0, 0, -3),
        intensity: COLOR_WHITE,
    };

    world.objects.push(Box::new(plane));

    world
}

fn prepare_camera() -> Camera {
    let mut camera = Camera::new(SCREEN_WIDTH, SCREEN_WIDTH / 2, PI / 3.0);

    camera.transform = Matrix::view_transform(
        &Tuple::point(-10, 1.5, 10),
        &Tuple::point(0, 1, 0),
        &Tuple::vector(0, 1, 0),
    );

    camera
}

pub fn practice() {
    let world = prepare_world();
    let camera = prepare_camera();

    let mut interface: Sdl2Interface = camera.render(&world);

    interface.wait_keypress();
}
