/*
This is a copy of the practice15 exercises, with a few changes:

- renders to a PPM file;
- the model filename and horizontal resolution are specified in the commandline.

*/

use std::{
    f64::consts::PI,
    fs::File,
    io::{BufReader, Write},
    sync::Arc,
};

use library::{
    interface::VirtualImage,
    math::{Matrix, Tuple},
    space::*,
    utils::{ObjParser, PpmEncoder},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// COMMANDLINE DECODING/MODEL LOADING
////////////////////////////////////////////////////////////////////////////////////////////////////

// PathBuf is the rigorous type to use. It's also very ugly to handle.
//
fn load_commandline_params() -> (String, u16) {
    // For simplicity, ignore invalid (non-UTF8) filenames.
    //
    let params = std::env::args().skip(1).collect::<Vec<_>>();

    if params.len() != 2 {
        panic!("Wrong number of args (2 expected: model_filename, horizontal_resolution; current: {:?}", params);
    }

    (String::from(params[0].clone()), params[1].parse().unwrap())
}

fn load_model(model_filename: &str) -> Arc<Group> {
    let file_reader = BufReader::new(File::open(model_filename).unwrap());

    let parser = ObjParser::parse(file_reader).unwrap();

    parser.default_group()
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// WORLD/CAMERA
////////////////////////////////////////////////////////////////////////////////////////////////////

fn prepare_world(model: Arc<Group>) -> World {
    let light_position = (0, -50, -100);

    let light_source = PointLight::new(light_position, (1, 1, 1));

    let mut objects = vec![];

    objects.push(model as Arc<dyn Shape>);

    World {
        objects,
        light_source,
    }
}

fn prepare_camera(horizontal_resolution: u16) -> Camera {
    let mut camera = Camera::new(horizontal_resolution, horizontal_resolution / 2, PI / 3.0);

    camera.transform = Matrix::view_transform(
        &Tuple::point(50, -50, -20),
        &Tuple::point(-70, 30, -10),
        &Tuple::vector(0, 1, 0),
    );

    camera
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// OUTPUT
////////////////////////////////////////////////////////////////////////////////////////////////////

fn write_output_file(image: VirtualImage, model_filename: &str) -> String {
    let output_filename = String::from(model_filename.split("/").last().unwrap()) + ".ppm";
    let mut output_file = File::create(&output_filename).unwrap();

    let mut buffer_bytes = Vec::new();
    PpmEncoder::export_image(&image, &mut buffer_bytes);

    output_file.write_all(&buffer_bytes).unwrap();

    output_filename
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// MAIN
////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let (model_filename, horizontal_resolution) = load_commandline_params();
    let model = load_model(&model_filename);

    let world = prepare_world(model);
    let camera = prepare_camera(horizontal_resolution);

    let image = camera.render::<VirtualImage>(&world);

    let output_filename = write_output_file(image, &model_filename);

    println!("Rendering completed to {}", output_filename);
}
