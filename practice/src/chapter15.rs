use std::{f64::consts::PI, fs::File, io::BufReader, path::Path, sync::Arc};

use library::{
    math::{Matrix, Tuple},
    space::*,
    utils::ObjParser,
};
use sdl2_interface::Sdl2Interface;

const ASSETS_PATH: &str = "assets/testing";

const SCREEN_WIDTH: u16 = 100; // height is half

fn add_astronaut(objects: &mut Vec<Arc<dyn Shape>>) {
    let file_path = Path::new(ASSETS_PATH).join("astronaut1.obj");
    let file_reader = BufReader::new(File::open(file_path).unwrap());

    let parser = ObjParser::parse(file_reader).unwrap();

    let default_group = parser.default_group();

    objects.push(default_group as Arc<dyn Shape>);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// WORLD/CAMERA
////////////////////////////////////////////////////////////////////////////////////////////////////

fn prepare_world() -> World {
    let light_position = (0, -50, -100);

    let light_source = PointLight::new(light_position, (1, 1, 1));

    let mut objects = vec![];

    add_astronaut(&mut objects);

    World {
        objects,
        light_source,
    }
}

fn prepare_camera() -> Camera {
    let mut camera = Camera::new(SCREEN_WIDTH, SCREEN_WIDTH / 2, PI / 3.0);

    camera.transform = Matrix::view_transform(
        &Tuple::point(50, -50, -20),
        &Tuple::point(-70, 30, -10),
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
