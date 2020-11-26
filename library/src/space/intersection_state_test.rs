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
                let intersections = [
                    Intersection { t: -sqrt(2) / 2.0, object: world.objects[0].as_ref() },
                    Intersection { t: sqrt(2) / 2.0, object: world.objects[0].as_ref() },
                ];
                let intersection_state = ray.intersection_state(intersections[1].t, intersections[1].object, &intersections);

                let expected_reflectance = 1.0;

                assert_eq!(intersection_state.schlick(), expected_reflectance);
            }

            it "should be computed with a perpendicular viewing angle" {
                let ray = Ray::new((0, 0, 0), (0, 1, 0));
                let intersections = [
                    Intersection { t: -1.0, object: world.objects[0].as_ref() },
                    Intersection { t: 1.0, object: world.objects[0].as_ref() },
                ];
                let intersection_state = ray.intersection_state(intersections[1].t, intersections[1].object, &intersections);

                let expected_reflectance = 0.04;

                assert_float_absolute_eq!(intersection_state.schlick(), expected_reflectance);
            }

            it "should be computed with small angle and n2 > n1" {
                let ray = Ray::new((0.0, 0.99, -2.0), (0, 0, 1));
                let intersections = [
                    Intersection { t: 1.8589, object: world.objects[0].as_ref() },
                ];
                let intersection_state = ray.intersection_state(intersections[0].t, intersections[0].object, &intersections);

                let expected_reflectance = 0.48873;

                assert_float_absolute_eq!(intersection_state.schlick(), expected_reflectance);
            }
        } // context "Schlick approximation"
    }
}
