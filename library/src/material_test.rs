use demonstrate::demonstrate;

demonstrate! {
    describe "Material" {
        use crate::*;
        use crate::math::*;
        use crate::space::*;

        describe "Material" {
            context "lighting" {
                before {
                    let material = Material::default();
                    let position = Tuple::point(0, 0, 0);
                }

                it "should be computed when the eye is between the light and the surface" {
                    let eyev = Tuple::vector(0, 0, -1);
                    let normalv = Tuple::vector(0, 0, -1);
                    let light = PointLight::new((0, 0, -10), (1, 1, 1));

                    let actual_result  = material.lighting(&light, &position, &eyev, &normalv, false);
                    let expected_result = Color::new(1.9, 1.9, 1.9);

                    assert_eq!(actual_result, expected_result);
                }

                it "should be computed when the eye is in the path of the reflection vector" {
                    let eyev = Tuple::vector(0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
                    let normalv = Tuple::vector(0, 0, -1);
                    let light = PointLight::new((0, 10, -10), (1, 1, 1));

                    let actual_result  = material.lighting(&light, &position, &eyev, &normalv, false);
                    let expected_result = Color::new(1.6364, 1.6364, 1.6364);

                    assert_eq!(actual_result, expected_result);
                }

                it "should be computed in the shadow" {
                    let eyev = Tuple::vector(0, 0, -1);
                    let normalv = Tuple::vector(0, 0, -1);
                    let light = PointLight::new((0, 10, -10), (1, 1, 1));

                    let actual_result  = material.lighting(&light, &position, &eyev, &normalv, true);
                    let expected_result = Color::new(0.1, 0.1, 0.1);

                    assert_eq!(actual_result, expected_result);
                }
            }
        }
    }
}
