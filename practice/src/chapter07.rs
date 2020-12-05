use std::{f64::consts::PI, sync::Arc};

use library::{
    interface::Sdl2Interface,
    math::{Matrix, Tuple},
    properties::FlatPattern,
    properties::Material,
    space::{Camera, PointLight, Shape, Sphere, World},
    Axis,
};

fn prepare_material() -> Material {
    Material {
        pattern: Box::new(FlatPattern::new(1, 0.9, 0.9)),
        specular: 0.0,
        ..Material::default()
    }
}

fn prepare_world() -> World {
    let floor = Sphere {
        transform: Matrix::scaling(10.0, 0.01, 10.0),
        material: prepare_material(),
        ..Sphere::default()
    };

    let left_wall = Sphere {
        transform: Matrix::translation(0, 0, 5)
            * &Matrix::rotation(Axis::Y, -PI / 4.0)
            * &Matrix::rotation(Axis::X, PI / 2.0)
            * &Matrix::scaling(10.0, 0.01, 10.0),
        material: prepare_material(),
        ..Sphere::default()
    };

    let right_wall = Sphere {
        transform: Matrix::translation(0, 0, 5)
            * &Matrix::rotation(Axis::Y, PI / 4.0)
            * &Matrix::rotation(Axis::X, PI / 2.0)
            * &Matrix::scaling(10.0, 0.01, 10.0),
        material: prepare_material(),
        ..Sphere::default()
    };

    let middle = Sphere {
        transform: Matrix::translation(-0.5, 1.0, 0.5),
        material: Material {
            pattern: Box::new(FlatPattern::new(0.1, 1, 0.5)),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
        ..Sphere::default()
    };

    let right = Sphere {
        transform: Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scaling(0.5, 0.5, 0.5),
        material: Material {
            pattern: Box::new(FlatPattern::new(0.5, 1, 0.1)),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
        ..Sphere::default()
    };

    let left = Sphere {
        transform: Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scaling(0.33, 0.33, 0.33),
        material: Material {
            pattern: Box::new(FlatPattern::new(1, 0.8, 0.1)),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
        ..Sphere::default()
    };

    let objects: Vec<Arc<dyn Shape>> = vec![
        Arc::new(floor),
        Arc::new(left_wall),
        Arc::new(right_wall),
        Arc::new(middle),
        Arc::new(right),
        Arc::new(left),
    ];

    World {
        objects,
        light_source: PointLight::new((-10, 10, -10), (1, 1, 1)),
    }
}

fn prepare_camera() -> Camera {
    let mut camera = Camera::new(100, 50, PI / 3.0);

    camera.transform = Matrix::view_transform(
        &Tuple::point(0, 1.5, -5),
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
