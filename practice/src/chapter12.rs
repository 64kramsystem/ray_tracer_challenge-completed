use std::f64::consts::PI;

use library::{
    interface::Sdl2Interface,
    math::{Matrix, Tuple},
    properties::*,
    space::Cube,
    space::Plane,
    space::{Camera, PointLight, Shape, Sphere, World},
    Axis,
};
use rand::Rng;

enum MaterialQuality {
    Reflective,
    Refractive,
}

const SCREEN_WIDTH: u16 = 400; // height is half

const LIGHT_POSITION: (i32, i32, i32) = (-8, 10, -10);
const PATTERN: Option<u32> = None; // Some(n: ring, checkers+stripe, gradient, stripe, flat), or None for random
const PATTERN_SCALE: f64 = 0.33;
const REFRACTIVE_INDEX: f64 = 1.07;
const ROTATE_VOLUMES: bool = true;

#[rustfmt::skip]
fn add_objects(objects: &mut Vec<Box<dyn Shape>>) {
    let left_sphere = Sphere {
        transform: Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scaling(0.33, 0.33, 0.33) * &random_rotation(),
        material: random_material(None),
        ..Sphere::default()
    };

    let middle_sphere = Sphere {
        transform: Matrix::translation(-0.5, 1.0, 0.5) * &random_rotation(),
        material: random_material(Some(MaterialQuality::Refractive)),
        ..Sphere::default()
    };

    let back_sphere = Sphere {
        transform: Matrix::translation(0.0, 0.20, 3.0) * &Matrix::scaling(0.20, 0.20, 0.20) * &random_rotation(),
        material: random_material(None),
        ..Sphere::default()
    };

    let right_cube = Cube {
        transform: Matrix::translation(1.2, 1.0, 0.7) * &Matrix::scaling(0.33, 0.33, 0.33) * &random_rotation(),// * &Matrix::rotation(Axis::Y, PI / 32.0),
        material: random_material(Some(MaterialQuality::Reflective)),
        ..Cube::default()
    };

    let left_wall = Plane {
        transform: Matrix::translation(0, 0, 5) * &Matrix::rotation(Axis::Y, -PI / 4.0) * &Matrix::rotation(Axis::X, -PI / 2.0),
        material: random_material(None),
        ..Plane::default()
    };

    let floor = Plane {
        material: random_material(None),
        ..Plane::default()
    };

    let right_wall = Plane {
        transform: Matrix::translation(0, 0, 5) * &Matrix::rotation(Axis::Y, PI / 4.0) * &Matrix::rotation(Axis::X, -PI / 2.0),
        material: random_material(Some(MaterialQuality::Reflective)),
        ..Plane::default()
    };

    objects.push(Box::new(left_sphere));
    objects.push(Box::new(middle_sphere));
    objects.push(Box::new(back_sphere));
    objects.push(Box::new(right_cube));

    objects.push(Box::new(left_wall));
    objects.push(Box::new(floor));
    objects.push(Box::new(right_wall));
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// RAND-O-MICE-RS
////////////////////////////////////////////////////////////////////////////////////////////////////

fn random_color() -> Color {
    Color {
        r: rand::random(),
        g: rand::random(),
        b: rand::random(),
    }
}

fn random_pattern() -> Box<dyn Pattern> {
    let pattern_index = if let Some(pattern_index) = PATTERN {
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
    if ROTATE_VOLUMES {
        Matrix::rotation(Axis::X, rand::thread_rng().gen_range(0.0, 2.0 * PI))
            * &Matrix::rotation(Axis::Y, rand::thread_rng().gen_range(0.0, 2.0 * PI))
            * &Matrix::rotation(Axis::Z, rand::thread_rng().gen_range(0.0, 2.0 * PI))
    } else {
        Matrix::identity(4)
    }
}

fn random_material(fancy_material: Option<MaterialQuality>) -> Material {
    let pattern = if let Some(_) = fancy_material {
        Box::new(FlatPattern {
            color: random_color(),
            ..FlatPattern::default()
        })
    } else {
        random_pattern()
    };

    let (reflective, transparency, refractive_index) = match fancy_material {
        Some(MaterialQuality::Reflective) => (1.0, 0.0, 0.0),
        Some(MaterialQuality::Refractive) => (0.0, 1.0, REFRACTIVE_INDEX),
        None => (0.0, 0.0, 0.0),
    };

    let reflective = reflective as u32 as f64;

    Material {
        pattern: pattern,
        diffuse: 0.7,
        specular: 0.3, // walls originally had 0.0
        reflective,
        transparency,
        refractive_index,
        ..Material::default()
    }
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
