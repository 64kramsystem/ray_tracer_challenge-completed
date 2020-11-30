use demonstrate::demonstrate;

demonstrate! {
    describe "Cylinder" {
        use crate::math::*;
        use crate::lang::ApproximateFloat64Ops;
        use crate::space::{*, shape::private::ShapeLocal};

        before {
            #[allow(unused_mut)]
            let mut cylinder = Cylinder::default();
        }

        it "A ray misses a cylinder" {
            let examples = [
                // origin    direction
                ((1, 0,  0), (0, 1, 0)),
                ((0, 0,  0), (0, 1, 0)),
                ((0, 0, -5), (1, 1, 1)),
            ];

            for ((ox, oy, oz), (dx, dy, dz)) in examples.iter() {
                let origin    = Tuple::point(*ox, *oy, *oz);
                let direction = Tuple::vector(*dx, *dy, *dz).normalize();
                let ray = Ray { origin, direction };

                assert_eq!(cylinder.local_intersections(&ray), (None, None));
            }
        }

        it "A ray strikes a cylinder" {
            let examples = [
                // origin    direction      t1       t2
                ((1.0, 0, -5), (0.0, 0, 1), 5.0,     5.0    ),
                ((0.0, 0, -5), (0.0, 0, 1), 4.0,     6.0    ),
                ((0.5, 0, -5), (0.1, 1, 1), 6.80798, 7.08872),
            ];

            for ((ox, oy, oz), (dx, dy, dz), t1, t2) in examples.iter() {
                let origin    = Tuple::point(*ox, *oy, *oz);
                let direction = Tuple::vector(*dx, *dy, *dz).normalize();
                let ray = Ray { origin, direction };

                let (actual_t1, actual_t2) = cylinder.local_intersections(&ray);

                assert!(actual_t1.unwrap().approximate_equals(*t1));
                assert!(actual_t2.unwrap().approximate_equals(*t2));
            }
        }

        it "Normal vector on a cylinder" {
            let examples = [
                // point       normal
                (( 1,  0,  0), ( 1, 0,  0)),
                (( 0,  5, -1), ( 0, 0, -1)),
                (( 0, -2,  1), ( 0, 0,  1)),
                ((-1,  1,  0), (-1, 0,  0)),
            ];

            for ((px, py, pz), (nx, ny, nz)) in examples.iter() {
                let point = Tuple::point(*px, *py, *pz);
                let expected_normal = Tuple::vector(*nx, *ny, *nz);

                assert_eq!(cylinder.local_normal(&point), expected_normal);
            }
        }

        it "The default minimum and maximum for a cylinder" {
            assert_eq!(cylinder.minimum, f64::NEG_INFINITY);
            assert_eq!(cylinder.maximum, f64::INFINITY);
        }

        it "Intersecting a constrained cylinder" {
            cylinder.minimum = 1.0;
            cylinder.maximum = 2.0;

            let examples = [
                // origin      direction    count
                ((0, 1.5,  0), (0.1, 1, 0), 0), // diagonal ray from inside
                ((0, 3.0, -5), (0.0, 0, 1), 0), // perpendicular to the y axis
                ((0, 0.0, -5), (0.0, 0, 1), 0), // perpendicular to the y axis
                ((0, 2.0, -5), (0.0, 0, 1), 0), // edge case: doesn't touch the max
                ((0, 1.0, -5), (0.0, 0, 1), 0), // edge case: doesn't touch the min
                ((0, 1.5, -2), (0.0, 0, 1), 2), // perpendicular through mid-cylinder
            ];

            for ((ox, oy, oz), (dx, dy, dz), count) in examples.iter() {
                let origin    = Tuple::point(*ox, *oy, *oz);
                let direction = Tuple::vector(*dx, *dy, *dz).normalize();
                let ray = Ray { origin, direction };

                let intersections = cylinder.local_intersections(&ray);

                match count {
                    2 => {
                        assert!(intersections.0.is_some());
                        assert!(intersections.1.is_some());
                    },
                    0 => {
                        assert_eq!(intersections, (None, None));
                    },
                    _ => {
                        unreachable!();
                    }
                };
            }
        }
    }
}
