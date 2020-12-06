use demonstrate::demonstrate;

demonstrate! {
    describe "PpmEncoder" {
        use crate::interface::{Image,VirtualImage};
        use crate::properties::Color;
        use crate::utils::PpmEncoder;
        use indoc::indoc;

        it "should encode an exportable image" {
            let mut virtual_image = VirtualImage::new(5, 3);

            let c1 = Color::new(1.5, 0, 0);
            let c2 = Color::new(0, 0.5, 0);
            let c3 = Color::new(-0.5, 0, 1);

            virtual_image.write_pixel(0, 0, c1);
            virtual_image.write_pixel(2, 1, c2);
            virtual_image.write_pixel(4, 2, c3);

            let mut buffer_bytes = Vec::new();

            PpmEncoder::export_image(&virtual_image, &mut buffer_bytes);

            let buffer_string = String::from_utf8(buffer_bytes).unwrap();

            let expected_string = indoc! {"
                P3
                5 3
                255
                255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 0 0 0
                0 0 0 0 0 0 0 0 0 0 0 255
            "};

            assert_eq!(buffer_string, expected_string);
        }
    }
}
