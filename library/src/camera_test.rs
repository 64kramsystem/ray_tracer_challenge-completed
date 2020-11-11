use demonstrate::demonstrate;

demonstrate! {
    describe "Camera" {
        use crate::*;
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
    }
}
