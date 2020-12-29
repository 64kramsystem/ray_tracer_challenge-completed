use std::{f64::consts::PI, sync::Arc};

use library::{
    interface::Sdl2Interface,
    math::{Matrix, Tuple},
    properties::{Color, FlatPattern, Material, COLOR_BLUE, COLOR_RED},
    space::*,
    Axis,
};

const SCREEN_WIDTH: u16 = 800; // height is half

const LIGHT_POSITION: (i32, i32, i32) = (-8, 10, -10);

fn random_color() -> Color {
    Color {
        r: rand::random(),
        g: rand::random(),
        b: rand::random(),
    }
}

fn add_walls(objects: &mut Vec<Arc<dyn Shape>>) {
    let left_wall = Plane {
        transform: Matrix::rotation(Axis::X, -PI / 2.0)
            .rotate(Axis::Y, -PI / 4.0)
            .translate(0, 0, 5),
        material: Material {
            pattern: Box::new(FlatPattern {
                color: random_color(),
                ..FlatPattern::default()
            }),
            ..Material::default()
        },
        ..Plane::default()
    };

    let floor = Plane {
        material: Material {
            pattern: Box::new(FlatPattern {
                color: random_color(),
                ..FlatPattern::default()
            }),
            ..Material::default()
        },
        ..Plane::default()
    };

    let right_wall = Plane {
        transform: Matrix::rotation(Axis::X, -PI / 2.0)
            .rotate(Axis::Y, PI / 4.0)
            .translate(0, 0, 5),
        material: Material {
            pattern: Box::new(FlatPattern {
                color: random_color(),
                ..FlatPattern::default()
            }),
            ..Material::default()
        },
        ..Plane::default()
    };

    objects.push(Arc::new(left_wall));
    objects.push(Arc::new(floor));
    objects.push(Arc::new(right_wall));
}

fn add_csg(objects: &mut Vec<Arc<dyn Shape>>) {
    let sphere: Arc<dyn Shape> = Arc::new(Sphere {
        material: Material {
            pattern: Box::new(FlatPattern {
                color: COLOR_RED,
                ..FlatPattern::default()
            }),
            transparency: 0.9,
            ..Material::default()
        },
        ..Sphere::default()
    });
    let cube: Arc<dyn Shape> = Arc::new(Cube {
        transform: Matrix::scaling(0.5, 0.5, 0.5).translate(-0.2, 0.0, -0.5),
        material: Material {
            pattern: Box::new(FlatPattern {
                color: COLOR_BLUE,
                ..FlatPattern::default()
            }),
            transparency: 0.9,
            ..Material::default()
        },
        ..Cube::default()
    });

    let csg = Csg::new(
        csg::Operation::Difference,
        sphere,
        cube,
        Matrix::translation(0.0, 1.0, 0.0),
    );

    objects.push(csg);

    // objects.push(cube);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// WORLD/CAMERA
////////////////////////////////////////////////////////////////////////////////////////////////////

fn prepare_world() -> World {
    let light_source = PointLight::new(LIGHT_POSITION, (1, 1, 1));

    let mut objects = vec![];

    add_walls(&mut objects);
    add_csg(&mut objects);

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
