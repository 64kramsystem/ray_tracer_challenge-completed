use demonstrate::demonstrate;

demonstrate! {
    describe "Cone" {
        use crate::math::*;
        use crate::lang::math::sqrt;
        use crate::lang::ApproximateFloat64Ops;
        use crate::space::{*, shape::private::ShapeLocal};

        before {
            #[allow(unused_mut)]
            let mut cone = Cone::default();
        }

        it "Intersecting a cone with a ray" {
            let examples = [
                // origin     direction     t0       t1
                ((0, 0, -5), ( 0.0,  0, 1), 5.0,     5.0     ),
                ((0, 0, -5), ( 1.0,  1, 1), 8.66025, 8.66025 ),
                ((1, 1, -5), (-0.5, -1, 1), 4.55006, 49.44994),
            ];

            for ((ox, oy, oz), (dx, dy, dz), t1, t2) in examples.iter() {
                let origin    = Tuple::point(*ox, *oy, *oz);
                let direction = Tuple::vector(*dx, *dy, *dz).normalize();
                let ray = Ray { origin, direction };

                let actual_intersections = cone.local_intersections(&ray);

                assert_eq!(actual_intersections.len(), 2);
                assert!(actual_intersections[0].approximate_equals(*t1));
                assert!(actual_intersections[1].approximate_equals(*t2));
            }
        }


        it "Intersecting a cone with a ray parallel to one of its halves" {
            let origin    = Tuple::point(0, 0, -1);
            let direction = Tuple::vector(0, 1, 1).normalize();
            let ray = Ray { origin, direction };

            let actual_intersections = cone.local_intersections(&ray);

            assert_eq!(actual_intersections.len(), 1);
            assert!(actual_intersections[0].approximate_equals(0.35355));
        }

        it "Intersecting a cone\'s end caps" {
            let examples = [
                //  origin      direction  count
                // ((0, 0, -5.0),  (0, 1, 0), 0),
                // ((0, 0, -0.25), (0, 1, 1), 2),
                ((0, 0, -0.25), (0, 1, 0), 4),
            ];

            cone.minimum = -0.5;
            cone.maximum = 0.5;
            cone.closed = true;

            for ((ox, oy, oz), (dx, dy, dz), expected_count) in examples.iter() {
                let origin    = Tuple::point(*ox, *oy, *oz);
                let direction = Tuple::vector(*dx, *dy, *dz).normalize();
                let ray = Ray { origin, direction };

                let intersections = cone.local_intersections(&ray);

                assert_eq!(intersections.len(), *expected_count);
            }
        }

        it "Computing the normal vector on a cone" {

            let examples = [
                // point      normal
                (( 0,  0, 0), ( 0,      0.0, 0)),
                (( 1,  1, 1), ( 1, -sqrt(2), 1)),
                ((-1, -1, 0), (-1,      1.0, 0)),
            ];

            for ((px, py, pz), (nx, ny, nz)) in examples.iter() {
                let point = Tuple::point(*px, *py, *pz);
                let expected_normal = Tuple::vector(*nx, *ny, *nz);

                assert_eq!(cone.local_normal(&point), expected_normal);
            }
        }
    }
}
