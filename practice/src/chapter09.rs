use std::f64::consts::PI;

use library::{
    interface::Sdl2Interface,
    math::{Matrix, Tuple},
    properties::{Color, Material},
    space::Plane,
    space::{Camera, PointLight, Shape, Sphere, World},
    Axis,
};

fn prepare_material() -> Material {
    Material {
        color: Color::new(1, 0.9, 0.9),
        specular: 0.0,
        ..Material::default()
    }
}

fn prepare_world() -> World {
    let light_source = PointLight::new((-10, 10, -10), (1, 1, 1));

    let floor = Plane {
        material: prepare_material(),
        ..Plane::default()
    };

    let left_wall = Plane {
        transformation: Matrix::translation(0, 0, 5)
            * &Matrix::rotation(Axis::Y, -PI / 4.0)
            * &Matrix::rotation(Axis::X, -PI / 2.0),
        material: prepare_material(),
        ..Plane::default()
    };

    let right_wall = Plane {
        transformation: Matrix::translation(0, 0, 5)
            * &Matrix::rotation(Axis::Y, PI / 4.0)
            * &Matrix::rotation(Axis::X, -PI / 2.0),
        material: prepare_material(),
        ..Plane::default()
    };

    let middle = Sphere {
        transformation: Matrix::translation(-0.5, 1.0, 0.5),
        material: Material {
            color: Color::new(0.1, 1, 0.5),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
        ..Sphere::default()
    };

    let right = Sphere {
        transformation: Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scaling(0.5, 0.5, 0.5),
        material: Material {
            color: Color::new(0.5, 1, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
        ..Sphere::default()
    };

    let left = Sphere {
        transformation: Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scaling(0.33, 0.33, 0.33),
        material: Material {
            color: Color::new(1, 0.8, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
        ..Sphere::default()
    };

    let objects: Vec<Box<dyn Shape>> = vec![
        Box::new(floor),
        Box::new(left_wall),
        Box::new(right_wall),
        Box::new(middle),
        Box::new(right),
        Box::new(left),
    ];

    World {
        objects,
        light_source,
    }
}

fn prepare_camera() -> Camera {
    let mut camera = Camera::new(400, 200, PI / 3.0);

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
