use demonstrate::demonstrate;

demonstrate! {
    describe "Material" {
        use crate::math::*;
        use crate::lang::math::sqrt;
        use crate::properties::*;
        use crate::space::*;

        // The tests assume a default Sphere as object; since the transform is an identity, the world
        // and object coordinates are the same, so we pass twice the same value as argument to lighting().
        //
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

                    let actual_result  = material.lighting(&light, &position, &position, &eyev, &normalv, false);
                    let expected_result = Color::new(1.9, 1.9, 1.9);

                    assert_eq!(actual_result, expected_result);
                }

                it "should be computed when the eye is in the path of the reflection vector" {
                    let eyev = Tuple::vector(0, -sqrt(2.0) / 2.0, -sqrt(2.0) / 2.0);
                    let normalv = Tuple::vector(0, 0, -1);
                    let light = PointLight::new((0, 10, -10), (1, 1, 1));

                    let actual_result  = material.lighting(&light, &position, &position, &eyev, &normalv, false);
                    let expected_result = Color::new(1.6364, 1.6364, 1.6364);

                    assert_eq!(actual_result, expected_result);
                }

                it "should be computed in the shadow" {
                    let eyev = Tuple::vector(0, 0, -1);
                    let normalv = Tuple::vector(0, 0, -1);
                    let light = PointLight::new((0, 10, -10), (1, 1, 1));

                    let actual_result  = material.lighting(&light, &position, &position, &eyev, &normalv, true);
                    let expected_result = Color::new(0.1, 0.1, 0.1);

                    assert_eq!(actual_result, expected_result);
                }

                it "should be computed with a pattern" {
                    let material = Material {
                        pattern: Box::new(StripePattern::default()),
                        ambient: 1.0,
                        diffuse: 0.0,
                        specular: 0.0,
                        ..Material::default()
                    };

                    let eyev = Tuple::vector(0, 0, -1);
                    let normalv = Tuple::vector(0, 0, -1);
                    let light = PointLight::new((0, 0, -10), (1, 1, 1));

                    let position_c1 = Tuple::point(0.9, 0, 0);
                    let actual_result_c1 = material.lighting(&light, &position_c1, &position_c1, &eyev, &normalv, false);

                    assert_eq!(actual_result_c1, COLOR_WHITE);

                    let position_c2 = Tuple::point(1.1, 0, 0);
                    let actual_result_c2 = material.lighting(&light, &position_c2, &position_c2, &eyev, &normalv, false);

                    assert_eq!(actual_result_c2, COLOR_BLACK);
                }
            }
        }
    }
}
