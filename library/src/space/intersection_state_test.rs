use demonstrate::demonstrate;

demonstrate! {
    describe "IntersectionState" {
        use crate::lang::math::sqrt;
        use crate::properties::*;
        use crate::space::*;

        context "Schlick approximation" {
            before {
                let mut world = World::default();

                let glass_sphere = Sphere {
                    material: Material {
                        transparency: 1.0,
                        refractive_index: 1.5,
                        ..Material::default()
                    },
                    ..Sphere::default()
                };

                world.objects = vec![Box::new(glass_sphere)];
            }

            it "should be computed under total internal reflection" {
                let ray = Ray::new((0.0, 0.0, sqrt(2) / 2.0), (0, 1, 0));
                let intersection_state = ray.intersection_state(sqrt(2) / 2.0, world.objects[0].as_ref(), &world);
                let expected_reflectance = 1.0;

                assert_eq!(intersection_state.schlick(), expected_reflectance);
            }

            it "should be computed with a perpendicular viewing angle" {
                let ray = Ray::new((0, 0, 0), (0, 1, 0));
                let intersection_state = ray.intersection_state(1.0, world.objects[0].as_ref(), &world);
                let expected_reflectance = 0.04;

                assert_float_absolute_eq!(intersection_state.schlick(), expected_reflectance);
            }

            it "should be computed with small angle and n2 > n1" {
                let ray = Ray::new((0.0, 0.99, -2.0), (0, 0, 1));
                let intersection_state = ray.intersection_state(1.8589, world.objects[0].as_ref(), &world);
                let expected_reflectance = 0.48873;

                assert_float_absolute_eq!(intersection_state.schlick(), expected_reflectance);
            }
        } // context "Schlick approximation"
    }
}
