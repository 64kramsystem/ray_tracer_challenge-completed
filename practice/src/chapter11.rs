use std::f64::consts::PI;

use library::{
    interface::Sdl2Interface,
    math::{Matrix, Tuple},
    properties::*,
    space::Plane,
    space::{Camera, PointLight, Shape, Sphere, World},
    Axis,
};
use rand::Rng;

const SCREEN_WIDTH: u16 = 400; // height is half

const PATTERN_INDEX: Option<u32> = None; // Some(n: ring, checkers+stripe, gradient, stripe, flat), or None for random
const PATTERN_SCALE: f64 = 0.33;
const ROTATE_SPHERES: bool = true;
const REFLECTIVE_CHANCE: f64 = 0.33;

const MIDDLE_SPHERE_COORDS: (f64, f64, f64) = (-0.5, 1.0, 0.5);
const RIGHT_SPHERE_COORDS: (f64, f64, f64) = (1.5, 0.5, -0.5);
const LEFT_SPHERE_COORDS: (f64, f64, f64) = (-1.5, 0.33, -0.75);

fn random_color() -> Color {
    Color {
        r: rand::random(),
        g: rand::random(),
        b: rand::random(),
    }
}

// If reflective is true, the pattern is always flat, otherwise, it gets ugly.
//
fn random_pattern(reflective: bool) -> Box<dyn Pattern> {
    let pattern_index = if reflective {
        4
    } else if let Some(pattern_index) = PATTERN_INDEX {
        pattern_index
    } else {
        rand::thread_rng().gen_range(0, 5)
    };

    match pattern_index {
        0 => Box::new(RingPattern {
            transform: Matrix::scaling(PATTERN_SCALE, PATTERN_SCALE, PATTERN_SCALE),
            color_a: random_color(),
            color_b: random_color(),
            ..RingPattern::default()
        }),
        1 => Box::new(CheckersPattern {
            transform: Matrix::scaling(PATTERN_SCALE, PATTERN_SCALE, PATTERN_SCALE),
            color_a: random_color(),
            color_b: random_color(),
            previous_pattern: Some(Box::new(StripePattern {
                transform: Matrix::scaling(PATTERN_SCALE, PATTERN_SCALE, PATTERN_SCALE),
                color_a: random_color(),
                color_b: random_color(),
                ..StripePattern::default()
            })),
            ..CheckersPattern::default()
        }),
        2 => Box::new(GradientPattern {
            transform: Matrix::scaling(PATTERN_SCALE, PATTERN_SCALE, PATTERN_SCALE),
            color_a: random_color(),
            color_b: random_color(),
            ..GradientPattern::default()
        }),
        3 => Box::new(StripePattern {
            transform: Matrix::scaling(PATTERN_SCALE, PATTERN_SCALE, PATTERN_SCALE),
            color_a: random_color(),
            color_b: random_color(),
            ..StripePattern::default()
        }),
        4 => Box::new(FlatPattern {
            color: random_color(),
            ..FlatPattern::default()
        }),
        _ => unreachable!(),
    }
}

fn random_rotation() -> Matrix {
    if ROTATE_SPHERES {
        Matrix::rotation(Axis::X, rand::thread_rng().gen_range(0.0, 2.0 * PI))
            * &Matrix::rotation(Axis::Y, rand::thread_rng().gen_range(0.0, 2.0 * PI))
            * &Matrix::rotation(Axis::Z, rand::thread_rng().gen_range(0.0, 2.0 * PI))
    } else {
        Matrix::identity(4)
    }
}

fn random_sphere_material() -> Material {
    let reflective: bool = rand::random::<f64>() <= REFLECTIVE_CHANCE;

    Material {
        pattern: random_pattern(reflective),
        diffuse: 0.7,
        specular: 0.3,
        reflective: (reflective as u32) as f64,
        ..Material::default()
    }
}

fn random_wall_material() -> Material {
    let reflective: bool = rand::random::<f64>() <= REFLECTIVE_CHANCE;

    Material {
        pattern: random_pattern(reflective),
        diffuse: 0.7,  // book: default
        specular: 0.0, // book: 0.0
        reflective: (reflective as u32) as f64,
        ..Material::default()
    }
}

fn add_spheres(objects: &mut Vec<Box<dyn Shape>>) {
    let middle = Sphere {
        transform: Matrix::translation(
            MIDDLE_SPHERE_COORDS.0,
            MIDDLE_SPHERE_COORDS.1,
            MIDDLE_SPHERE_COORDS.2,
        ) * &random_rotation(),
        material: random_sphere_material(),
        ..Sphere::default()
    };

    let right = Sphere {
        transform: Matrix::translation(
            RIGHT_SPHERE_COORDS.0,
            RIGHT_SPHERE_COORDS.1,
            RIGHT_SPHERE_COORDS.2,
        ) * &Matrix::scaling(0.5, 0.5, 0.5)
            * &random_rotation(),
        material: random_sphere_material(),
        ..Sphere::default()
    };

    let left = Sphere {
        transform: Matrix::translation(
            LEFT_SPHERE_COORDS.0,
            LEFT_SPHERE_COORDS.1,
            LEFT_SPHERE_COORDS.2,
        ) * &Matrix::scaling(0.33, 0.33, 0.33)
            * &random_rotation(),
        material: random_sphere_material(),
        ..Sphere::default()
    };

    objects.push(Box::new(middle));
    objects.push(Box::new(right));
    objects.push(Box::new(left));
}

fn add_walls(objects: &mut Vec<Box<dyn Shape>>) {
    let floor = Plane {
        material: random_wall_material(),
        ..Plane::default()
    };

    let left = Plane {
        transform: Matrix::translation(0, 0, 5)
            * &Matrix::rotation(Axis::Y, -PI / 4.0)
            * &Matrix::rotation(Axis::X, -PI / 2.0),
        material: random_wall_material(),
        ..Plane::default()
    };

    let right = Plane {
        transform: Matrix::translation(0, 0, 5)
            * &Matrix::rotation(Axis::Y, PI / 4.0)
            * &Matrix::rotation(Axis::X, -PI / 2.0),
        material: random_wall_material(),
        ..Plane::default()
    };

    objects.push(Box::new(floor));
    objects.push(Box::new(left));
    objects.push(Box::new(right));
}

fn prepare_world() -> World {
    let light_source = PointLight::new((-10, 10, -10), (1, 1, 1));

    let mut objects = vec![];

    add_walls(&mut objects);
    add_spheres(&mut objects);

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

pub fn practice() {
    let world = prepare_world();
    let camera = prepare_camera();

    let mut interface: Sdl2Interface = camera.render(&world);

    interface.wait_keypress();
}
