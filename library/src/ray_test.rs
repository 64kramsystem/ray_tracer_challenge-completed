use demonstrate::demonstrate;

demonstrate! {
    describe "Ray" {
        use crate::*;

        it "should compute a position at t" {
            let ray = Ray::new((2, 3, 4), (1, 0, 0));

            assert_eq!(ray.position(0), Tuple::point(2, 3, 4));
            assert_eq!(ray.position(1), Tuple::point(3, 3, 4));
            assert_eq!(ray.position(-1), Tuple::point(1, 3, 4));
            assert_eq!(ray.position(2.5), Tuple::point(4.5, 3, 4));
        }

        context "returns the intersections" {
            context "with an untransformed sphere" {
                it "at two points" {
                    let ray = Ray::new((0, 0, -5), (0, 0, 1));

                    let sphere = Sphere::default();

                    assert_eq!(ray.intersections(&sphere), Some((4.0, 6.0)));
                }

                it "at a tangent" {
                    let ray = Ray::new((0, 1, -5), (0, 0, 1));

                    let sphere = Sphere::default();

                    assert_eq!(ray.intersections(&sphere), Some((5.0, 5.0)));
                }

                it "at no point (miss)" {
                    let ray = Ray::new((0, 2, -5), (0, 0, 1));

                    let sphere = Sphere::default();

                    assert_eq!(ray.intersections(&sphere), None);
                }
            } // context "with an untransformed sphere"

            context "with a transformed sphere" {
                it "scaled" {
                    let ray = Ray::new((0, 0, -5), (0, 0, 1));

                    let sphere = Sphere::default().scale(2, 2, 2);

                    assert_eq!(ray.intersections(&sphere), Some((3.0, 7.0)));
                }

                it "translated" {
                    let ray = Ray::new((0, 0, -5), (0, 0, 1));

                    let sphere = Sphere::default().translate(5, 0, 0);

                    assert_eq!(ray.intersections(&sphere), None);
                }
            } // context "with a transformed sphere"
        } // context "returns the intersections"

        context "intersection state" {
            context "should be computed from an intersection and an object" {
                it "with the ray outside the object" {
                    let ray = Ray::new((0, 0, -5), (0, 0, 1));
                    let object = &Sphere::default();
                    let t = 4.0;

                    let expected_intersection_state = IntersectionState {
                        t,
                        object,
                        point: Tuple::point(0, 0, -1),
                        eyev: Tuple::vector(0, 0, -1),
                        normalv: Tuple::vector(0, 0, -1),
                        inside: false,
                    };

                    assert_eq!(ray.intersection_state(t, &object), expected_intersection_state);
                }

                it "with the ray inside the object" {
                    let ray = Ray::new((0, 0, 0), (0, 0, 1));
                    let object = &Sphere::default();
                    let t = 1.0;

                    let expected_intersection_state = IntersectionState {
                        t,
                        object,
                        point: Tuple::point(0, 0, 1),
                        eyev: Tuple::vector(0, 0, -1),
                        normalv: Tuple::vector(0, 0, -1),
                        inside: true,
                    };

                    assert_eq!(ray.intersection_state(t, &object), expected_intersection_state);
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
