use std::{f64::consts::PI, sync::Arc};

use library::{
    interface::Sdl2Interface,
    math::{Matrix, Tuple},
    space::*,
    Axis,
};

const SCREEN_WIDTH: u16 = 400; // height is half

const LIGHT_POSITION: (i32, i32, i32) = (-8, 10, -10);

fn hexagon_corner() -> Arc<dyn Shape> {
    Arc::new(Sphere {
        transform: Matrix::translation(0, 0, -1) * &Matrix::scaling(0.25, 0.25, 0.25),
        ..Sphere::default()
    })
}

fn hexagon_edge() -> Arc<dyn Shape> {
    Arc::new(Cylinder {
        minimum: 0.0,
        maximum: 1.0,
        transform: Matrix::translation(0, 0, -1)
            * &Matrix::rotation(Axis::Y, -PI / 6.0)
            * &Matrix::rotation(Axis::Z, -PI / 2.0)
            * &Matrix::scaling(0.25, 1.0, 0.25),
        ..Cylinder::default()
    })
}

fn hexagon_side(transform: Matrix) -> Arc<dyn Shape> {
    let side: Arc<Group> = Arc::new(Group {
        transform,
        ..Group::default()
    });

    Group::add_child(&side, &hexagon_corner());
    Group::add_child(&side, &hexagon_edge());

    side
}

fn hexagon() -> Arc<dyn Shape> {
    // Transformation added to make it look nicer.
    //
    let hex: Arc<Group> = Arc::new(Group {
        transform: Matrix::translation(-0.2, 0.7, 0.0)
            * &Matrix::rotation(Axis::Y, PI / 6.0)
            * &Matrix::rotation(Axis::X, -PI / 6.0),
        ..Group::default()
    });

    for n in 0..6 {
        let transform = Matrix::rotation(Axis::Y, n as f64 * PI / 3.0);
        let side: Arc<dyn Shape> = hexagon_side(transform);

        Group::add_child(&hex, &side);
    }

    hex
}

fn add_objects(objects: &mut Vec<Arc<dyn Shape>>) {
    let hexagon = hexagon();

    objects.push(hexagon);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// WORLD/CAMERA
////////////////////////////////////////////////////////////////////////////////////////////////////

fn prepare_world() -> World {
    let light_source = PointLight::new(LIGHT_POSITION, (1, 1, 1));

    let mut objects = vec![];

    add_objects(&mut objects);

    World {
        objects,
        light_source,
    }
}

fn prepare_camera() -> Camera {
    let mut camera = Camera::new(SCREEN_WIDTH, SCREEN_WIDTH / 2, PI / 3.0);

    camera.transform = Matrix::view_transform(
        &Tuple::point(0, 1.5, -5),
        &Tuple::point(0, 1, 0),
        &Tuple::vector(0, 1, 0),
    );

    camera
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// MAIN
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn practice() {
    let world = prepare_world();
    let camera = prepare_camera();

    let mut interface: Sdl2Interface = camera.render(&world);

    interface.wait_keypress();
}
