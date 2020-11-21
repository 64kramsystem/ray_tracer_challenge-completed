use demonstrate::demonstrate;

demonstrate! {
    describe "Color" {
        use crate::properties::*;

        it "can be added to another color" {
            let color1 = Color { r: 0.9, g: 0.6, b: 0.75 };
            let color2 = Color { r: 0.7, g: 0.1, b: 0.25 };

            let expected_color = Color { r: 1.6, g: 0.7, b: 1.0 };

            assert_eq!(color1 + &color2, expected_color);
        }

        it "can be subtracted from another color" {
            let color1 = Color { r: 0.9, g: 0.6, b: 0.75 };
            let color2 = Color { r: 0.7, g: 0.1, b: 0.25 };

            let expected_color = Color { r: 0.2, g: 0.5, b: 0.5};

            assert_eq!(color1 - &color2, expected_color);
        }

        it "can be multiplied by a scalar" {
            let color = Color { r: 0.2, g: 0.3, b: 0.4 };

            let expected_color = Color { r: 0.4, g: 0.6, b: 0.8 };

            assert_eq!(color * 2, expected_color);
        }

        it "can be multiplied by another color" {
            let color1 = Color { r: 0.2, g: 0.3, b: 0.4 };
            let color2 = Color { r: 0.3, g: 0.1, b: 0.5 };

            let expected_color = Color { r: 0.06, g: 0.03, b: 0.2 };

            assert_eq!(color1 * &color2, expected_color);
        }
    }
}
