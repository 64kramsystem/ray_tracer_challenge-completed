use demonstrate::demonstrate;

demonstrate! {
    describe "Cylinder" {
        use std::sync::Arc;
        use crate::math::*;
        use crate::lang::ApproximateFloat64Ops;
        use crate::space::{*, shape::private::ShapeLocal};

        before {
            #[allow(unused_mut,unused_variables)]
            let cylinder = Arc::new(Cylinder::default());
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

                assert_eq!(Arc::clone(&cylinder).local_intersections(&ray), vec![]);
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

                let actual_intersections = Arc::clone(&cylinder).local_intersections(&ray);

                assert!(actual_intersections[0].t.approximate_equals(*t1));
                assert!(actual_intersections[1].t.approximate_equals(*t2));
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

                assert_eq!(cylinder.local_normal(&point, &Intersection::default()), expected_normal);
            }
        }

        it "The default minimum and maximum for a cylinder" {
            assert_eq!(cylinder.minimum, f64::NEG_INFINITY);
            assert_eq!(cylinder.maximum, f64::INFINITY);
        }

        it "Intersecting a constrained cylinder" {
            let cylinder = Arc::new(Cylinder {
                minimum: 1.0,
                maximum: 2.0,
                ..Cylinder::default()
            });

            let examples = [
                // origin      direction    count
                ((0, 1.5,  0), (0.1, 1, 0), 0), // diagonal ray from inside
                ((0, 3.0, -5), (0.0, 0, 1), 0), // perpendicular to the y axis
                ((0, 0.0, -5), (0.0, 0, 1), 0), // perpendicular to the y axis
                ((0, 2.0, -5), (0.0, 0, 1), 0), // edge case: doesn't touch the max
                ((0, 1.0, -5), (0.0, 0, 1), 0), // edge case: doesn't touch the min
                ((0, 1.5, -2), (0.0, 0, 1), 2), // perpendicular through mid-cylinder
            ];

            for ((ox, oy, oz), (dx, dy, dz), expected_count) in examples.iter() {
                let origin    = Tuple::point(*ox, *oy, *oz);
                let direction = Tuple::vector(*dx, *dy, *dz).normalize();
                let ray = Ray { origin, direction };

                let actual_intersections = Arc::clone(&cylinder).local_intersections(&ray);

                assert_eq!(actual_intersections.len(), *expected_count);
            }
        }

        it "The default closed value for a cylinder" {
            assert!(!cylinder.closed)
        }

        it "Intersecting the caps of a closed cylinder" {
            let examples = [
                // point      direction   count
                ((0,  3,  0), (0, -1, 0), 2),
                ((0,  3, -2), (0, -1, 2), 2),
                ((0,  4, -2), (0, -1, 1), 2), // corner case
                ((0,  0, -2), (0,  1, 2), 2),
                ((0, -1, -2), (0,  1, 1), 2), // corner case
            ];

            let cylinder = Arc::new(Cylinder {
                minimum: 1.0,
                maximum: 2.0,
                closed: true,
                ..Cylinder::default()
            });

            for ((ox, oy, oz), (dx, dy, dz), expected_count) in examples.iter() {
                let origin    = Tuple::point(*ox, *oy, *oz);
                let direction = Tuple::vector(*dx, *dy, *dz).normalize();
                let ray = Ray { origin, direction };

                let actual_intersections = Arc::clone(&cylinder).local_intersections(&ray);

                assert_eq!(actual_intersections.len(), *expected_count);
            }
        }

        it "The normal vector on a cylinder\'s end caps" {
            let cylinder = Arc::new(Cylinder {
                minimum: 1.0,
                maximum: 2.0,
                closed: true,
                ..Cylinder::default()
            });

            let examples = [
                // point        normal
                ((0.0, 1, 0.0), (0, -1, 0)),
                ((0.5, 1, 0.0), (0, -1, 0)),
                ((0.0, 1, 0.5), (0, -1, 0)),
                ((0.0, 2, 0.0), (0, 1, 0)),
                ((0.5, 2, 0.0), (0, 1, 0)),
                ((0.0, 2, 0.5), (0, 1, 0)),
            ];

            for ((px, py, pz), (nx, ny, nz)) in examples.iter() {
                let point = Tuple::point(*px, *py, *pz);
                let expected_normal = Tuple::vector(*nx, *ny, *nz);

                assert_eq!(cylinder.local_normal(&point, &Intersection::default()), expected_normal);
            }
        }
    }
}
