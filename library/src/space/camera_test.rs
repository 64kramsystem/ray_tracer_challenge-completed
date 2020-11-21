use demonstrate::demonstrate;

demonstrate! {
    describe "Camera" {
        use crate::*;
        use crate::math::*;
        use crate::space::*;
        use std::f64::consts::PI;

        context "pixel size" {
            it "should be computed when width > height" {
                let camera = Camera::new(200, 125, PI / 2.0);

                let expected_pixel_size = 0.01;

                assert_float_absolute_eq!(camera.pixel_size, expected_pixel_size);
                assert_float_absolute_eq!(camera.half_width, 1.0);
                assert_float_absolute_eq!(camera.half_height, 0.625);
            }

            it "should be computed when height > width" {
                let camera = Camera::new(125, 200, PI / 2.0);

                let expected_pixel_size = 0.01;

                assert_float_absolute_eq!(camera.pixel_size, expected_pixel_size);
                assert_float_absolute_eq!(camera.half_width, 0.625);
                assert_float_absolute_eq!(camera.half_height, 1.0);
            }
        } // context "pixel size"

        context "constructs a ray" {
            it "through the center of the canvas" {
                let camera = Camera::new(201, 101, PI / 2.0);

                let ray = camera.ray_for_pixel(100, 50);

                assert_eq!(ray.origin, Tuple::point(0, 0, 0));
                assert_eq!(ray.direction, Tuple::vector(0, 0, -1));
            }

            it "through a corner of the canvas" {
                let camera = Camera::new(201, 101, PI / 2.0);

                let ray = camera.ray_for_pixel(0, 0);

                assert_eq!(ray.origin, Tuple::point(0, 0, 0));
                assert_eq!(ray.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
            }

            it "when the camera is transformed" {
                let mut camera = Camera::new(201, 101, PI / 2.0);
                camera.transform = Matrix::rotation(Axis::Y, PI / 4.0) * &Matrix::translation(0, -2, 5);

                let ray = camera.ray_for_pixel(100, 50);

                let sqrt_2 = 2.0_f64.sqrt();

                assert_eq!(ray.origin, Tuple::point(0, 2, -5));
                assert_eq!(ray.direction, Tuple::vector(sqrt_2 / 2.0, 0, -sqrt_2 / 2.0));
            }
        } // context "constructs a ray"

        it "should render a world" {
            let world = World::default();

            let mut camera = Camera::new(11, 11, PI / 2.0);

            camera.transform = Matrix::view_transform(
                &Tuple::point(0, 0, -5),
                &Tuple::point(0, 0, 0),
                &Tuple::vector(0, 1, 0),
            );

            let image: VirtualImage = camera.render(&world);

            let expected_color = &Color::new(0.38066, 0.47583, 0.2855);

            assert_eq!(image.pixel_at(5, 5).unwrap(), expected_color);
        }
    }
}
