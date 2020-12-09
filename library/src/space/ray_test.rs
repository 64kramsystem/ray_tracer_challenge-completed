use demonstrate::demonstrate;

demonstrate! {
    describe "Ray" {
        use std::sync::Arc;
        use crate::math::*;
        use crate::lang::math::sqrt;
        use crate::space::*;
        use crate::properties::*;

        it "should compute a position at t" {
            let ray = Ray::new((2, 3, 4), (1, 0, 0));

            assert_eq!(ray.position(0), Tuple::point(2, 3, 4));
            assert_eq!(ray.position(1), Tuple::point(3, 3, 4));
            assert_eq!(ray.position(-1), Tuple::point(1, 3, 4));
            assert_eq!(ray.position(2.5), Tuple::point(4.5, 3, 4));
        }

        context "intersection state" {
            context "should be computed from an intersection and an object" {
                it "with the ray outside the object" {
                    let ray = Ray::new((0, 0, -5), (0, 0, 1));
                    let intersection = Intersection { t: 4.0, object: Arc::new(Sphere::default()), ..Intersection::default() };
                    let intersection_clone = intersection.clone();

                    let expected_intersection_state = IntersectionState {
                        t: intersection_clone.t.clone(),
                        object: intersection_clone.object,
                        point: Tuple::point(0, 0, -1),
                        over_point: Tuple::point(0, 0, -1),
                        under_point: Tuple::point(0, 0, -0.9999),
                        eyev: Tuple::vector(0, 0, -1),
                        normalv: Tuple::vector(0, 0, -1),
                        reflectv: Tuple::vector(0, 0, -1),
                        n1: REFRACTIVE_INDEX_VACUUM,
                        n2: REFRACTIVE_INDEX_VACUUM,
                        inside: false,
                    };

                    let actual_intersection_state = ray.intersection_state(&intersection, &[]);

                    assert_eq!(actual_intersection_state, expected_intersection_state);
                }

                it "with the ray inside the object" {
                    let ray = Ray::new((0, 0, 0), (0, 0, 1));
                    let intersection = Intersection { t: 1.0, object: Arc::new(Sphere::default()), ..Intersection::default() };
                    let intersection_clone = intersection.clone();

                    let expected_intersection_state = IntersectionState {
                        t: intersection_clone.t,
                        object: intersection_clone.object,
                        point: Tuple::point(0, 0, 1),
                        over_point: Tuple::point(0, 0, 0.9999),
                        under_point: Tuple::point(0, 0, 1),
                        eyev: Tuple::vector(0, 0, -1),
                        normalv: Tuple::vector(0, 0, -1),
                        reflectv: Tuple::vector(0, 0, -1),
                        n1: REFRACTIVE_INDEX_VACUUM,
                        n2: REFRACTIVE_INDEX_VACUUM,
                        inside: true,
                    };

                    let actual_intersection_state = ray.intersection_state(&intersection, &[]);

                    assert_eq!(actual_intersection_state, expected_intersection_state);
                }

                it "with reflection" {
                    let object = Arc::new(Plane::default());
                    let ray = Ray::new((0, 1, -1), (0.0, -sqrt(2) / 2.0, sqrt(2) / 2.0));
                    let intersection = Intersection { t: sqrt(2), object: object, ..Intersection::default() };

                    let actual_intersection_state = ray.intersection_state(&intersection, &[]);
                    let expected_reflectv = Tuple::vector(0.0, sqrt(2) / 2.0, sqrt(2) / 2.0);

                    assert_eq!(actual_intersection_state.reflectv, expected_reflectv);
                }
            } // context "should be computed from an intersection and an object"
        } // context "intersection state"

        context "transformations" {
            it "should translate a ray" {
                let ray = Ray::new((1, 2, 3), (0, 1, 0));

                let expected_ray = Ray::new((4, 6, 8), (0, 1, 0));

                assert_eq!(ray.translate(3, 4, 5), expected_ray);
            }

            it "should scale a ray" {
                let ray = Ray::new((1, 2, 3), (0, 1, 0));

                let expected_ray = Ray::new((2, 6, 12), (0, 3, 0));

                assert_eq!(ray.scale(2, 3, 4), expected_ray);
            }
        } // context "transformations"
    }
}
