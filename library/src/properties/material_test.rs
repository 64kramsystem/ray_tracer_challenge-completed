use demonstrate::demonstrate;

demonstrate! {
    describe "Material" {
        use crate::math::*;
        use crate::properties::*;
        use crate::space::*;

        describe "Material" {
            context "lighting" {
                before {
                    #[allow(unused_variables, unused_mut)]
                    let mut material = Material::default();
                    #[allow(unused_variables)]
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

                it "should be computed with a pattern" {
                    let material = Material {
                        pattern: Box::new(StripePattern::default()),
                        ambient: 1.0,
                        diffuse: 0.0,
                        specular: 0.0,
                        shininess: 200.0,
                    };

                    let eyev = Tuple::vector(0, 0, -1);
                    let normalv = Tuple::vector(0, 0, -1);
                    let light = PointLight::new((0, 0, -10), (1, 1, 1));

                    let actual_result_c1 = material.lighting(&light, &Tuple::point(0.9, 0, 0), &eyev, &normalv, false);

                    assert_eq!(actual_result_c1, COLOR_WHITE);

                    let actual_result_c2 = material.lighting(&light, &Tuple::point(1.1, 0, 0), &eyev, &normalv, false);

                    assert_eq!(actual_result_c2, COLOR_BLACK);
                }
            }
        }
    }
}
