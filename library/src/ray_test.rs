use demonstrate::demonstrate;

demonstrate! {
    describe "Ray" {
        use crate::ray::Ray;
        use crate::sphere::Sphere;
        use crate::tuple::Tuple;
        use crate::intersection_state::IntersectionState;

        it "should compute a position at t" {
            let ray = Ray {
                origin: Tuple::point(2, 3, 4),
                direction: Tuple::vector(1, 0, 0),
            };

            assert_eq!(ray.position(0), Tuple::point(2, 3, 4));
            assert_eq!(ray.position(1), Tuple::point(3, 3, 4));
            assert_eq!(ray.position(-1), Tuple::point(1, 3, 4));
            assert_eq!(ray.position(2.5), Tuple::point(4.5, 3, 4));
        }

        context "returns the intersections" {
            context "with an untransformed sphere" {
                it "at two points" {
                    let ray = Ray {
                        origin: Tuple::point(0, 0, -5),
                        direction: Tuple::vector(0, 0, 1)
                    };

                    let sphere = Sphere::new();

                    assert_eq!(ray.intersections(&sphere), Some((4.0, 6.0)));
                }

                it "at a tangent" {
                    let ray = Ray {
                        origin: Tuple::point(0, 1, -5),
                        direction: Tuple::vector(0, 0, 1)
                    };

                    let sphere = Sphere::new();

                    assert_eq!(ray.intersections(&sphere), Some((5.0, 5.0)));
                }

                it "at no point (miss)" {
                    let ray = Ray {
                        origin: Tuple::point(0, 2, -5),
                        direction: Tuple::vector(0, 0, 1)
                    };

                    let sphere = Sphere::new();

                    assert_eq!(ray.intersections(&sphere), None);
                }
            } // context "with an untransformed sphere"

            context "with a transformed sphere" {
                it "scaled" {
                    let ray = Ray {
                        origin: Tuple::point(0, 0, -5),
                        direction: Tuple::vector(0, 0, 1)
                    };

                    let sphere = Sphere::new().scale(2, 2, 2);

                    assert_eq!(ray.intersections(&sphere), Some((3.0, 7.0)));
                }

                it "translated" {
                    let ray = Ray {
                        origin: Tuple::point(0, 0, -5),
                        direction: Tuple::vector(0, 0, 1)
                    };

                    let sphere = Sphere::new().translate(5, 0, 0);

                    assert_eq!(ray.intersections(&sphere), None);
                }
            } // context "with a transformed sphere"
        } // context "returns the intersections"

        context "intersection state" {
            it "should be computed from an intersection and an object" {
                let ray = Ray {
                    origin: Tuple::point(0, 0, -5),
                    direction: Tuple::vector(0, 0, 1)
                };
                let object = &Sphere::new();
                let t = 4.0;

                let expected_intersection_state = IntersectionState {
                    t,
                    object,
                    point: Tuple::point(0, 0, -1),
                    eyev: Tuple::vector(0, 0, -1),
                    normalv: Tuple::vector(0, 0, -1),
                };

                assert_eq!(ray.intersection_state(t, &object), expected_intersection_state);
            }
        } // context "intersection state"

        context "transformations" {
            it "should translate a ray" {
                let ray = Ray {
                    origin: Tuple::point(1, 2, 3),
                    direction: Tuple::vector(0, 1, 0),
                };

                let expected_ray = Ray {
                    origin: Tuple::point(4, 6, 8),
                    direction: Tuple::vector(0, 1, 0),
                };

                assert_eq!(ray.translate(3, 4, 5), expected_ray);
            }

            it "should scale a ray" {
                let ray = Ray {
                    origin: Tuple::point(1, 2, 3),
                    direction: Tuple::vector(0, 1, 0),
                };

                let expected_ray = Ray {
                    origin: Tuple::point(2, 6, 12),
                    direction: Tuple::vector(0, 3, 0),
                };

                assert_eq!(ray.scale(2, 3, 4), expected_ray);
            }
        } // context "transformations"
    }
}
