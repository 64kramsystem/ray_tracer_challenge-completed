use demonstrate::demonstrate;

demonstrate! {
    describe "Ray" {
        use crate::ray::Ray;
        use crate::tuple::Tuple;

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

        context "intersects with a sphere" {
            it "at two points" {
                let ray = Ray {
                    origin: Tuple::point(0, 0, -5),
                    direction: Tuple::vector(0, 0, 1)
                };

                assert_eq!(ray.sphere_intersections(), Some((4.0, 6.0)));
            }

            it "at a tangent" {
                let ray = Ray {
                    origin: Tuple::point(0, 1, -5),
                    direction: Tuple::vector(0, 0, 1)
                };

                assert_eq!(ray.sphere_intersections(), Some((5.0, 5.0)));
            }

            it "at no point (miss)" {
                let ray = Ray {
                    origin: Tuple::point(0, 2, -5),
                    direction: Tuple::vector(0, 0, 1)
                };

                assert_eq!(ray.sphere_intersections(), None);
            }
        } // context "intersects with a sphere"
    }
}
